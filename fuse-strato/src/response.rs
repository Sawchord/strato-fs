use std::time::SystemTime;

use std::path::PathBuf;
use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::OsStrExt;

use fuse_sys::abi::*;
use fuse_sys::abi::consts::*;
use fuse_sys::abi::fuse_opcode::*;

use crate::file::{FileAttr, FileType};
use crate::file::{system_time_decompose, fuse_attr_from_attr};
use crate::file::mode_from_kind_and_perm;

// For ReplyEmpty, as we can use ()
// For ReplyData as we can use Vec<u8>
// For XAttr we can use Vec<u8>

pub fn entry(ttl: &SystemTime, attr: &FileAttr, generation: u64) -> fuse_entry_out {

    let (valid_s, valid_n) = system_time_decompose(ttl);

    fuse_entry_out {
        nodeid: attr.ino,
        generation,
        entry_valid: valid_s,
        attr_valid: valid_s,
        entry_valid_nsec: valid_n,
        attr_valid_nsec: valid_n,
        attr: fuse_attr_from_attr(attr),
    }
}


pub fn attr(ttl: &SystemTime, attr: &FileAttr) -> fuse_attr_out {
    let (valid_s, valid_n) = system_time_decompose(ttl);

    fuse_attr_out {
        attr_valid: valid_s,
        attr_valid_nsec: valid_n,
        dummy: 0,
        attr: fuse_attr_from_attr(attr),
    }
}


#[cfg(target_os = "macos")]
pub fn reply_x_times(bkuptime: &SystemTime, crtime: &SystemTime) -> fuse_getxtimes_out {
    let (bkuptime_s, bkuptime_n) = system_time_decompose(bkuptime);
    let (crtime_s, crtime_n) = system_time_decompose(crtime);

    fuse_getxtimes_out {
        bkuptime: bkuptime_s,
        crtime: crtime_s,
        bkuptimensec: bkuptime_n,
        crtimensec: crtime_n,
    }
}


pub fn open(fh: u64, flags: u32) -> fuse_open_out {
    fuse_open_out {
        fh,
        open_flags: flags,
        padding: 0,
    }
}


pub fn write(size: u32) -> fuse_write_out {
    fuse_write_out {
        size,
        padding: 0,
    }
}


pub fn statfs(blocks: u64, bfree: u64, bavail: u64, files: u64, ffree: u64,
              bsize: u32, namelen: u32, frsize: u32) -> fuse_statfs_out {
    fuse_statfs_out {
        st: fuse_kstatfs {
            blocks,
            bfree,
            bavail,
            files,
            ffree,
            bsize,
            namelen,
            frsize,
            padding: 0,
            spare: [0; 6]
        }
    }
}


pub fn create(ttl: &SystemTime, attr: &FileAttr, generation: u64,
              fh: u64, flags: u32) -> (fuse_entry_out, fuse_open_out) {

    let (valid_s, valid_n) = system_time_decompose(ttl);

    (fuse_entry_out {
        nodeid: attr.ino,
        generation,
        entry_valid: valid_s,
        attr_valid: valid_s,
        entry_valid_nsec: valid_n,
        attr_valid_nsec: valid_n,
        attr: fuse_attr_from_attr(attr)
    },
    fuse_open_out {
        fh,
        open_flags: flags,
        padding: 0,
    })
}


pub fn lock(start: u64, end: u64, typ: u32, pid: u32) -> fuse_lk_out {
    fuse_lk_out {
        lk: fuse_file_lock {
            start,
            end,
            typ,
            pid,
        }
    }
}


pub fn bmap(block: u64) -> fuse_bmap_out {
    fuse_bmap_out {
        block,
    }
}



#[derive(Debug, Clone, PartialEq)]
pub struct DirReply(Vec<u8>);

impl DirReply {
    pub fn new() -> Self {
        DirReply(Vec::new())
    }

    pub fn entry<T: Into<PathBuf>>(&mut self, ino: u64, offset: i64, kind: FileType, name: T) {
        use std::mem::size_of;
        use std::slice::from_raw_parts;

        let pb = name.into().as_os_str().to_owned();

        // Calculate the length of the entry with padding
        let entry_len = size_of::<fuse_dirent>() + pb.len();
        let len = (entry_len + 0b111) & !0b111;
        let pad_size = len - entry_len;

        let mut some_vec = unsafe{
            from_raw_parts( &fuse_dirent {
                ino,
                off: offset,
                namelen: pb.len() as u32,
                typ: mode_from_kind_and_perm(&kind, 0) >> 12,
            } as *const fuse_dirent as *const u8,
                           size_of::<fuse_dirent>())
        }.to_vec();

        self.0.append(&mut some_vec);

        let mut name = pb.into_boxed_os_str().as_ref().as_bytes().to_vec();
        self.0.append(&mut name);

        for _ in 0..pad_size { self.0.push(0); }

    }

    pub(crate) fn to_vec(self) -> Vec<u8> {
        self.0
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dir_reply() {

        let mut dir_reply = DirReply::new();

        dir_reply.entry(1, 0, FileType::Directory, "test_dir");
        dir_reply.entry(2, 4096, FileType::RegularFile, "test_file");


        let vec = dir_reply.to_vec();
        hexdump::hexdump(&vec);

        assert_eq!(vec, vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 4, 0, 0, 0,
                             116, 101, 115, 116, 95, 100, 105, 114, 2, 0, 0, 0, 0, 0, 0, 0, 0, 16,
                             0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 8, 0, 0, 0, 116, 101, 115, 116, 95, 102,
                             105, 108, 101, 0, 0, 0, 0, 0, 0, 0]);

    }

}