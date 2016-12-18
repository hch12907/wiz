// Here lists all the paths that will be used by wiz
use std::path::{ Path, PathBuf };

///
/// Appends `append` to `path`.
///
pub trait PathAppend {
    fn append(&mut self, input: &str) -> &Path;
}

impl PathAppend for PathBuf {
    fn append(&mut self, input: &str) -> &Path {
        self.push(input);
        self.as_path()
    }
}

pub const PACKAGE_LIST_DIR: &'static str = "package_lists/";