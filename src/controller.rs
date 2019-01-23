use std::sync::Arc;

use crate::Registry;
use crate::engine::Engine;
use crate::handler::Handler;
use crate::utils::InoGenerator;
use crate::driver::Driver;

// TODO: Split controller into controller and request, Request changes, controller stays constant per request
/// This object gets handed down to functions implementing a File System handler trait, such as
/// File or Directory. The controller exposes information about the handlers context and can also
/// be used to manipulate (e.g. delete) the handler.
pub struct Controller {

    this_ino : u64,

    ino_generator : Arc<InoGenerator>,
    registry : Registry,

    handle : Arc<Handler>,
}

impl Controller {

    pub fn get_handle(&self) -> Arc<Handler> {
        self.handle.clone()
    }


    pub(crate) fn create_from_driver(driver: &Driver, ino: u64, handle : Arc<Handler>) -> Self {
        Controller {
            this_ino : ino,

            ino_generator : driver.get_ino_generator(),
            registry : driver.get_registry(),

            handle,
        }
    }

    pub(crate) fn create_from_engine(engine: &Engine, ino: u64, handle : Arc<Handler>) -> Self {
        Controller {
            this_ino : ino,

            ino_generator : engine.get_ino_generator(),
            registry : engine.get_registry(),

            handle,
        }
    }

    // TODO: Get ID functions
    // TODO: Add Handlers to Engine functions
}
