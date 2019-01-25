use std::sync::Arc;
use std::ffi::OsStr;

use libc::*;

use fuse::{Filesystem, Request, ReplyDirectory, ReplyData, ReplyEntry, ReplyAttr};

use crate::handler::HandleDispatcher;
use crate::utils::InoGenerator;
use crate::link::DirectoryEntry;
use crate::Registry;


/// This macro looks up the ino from the registry and returns the corresponding handler
/// It sends an `ENOENT` to the FUSE driver, if the ino does not exist.
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


pub(crate) struct Driver {
    registry : Registry,
    //ino_generator : Arc<InoGenerator>,
}

impl Driver {

    pub(crate) fn new(registry: Registry, _ino_generator : Arc<InoGenerator>) -> Self {
        Driver {
            registry : registry.clone(),
            //ino_generator : ino_generator.clone(),
        }
    }

    //pub(crate) fn get_registry(&self) -> Registry {
    //    self.registry.clone()
    //}

    //pub(crate) fn get_ino_generator(&self) -> Arc<InoGenerator> {
    //    self.ino_generator.clone()
    //}

}


// TODO: Implement macros to check if directory or file with appropriate errors
impl Filesystem for Driver {

    fn lookup(&mut self, req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {

        let handle = get_handle!(self, parent, reply);
        let n = name.to_string_lossy().to_string();

        let result = match handle.write().dispatch() {
            HandleDispatcher::Dir(ref mut dir) => {
                dir.lookup(req, n)
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
            Some(entry) => {
                // TODO: What does Generation do?
                reply.entry(&entry.get_ttl(), &entry.to_attr(), 0);
            }
        }

    }

    fn getattr(&mut self, req: &Request, ino: u64, reply: ReplyAttr) {

        let handle = get_handle!(self, ino, reply);
        let base_entry = DirectoryEntry::new("".to_string(), handle.clone());

        let result = match handle.write().dispatch() {
            HandleDispatcher::Dir(ref mut dir) => {
                dir.read_attributes(req, base_entry)
            }
            HandleDispatcher::File(ref mut file) => {
                file.read_attributes(req, base_entry)
            }
        };

        match result {
            None => reply.error(EPERM),
            Some(entry) => reply.attr(&entry.get_ttl(), &entry.to_attr())
        }

    }

    // TODO: Implement correct behaviour of offset and size... how to handle streaming?
    fn read(&mut self, req: &Request, ino: u64, _fh: u64,
            offset: i64, size: u32, reply: ReplyData) {

        let handle = get_handle!(self, ino, reply);

        let result = match handle.write().dispatch() {
            HandleDispatcher::File(ref mut file) => {
                file.read(req)
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
                println!("Request params: Offset {} Size {}", offset, size);
                reply.data(&vec[offset as usize..]);
            }
        }



    }

    fn readdir(&mut self, req: &Request, ino: u64, _fh: u64,
               offset: i64, mut reply: ReplyDirectory) {

        let handle = get_handle!(self, ino, reply);

        // Check that the handle references a directory
        let result = match handle.write().dispatch() {
            // Check that this is actually a directory
            HandleDispatcher::Dir(ref mut dir) => {
                dir.readdir(req)
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
                let to_skip = if offset == 0 { offset } else { offset + 1 } as usize;
                for (i, entry) in vec.into_iter().enumerate().skip(to_skip) {

                    let rep = entry.to_reply();
                    reply.add(rep.0, i as i64,rep.1, rep.2);

                }
                reply.ok();
            }
        }
    }


}




