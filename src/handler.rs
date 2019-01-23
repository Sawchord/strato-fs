use std::cmp::{PartialEq, Eq};
use std::sync::Arc;

use parking_lot::RwLock;

use crate::{FileImpl, DirImpl};

pub type ProtectedHandle = Arc<RwLock<Handle>>;

// FIXME: What are the visibility rules here?
// FIXME: Are the Handle wrappers needed?
// FIXME: Rename to Handle to avoid convusion with the Trait Implementations

pub(crate) struct FileHandle {
    object: FileImpl,
}

impl FileHandle {
    pub(crate) fn get_object(&mut self) -> &mut FileImpl {
        &mut self.object
    }
}




pub(crate) struct DirHandle {
    object: DirImpl,
}

impl DirHandle {

    pub(crate) fn get_object(&mut self) -> &mut DirImpl {
        &mut self.object
    }
}


pub(crate) enum HandleDispatcher {
    File(FileHandle),
    Dir(DirHandle)
}

pub struct Handle {
    ino : u64,
    dispatch : HandleDispatcher,
}

impl PartialEq for Handle {
    fn eq (&self, other : &Handle) -> bool {
        self.ino == other.ino
    }
}
impl Eq for Handle {}


impl Handle {

    pub fn is_directory(&self) -> bool {
        match self.dispatch {
            HandleDispatcher::Dir(_) => true,
            _ => false,
        }
    }

    pub fn is_file(&self) -> bool {
        match self.dispatch {
            HandleDispatcher::File(_) => true,
            _ => false,
        }
    }



    pub(crate) fn new_file(ino: u64, object: FileImpl) -> Self {
        Handle{
            ino,
            dispatch : HandleDispatcher::File(
                FileHandle {
                    object
                }
            ),
        }
    }

    pub(crate) fn new_dir(ino: u64, object: DirImpl) -> Self {
        Handle{
            ino,
            dispatch : HandleDispatcher::Dir(
                DirHandle {
                    object
                }
            ),
        }
    }


    pub(crate) fn dispatch_ref(&self) -> &HandleDispatcher {
        &self.dispatch
    }

    pub(crate) fn dispatch(&mut self) -> &mut HandleDispatcher {
        &mut self.dispatch
    }

    pub(crate) fn ino(&self) -> u64 {
        self.ino
    }

}