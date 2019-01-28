use std::fmt;
use std::error::Error;

#[derive(Clone, Debug)]
pub(crate) struct LibcError {
    error : usize,
}

impl LibcError {
    pub(crate) fn new(error: usize) -> Self {
        LibcError {error}
    }
}

impl fmt::Display for LibcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LibC Error: {}", self.error)
    }
}

impl Error for LibcError {
    fn description(&self) -> &str {
        "The filesytem operation could not be carried out and an error was returned"
    }
}