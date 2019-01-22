#![feature(integer_atomics)]
extern crate parking_lot;
extern crate fuse;
extern crate libc;

mod engine;
mod handler;
pub mod link;

use crate::link::Link;

pub trait Directory {

    fn readdir(&self) -> Option<Vec<Link>> {
        None
    }

}

pub trait File {

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
