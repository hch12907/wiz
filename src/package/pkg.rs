use std::error::Error;

//Following semver here
#[derive(Clone, Debug, PartialOrd, RustcDecodable, RustcEncodable)]
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
    pub packages: Vec<Package>,
    pub parent_url: String, // Examples: https://www.foo.bar
    pub version: u32
}

impl Version {
    pub fn new() -> Version {
        Version {
            major: 0u16, 
            minor: 0u16, 
            patch: 0u32
        }
    }

    pub fn from(major: u16, minor: u16, patch: u32) -> Version {
        Version {
            major: major,
            minor: minor,
            patch: patch
        }
    }

    pub fn from_str(input: &str) -> Result<Version, String> {
        let versions: Vec<&str> = input.split('.').collect();
        if versions.len() == 3 {
            return Ok(
                Version { 
                    major: get!(versions[0].parse::<u16>(),"Version: Unable to parse &str into u16 (major)"), 
                    minor: get!(versions[1].parse::<u16>(),"Version: Unable to parse &str into u16 (minor)"), 
                    patch: get!(versions[2].parse::<u32>(),"Version: Unable to parse &str into u32 (patch)") 
                }
            )
        } else {
            return Err("Version::from_str() cannot parse provided input into Version: invalid input".to_string())
        }
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Version) -> bool {
        self.major == other.major &&
        self.minor == other.minor &&
        self.patch == other.patch
    }
}

impl ToString for Version {
    fn to_string(&self) -> String {
        String::from(self.major.to_string() + "." + &self.minor.to_string() + "." + &self.minor.to_string())
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