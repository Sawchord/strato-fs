#![feature(integer_atomics)]
extern crate parking_lot;
extern crate fuse;
extern crate libc;

mod driver;
mod utils;


mod engine;
pub use crate::engine::Engine;

mod handler;
pub use crate::handler::Handle;

mod controller;
pub use crate::controller::Controller;

pub mod link;
pub mod error;

use std::sync::Arc;
use std::collections::BTreeMap;

use parking_lot::RwLock;

use crate::link::NodeEntry;

use crate::error::{NodeError, FileError, DirError};

pub use fuse::Request;


pub(crate) type Registry = Arc<RwLock<BTreeMap<u64, Handle>>>;

pub(crate) type FileImpl = Box<dyn File + Send + Sync>;
pub(crate) type DirImpl = Box<dyn Directory + Send + Sync>;


// TODO: Implement own Request type which is Cloneable, and contains information about offset and size

// TODO: F U T U R E S

/// This trait contains all the base functions, that need to be implemented for the object
/// to behave as a node in the file system.
pub trait Node {

    fn init(&mut self, _: Controller) {}

    fn read_attributes(&mut self, _: &Request, _: NodeEntry) -> Result<NodeEntry, NodeError> {
        Err(NodeError::new(NodeError::NotImplemented))
    }

}


pub trait Directory: Node {

    fn lookup(&mut self, _: &Request, _: String) -> Result<NodeEntry, NodeError> {
        Err(NodeError::new(NodeError::NotImplemented))
    }

    fn readdir(&mut self, _: &Request) -> Result<Vec<NodeEntry>, DirError> {
        Err(DirError::new(NodeError::NotImplemented))
    }

}

pub trait File: Node {

    fn read(&mut self, _: &Request) -> Result<Vec<u8>, FileError> {
        Err(FileError::new(NodeError::NotImplemented))
    }

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
