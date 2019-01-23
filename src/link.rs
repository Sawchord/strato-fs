use std::sync::Arc;

use fuse::FileType;

use crate::handler::{Handler, HandlerDispatcher};

pub struct DirectoryEntry {
    name : String,
    handler : Arc<Handler>
}


impl DirectoryEntry {

    /// Creates a directory entry
    pub fn new(name : String, handler : Arc<Handler>) -> Self {
        DirectoryEntry {
            name,
            handler : handler.clone(),
        }
    }

    // TODO: Add attribute functions

    fn to_reply(&self) -> (u64, FileType, String) {
        match self.handler.dispatch() {
            HandlerDispatcher::Dir(ref dir) => {
                (self.handler.ino(), FileType::Directory, self.name.clone())
            }
            HandlerDispatcher::File(ref file) => {
                (self.handler.ino(), FileType::RegularFile, self.name.clone())
            }
        }

    }

}