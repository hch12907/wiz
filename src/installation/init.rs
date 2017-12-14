use std::path::{ Path, PathBuf };

use config::Config;
use installation::{ Cache, PackageState };
use repository::RepositoryList;

/// Gets the config, fill it with default values, and return it.
pub fn get_config<P: AsRef<Path>>(path: P) -> Config {
    Config::read_from(path)
        .map(|x| x.fill_with_default())
        .unwrap()
}

/// Gets the list of installed packages.
pub fn get_cache<P: AsRef<Path>>(path: P) -> Cache {
    Cache::read_from(path).unwrap()
}

pub fn get_repositories<P: AsRef<Path>>(path: P) -> RepositoryList {
    RepositoryList::read_from(path).unwrap()
}