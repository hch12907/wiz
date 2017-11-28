use std::fs::File;
use std::io::Read;
use std::path::Path;
use toml::Value as Toml;

use error::PackageError;
use package::Package;

/// The struct which represents the repos.
#[derive(Deserialize)]
pub struct Repository {
    url: String,
    packages: Vec<Package>,
}

/// Struct which represents a list of Repositories.
#[derive(Deserialize)]
struct RepositoryList(Vec<Repository>);

impl Repository {
    /// Returns the URL of the Repository.
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Returns the packages which are provided by the Repository.
    pub fn packages(&self) -> &[Package] {
        self.packages.as_slice()
    }

    // Attempts to find the package which satifies the predicate.
    // If there are no packages that match the predicate, a `None` is returned.
    pub fn find_by_predicate<F>(&self, predicate: F) -> Option<&Package> 
        where F: Fn(&Package) -> bool {
        for pkg in &self.packages {
            if predicate(&pkg) {
                return Some(&pkg)
            }
        };

        None
    }
}

impl RepositoryList {
    fn read_from(path: &Path) -> Result<Self, PackageError> {
        let mut list = File::open(path)?;
        let mut content = String::new();
        list.read_to_string(&mut content)?;
        let list = content.parse::<Toml>()?;
        let list = list.try_into::<Self>()?;
        Ok(list)
    }
}