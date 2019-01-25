use std::sync::Arc;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use parking_lot::RwLock;

use std::io;

use fuse::BackgroundSession;

use crate::{File, Directory, Registry};
use crate::handler::{Handle, HandleDispatcher::*};
use crate::controller::Controller;
use crate::driver::Driver;
use crate::utils::InoGenerator;


pub struct Engine<'a> {
    mount_point : PathBuf,
    registry : Registry,
    ino_generator : Arc<InoGenerator>,
    fuse_session : Option<BackgroundSession<'a>>,

}

impl<'a> Engine<'a> {

    pub fn new<T: 'static>(path: &Path, root: T) -> Self
    where T: Directory + Send + Sync {

        let mut engine = Engine{
            mount_point : path.to_path_buf(),
            registry : Arc::new(RwLock::new(BTreeMap::new())),
            ino_generator : Arc::new(InoGenerator::new()),
            fuse_session : None,
        };

        engine.add_directory(root);
        engine
    }


    pub fn start(&mut self) -> io::Result<()> {

        // TODO: Find a way to use options appropriately
        //let options = vec![OsStr::new("-o"), OsStr::new("fsname=test")];
        let mount_point = self.mount_point.clone();
        let options = [];


        let driver = Driver::new(self.registry.clone(), self.ino_generator.clone());
        let session = unsafe {fuse::spawn_mount(driver, &mount_point, &options[..])}?;
        self.fuse_session = Some(session);
        Ok(())
    }


    pub fn add_file<T: 'static>(&mut self, object: T) -> Handle
    where T: File + Send + Sync {

        let boxed = Box::new(object);
        let ino = self.ino_generator.generate();
        let handle = Handle::new_file(ino, boxed);

        self.registry.write().insert(ino, handle.clone());

        let controller = Controller::create_from_engine(self, ino, handle.clone());
        if let RegularFile(ref mut file) = handle.write().dispatch() {
            file.init(controller)
        } else {
            // Can not happen
            panic!();
        }
        handle
    }

    pub fn add_directory<T: 'static>(&mut self, object: T) -> Handle
    where T: Directory + Send + Sync {

        let boxed = Box::new(object);
        let ino = self.ino_generator.generate();
        let handle = Handle::new_dir(ino, boxed);

        self.registry.write().insert(ino,handle.clone());

        let controller = Controller::create_from_engine(self, ino, handle.clone());
        if let Dir(ref mut dir) = handle.write().dispatch() {
            dir.init(controller)
        } else {
            // Can not happen
            panic!();
        }
        handle
    }



    pub(crate) fn get_registry(&self) -> Registry {
        self.registry.clone()
    }

    pub(crate) fn get_ino_generator(&self) -> Arc<InoGenerator> {
        self.ino_generator.clone()
    }
}