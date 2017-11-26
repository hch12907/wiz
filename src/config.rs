// In heavy WIP
use std::fs::File;
use std::io::Read;
use toml;

use error::PackageError;

#[derive(Deserialize)]
struct Config {
    buffer_size: Option<u64>,
    config_path: Option<String>,
    download_path: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            buffer_size: Some(65536),
            config_path: Some(String::from("~/.wiz/config/")),
            download_path: Some(String::from("~/.wiz/downloads/")),
        }
    }
}

