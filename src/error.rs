use std::error;
use std::fmt;
use std::io;

use crate::buf;

#[derive(Debug)]
pub enum Error {
	Unknown,
	General(&'static str),
	IO(io::Error),
	Parse(buf::Error),
}

impl fmt::Display for Error {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Error::Unknown        => write!(fmt, "Unknown error"),
			Error::General(ref s) => write!(fmt, "{}", s),
			Error::IO(ref e)      => write!(fmt, "IO error: {}", e),
			Error::Parse(ref e)   => write!(fmt, "Parsing error: {}", e),
		}
	}
}

impl error::Error for Error {
	fn cause(&self) -> Option<&dyn error::Error> {
		match *self {
			Error::Unknown      => None,
			Error::General(_)   => None,
			Error::IO(ref e)    => Some(e),
			Error::Parse(ref e) => Some(e),
		}
	}
}

impl From<&'static str> for Error {
	fn from(s: &'static str) -> Self {
		Error::General(s)
	}
}

impl From<io::Error> for Error {
	fn from(err: io::Error) -> Self {
		Error::IO(err)
	}
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
