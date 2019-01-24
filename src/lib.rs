#![feature(integer_atomics)]
extern crate parking_lot;
extern crate fuse;
extern crate libc;

pub mod engine;
pub mod driver;
pub mod handler;
pub mod controller;
pub mod link;

mod utils;

use std::sync::Arc;
use std::collections::BTreeMap;

use parking_lot::RwLock;

use crate::link::DirectoryEntry;
use crate::handler::ProtectedHandle;
use crate::controller::Controller;


pub use fuse::Request;


pub(crate) type Registry = Arc<RwLock<BTreeMap<u64, ProtectedHandle>>>;

//pub(crate) type NodeImpl = Box<dyn Node + Send + Sync>;
pub(crate) type FileImpl = Box<dyn File + Send + Sync>;
pub(crate) type DirImpl = Box<dyn Directory + Send + Sync>;


// TODO: Implement Error Types
// TODO: Clean up handler
// TODO: Implement own Request type which is Cloneable, and contains information about offset and size

/// This trait contains all the base functions, that need to be implemented for the object
/// to behave as a node in the file system.
pub trait Node {

    fn init(&mut self, _: Controller) {}

    fn read_attributes(&mut self, _: Controller, _: &Request, _: DirectoryEntry)
        -> Option<DirectoryEntry> {
        None
    }

}


pub trait Directory: Node {

    fn lookup(&mut self, _: Controller, _: &Request, _: String) -> Option<DirectoryEntry> {
        None
    }

    fn readdir(&mut self, _: Controller, _: &Request) -> Option<Vec<DirectoryEntry>> {
        None
    }

}

pub trait File: Node {

    fn read(&mut self, _: Controller, _: &Request) -> Option<Vec<u8>> {
        None
    }

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
