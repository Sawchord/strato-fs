use std::cmp::{PartialEq, Eq};
use std::sync::Arc;
use std::fmt;

use std::ops::Deref;

use parking_lot::RwLock;

use crate::{FileImpl, DirImpl};
use self::HandleDispatcher::*;

#[derive (Clone, Debug)]
pub struct Handle (Arc<RwLock<HandleInner>>);

impl Handle {

    pub(crate) fn new_file(ino: u64, object: FileImpl) -> Self {
        Handle(Arc::new(RwLock::new(
            HandleInner {
                ino,
                dispatch : RegularFile(object),
            }
        )))
    }

    pub(crate) fn new_dir(ino: u64, object: DirImpl) -> Self {
        Handle(Arc::new(RwLock::new(
            HandleInner {
                ino,
                dispatch : Dir(object),
            }
        )))
    }

}

impl Deref for Handle {
    type Target = Arc<RwLock<HandleInner>>;
    fn deref(&self) -> &Arc<RwLock<HandleInner>> {
        &self.0
    }
}


pub(crate) enum HandleDispatcher {
    RegularFile(FileImpl),
    Dir(DirImpl)
}


pub struct HandleInner {
    ino : u64,
    dispatch : HandleDispatcher,
}

impl PartialEq for HandleInner {
    fn eq (&self, other : &HandleInner) -> bool {
        self.ino == other.ino
    }
}
impl Eq for HandleInner {}


impl HandleInner {

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

impl fmt::Debug for HandleInner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self.dispatch {
            Dir(_) => {
                write!(f, "ino:{} (Directory)", self.ino)
            }
            RegularFile(_) => {
                write!(f, "ino:{} (File)", self.ino)
            }
        }
    }
}