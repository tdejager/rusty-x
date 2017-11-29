use std::error::Error as StdError;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    FileError(io::Error),
    InternalError
}

/// Implement display for error type
impl fmt::Display for Error {  
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::FileError(ref e) => write!(f, "FileError: {}", e),
            Error::InternalError => f.write_str("InternalServerError"),
        }
    }
}

/// This makes it an actual error
impl StdError for Error {  
    fn description(&self) -> &str {
        match *self {
            Error::FileError(ref e) => e.description(),
            Error::InternalError => "Internal processing error",
        }
    }
}
/// Conversion from default error to custom ones
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::FileError(err)
    }
}

