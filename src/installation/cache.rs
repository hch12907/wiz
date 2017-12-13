use std::fs::File;
use std::io::{ BufReader, Read };
use std::path::Path;
use toml::Value;

use error::PackageError;
use installation::PackageState;
use utils::Hash;

#[derive(Serialize, Deserialize)]
pub struct Cache(Vec<(Hash, PackageState)>);

impl Cache {
    fn read_from<P: AsRef<Path>>(path: P) -> Result<Cache, PackageError> {
        let mut file = File::open(path)?;
        let mut file = BufReader::new(file);
        let mut content = String::new();
        file.read_to_string(&mut content);
        let content = content.trim().parse::<Value>()?;
        Ok(content.try_into::<Self>()?)
    }   
}