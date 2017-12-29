/// An enum for the package types.
#[derive(Debug, Serialize,Deserialize)]
pub enum PackageType {
    /// Stands for binary
    Bin,
    /// Stands for library
    Lib,
    /// Stands for source
    Src,

    // the comments are really obvious, haha
}
