use std::error::Error;
use std::fmt::{Display, Formatter, Result};

use libc::*;

use self::NodeError::*;
use self::FileError::*;
use self::DirError::*;


#[derive (Debug)]
pub enum NodeError {
    NotImplemented,
    IOError,
    PermissionDenied,
    TryAgain,
    ReadOnly,
}

impl Display for NodeError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.description())
    }
}

impl Error for NodeError {
    fn description(&self) -> &str {
        match *self {
            NotImplemented => "The requested operation is not implemented",
            IOError => "An I/O Error occurred",
            PermissionDenied => "The user has no permission to execute this operation",
            TryAgain => "Try again later",
            ReadOnly => "This is a ReadOnly file system",
        }
    }
}

impl NodeError {
    pub(crate) fn get_libc_code(&self) -> i32 {
        match *self {
            NotImplemented => EPERM,
            IOError => EIO,
            PermissionDenied => EACCES,
            TryAgain => EAGAIN,
            ReadOnly => EROFS,
        }
    }
}




#[derive (Debug)]
pub enum FileError {
    FileNodeErr(NodeError),
    NoSuchFile,
    IsDirectory,
    FileExists,
}

impl Display for FileError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.description())
    }
}

impl Error for FileError {
    fn description(&self) -> &str {
        match *self {
            FileNodeErr(ref inner) => inner.description(),
            NoSuchFile => "The file does not exist",
            IsDirectory => "The requested object is a directory",
            FileExists => "The file already exists",
        }
    }
}


impl FileError {
    pub(crate) fn get_libc_code(&self) -> i32 {
        match *self {
            FileNodeErr(ref inner) => inner.get_libc_code(),
            NoSuchFile => ENOENT,
            IsDirectory => EISDIR,
            FileExists => EEXIST,
        }
    }
}


#[derive (Debug)]
pub enum DirError {
    DirNodeErr(NodeError),
    NoSuchDirectory,
    IsNotDirectory,
    DirectoryNotEmpty,
}


impl Display for DirError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.description())
    }
}

impl Error for DirError {
    fn description(&self) -> &str {
        match *self {
            DirNodeErr(ref inner) => inner.description(),
            NoSuchDirectory => "The directory does not exist",
            IsNotDirectory => "The requested object is not a directory",
            DirectoryNotEmpty => "The directory is not empty",
        }
    }
}


impl DirError {
    pub(crate) fn get_libc_code(&self) -> i32 {
        match *self {
            DirNodeErr(ref inner) => inner.get_libc_code(),
            NoSuchDirectory => ENOENT,
            IsNotDirectory => ENOTDIR,
            DirectoryNotEmpty => ENOTEMPTY,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]

    fn description() {
        use super::*;
        let error1 = IsNotDirectory;
        let error2 = FileNodeErr(PermissionDenied);

        println!("Error: {}", error1);
        println!("Error: {}", error2);
    }
}