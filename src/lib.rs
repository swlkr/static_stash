pub use static_stash_macros::StaticFiles;
extern crate self as static_stash;

pub type Js = String;
pub type Css = String;
pub type Wasm = String;
pub type Octet = String;

#[cfg(test)]
mod tests {
    use crate::*;

    #[derive(StaticFiles, Debug, PartialEq)]
    struct StaticFile {
        #[file("/static/test.css")]
        test: Js,
    }

    #[test]
    fn it_works() {
        let static_files = StaticFile::new();
        let uri = "/static/test.css";
        let (content_type, bytes) = StaticFile::get(uri).unwrap();
        assert_eq!(
            std::str::from_utf8(&bytes).unwrap(),
            "/* this is test.css */"
        );
        assert_eq!(static_files.test, "/static/test.css?v=13915677119532516144");
        assert_eq!(content_type, "text/javascript");
    }
}
