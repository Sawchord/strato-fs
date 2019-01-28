use std::error::Error;
use std::fmt::{Display, Formatter, Result};

use libc::*;

use self::NodeError::*;
use self::FileError::*;
use self::DirError::*;

use std::any::Any;

pub trait IsFileError {}
pub trait IsDirError {}
impl IsFileError for NodeError{}
impl IsDirError for NodeError{}
impl IsFileError for FileError{}
impl IsDirError for DirError{}


#[derive (Debug, Clone)]
pub enum NodeError {
    NotImplemented,
    NoSuchEntry,
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
            NoSuchEntry => "The requested node does not exist",
            IOError => "An I/O Error occurred",
            PermissionDenied => "The user does not have permission to execute the operation",
            TryAgain => "Try again later",
            ReadOnly => "This is a ReadOnly file system",
        }
    }
}

impl NodeError {

    pub fn new(val: NodeError) -> Self {
        val
    }

    pub(crate) fn get_libc_code(&self) -> i32 {
        match *self {
            NotImplemented => EPERM,
            NoSuchEntry => ENOENT,
            IOError => EIO,
            PermissionDenied => EACCES,
            TryAgain => EAGAIN,
            ReadOnly => EROFS,
        }
    }
}




#[derive (Debug, Clone)]
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

    pub fn new<T: Any + IsFileError>(val: T) -> Self {
        let val_ref = &val;
        let val_any = val_ref as &dyn Any;

        if let Some(node_err) = val_any.downcast_ref::<NodeError>() {
            FileNodeErr(node_err.clone())
        } else if let Some (file_err) = val_any.downcast_ref::<FileError>() {
            file_err.clone()
        } else {
            FileNodeErr(NotImplemented)
        }
    }


    pub(crate) fn get_libc_code(&self) -> i32 {
        match *self {
            FileNodeErr(ref inner) => inner.get_libc_code(),
            NoSuchFile => ENOENT,
            IsDirectory => EISDIR,
            FileExists => EEXIST,
        }
    }
}


#[derive (Debug, Clone)]
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

    pub fn new<T: Any + IsDirError>(val: T) -> Self {
        let val_ref = &val;
        let val_any = val_ref as &dyn Any;

        if let Some(node_err) = val_any.downcast_ref::<NodeError>() {
            DirNodeErr(node_err.clone())
        } else if let Some (dir_err) = val_any.downcast_ref::<DirError>() {
            dir_err.clone()
        } else {
            DirNodeErr(NotImplemented)
        }
    }


    pub(crate) fn get_libc_code(&self) -> i32 {
        match *self {
            DirNodeErr(ref inner) => inner.get_libc_code(),
            NoSuchDirectory => ENOENT,
            IsNotDirectory => ENOTDIR,
            DirectoryNotEmpty => ENOTEMPTY,
        }
    }
}

//#[cfg(test)]
//mod tests {
//    #[test]
//
//    fn description() {
//        use super::*;
//        let error1 = FileError::new(NoSuchFile);
//        let error2 = FileError::new(PermissionDenied);
//        let error3 = DirError::new(IsNotDirectory);
//        let error4 = DirError::new(ReadOnly);
//
//        println!("Error: {}", error1);
//        println!("Error: {}", error2);
//        println!("Error: {}", error3);
//        println!("Error: {}", error4);
//    }
//}