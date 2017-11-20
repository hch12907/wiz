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
