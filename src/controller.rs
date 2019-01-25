use std::sync::Arc;

use parking_lot::RwLock;

use crate::{Registry, File, Directory};
use crate::engine::Engine;
use crate::handler::{ProtectedHandle, Handle, HandleDispatcher::*};
use crate::utils::InoGenerator;

/// This object gets handed down to functions implementing a File System Handle trait, such as
/// File or Directory. The controller exposes information about the Handles context and can also
/// be used to manipulate (e.g. delete) the Handle.
#[derive(Clone)]
pub struct Controller {

    this_ino : u64,

    ino_generator : Arc<InoGenerator>,
    registry : Registry,

    handle : ProtectedHandle,
}

impl Controller {


    pub fn add_file<T: 'static>(&mut self, object: T) -> ProtectedHandle
        where T: File + Send + Sync {

        let boxed = Box::new(object);
        let ino = self.ino_generator.generate();
        let handle = Arc::new(RwLock::new(Handle::new_file(ino, boxed)));

        self.registry.write().insert(ino, handle.clone());

        let controller = Controller::create_from_controller(self, ino, handle.clone());
        if let RegularFile(ref mut file) = handle.write().dispatch() {
            file.init(controller)
        } else {
            // Can not happen
            panic!();
        }
        handle
    }

    pub fn add_directory<T: 'static>(&mut self, object: T) -> ProtectedHandle
        where T: Directory + Send + Sync {

        let boxed = Box::new(object);
        let ino = self.ino_generator.generate();
        let handle = Arc::new(RwLock::new(Handle::new_dir(ino, boxed)));

        self.registry.write().insert(ino,handle.clone());

        let controller = Controller::create_from_controller(self, ino, handle.clone());
        if let Dir(ref mut dir) = handle.write().dispatch() {
            dir.init(controller)
        } else {
            // Can not happen
            panic!();
        }
        handle
    }

    pub fn get_handle(&self) -> ProtectedHandle {
        self.handle.clone()
    }


    pub(crate) fn create_from_engine(engine: &Engine, ino: u64, handle : ProtectedHandle) -> Self {
        Controller {
            this_ino : ino,

            ino_generator : engine.get_ino_generator(),
            registry : engine.get_registry(),

            handle,
        }
    }

    pub(crate) fn create_from_controller(controller: &Controller, ino: u64, handle : ProtectedHandle) -> Self {
        Controller {
            this_ino : ino,

            ino_generator : controller.ino_generator.clone(),
            registry : controller.registry.clone(),

            handle,
        }
    }



}
