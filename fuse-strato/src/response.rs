use std::time::SystemTime;

use fuse_sys::abi::*;
use fuse_sys::abi::consts::*;
use fuse_sys::abi::fuse_opcode::*;

use crate::file::FileAttr;
use crate::file::{system_time_decompose, fuse_attr_from_attr};

// No need for ReplyEmpty, as we can use () and ReplyData as we can use Vec<u8>

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


//TODO: Statfs, Create, Lock, Bmap, Directory, Xattr