use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::path::PathBuf;

use parking_lot::Mutex;

use evmap::{ReadHandle, WriteHandle};
use fuse::{BackgroundSession, Filesystem};

use crate::handler::Handler;
type RegistryValue = Arc<Handler>;


struct Engine<'a> {
    mount_point : PathBuf,

    registry : ReadHandle<u64, RegistryValue>,
    registry_writer : Arc<Mutex<WriteHandle<u64, RegistryValue>>>,

    fuse_session : Option<BackgroundSession<'a>>,
    ino_generator : InoGenerator,
}

impl<'a> Engine<'a> {

    pub fn new(path: PathBuf) -> Self {
        let (reader, writer) = evmap::new();

        // TODO: Add root directory

        Engine{
            mount_point : path,

            registry : reader,
            registry_writer : Arc::new(Mutex::new(writer)),

            fuse_session : None,
            ino_generator : InoGenerator::new(),
        }
    }

    pub fn start(mut self) {

        // TODO: Find a way to use options appropriately
        let options = [];

        let mount_point = self.mount_point.clone();
        let session = unsafe {fuse::spawn_mount(self, &mount_point, &options).unwrap() };

        self.fuse_session = Some(session);
    }
}


impl<'a> Filesystem for Engine<'a> {

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