use std::sync::Arc;

use crate::Registry;
use crate::engine::Engine;
use crate::handler::ProtectedHandle;
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

}
