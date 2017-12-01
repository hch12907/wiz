use std::fs::File;
use std::io::Read;
use std::path::Path;
use toml::Value as Toml;

use download;
use error::PackageError;
use list::RepositoryList;
use package::Package;

/// The struct which represents the repos.
#[derive(Serialize, Deserialize)]
pub struct Repository {
    url: String,
    packages: Vec<Package>,
}

/// Struct which represents a list of Repositories.
#[derive(Serialize, Deserialize)]
pub struct RepositoryUrlList(pub Vec<String>);

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

impl RepositoryUrlList {
    fn read_from(path: &Path) -> Result<Self, PackageError> {
        let mut list = File::open(path)?;
        let mut content = String::new();
        list.read_to_string(&mut content)?;
        let list = content.parse::<Toml>()?;
        let list = list.try_into::<Self>()?;
        Ok(list)
    }

    fn update(&self, path: &Path) -> Result<RepositoryList, PackageError> {
        let &RepositoryUrlList(ref url_list) = self;
        
        let mut repo_list = RepositoryList(Vec::new()); {
            let RepositoryList(ref mut vec) = repo_list;
            for url in url_list {
                let content = download::download_buf(&url)?;
                let content = String::from_utf8(content)?;
                let content = content.parse::<Toml>()?;
                let content = content.try_into::<Repository>()?;
                vec.push(content);
            }
        }
        Ok(repo_list)
    }
}