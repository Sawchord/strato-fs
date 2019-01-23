use std::sync::Arc;

use fuse::FileType;

use crate::handler::Handler;

pub struct DirectoryEntry {
    name : String,
    handler : Arc<Handler>
}


impl DirectoryEntry {

    /// Creates a directory
    pub fn new(name : String, handler : Arc<Handler>) -> Self {
        DirectoryEntry {
            name,
            handler : handler.clone(),
        }
    }


    fn to_reply(&self) -> (u64, FileType, String) {
        match *self.handler {
            Handler::Dir(ref dir) => {
                (dir.ino, FileType::Directory, self.name)
            }
            Handler::File(ref file) => {
                (file.ino, FileType::RegularFile, self.name)
            }
        }

    }

}