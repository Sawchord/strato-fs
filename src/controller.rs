use std::sync::Arc;
use std::collections::BTreeMap;

use parking_lot::RwLock;

use fuse::Request;

use crate::handler::Handler;
use crate::utils::InoGenerator;
use crate::driver::Driver;

type Registry = Arc<RwLock<BTreeMap<u64, Arc<Handler>>>>;

pub struct Controller {

    request_id : u64,
    uid : u32,
    gid : u32,
    pid : u32,

    ino_generator : Arc<InoGenerator>,
    registry : Registry

}

impl Controller {

    pub(crate) fn create(driver: &Driver, req: &Request) -> Self {
        Controller {
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
