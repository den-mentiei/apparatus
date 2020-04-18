use std::error;
use std::fmt;
use std::io;

use crate::buf;

#[derive(Debug)]
pub enum ApsError {
	Unknown,
	General(&'static str),
	IO(io::Error),
	Parse(buf::Error),
}

impl fmt::Display for ApsError {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			ApsError::Unknown        => write!(fmt, "Unknown error"),
			ApsError::General(ref s) => write!(fmt, "{}", s),
			ApsError::IO(ref e)      => write!(fmt, "IO error: {}", e),
			ApsError::Parse(ref e)   => write!(fmt, "Parsing error: {}", e),
		}
	}
}

impl error::Error for ApsError {
	fn cause(&self) -> Option<&dyn error::Error> {
		match *self {
			ApsError::Unknown      => None,
			ApsError::General(_)   => None,
			ApsError::IO(ref e)    => Some(e),
			ApsError::Parse(ref e) => Some(e),
		}
	}
}

impl From<&'static str> for ApsError {
	fn from(s: &'static str) -> Self {
		ApsError::General(s)
	}
}

impl From<io::Error> for ApsError {
	fn from(err: io::Error) -> Self {
		ApsError::IO(err)
	}
}

pub type Result<T, E = ApsError> = std::result::Result<T, E>;
