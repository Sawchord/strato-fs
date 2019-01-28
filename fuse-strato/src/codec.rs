use std::mem::size_of;
use std::io::{Error, ErrorKind::*};
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;
use std::ffi::{OsStr, OsString};

use std::fmt::Debug;

use bytes::BytesMut;
use tokio::codec::Decoder;

use fuse_sys::abi::*;
use fuse_sys::abi::consts::*;
use fuse_sys::abi::fuse_opcode::*;

use crate::request::{FuseRequest, FuseRequestBody};
use crate::request::FuseRequestBody::*;

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

macro_rules! req {($header: ident, $body: ident)
    => [Ok(Some(FuseRequest::new($header, $body)))]
}

impl Decoder for FuseRequestDecoder {

    type Item = FuseRequest;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<FuseRequest>, Error> {

        if src.len() > size_of::<fuse_in_header>() {
            return Err(Error::new(InvalidInput, "The header was to short"));
        }

        let header = fetch::<fuse_in_header>(src);

        let opcode = match fuse_opcode::from_u32(header.opcode) {
            None => {
                warn!("Unknown FUSE operation {} ... skipped", header.opcode);
                return Err(Error::new(InvalidInput, "Unknown FUSE opcode"));
            },
            Some (op) => op,
        };

        match opcode {

            FUSE_INIT => {
                let body = Init(fetch(src));
                req!(header, body)
            },
            FUSE_DESTROY => {
                let body = Destroy();
                req!(header, body)
            }
            FUSE_INTERRUPT => {
                return Err(Error::new(Other, "Interrupting is not implemented"));
            }
            FUSE_LOOKUP => {
                let body = Lookup(fetch_str(src));
                req!(header, body)
            }
            FUSE_FORGET => {
                let body = Forget(fetch(src));
                req!(header, body)
            }
            FUSE_GETATTR => {
                let body = GetAttr();
                req!(header, body)
            }
            FUSE_SETATTR => {
                let body = SetAttr(fetch(src));
                req!(header, body)
            }
            FUSE_READLINK => {
                let body = ReadLink();
                req!(header, body)
            }
            FUSE_MKNOD => {
                let body = MkNod(fetch(src));
                req!(header, body)
            }
            FUSE_MKDIR => {
                let body = MkDir(fetch(src));
                req!(header, body)
            }
            FUSE_UNLINK => {
                let body = Unlink(fetch_str(src));
                req!(header, body)
            }
            FUSE_RMDIR => {
                let body = RmDir(fetch_str(src));
                req!(header, body)
            }
            FUSE_SYMLINK => {
                let body = Symlink(fetch_str(src), fetch_path(src));
                req!(header, body)
            }
            FUSE_RENAME => {
                let body = Rename(fetch(src), fetch_str(src), fetch_str(src));
                req!(header, body)
            }
            FUSE_LINK => {
                let body = Link(fetch(src), fetch_str(src));
                req!(header, body)
            }
            FUSE_OPEN => {
                let body = Open(fetch(src));
                req!(header, body)
            }
            FUSE_READ => {
                let body = Read(fetch(src));
                req!(header, body)
            }
            FUSE_WRITE => {
                let body = Write(fetch(src), src.to_vec());
                req!(header, body)
            }
            FUSE_FLUSH => {
                let body = Flush(fetch(src));
                req!(header, body)
            }
            FUSE_RELEASE => {
                let body = Release(fetch(src));
                req!(header, body)
            }
            FUSE_FSYNC => {
                let body = FSync(fetch(src));
                req!(header, body)
            }
            FUSE_OPENDIR => {
                let body = OpenDir(fetch(src));
                req!(header, body)
            }
            FUSE_READDIR => {
                let body = ReadDir(fetch(src));
                req!(header, body)
            }
            FUSE_RELEASEDIR => {
                let body = ReleaseDir(fetch(src));
                req!(header, body)
            }
            FUSE_FSYNCDIR => {
                let body = FSyncDir(fetch(src));
                req!(header, body)
            }
            FUSE_STATFS => {
                let body = StatFS();
                req!(header, body)
            }
            FUSE_SETXATTR => {
                let body = SetXAttr(fetch(src), src.to_vec());
                req!(header, body)
            }
            FUSE_GETXATTR => {
                let body = GetXAttr(fetch(src));
                req!(header, body)
            }
            FUSE_LISTXATTR => {
                let body = ListXAttr(fetch(src));
                req!(header, body)
            }
            FUSE_REMOVEXATTR => {
                let body = RemoveXAttr(fetch(src));
                req!(header, body)
            }
            FUSE_ACCESS => {
                let body = Access(fetch(src));
                req!(header, body)
            }
            FUSE_CREATE => {
                let body = Create(fetch(src));
                req!(header, body)
            }
            FUSE_GETLK => {
                let body = GetLock(fetch(src));
                req!(header, body)
            }
            FUSE_SETLK | FUSE_SETLKW => {
                let body = SetLock(fetch(src));
                req!(header, body)
            }
            FUSE_BMAP => {
                let body = Bmap(fetch(src));
                req!(header, body)
            }
            #[cfg(target_os = "macos")]
            FUSE_SETVOLUMENAME => {
                let body = SetVolumeName(fetch_str(src));
                req!(header, body)
            }
            #[cfg(target_os = "macos")]
            FUSE_EXCHANGE => {
                let body = Exchange(fetch(src));
                req!(header, body)
            }
            #[cfg(target_os = "macos")]
            FUSE_GETXTIMES => {
                let body = GetXTimes();
                req!(header, body)
            }

        }

    }

}



/// Helper functions
// These functions are based on the work of Andreas Neuhaus under MIT license
pub fn fetch<T: Clone>(src: &mut BytesMut) -> T {
    let len = size_of::<T>();
    assert!(len <= src.len(), "out of data while fetching typed argument");

    // Return the data as the corresponding data type
    let bytes = src.split_to(len);

    let dst: &T = unsafe {std::mem::transmute(bytes.as_ptr())};
    dst.clone()
}

pub fn fetch_str(src: &mut BytesMut) -> OsString {
    let len = src.iter().position(|&c| c == 0)
        .expect("Ran out of data while parsing string");

    let bytes = src.split_to(len);

    // Discard null byte
    src.advance(1);

    OsString::from(std::str::from_utf8(&bytes).unwrap())
}

pub fn fetch_path(src: &mut BytesMut) -> PathBuf {
    PathBuf::from(fetch_str(src))
}

#[cfg(test)]
mod tests {

    #[test]
    fn fetch() {
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
        // Let buf go out of scope to test, that the fetched value persists

        println!("{:x?}", ts);
        assert_eq!(ts, TestStruct { buff: 0x66667542, er: 0x20207265 });

    }

    #[test]
    fn fetch_str() {
        use super::*;
        use bytes::{BytesMut, BufMut};

        let s = {
            let mut buf = BytesMut::with_capacity(64);
            buf.put("This is a test\0This is not fetched");

            let s = fetch_str(&mut buf);
            assert_eq!(buf.to_vec(), "This is not fetched".to_string().into_bytes());
            s
        };

        assert_eq!(s, OsString::from("This is a test"));

    }

    #[test]
    fn fetch_path() {
        use super::*;
        use bytes::{BytesMut, BufMut};

        let p = {
            let mut buf = BytesMut::with_capacity(64);
            buf.put("/dev/fuse\0This is not fetched");

            let p = fetch_path(&mut buf);
            assert_eq!(buf.to_vec(), "This is not fetched".to_string().into_bytes());
            p
        };

        assert_eq!(p, PathBuf::from("/dev/fuse"));
    }

}
