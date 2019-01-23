use std::sync::Arc;

use fuse::FileType;

use crate::RegistryEntry;
use crate::handler::{Handler, HandlerDispatcher};

pub struct DirectoryEntry {
    name : String,
    handler : RegistryEntry
}


impl DirectoryEntry {

    /// Creates a directory entry
    pub fn new(name : String, handler : RegistryEntry) -> Self {
        DirectoryEntry {
            name,
            handler : handler.clone(),
        }
    }

    // TODO: Add attribute functions

    pub(crate) fn to_reply(&self) -> (u64, FileType, String) {
        match self.handler.read().dispatch_ref() {
            HandlerDispatcher::Dir(_) => {
                (self.handler.read().ino(), FileType::Directory, self.name.clone())
            }
            HandlerDispatcher::File(_) => {
                (self.handler.read().ino(), FileType::RegularFile, self.name.clone())
            }
        }

    }

}