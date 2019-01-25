use std::cmp::{PartialEq, Eq};
use std::sync::Arc;
use std::fmt;

use parking_lot::RwLock;

use crate::{FileImpl, DirImpl};

pub type ProtectedHandle = Arc<RwLock<Handle>>;

// FIXME: What are the visibility rules here?
// FIXME: Are the Handle wrappers needed?



pub(crate) enum HandleDispatcher {
    File(FileImpl),
    Dir(DirImpl)
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

    pub(crate) fn new_file(ino: u64, object: FileImpl) -> Self {
        Handle{
            ino,
            dispatch : HandleDispatcher::File(object),
        }
    }

    pub(crate) fn new_dir(ino: u64, object: DirImpl) -> Self {
        Handle{
            ino,
            dispatch : HandleDispatcher::Dir(object),
        }
    }


    pub(crate) fn dispatch_ref(&self) -> &HandleDispatcher {
        &self.dispatch
    }

    pub(crate) fn dispatch(&mut self) -> &mut HandleDispatcher {
        &mut self.dispatch
    }

    pub(crate) fn get_ino(&self) -> u64 {
        self.ino
    }

}

impl fmt::Debug for Handle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self.dispatch {
            HandleDispatcher::Dir(_) => {
                write!(f, "ino:{} (Directory)", self.ino)
            }
            HandleDispatcher::File(_) => {
                write!(f, "ino:{} (File)", self.ino)
            }
        }
    }
}