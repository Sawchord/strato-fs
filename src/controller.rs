use std::sync::Arc;
use std::collections::BTreeMap;

use parking_lot::RwLock;

use fuse::Request;

use crate::Registry;
use crate::handler::Handler;
use crate::utils::InoGenerator;
use crate::driver::Driver;

// TODO: Split controller into controller and request, Request changes, controller stays constant per request
/// This object gets handed down to functions implementing a File System handler trait, such as
/// File or Directory. The controller exposes information about the handlers context and can also
/// be used to manipulate (e.g. delete) the handler.
pub struct Controller {

    this_ino : u64,

    request_id : u64,
    uid : u32,
    gid : u32,
    pid : u32,

    ino_generator : Arc<InoGenerator>,
    registry : Registry

}

impl Controller {

    pub(crate) fn create(driver: &Driver, req: &Request, ino: u64) -> Self {
        Controller {
            this_ino : ino,

            request_id : req.unique(),
            uid : req.uid(),
            gid : req.gid(),
            pid : req.pid(),

            ino_generator : driver.get_ino_generator(),
            registry : driver.get_registry(),
        }
    }

    // TODO: Get ID functions
    // TODO: Add Handlers to Engine functions
}
