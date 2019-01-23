use fuse::FileType;

use crate::handler::{ProtectedHandle, HandleDispatcher};

pub struct DirectoryEntry {
    name : String,
    handle : ProtectedHandle
}


impl DirectoryEntry {

    /// Creates a directory entry
    pub fn new(name : String, handle : ProtectedHandle) -> Self {
        DirectoryEntry {
            name,
            handle : handle.clone(),
        }
    }

    // TODO: Add attribute functions

    pub(crate) fn to_reply(&self) -> (u64, FileType, String) {
        match self.handle.read().dispatch_ref() {
            HandleDispatcher::Dir(_) => {
                (self.handle.read().ino(), FileType::Directory, self.name.clone())
            }
            HandleDispatcher::File(_) => {
                (self.handle.read().ino(), FileType::RegularFile, self.name.clone())
            }
        }

    }

}