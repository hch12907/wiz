use std::path::Path;

use error::PackageError;

/// Enum for representing the state of the package downloads.
pub enum PackageDownload<'a> {
    /// The state is switched to IsFinished when the download of the
    /// package is finished, leaving only a `&Path` which is the location
    /// of the downloaded package.
    IsFinished(&'a Path),
    /// The state is switched to InProgress when the download of the
    /// package is started, the `&Path` is the location where the
    /// half-downloaded package is located at, while the `u32` is the progress
    /// of the current download process.
    InProgress(&'a Path, u32),
    /// This is the initial state of PackageDownload, where the package is yet
    /// to be downloaded. It has a `&str` for storing the URL, and a `&Path`
    /// of which the package is going to be located at, when the package is
    /// downloaded.
    NotStarted(&'a str, &'a Path)
}

impl<'a> PackageDownload<'a> {
    /// Create a new PackageDownload.
    fn new(url: &'a str, path: &'a Path) -> Self {
        PackageDownload::NotStarted(url, path)
    }

    /// A bool to determine whether the package is downloaded.
    fn is_downloaded(&self) -> bool {
        match self {
            &PackageDownload::IsFinished(_) => true,
            _ => false
        }
    }

    /// Returns the path of where the package is (or is going to be) located
    /// at.
    fn store_path(&self) -> &'a Path {
        match self {
            &PackageDownload::IsFinished(ref path) => path,
            &PackageDownload::InProgress(ref path, _) => path,
            &PackageDownload::NotStarted(_, ref path) => path,
        }
    }
}