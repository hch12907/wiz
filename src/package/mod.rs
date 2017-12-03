/// The module for `Package`.
mod package;
/// The module for `PackageType`.
mod package_type;
/// The module for `Version`.
mod version;

// Reexports
pub use self::package::Package as Package;
pub use self::version::Version as Version;
pub use self::package_type::PackageType as PackageType;