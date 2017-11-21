use std::path::Path;

use error::PackageError;

enum PackageDownload<'a> {
    HasDownloaded(&'a Path),
    YetDownloaded(&'a str)
}

impl<'a> PackageDownload<'a> {
    fn from_url(url: &'a str) -> Self {
        PackageDownload::YetDownloaded(url)
    }

    fn is_downloaded(&self) -> bool {
        match self {
            &PackageDownload::HasDownloaded(_) => true,
            _ => false
        }
    }
}