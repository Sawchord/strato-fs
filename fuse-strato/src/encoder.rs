use std::fmt::Debug;
use std::mem::size_of;
use std::io::{Error, ErrorKind::*};

use bytes::BytesMut;
use bytes::BufMut;

use tokio::codec::Encoder;

use fuse_sys::abi::*;
use fuse_sys::abi::consts::*;
use fuse_sys::abi::fuse_opcode::*;

use crate::response::FuseResponse;
use crate::response::FuseResponseBody::*;

pub(crate) struct FuseResponseEncoder;

impl FuseResponseEncoder {
    pub fn new() -> Self {
        FuseResponseEncoder
    }
}

impl Encoder for FuseResponseEncoder {

    type Item = FuseResponse;
    type Error = Error;

    fn encode(&mut self, item: FuseResponse, dst: &mut BytesMut) -> Result<(), Error> {


        // If the header contains an error, we will never return a body.
        // Simply write out the header and finish
        if item.get_header().error != 0 {
            dst.reserve(size_of::<fuse_out_header>());
            let mut header = item.get_header().to_owned();
            header.error = -header.error;
            dst.put_slice(as_u8_slice(&header));
            return Ok(())
        }

        // Otherwise we need to check the body in order to know how to behave
        match item.get_body() {

            // These are the responses, that do not have a response body/
            // The header is simply written out
            Init() | Destroy()
            => {
                dst.reserve(size_of::<fuse_out_header>());
                dst.put_slice(as_u8_slice(item.get_header()));
                ()
            },


            Interrupt() =>
                return Err(Error::new(Other, "Interrupting is not implemented")),


            // These are the responses that respond with an Entry
            // We need to write the header as well as the the entry
            Lookup(body) => {
                dst.reserve(size_of::<fuse_out_header>() + size_of::<fuse_entry_out>());
                dst.put_slice(as_u8_slice(item.get_header()));
                dst.put_slice(as_u8_slice(body));
            },


            _ => return Err(Error::new(Other, "This Response is unimplemented")),
        }

        Ok(())
    }

}

fn as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    use std::slice::from_raw_parts;
    unsafe {
        from_raw_parts(p as *const T as *const u8, size_of::<T>())
    }
}


#[cfg(test)]
mod tests {
    use std::mem::size_of;
    use std::time::{SystemTime, Duration};

    use bytes::{BytesMut, BufMut};

    use fuse_sys::abi::*;
    use fuse_sys::abi::consts::*;
    use fuse_sys::abi::fuse_opcode::*;

    use super::*;
    use crate::file::{FileAttr, FileType};

    fn create_fuse_header(error: i32, len: u32) -> fuse_out_header {
        use rand::random;
        fuse_out_header{
            len,
            error,
            unique: random(),
        }
    }

    fn build_fuse_header_from_body<T>(error: i32, _body: &T) -> fuse_out_header {
        let len = size_of::<fuse_in_header>() + size_of::<T>();
        create_fuse_header(error, len as u32)
    }


    /// Generate a duration withing a scope, given in seconds
    fn random_duration(scope: u64) -> Duration {
        use rand::random;
        Duration::new(random::<u64>() % scope, random::<u32>() % 1000000)
    }

    fn random_file_attr(kind: FileType) -> FileAttr {
        use rand::random;
        FileAttr {
            ino: random(),
            size: random(),
            blocks: random(),
            atime: SystemTime::now() + random_duration(1000),
            mtime: SystemTime::now() + random_duration(1000),
            ctime: SystemTime::now() + random_duration(1000),
            crtime: SystemTime::now() + random_duration(1000),
            kind,
            perm: random(),
            nlink: random(),
            uid: random(),
            gid: random(),
            rdev: random(),
            flags: random(),
        }
    }


    fn build_entry_out() -> fuse_entry_out {
        use rand::random;
        use crate::file::fuse_attr_from_attr;
        fuse_entry_out{
            nodeid: random(),
            generation: random(),
            entry_valid: random(),
            attr_valid: random(),
            entry_valid_nsec: random(),
            attr_valid_nsec: random(),
            attr: fuse_attr_from_attr(&random_file_attr(FileType::RegularFile)),
        }
    }

    #[test]
    fn lookup() {
        let mut buf = BytesMut::with_capacity(128);
        let mut encoder = FuseResponseEncoder::new();

        let entry_out = build_entry_out();
        let header = build_fuse_header_from_body(0, &entry_out);
        let body = Lookup(entry_out.clone());

        let response = FuseResponse::new(
            header.clone(),
            body,
        );

        encoder.encode(response, &mut buf);

        hexdump::hexdump(&buf);

        assert_eq!(0, 1);
    }

}