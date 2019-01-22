use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::cmp::{PartialEq, Eq};
use parking_lot::Mutex;

use evmap::{ReadHandle, WriteHandle};
use fuse::BackgroundSession;

use crate::handler::Handler;
type RegistryValue = Arc<Handler>;


struct Engine<'a> {
    registry : ReadHandle<u64, RegistryValue>,
    registry_writer : Arc<Mutex<WriteHandle<u64, RegistryValue>>>,
    fuse_session : Option<BackgroundSession<'a>>
}

impl<'a> Engine<'a> {
    pub fn new() -> Self {
        let (reader, writer) = evmap::new();
        Engine{
            registry : reader,
            registry_writer : Arc::new(Mutex::new(writer)),
            fuse_session : None
        }
    }
}

struct InoGenerator {
    next_ino : AtomicU64
}