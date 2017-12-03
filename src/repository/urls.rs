use std::fs::File;
use std::io::Read;
use std::path::Path;
use toml::Value as Toml;

use utils::download;
use error::PackageError;
use repository::{ Repository, RepositoryList };

/// Struct which represents a list of Repositories.
#[derive(Serialize, Deserialize)]
pub struct RepositoryUrlList(pub Vec<String>);

impl RepositoryUrlList {
    fn read_from(path: &Path) -> Result<Self, PackageError> {
        let mut list = File::open(path)?;
        let mut content = String::new();
        list.read_to_string(&mut content)?;
        let list = content.parse::<Toml>()?;
        let list = list.try_into::<Self>()?;
        Ok(list)
    }

    fn update(&self) -> Result<RepositoryList, PackageError> {
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