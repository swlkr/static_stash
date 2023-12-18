use std::{borrow::Cow, fmt::Display};

pub use static_files_enum_macros::StaticFiles;
extern crate self as static_files_enum;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct StaticFileMeta {
    pub content: &'static str,
    pub content_type: &'static str,
    pub filename: &'static str,
}

impl Display for StaticFileMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.filename)
    }
}

pub type Js = StaticFileMeta;
pub type Css = StaticFileMeta;

pub trait StaticFiles {
    fn get(&self, uri: &str) -> Option<StaticFileMeta>;
}

type CowStr = Cow<'static, str>;

impl Into<CowStr> for StaticFileMeta {
    fn into(self) -> CowStr {
        Cow::Borrowed(self.filename)
    }
}

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
        let contents = static_files.get(uri).unwrap().content;
        assert_eq!(contents, "/* this is test.css */");
        assert_eq!(static_files.test.to_string(), "/static/test.css".to_owned());
        assert_eq!(static_files.test.content_type, "text/javascript");
    }
}
