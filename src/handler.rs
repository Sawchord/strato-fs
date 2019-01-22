use std::cmp::{PartialEq, Eq};
use crate::{Directory, File};


struct FileHandler {
    ino : u64,
    handler: Box<File>
}

impl PartialEq for FileHandler {
    fn eq (&self, other : &FileHandler) -> bool {
        self.ino == other.ino
    }
}
impl Eq for FileHandler {}



struct DirHandler {
    ino : u64,
    handler: Box<Directory>
}

impl PartialEq for DirHandler {
    fn eq (&self, other : &DirHandler) -> bool { self.ino == other.ino
    }
}
impl Eq for DirHandler {}



#[derive(PartialEq, Eq)]
pub enum Handler {
    File(FileHandler),
    Dir(DirHandler)
}