use reqwest;
use std::error::Error as StdError;
use std::io;
use toml::de::Error as TomlDeserializeError;

/// The error type for the program's every operations, such as TOML Parsing
/// or IO-related errors. Wherever there's an error, it _should_ be wrapped
/// into this type.
pub enum PackageError {
    Download(String),
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

impl From<reqwest::Error> for PackageError {
    fn from(err: reqwest::Error) -> PackageError {
        let related_url = 
            err
                // Try to retrieve the final stop if an error occured.
                .url()
                // If there's none, return "none"
                .map_or("none", |url| { 
                    url.as_str()
                });

        let status_code = 
            &err
                // Try to retrieve the status code if the error was generated
                // from an response.
                .status()
                // If there's none, return "none"
                .map_or(String::from("none"), |code| { 
                    code.as_u16().to_string()
                });

        PackageError::Download(
            String::new() +
            "related url:           " + related_url +
            "http related:          " + &err.is_http().to_string() +
            "serialization related: " + &err.is_serialization().to_string() +
            "redirect related:      " + &err.is_redirect().to_string() +
            "client related:        " + &err.is_client_error().to_string() +
            "server related:        " + &err.is_server_error().to_string() +
            "status code:           " + status_code +
            "other messages:        " + err.description()
        )
    }
}