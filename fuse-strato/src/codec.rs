use std::mem::size_of;
use std::io::{Error, ErrorKind::*};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::ffi::OsStr;

use std::fmt::Debug;

use bytes::BytesMut;
use tokio::codec::Decoder;

use fuse_sys::abi::*;
use fuse_sys::abi::consts::*;
use fuse_sys::abi::fuse_opcode::*;

use crate::request::FuseRequest;


pub(crate) enum FuseRequestDecoder {
    DecodingHeader(),
    DecodingBody(usize),
}

impl FuseRequestDecoder {

    pub(crate) fn new() -> Self {
        FuseRequestDecoder::DecodingHeader()
    }

    pub(crate) fn init(&mut self) {
       *self = FuseRequestDecoder::DecodingHeader()
    }

}

impl Decoder for FuseRequestDecoder {

    type Item = FuseRequest;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<FuseRequest>, Error> {

        if src.len() > size_of::<fuse_in_header>() {
            return Err(Error::new(InvalidInput, "The header was to short"));
        }

        //let header = fetch::<fuse_in_header>(src);
//
//        match fuse_opcode::from_u32(header.opcode) {
//            _=> ()
//        }



        Err(Error::new(Other, "Unimplemented"))
    }

}


/// Helper functions
// These functions are based on the work of Andreas Neuhaus under MIT license
pub fn fetch<'a, T: Clone>(src: &mut BytesMut) -> T {
    let len = size_of::<T>();
    assert!(len <= src.len(), "out of data while fetching typed argument");

    // Return the data as the corresponding data type
    let mut bytes = src.split_to(len);

    let dst: &T = unsafe {std::mem::transmute(bytes.as_ptr())};
    dst.clone()
}

//pub fn fetch_str<'a>(src: &mut BytesMut) -> &'a OsStr {
//    let len = src.iter().position(|&c| c == 0)
//        .expect("Ran out of data while parsing string");
//
//    let bytes = src.split_to(len+1);
//    OsStr::new(str::from(&bytes))
//}
//
//pub fn fetch_path<'a>(src: &mut BytesMut) -> &'a Path {
//    Path::new(fetch_str(src))
//}

#[cfg(test)]
mod tests {
    #[test]
    fn fetch_test1() {
        use super::*;
        use bytes::{BytesMut, BufMut};

        #[derive(Debug, Clone, PartialEq)]
        struct TestStruct{
            buff: u32,
            er: u32,
        }


        let ts: TestStruct = {
            let mut buf = BytesMut::with_capacity(64);
            buf.put("Buffer  Test");
            buf.put("Test");

            let tmp: TestStruct = fetch(&mut buf);
            assert_eq!(buf.to_vec(), "TestTest".to_string().into_bytes());
            tmp
        };

        println!("{:x?}", ts);
        assert_eq!(ts, TestStruct { buff: 0x66667542, er: 0x20207265 });

    }
}
