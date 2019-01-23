use std::cmp::{PartialEq, Eq};
use crate::{Directory, File};

type FileImpl = Box<dyn File + Send + Sync>;
type DirImpl = Box<dyn Directory + Send + Sync>;

// FIXME: What are the visibility rules here?
// FIXME: Are the handler wrappers needed?

pub(crate) struct FileHandler {
    file_impl: FileImpl,
}

impl FileHandler {
    pub(crate) fn file_impl(&self) -> &FileImpl {
        &self.file_impl
    }
}

pub(crate) struct DirHandler {
    name : String,
    dir_impl: DirImpl,
}

impl DirHandler {
    pub(crate) fn dir_impl(&self) -> &DirImpl {
        &self.dir_impl
    }
}

pub(crate) enum HandlerDispatcher {
    File(FileHandler),
    Dir(DirHandler)
}

pub struct Handler {
    ino : u64,
    dispatch : HandlerDispatcher,
}

impl PartialEq for Handler {
    fn eq (&self, other : &Handler) -> bool {
        self.ino == other.ino
    }
}
impl Eq for Handler {}


impl Handler {

    pub fn is_directory(&self) -> bool {
        match self.dispatch {
            HandlerDispatcher::Dir(_) => true,
            _ => false,
        }
    }

    pub fn is_file(&self) -> bool {
        match self.dispatch {
            HandlerDispatcher::File(_) => true,
            _ => false,
        }
    }


    pub(crate) fn dispatch(&self) -> &HandlerDispatcher {
        &self.dispatch
    }

    pub(crate) fn ino(&self) -> u64 {
        self.ino
    }

}