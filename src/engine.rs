use std::sync::Arc;
use std::path::PathBuf;
use std::collections::BTreeMap;

use parking_lot::RwLock;

//use libc::{ENOENT, ENOTDIR, EPERM};
use libc::*;

use fuse::{BackgroundSession, Filesystem, Request, ReplyDirectory};

use crate::handler::{Handler, HandlerDispatcher};
use crate::utils::InoGenerator;

type Registry = Arc<RwLock<BTreeMap<u64, Arc<Handler>>>>;

struct Driver {
    registry : Registry,
    ino_generator : Arc<InoGenerator>,
}

pub struct Engine<'a> {
    mount_point : PathBuf,
    registry : Registry,
    ino_generator : Arc<InoGenerator>,
    fuse_session : Option<BackgroundSession<'a>>,

}

impl<'a> Engine<'a> {

    pub fn new(path: PathBuf) -> Self {
        // TODO: Add root directory

        Engine{
            mount_point : path,
            registry : Arc::new(RwLock::new(BTreeMap::new())),
            ino_generator : Arc::new(InoGenerator::new()),
            fuse_session : None,
        }
    }


    pub fn start(&mut self) {

        // TODO: Find a way to use options appropriately
        let options = [];
        let mount_point = self.mount_point.clone();

        let driver = Driver {
            registry : self.registry.clone(),
            ino_generator : self.ino_generator.clone(),
        };


        let session = unsafe {fuse::spawn_mount(driver, &mount_point, &options).unwrap() };
        self.fuse_session = Some(session);
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




