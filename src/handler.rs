use std::cmp::{PartialEq, Eq};
use crate::{Directory, File};

type FileImpl = Box<dyn File + Send + Sync>;
type DirImpl = Box<dyn Directory + Send + Sync>;

//FIXME: What are the visibility rules here?

pub(crate) struct FileHandler {
    ino : u64,
    file_impl: FileImpl,
}

impl FileHandler {
    pub(crate) fn file_impl(&self) -> &FileImpl {
        &self.file_impl
    }
}

impl PartialEq for FileHandler {
    fn eq (&self, other : &FileHandler) -> bool {
        self.ino == other.ino
    }
}
impl Eq for FileHandler {}



pub(crate) struct DirHandler {
    ino : u64,
    dir_impl: DirImpl,
}

impl DirHandler {
    pub(crate) fn dir_impl(&self) -> &DirImpl {
        &self.dir_impl
    }
}

impl PartialEq for DirHandler {
    fn eq (&self, other : &DirHandler) -> bool {
        self.ino == other.ino
    }
}
impl Eq for DirHandler {}


#[derive(PartialEq, Eq)]
pub(crate) enum Handler {
    File(FileHandler),
    Dir(DirHandler)
}

impl Handler {

    pub(crate) fn new_file_handler(file : FileImpl, ino : u64) -> Self {
        Handler::File(FileHandler {
            ino,
            file_impl : file,
        })
    }

    pub(crate) fn new_dir_handler(dir : DirImpl, ino : u64) -> Self {
        Handler::Dir(DirHandler {
            ino,
            dir_impl : dir,
        })
    }

}