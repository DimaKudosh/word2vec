use std::fmt;
use std::io;
use std::string::FromUtf8Error;


#[derive(Debug)]
pub enum Word2VecError {
    Io(io::Error),
    Decode(FromUtf8Error),
    WrongHeader,
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
