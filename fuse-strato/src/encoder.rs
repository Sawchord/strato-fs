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

        dst.reserve(size_of::<fuse_out_header>());
        dst.put_slice(as_u8_slice(item.get_header()));

        // If the header contains an error, we will not return a body and are done
        if item.get_header().error != 0 {
            return Ok(())
        }

        match item.get_body() {
            Init() => (),
            Destroy() => (),
            Interrupt() => return Err(Error::new(Other, "Interrupting is not implemented")),
            Lookup(bod) => {
                dst.reserve(size_of::<fuse_entry_out>());
                dst.put_slice(as_u8_slice(bod));
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
    


}