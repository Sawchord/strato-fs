use std::cmp::{PartialEq, Eq};
use crate::{Directory, File};


//FIXME: What are the visibility rules here?

pub(crate) struct FileHandler {
    ino : u64,
    handler: Box<dyn File + Send + Sync>
}

impl PartialEq for FileHandler {
    fn eq (&self, other : &FileHandler) -> bool {
        self.ino == other.ino
    }
}
impl Eq for FileHandler {}



pub(crate) struct DirHandler {
    ino : u64,
    handler: Box<dyn Directory + Send + Sync>
}

impl PartialEq for DirHandler {
    fn eq (&self, other : &DirHandler) -> bool { self.ino == other.ino
    }
}
impl Eq for DirHandler {}



#[derive(PartialEq, Eq)]
pub(crate) enum Handler {
    File(FileHandler),
    Dir(DirHandler)
}