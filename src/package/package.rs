//Following semver here
#[derive(Clone, RustcDecodable, RustcEncodable)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub patch: u32
}

#[derive(Clone, RustcDecodable, RustcEncodable)]
pub struct Package {
    pub name: String,
    pub version: Version,
    pub crc32: u32,
    pub child_url: String, // Examples: /some_dir/something.exe
    pub dependencies: Vec<Package> 
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct PackageList {
    packages: Vec<Package>,
    parent_url: String, // Examples: https://www.foo.bar
    version: u32
}

impl PartialEq for Version {
    fn eq(&self, other: &Version) -> bool {
        self.major == other.major &&
        self.minor == other.minor &&
        self.patch == other.patch
    }
}

impl PartialEq for Package {
    fn eq(&self, other: &Package) -> bool {
        self.name == other.name &&
        self.version == other.version &&
        self.crc32 == other.crc32 &&
        self.dependencies == other.dependencies
    }
}