use std::sync::Arc;
use std::ffi::OsStr;

use libc::*;

use time::Timespec;

use fuse::{Filesystem, Request, ReplyDirectory, ReplyData, ReplyEntry};

use crate::handler::HandleDispatcher;
use crate::controller::Controller;
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

// TODO: Implement macros to check if directory or file with appropriate errors


impl Filesystem for Driver {

    fn lookup(&mut self, req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {

        let handle = get_handle!(self, parent, reply);
        let controller = Controller::create_from_driver(self, parent, handle.clone());
        let n = name.to_string_lossy().to_string();

        let result = match handle.write().dispatch() {
            HandleDispatcher::Dir(ref mut dir) => {
                dir.get_object().lookup(controller, req, n)
            }
            _ => {
                reply.error(ENOTDIR);
                return;
            }
        };

        match result {
            None => {
                reply.error(ENOENT);
            }
            Some((entry, duration)) => {
                // TODO: Fix timespec
                // TODO: What does Generation do?
                reply.entry(&Timespec::new(1, 0), &entry.to_attr(), 0);
            }
        }

    }


    fn read(&mut self, req: &Request, ino: u64, _fh: u64,
            _offset: i64, _size: u32, reply: ReplyData) {

        let handle = get_handle!(self, ino, reply);
        let controller = Controller::create_from_driver(self, ino, handle.clone());

        let result = match handle.write().dispatch() {
            HandleDispatcher::File(ref mut file) => {
                file.get_object().read(controller, req)
            }
            _ => {
                reply.error(EISDIR);
            return;
            }
        };

        match result {
            None => {
                reply.error(EPERM);
            }
            Some(vec) => {
                reply.data(&vec);
            }
        }



    }

    // TODO: Implement Offset
    // TODO: Implement Error Types
    fn readdir(&mut self, req: &Request, ino: u64, _fh: u64,
               _offset: i64, mut reply: ReplyDirectory) {

        let handle = get_handle!(self, ino, reply);
        let controller = Controller::create_from_driver(self, ino, handle.clone());

        // Check that the handle references a directory
        let result = match handle.write().dispatch() {
            // Check that this is actually a directory
            HandleDispatcher::Dir(ref mut dir) => {

                dir.get_object().readdir(controller, req)
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
                let mut x: i64 = 0;
                for i in vec {

                    let rep = i.to_reply();
                    reply.add(rep.0, x,rep.1, rep.2);
                    x += 1;

                }
            }
        }
    }


}




