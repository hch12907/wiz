use std::cmp::Ordering;

use package::Version;
use package::PackageType;

/// A struct for storing package informations, such as its name and version.
#[derive(Serialize, Deserialize)]
pub struct Package {
    name: String,
    hash: u32,
    version: Version,
    package_type: PackageType,
    path: String, // relative path
    dependencies: Vec<Package>,
}

impl PartialEq for Package {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
        self.version == other.version
    }
}

impl Package {
    /// Returns the name of the package.
    pub fn name(&self) -> &str { 
        &self.name 
    }

    /// Returns the version of the package.
    pub fn version(&self) -> &Version {
        &self.version
    }

    /// Returns the hash of the package.
    pub fn hash(&self) -> u32 {
        self.hash
    }

    /// Returns the type of the package.
    pub fn package_type(&self) -> &PackageType {
        &self.package_type
    }
}