use std::fs::File;
use std::io::{ BufReader, BufWriter, Read, Write };
use std::path::Path;
use toml::{ to_string as toml_to_string, Value };

use error::PackageError;
use repository::Repository;

/// Caches all the repositories into a single list, so they can be accessed
/// everytime without downloading all of them again.
#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryList(pub Vec<Repository>);

impl RepositoryList {
    pub fn read_from<P: AsRef<Path>>(path: P) -> Result<Self, PackageError> {
        let file = File::open(path)?;
        let mut file = BufReader::new(file);
        let mut content = String::new();
        file.read_to_string(&mut content);
        let content = content.trim().parse::<Value>()?;
        Ok(content.try_into::<Self>()?)
    }

    pub fn write_to<P: AsRef<Path>>(&self, path: P) -> Result<(), PackageError> {
        let file = File::create(path)?;
        let mut file = BufWriter::new(file);
        let content = toml_to_string(self)?;
        Ok(file.write_all(content.as_bytes())?)
    }
}

impl Default for RepositoryList {
    fn default() -> Self {
        RepositoryList(Vec::new())
    }
}