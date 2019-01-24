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
use std::time::Duration;
use std::collections::BTreeMap;

use parking_lot::RwLock;

use crate::link::DirectoryEntry;
use crate::handler::ProtectedHandle;
use crate::controller::Controller;


pub use fuse::Request;


pub(crate) type Registry = Arc<RwLock<BTreeMap<u64, ProtectedHandle>>>;

pub(crate) type FileImpl = Box<dyn File + Send + Sync>;
pub(crate) type DirImpl = Box<dyn Directory + Send + Sync>;

pub trait Directory {

    fn init(&mut self, controller: Controller) {}

    fn lookup(&mut self, controller: Controller, req: &Request, name: String)
        -> Option<DirectoryEntry> {
        None
    }

    fn readdir(&mut self, controller: Controller, req: &Request) -> Option<Vec<DirectoryEntry>> {
        None
    }

}

pub trait File {

    fn init(&mut self, controller: Controller) {}

    fn read(&mut self, controller: Controller, req: &Request) -> Option<Vec<u8>> {
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
