use std::error;
use std::fmt;
use std::io;
use std::string::FromUtf8Error;


/// Common error type for errors  concerning loading and processing binary word vectors
///
/// This error type mostly wraps I/O and encoding errors, but also adds crate-specific error
/// variants.
#[derive(Debug)]
pub enum Word2VecError {
    Io(io::Error),
    Decode(FromUtf8Error),
    WrongHeader,
}

impl error::Error for Word2VecError {
    fn description(&self) -> &str {
        match *self {
            Word2VecError::Decode(ref err) => err.description(),
            Word2VecError::Io(ref err) => err.description(),
            Word2VecError::WrongHeader => "Wrong header format",
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Word2VecError::Decode(ref e) => e.source(),
            Word2VecError::Io(ref e) => e.source(),
            _ => None,
        }
    }
}

impl fmt::Display for Word2VecError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Word2VecError::Io(ref err) => write!(f, "IO error: {}", err),
            Word2VecError::Decode(ref err) => write!(f, "Decode error: {}", err),
            Word2VecError::WrongHeader => write!(f, "Wrong header length."),
        }
    }
}

impl From<io::Error> for Word2VecError {
    fn from(err: io::Error) -> Word2VecError {
        Word2VecError::Io(err)
    }
}

impl From<FromUtf8Error> for Word2VecError {
    fn from(err: FromUtf8Error) -> Word2VecError {
        Word2VecError::Decode(err)
    }
}

