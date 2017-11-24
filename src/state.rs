use std::path::Path;

/// Enum for representing the state of the package.
pub enum PackageState<'a> {    
    /// This is the initial state of a package. 
    NotDownloaded {
        /// The path where the package is going to be located at.
        location: &'a Path,
        /// The URL of where the package is going to be downloaded from.
        url: &'a str,
    },
    
    /// The state is switched to Downloaded when the download of the
    /// package is started and finished.
    Downloaded {
        /// Indicates whether the download process is interrupted.
        in_progress: bool,
        /// The path where the half-downloaded package is located at.
        location: &'a Path,
        /// The total file size of the package.
        file_size: u32,
    },

    /// The state is switched to Unpacked when the package is decompressed
    /// after it's been downloaded.
    Unpacked {
        /// The path where the unpacked package is located at.
        location: &'a Path,
        /// The total file size of the package.
        file_size: u32,
    },

    /// The final state of a package, which indicates that the unpacked package
    /// is moved into a directory and setup properly (i.e environment variables
    /// and other finishing touches)
    Installed {
        /// The path where the package is located at.
        location: &'a Path,
        /// The total file size of the package.
        file_size: u32,
    },
}

impl<'a> PackageState<'a> {
    /// Create a new PackageState.
    fn new(url: &'a str, location: &'a Path) -> Self {
        PackageState::NotDownloaded { location, url }
    }

    /// A bool to determine whether the package is downloaded.
    fn is_downloaded(&self) -> bool {
        match self {
            &PackageState::Downloaded { .. } => true,
            _ => false
        }
    }

    /// A bool to determine whether the package is downloaded.
    fn is_unpacked(&self) -> bool {
        match self {
            &PackageState::Unpacked { .. } => true,
            _ => false
        }
    }

    /// A bool to determine whether the package is downloaded.
    fn is_installed(&self) -> bool {
        match self {
            &PackageState::Installed { .. } => true,
            _ => false
        }
    }

    /// Returns the path of where the package is (or is going to be) located
    /// at.
    fn store_path(&self) -> &'a Path {
        match self {
            &PackageState::NotDownloaded { location: path, .. } => path,
            &PackageState::Downloaded { location: path, .. } => path,
            &PackageState::Unpacked { location: path, .. } => path,
            &PackageState::Installed { location: path, .. } => path,
        }
    }
}
