use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::path::PathBuf;

use std::collections::BTreeMap;

use parking_lot::{Mutex, RwLock};

use fuse::{BackgroundSession, Filesystem};

use crate::handler::Handler;
type Registry = Arc<RwLock<BTreeMap<u64, Handler>>>;

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

    pub fn start(mut self) {

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


impl Filesystem for Driver {

}



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