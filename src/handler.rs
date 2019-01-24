use std::cmp::{PartialEq, Eq};
use std::sync::Arc;
use std::fmt;

use parking_lot::RwLock;

use crate::{NodeImpl, FileImpl, DirImpl};

pub type ProtectedHandle = Arc<RwLock<Handle>>;

// FIXME: What are the visibility rules here?
// FIXME: Are the Handle wrappers needed?

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

    // TODO: Do we need these?
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


    //pub(crate) fn get_node(&mut self) -> &mut NodeImpl {
    //    match self.dispatch {
    //        HandleDispatcher::Dir(ref mut dir) => &mut dir.object as &mut NodeImpl,
    //        HandleDispatcher::File(ref mut file) => &mut file.object as &mut NodeImpl,
    //    }
    //}

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