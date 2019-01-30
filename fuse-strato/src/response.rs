use std::time::SystemTime;

use fuse_sys::abi::*;
use fuse_sys::abi::consts::*;
use fuse_sys::abi::fuse_opcode::*;

use crate::file::FileAttr;
use crate::file::{system_time_decompose, fuse_attr_from_attr};

// No need for ReplyEmpty, as we can use () and ReplyData as we can use Vec<u8>

#[derive(Debug, Clone, PartialEq)]
pub struct EntryResponse(fuse_entry_out);

impl EntryResponse {
    pub fn new(ttl: &SystemTime, attr: &FileAttr, generation: u64) -> Self {

        let (valid_s, valid_n) = system_time_decompose(ttl);

        EntryResponse(fuse_entry_out {
            nodeid: attr.ino,
            generation,
            entry_valid: valid_s,
            attr_valid: valid_s,
            entry_valid_nsec: valid_n,
            attr_valid_nsec: valid_n,
            attr: fuse_attr_from_attr(attr),
        })
    }
}

