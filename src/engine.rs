use std::sync::Arc;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use parking_lot::RwLock;

use std::io;

use fuse::BackgroundSession;

use crate::{FileImpl, DirImpl};
use crate::handler::{Handler, HandlerDispatcher};
use crate::controller::Controller;
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

    //pub fn new<P: AsRef<Path>>(path: &Path) -> Self {
    pub fn new(path: &Path) -> Self {

        Engine{
            mount_point : path.to_path_buf(),
            registry : Arc::new(RwLock::new(BTreeMap::new())),
            ino_generator : Arc::new(InoGenerator::new()),
            fuse_session : None,
        }

    }


    pub fn start(&mut self) -> io::Result<()> {

        // TODO: Find a way to use options appropriately
        let options = vec![OsStr::new("-o"), OsStr::new("fsname=TODO")];
        let mount_point = self.mount_point.clone();

        let driver = Driver::new(self.registry.clone(), self.ino_generator.clone());
        let session = unsafe {fuse::spawn_mount(driver, &mount_point, &options[..])}?;
        self.fuse_session = Some(session);
        Ok(())
    }


    pub fn add_file_handler(&mut self, object: FileImpl) -> Arc<Handler> {

        let ino = self.ino_generator.generate();
        let handle = Arc::new(Handler::new_file(ino, object));

        //let x = handle.dispatch();

        self.registry.write().insert(ino, handle.clone());

        let controller = Controller::create_from_engine(self, ino, handle.clone());
        if let HandlerDispatcher::File(ref file) = handle.dispatch() {
            file.get_object().init(controller)
        } else {
            // Can not happen
            panic!();
        }
        handle
    }

    pub fn add_directory_handler(&mut self, object: DirImpl) -> Arc<Handler> {


        let ino = self.ino_generator.generate();
        let handle = Arc::new(Handler::new_dir(ino, object));

        self.registry.write().insert(ino,handle.clone());

        let controller = Controller::create_from_engine(self, ino, handle.clone());
        if let HandlerDispatcher::Dir(dir) = handle.dispatch() {
            dir.get_object().init(controller)
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