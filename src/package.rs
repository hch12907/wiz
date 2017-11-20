use std::cmp::Ordering;

enum PackageType {
    Bin, // Binary
    Lib, // Library
    Src, // Source
}

struct Version {
    major: u32,
    minor: u32,
    patch: u32,
    additional: Option<String>,
}

struct Package {
    name: String,
    hash: u32,
    version: Version,
    package_type: PackageType,
    dependencies: Vec<Package>, 
}

struct Repository {
    url: String,
    packages: Vec<Package>,
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Eq for Version {}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    /// This compares one `Version` with another. It only compares the
    /// three main parts of Version (`major`, `minor`, `patch`) and ignores
    /// `additional` as it is meant to let the package host to put additional
    /// messages such as `alpha`, `beta`.
    fn cmp(&self, other: &Self) -> Ordering {
        if self.major > other.major { Ordering::Greater } else
        if self.major < other.major { Ordering::Less } 
        else {
            if self.minor > other.minor { Ordering::Greater } else
            if self.minor < other.minor { Ordering::Less }
            else {
                if self.patch > other.patch { Ordering::Greater } else
                if self.patch < other.patch { Ordering::Less } 
                else { Ordering::Equal }
            }
        }
    }
}
