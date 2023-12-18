use std::{borrow::Cow, fmt::Display};

pub use static_stash_macros::StaticFiles;
extern crate self as static_stash;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct StaticFileMeta {
    pub content: &'static str,
    pub content_type: &'static str,
    pub filename: &'static str,
    pub last_modified: Option<u64>,
}

impl Display for StaticFileMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.last_modified {
            Some(timestamp) => f.write_fmt(format_args!("{}?v={}", self.filename, timestamp)),
            None => f.write_str(self.filename),
        }
    }
}

pub type Js = StaticFileMeta;
pub type Css = StaticFileMeta;
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
    pub fn last_modified(path: &str) -> Option<u64> {
        let path = format!("{}{}", std::env::var("CARGO_MANIFEST_DIR").unwrap(), path);
        let metadata = std::fs::metadata(path).unwrap();
        let modified = metadata.modified().ok()?;
        let last_modified = modified
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Some(last_modified)
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
        assert_eq!(
            static_files.test.to_string(),
            "/static/test.css?v=1702905741".to_owned()
        );
        assert_eq!(static_files.test.content_type, "text/javascript");
    }
}
