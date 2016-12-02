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
    pub url: String,
    pub dependencies: Vec<Package> 
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct PackageList {
    packages: Vec<Package>,
    version: i32
}
