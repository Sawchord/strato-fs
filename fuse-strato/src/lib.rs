#![allow(unused_imports, dead_code)]

#[macro_use]
extern crate log;


mod error;
mod codec;
mod request;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
