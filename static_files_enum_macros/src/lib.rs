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

    let meta_fields = fields.iter().map(
        |StaticFileField {
             file,
             ident,
             r#type,
         }| {
            quote! {
                #ident: StaticFileMeta {
                    content: include_str!(concat!(env!("CARGO_MANIFEST_DIR"), #file)),
                    content_type: #r#type,
                    filename: #file
                }
            }
        },
    );

    let get_matches = fields.iter().map(|StaticFileField { file, ident, .. }| {
        quote! {
            #file => Some(self.#ident)
        }
    });

    Ok(quote! {
        impl #struct_ident {
            fn new() -> Self {
                Self {
                    #(#meta_fields,)*
                }
            }
        }

        impl StaticFiles for #struct_ident {
            fn get(&self, uri: &str) -> Option<StaticFileMeta> {
                match uri {
                    #(#get_matches,)*
                    _ => None
                }
            }
        }
    })
}

#[derive(Clone)]
struct StaticFileField {
    file: LitStr,
    ident: Ident,
    r#type: &'static str,
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
        let r#type = match value.ty {
            Type::Path(TypePath { path, .. }) => {
                if let Some(path_segment) = path.segments.first() {
                    match path_segment.ident.to_string().as_str() {
                        "Js" => "text/javascript",
                        "Css" => "text/css",
                        _ => panic!("Unsupported content type, try Js or Css"),
                    }
                } else {
                    panic!("Unsupported content type, try Js or Css")
                }
            }
            _ => todo!(),
        };

        Self {
            file,
            ident,
            r#type,
        }
    }
}
