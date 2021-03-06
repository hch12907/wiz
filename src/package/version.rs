use std::cmp::Ordering;

/// A struct made for storing the version of the packages.
/// 
/// The format follows [semver 2.0](http://semver.org/), meaning that:
/// 
/// - versions with different `major` indicates that they are incompatible.
/// - versions with different `minor` but same `major` are largely compatible.
/// - versions with different `patch` but same `major` and `minor` are 
///   compatible.
/// 
/// The `additional` field is an optional field for the package host to put
/// text such as `alpha` or `stable`, it's not involved in the comparisons.
#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
    additional: Option<String>,
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
    /// 
    /// This functions compares the two `Version`s *absolutely*, meaning that
    /// it doesn't care about whether the two versions are breaking-changes or
    /// not - it just compares.
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

impl Version {
    /// Returns the major version X (X.y.z).
    pub fn major(&self) -> u32 {
        self.major
    }

    /// Returns the minor version Y (x.Y.z).
    pub fn minor(&self) -> u32 {
        self.minor
    }

    /// Returns the patch version Z (x.y.Z).
    pub fn patch(&self) -> u32 {
        self.patch
    }

    /// Returns the additional message added by package host or its developers.
    pub fn additional(&self) -> &Option<String> {
        &self.additional
    }

    /// A helper function for checking whether the two versions contains
    /// breaking changes. 
    pub fn is_back_compatible(&self, other: &Self) -> bool {
        self.major == other.major
    }
}
