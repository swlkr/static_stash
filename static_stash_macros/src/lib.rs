use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Field, Ident, LitStr, Result, Type, TypePath};

#[proc_macro_derive(StaticFiles, attributes(file))]
pub fn static_files(s: TokenStream) -> TokenStream {
    let input = parse_macro_input!(s as DeriveInput);
    match static_files_macro(input) {
        Ok(s) => s.to_token_stream().into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn static_files_macro(input: DeriveInput) -> Result<TokenStream2> {
    let struct_ident = input.ident;
    let Data::Struct(data) = input.data else {
        panic!("Only structs are supported");
    };

    let fields = data
        .fields
        .into_iter()
        .map(|variant| StaticFileField::from(variant))
        .collect::<Vec<_>>();

    let consts= fields.iter().map(|StaticFileField { file, bytes_ident, hash_ident, .. }| {
        quote! {
            const #bytes_ident: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), #file));
            const #hash_ident: u64 = Self::hash(Self::#bytes_ident);
        }
    });

    let struct_fields = fields.iter().map(
        |StaticFileField {
             file,
             ident,
             hash_ident,
             ..
         }| {
            quote! {
                #ident: format!("{}?v={}", #file, Self::#hash_ident)
            }
        },
    );

    let get_matches = fields.iter().map(
        |StaticFileField {
             content_type,
             file,
             bytes_ident,
             ..
         }| {
            quote! {
                #file => {
                    Some((#content_type, Self::#bytes_ident))
                }
            }
        },
    );

    Ok(quote! {
        impl #struct_ident {
            #(#consts)*

            pub fn new() -> Self {
                Self {
                    #(#struct_fields,)*
                }
            }

            pub fn get<'a, 'b>(uri: &'a str) -> Option<(&'b str, &'static [u8])> {
                match uri {
                    #(#get_matches,)*
                    _ => None
                }
            }

            pub const fn hash(bytes: &[u8]) -> u64 {
                let mut hash = 0xcbf29ce484222325;
                let prime = 0x00000100000001B3;
                let mut i = 0;

                while i < bytes.len() {
                    hash ^= bytes[i] as u64;
                    hash = hash.wrapping_mul(prime);
                    i += 1;
                }

                hash
            }
        }
    })
}

#[derive(Clone)]
struct StaticFileField {
    file: LitStr,
    ident: Ident,
    bytes_ident: Ident,
    hash_ident: Ident,
    content_type: &'static str,
}

impl From<Field> for StaticFileField {
    fn from(value: Field) -> Self {
        let ident = value.ident.expect("Named fields only");
        let file = value
            .attrs
            .into_iter()
            .filter_map(|attr| attr.parse_args::<LitStr>().ok())
            .last()
            .expect("should be #[file]");
        let bytes_ident = Ident::new(
            &format!("{}_BYTES", ident.to_string().to_uppercase()),
            ident.span(),
        );
        let hash_ident = Ident::new(
            &format!("{}_HASH", ident.to_string().to_uppercase()),
            ident.span(),
        );
        let content_type = match value.ty {
            Type::Path(TypePath { path, .. }) => {
                if let Some(path_segment) = path.segments.last() {
                    match path_segment.ident.to_string().as_str() {
                        "Js" => "text/javascript",
                        "Css" => "text/css",
                        "Wasm" => "application/wasm",
                        _ => panic!("Unsupported content type, try Js, Css or Wasm"),
                    }
                } else {
                    panic!("Unsupported content type, try Js or Css")
                }
            }
            _ => unimplemented!(),
        };

        Self {
            file,
            ident,
            content_type,
            bytes_ident,
            hash_ident,
        }
    }
}
