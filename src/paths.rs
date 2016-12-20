// Here lists all the paths that will be used by wiz
use std::path::{ Path, PathBuf };

///
/// Appends `append` to `path`.
///
pub trait PathAppend {
    fn append(&self, input: &str) -> PathBuf;
}

impl PathAppend for PathBuf {
    fn append(&self, input: &str) -> PathBuf {
        let mut buf = self.to_path_buf().clone();
        buf.push(input);
        buf.clone()
    }
}

pub const DOWNLOAD_DIR: &'static str = "downloaded/";
pub const EXTRACT_DIR: &'static str = "extracted/";
pub const PACKAGE_INSTALL_DIR: &'static str = "installed_package/";
pub const PACKAGE_LIST_DIR: &'static str = "package_lists/";