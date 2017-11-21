use std::path::Path;

use error::PackageError;

/// Enum for representing the state of the package downloads.
pub enum PackageDownload<'a> {
    /// The state is switched to IsFinished when the download of the
    /// package is finished.
    IsFinished {
        /// The path where the downloaded package is located at.
        location: &'a Path,
        /// The total file size of the package.
        file_size: u32,
    },

    /// The state is switched to InProgress when the download of the
    /// package is started.
    InProgress {
        /// The path where the half-downloaded package is located at.
        location: &'a Path,
        /// The progress of the current download process.
        progress: u32,
        /// The total file size of the package.
        file_size: u32,
    },

    /// This is the initial state of PackageDownload, where the package is yet
    /// to be downloaded.
    NotStarted {
        /// The path where the package is going to be located at.
        location: &'a Path,
        /// The URL of where the package is going to be downloaded from.
        url: &'a str,
    }
}

impl<'a> PackageDownload<'a> {
    /// Create a new PackageDownload.
    fn new(url: &'a str, location: &'a Path) -> Self {
        PackageDownload::NotStarted { location, url }
    }

    /// A bool to determine whether the package is downloaded.
    fn is_downloaded(&self) -> bool {
        match self {
            &PackageDownload::IsFinished { location: _, file_size: _ } => true,
            _ => false
        }
    }

    /// Returns the path of where the package is (or is going to be) located
    /// at.
    fn store_path(&self) -> &'a Path {
        match self {
            &PackageDownload::IsFinished { location: path, file_size: _ } => path,
            &PackageDownload::InProgress { location: path, progress: _ , file_size: _} => path,
            &PackageDownload::NotStarted { location: path, url: _ } => path,
        }
    }
}