use std::sync::Arc;
use std::collections::BTreeMap;

use parking_lot::RwLock;

use crate::handler::Handler;
use crate::utils::InoGenerator;

type Registry = Arc<RwLock<BTreeMap<u64, Arc<Handler>>>>;

pub struct Controller {

    request_id : u64,
    uid : u32,
    gid : u32,
    pid : u32,

    ino_generator : Arc<InoGenerator>,
    registry : Registry

}

