use reqwest;
use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::string::FromUtf8Error;
use toml::de::Error as TomlDeserializeError;
use toml::ser::Error as TomlSerializeError;

/// The error type for the program's every operations, such as TOML Parsing
/// or IO-related errors. Wherever there's an error, it _should_ be wrapped
/// into this type.
#[derive(Debug)]
pub enum PackageError {
    Download(String),
    Parsing(String),
    Utf8(String),
    IO(String),
}

impl fmt::Display for PackageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::PackageError::*;
        match self {
            &Download(ref why) => write!(f, "An error occured during download: {}", why),
            &Parsing(ref why) => write!(f, "An error occured during TOML parsing: {}", why),
            &Utf8(ref why) => write!(f, "An error occured during UTF8 parsing: {}", why),
            &IO(ref why) => write!(f, "An error occured during an IO operation: {}", why),
        }
    }
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

impl From<TomlSerializeError> for PackageError {
    /// Converts a `toml::de:Error` into this type.
    fn from(err: TomlSerializeError) -> Self {
        PackageError::Parsing(
            String::from("TOML serialize error:") +
            err.description()
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

impl From<FromUtf8Error> for PackageError {
    fn from(err: FromUtf8Error) -> Self {
        let utf8_error = err.utf8_error();
        let errored_bytes = 
            err
            // Gets the invalid bytes of the string
            .as_bytes()
            // Convert the bytes into an iterator.
            .iter()
            // Reduce the array of bytes into a string.
            // Example output: `152, 84, 156, 78`
            .fold(String::from(""), |acc, cur| {
                acc + ", " + &cur.to_string()
            });

        PackageError::Utf8(
            String::new() +
            "description:         " + err.description() +
            "further description: " + utf8_error.description() +
            "bytes valid up to:   " + &utf8_error.valid_up_to().to_string() +
            "invalid bytes:       " + &errored_bytes
        )
    }
}