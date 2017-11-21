use std::error::Error;
use std::io;
use toml::de::Error as TomlDeserializeError;

/// The error type for the program's every operations, such as TOML Parsing
/// or IO-related errors. Wherever there's an error, it _should_ be wrapped
/// into this type.
pub enum PackageError {
    Parsing(String),
    IO(String),
}

impl From<io::Error> for PackageError {
    /// Converts an `io::Error` into this type.
    fn from(err: io::Error) -> Self {
        PackageError::IO(String::from("An IO error occured:") + err.description())
    }
}

impl From<TomlDeserializeError> for PackageError {
    /// Converts a `toml::de:Error` into this type.
    fn from(err: TomlDeserializeError) -> Self {
        let err_location = 
            err
                // Get the position of the error
                .line_col()
                // If it's not available, return a empty string.
                // If it is, then convert the position into a string.
                .map_or(String::from(""), |(line, col)| { 
                    format!("in Line: {}, Col: {}", 
                        line, 
                        col
                    )
                });

        let err_description = err.description();

        PackageError::Parsing(
            format!(
                "An error occured while parsing a TOML, {} due to: {}.",
                err_location, 
                err_description
            )
        )
    }
}