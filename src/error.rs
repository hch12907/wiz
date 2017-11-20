use std::io;

/// The error type for the program's every operations, such as TOML Parsing
/// or IO-related errors. Wherever there's an error, it _should_ be wrapped
/// into this type.
pub enum Error {
    Parsing(String),
    IO(String),
}

impl From<io::Error> for Error {
    /// Converts an `io::Error` into this type.
    fn from(err: io::Error) -> Self {
        Error::IO(String::from("An IO error occured:") + err.description())
    }
}