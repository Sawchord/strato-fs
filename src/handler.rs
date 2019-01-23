use std::cmp::{PartialEq, Eq};

use crate::{FileImpl, DirImpl};

// FIXME: What are the visibility rules here?
// FIXME: Are the handler wrappers needed?
// FIXME: Rename to Handle to avoid convusion with the Trait Implementations

pub(crate) struct FileHandler {
    object: FileImpl,
}

impl FileHandler {
    pub(crate) fn get_object(&mut self) -> &mut FileImpl {
        &mut self.object
    }
}




pub(crate) struct DirHandler {
    object: DirImpl,
}

impl DirHandler {

    pub(crate) fn get_object(&mut self) -> &mut DirImpl {
        &mut self.object
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



    pub(crate) fn new_file(ino: u64, object: FileImpl) -> Self {
        Handler{
            ino,
            dispatch : HandlerDispatcher::File(
                FileHandler{
                    object
                }
            ),
        }
    }

    pub(crate) fn new_dir(ino: u64, object: DirImpl) -> Self {
        Handler{
            ino,
            dispatch : HandlerDispatcher::Dir(
                DirHandler{
                    object
                }
            ),
        }
    }


    pub(crate) fn dispatch_ref(&self) -> &HandlerDispatcher {
        &self.dispatch
    }

    pub(crate) fn dispatch(&mut self) -> &mut HandlerDispatcher {
        &mut self.dispatch
    }

    pub(crate) fn ino(&self) -> u64 {
        self.ino
    }

}