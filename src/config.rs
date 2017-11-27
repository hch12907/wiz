// In heavy WIP
use std::fs::File;
use std::io::Read;
use std::path::Path;
use toml;

use error::PackageError;

/// Represents the config file.
#[derive(Deserialize)]
pub struct Config {
    pub buffer_size: Option<u64>,
    pub config_path: Option<String>,
    pub download_path: Option<String>,
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

impl Config {
    /// Read the config file from the config path specified in `path`.
    /// If the config file is read & parsed properly, it should return
    /// a `Config`.
    pub fn read_from(path: &Path) -> Result<Self, PackageError> {
        // Read the config file into a string
        let mut config = File::open(path)?;
        let mut content = String::new();
        config.read_to_string(&mut content)?;

        // Try parsing the string, and convert it into a `Config`.
        let config = content.parse::<toml::Value>()?;
        let config = config.try_into::<Self>()?;
        Ok(config)
    }

    
    /// Read the config file from the default config path.
    /// If the config file is read & parsed properly, it should return
    /// a `Config`.
    pub fn read_from_default() -> Result<Self, PackageError> {
        if let Self { config_path: Some(path), .. } = Self::default() {
            Self::read_from(Path::new(&path))
        } else {
            // This should never, ever, happen.
            Err(PackageError::Parsing(
                String::from("Unable to get config_path from the default cfg.")
            ))
        }
    }

}