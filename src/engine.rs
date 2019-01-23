use std::sync::Arc;
use std::collections::BTreeMap;
use std::path::PathBuf;

use parking_lot::RwLock;

use fuse::BackgroundSession;

use crate::driver::Driver;
use crate::utils::InoGenerator;
use crate::Registry;

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

        let driver = Driver::new(self.registry.clone(), self.ino_generator.clone());
        let session = unsafe {fuse::spawn_mount(driver, &mount_point, &options).unwrap() };
        self.fuse_session = Some(session);
    }
}