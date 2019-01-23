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
use crate::handler::Handler;
use crate::controller::Controller;

pub(crate) type Registry = Arc<RwLock<BTreeMap<u64, Arc<Handler>>>>;


pub trait Directory {

    fn readdir(&self, controller: Controller, name: String) -> Option<Vec<DirectoryEntry>> {
        None
    }

}

pub trait File {

    fn read(&self,controller: Controller) -> Option<Vec<u8>> {
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
