use std::sync::Arc;
use std::collections::BTreeMap;

use parking_lot::RwLock;

//use libc::{ENOENT, ENOTDIR, EPERM};
use libc::*;

use fuse::{Filesystem, Request, ReplyDirectory};

use crate::handler::HandlerDispatcher;
use crate::utils::InoGenerator;
use crate::Registry;


pub(crate) struct Driver {
    registry : Registry,
    ino_generator : Arc<InoGenerator>,
}

impl Driver {

    pub(crate) fn new(registry: Registry, ino_generator : Arc<InoGenerator>) -> Self {
        Driver {
            registry : registry.clone(),
            ino_generator : ino_generator.clone(),
        }
    }

    pub(crate) fn get_registry(&self) -> Registry {
        self.registry.clone()
    }

    pub(crate) fn get_ino_generator(&self) -> Arc<InoGenerator> {
        self.ino_generator.clone()
    }

}


/// This macro looks up the ino from the registry and returns the corresponding handler
/// It sends an ENOENT to the FUSE driver, if the ino does not exist
macro_rules! get_handle {
    ($driver:ident, $ino: ident, $reply:ident) => [
        match $driver.registry.read().get(&$ino) {
            None => {
                $reply.error(ENOENT);
                return;
            }
            Some(i) => i
        }.clone()
    ];
}

// TODO: Implement Controller
impl Filesystem for Driver {

    // TODO: Implement Offset
    // TODO: Implement Name
    // TODO: Implement Error Types
    fn readdir(&mut self, _req: &Request, ino: u64, _fh: u64,
               _offset: i64, mut reply: ReplyDirectory) {

        let handler = get_handle!(self, ino, reply);

        // Check that the handle references a directory
        let result = match handler.dispatch() {
            // Check that this is actually a directory
            HandlerDispatcher::Dir(ref dir) => {
                dir.dir_impl().readdir()
            },
            _ => {
                reply.error(ENOTDIR);
                return;
            },
        };

        match result {
            None => {
                reply.error(EPERM);
            }
            Some(vec) => {
                let x: i64 = 0;
                for i in vec {

                    let rep = i.to_reply();
                    reply.add(rep.0, x,rep.1, rep.2);

                }
            }
        }
    }



}




