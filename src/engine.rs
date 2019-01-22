use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::path::PathBuf;

use std::collections::BTreeMap;

use parking_lot::RwLock;


use libc::{ENOENT, ENOTDIR};

use fuse::{BackgroundSession, Filesystem, Request, ReplyDirectory};

use crate::handler::{Handler, FileHandler, DirHandler};

/// The thread safe generator if Inos
/// Used when spawning a new handler
struct InoGenerator {
    next_ino : AtomicU64
}

impl InoGenerator {

    fn new() -> Self {
        InoGenerator{
            next_ino : AtomicU64::new(1)
        }
    }

    fn get(self) -> u64 {
        self.next_ino.fetch_add(1, Ordering::SeqCst)
    }

}



type Registry = Arc<RwLock<BTreeMap<u64, Arc<Handler>>>>;

struct Driver {
    registry : Registry,
    ino_generator : InoGenerator,
}

struct Engine<'a> {
    mount_point : PathBuf,
    registry : Registry,
    fuse_session : Option<BackgroundSession<'a>>,

}

impl<'a> Engine<'a> {

    pub fn new(path: PathBuf) -> Self {
        // TODO: Add root directory

        Engine{
            mount_point : path,
            registry : Arc::new(RwLock::new(BTreeMap::new())),
            fuse_session : None,
        }
    }


    pub fn start(&mut self) {

        // TODO: Find a way to use options appropriately
        let options = [];

        let mount_point = self.mount_point.clone();

        let driver = Driver {
            registry : self.registry.clone(),
            ino_generator : InoGenerator::new(),
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

impl Filesystem for Driver {


    fn readdir(&mut self, _req: &Request, ino: u64, _fh: u64,
               offset: i64, mut reply: ReplyDirectory) {


        let handler = get_handle!(self, ino, reply);

        // If handle
        let result = match *handler {
            // Check that this is actually a directory
            Handler::Dir(ref dir) => {
                dir.dir_impl().readdir()
            },
            _ => {
                reply.error(ENOTDIR);
                return;
            },
        };



    }

}




