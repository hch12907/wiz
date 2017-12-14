use std::path::Path;

use config::Config;
use installation::{ Cache, PackageState };

/// Gets the config, fill it with default values, and return it.
fn get_config<P: AsRef<Path>>(path: P) -> Config {
    Config::read_from(path)
        .map(|x| x.fill_with_default())
        .unwrap()
}

/// Gets the cached repositories.
fn get_cache<P: AsRef<Path>>(path: P) -> Cache {
    Cache::read_from(path).unwrap()
}