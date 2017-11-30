// In heavy WIP
use std::fs::{ self, File };
use std::io::Read;
use std::path::Path;
use toml;

use error::PackageError;
use repository::RepositoryUrlList;

/// Represents the config file.
#[derive(Deserialize)]
pub struct Config {
    pub buffer_size: Option<u64>,
    pub config_path: Option<String>,
    pub download_path: Option<String>,
    pub unpack_path: Option<String>,
    pub repository: Option<RepositoryUrlList>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            buffer_size: Some(65536),
            config_path: Some(String::from("~/.wiz/config/")),
            download_path: Some(String::from("~/.wiz/downloads/")),
            unpack_path: Some(String::from("~/.wiz/downloads/unpacked/")),
            repository: Some(RepositoryUrlList(Vec::new())),
        }
    }
}

impl Config {
    /// Read the config file from the config path specified in `path`.
    /// If the config file is read & parsed properly, it should return
    /// a `Config`.
    pub fn read_from<P: AsRef<Path>>(path: P) -> Result<Self, PackageError> {
        let path = path.as_ref(); // maybe I should replace &Path with these
        
        let mut content = String::new();

        // Check whether there are tons of configs in the path
        if path.is_dir() {
            // Read every config files and put them into a string, if it is
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let mut config = File::open(entry.path())?;
                config.read_to_string(&mut content)?;
            }
        } else {
            // Read the config file into a string, if it isn't
            let mut config = File::open(path)?;
            config.read_to_string(&mut content)?;
        }

        // Try to parse the string, and convert it into a `Config`.
        let config = content.parse::<toml::Value>()?;
        let config = config.try_into::<Self>()?;
        Ok(config)
    }

    
    /// Read the config file from the default config path.
    /// If the config file is read & parsed properly, it should return
    /// a `Config`.
    pub fn read_from_default() -> Result<Self, PackageError> {
        if let Self { config_path: Some(path), .. } = Self::default() {
            Self::read_from(&path)
        } else {
            // This should never, ever, happen.
            Err(PackageError::Parsing(
                String::from("Unable to get config_path from the default cfg.")
            ))
        }
    }

    /// This function sets the None(s) in the config, to the default values.
    pub fn fill_with_default(mut self) -> Self {
        // Destructuring the default configs into individual variables.
        let Self {
            buffer_size: buf_size,
            config_path: conf_path,
            download_path: dl_path,
            unpack_path: unpk_path,
            repository: repo,
        } = Self::default();

        /// Internal macro, to ease the implementation of `fill_with_default`.
        macro_rules! set_on_none {
            ($dest:ident, $key:ident, $value:expr) => (
                match $dest {
                    Self { $key: ref mut x, .. } => {
                        match x {
                            &mut Some(_) => {},
                            &mut None => *x = $value,
                        }
                    }
                }
                
                // Wait for RFC-2086 to be implemented first.
                /* 
                    [allow(irrefutable_let_pattern)]
                    if let Config { $key: mut x, .. } = $dest {
                        if let None = x { x = $value }
                    }
                */
            );
        };

        // If there are None(s), set them to the default value.
        set_on_none!(self, buffer_size, buf_size);
        set_on_none!(self, config_path, conf_path);
        set_on_none!(self, download_path, dl_path);
        set_on_none!(self, unpack_path, unpk_path);
        set_on_none!(self, repository, repo);

        self
    }
}
