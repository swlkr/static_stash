use std::{borrow::Cow, fmt::Display};

pub use static_stash_macros::StaticFiles;
extern crate self as static_stash;

#[derive(Clone, PartialEq, Debug)]
pub struct StaticFileMeta {
    pub content: Vec<u8>,
    pub content_type: &'static str,
    pub filename: &'static str,
}

impl Display for StaticFileMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}?v={}", self.filename, self.hash()))
    }
}

pub type Js = StaticFileMeta;
pub type Css = StaticFileMeta;
pub type Wasm = StaticFileMeta;
pub type Octet = StaticFileMeta;

pub trait StaticFiles {
    fn get(&self, uri: &str) -> Option<StaticFileMeta>;
}

type CowStr = Cow<'static, str>;

impl Into<CowStr> for StaticFileMeta {
    fn into(self) -> CowStr {
        Cow::Owned(self.to_string())
    }
}

impl StaticFileMeta {
    pub fn hash(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        self.content.hash(&mut hasher);
        let hash_value = hasher.finish();

        hash_value
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
        assert_eq!(
            std::str::from_utf8(&contents).unwrap(),
            "/* this is test.css */"
        );
        assert_eq!(
            static_files.test.to_string(),
            "/static/test.css?v=5030498852996062457".to_owned()
        );
        assert_eq!(static_files.test.content_type, "text/javascript");
    }
}
