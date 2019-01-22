#![feature(integer_atomics)]
extern crate evmap;
extern crate parking_lot;
extern crate fuse;

mod engine;
mod handler;


pub trait Directory {

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
