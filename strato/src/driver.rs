use std::sync::Arc;
use std::ffi::OsStr;

use libc::*;

use fuse::{Filesystem, ReplyDirectory, ReplyData, ReplyEntry, ReplyAttr};
use fuse::Request as FuseRequest;

use futures::sync::mpsc::UnboundedSender;

use crate::handler::HandleDispatcher::*;
use crate::utils::InoGenerator;
use crate::link::NodeEntry;
use crate::controller::Request;
use crate::Registry;


/// This macro looks up the ino from the registry and returns the corresponding handler
/// It sends an `ENOENT` to the FUSE driver, if the ino does not exist.
macro_rules! get_handle {
    ($driver:ident, $ino: ident, $reply:ident) => [
        match $driver.registry.read().get(&$ino) {
            None => {
                $reply.error(ENOENT);
                return;
            }
            Some(i) => i
        }.clone()
    ];
}


pub(crate) struct Driver {
    registry : Registry,
    channel : UnboundedSender<ChannelEvent>
    //ino_generator : Arc<InoGenerator>,
}

impl Driver {

    pub(crate) fn new(registry: Registry, _ino_generator : Arc<InoGenerator>,
    channel : UnboundedSender<ChannelEvent>) -> Self {
        Driver {
            registry : registry.clone(),
            channel,
            //ino_generator : ino_generator.clone(),
        }
    }

}


// TODO: Implement macros to check if directory or file with appropriate errors
impl Filesystem for Driver {

    fn lookup(&mut self, req: &FuseRequest, parent: u64, name: &OsStr, reply: ReplyEntry) {

        let handle = get_handle!(self, parent, reply);
        let request = Request::new(req);
        let n = name.to_string_lossy().to_string();

        let result = match handle.write().dispatch() {
            Dir(ref mut dir) => {
                dir.lookup(request, n)
            }
            _ => {
                reply.error(ENOTDIR);
                return;
            }
        };

        match result {
            Ok(entry) => {
                // TODO: What does Generation do?
                reply.entry(&entry.get_ttl(), &entry.to_attr(), 0);
            },
            Err(error) => { reply.error(error.get_libc_code()); }
        }

    }

    fn getattr(&mut self, req: &FuseRequest, ino: u64, reply: ReplyAttr) {

        let handle = get_handle!(self, ino, reply);
        let request = Request::new(req);
        let base_entry = NodeEntry::new("".to_string(), handle.clone());

        let result = match handle.write().dispatch() {
            Dir(ref mut dir) => {
                dir.read_attributes(request, base_entry)
            }
            RegularFile(ref mut file) => {
                file.read_attributes(request, base_entry)
            }
        };

        match result {
            Ok(entry) => reply.attr(&entry.get_ttl(), &entry.to_attr()),
            Err(error) => reply.error(error.get_libc_code()),
        }

    }

    // TODO: Implement correct behaviour of offset and size... how to handle streaming?
    fn read(&mut self, request: &FuseRequest, ino: u64, fh: u64,
            offset: i64, size: u32, reply: ReplyData) {
        let req = Request::new(request);
        self.channel.unbounded_send(ChannelEvent::Read {
            req,
            ino,
            fh,
            offset,
            size,
            reply
        });
//        let handle = get_handle!(self, ino, reply);
//        let result = match handle.write().dispatch() {
//            RegularFile(ref mut file) => {
//                file.read(req)
//            }
//            _ => {
//                reply.error(EISDIR);
//            return;
//            }
//        };
//
//        match result {
//            Ok(vec) => {
//                println!("Request params: Offset {} Size {}", offset, size);
//                reply.data(&vec[offset as usize..]);
//            }
//            Err(error) => {
//                reply.error(error.get_libc_code());
//            }
//        }
    }

    fn readdir(&mut self, req: &FuseRequest, ino: u64, _fh: u64,
               offset: i64, mut reply: ReplyDirectory) {

        let handle = get_handle!(self, ino, reply);
        let request = Request::new(req);
        // Check that the handle references a directory
        let result = match handle.write().dispatch() {
            // Check that this is actually a directory
            Dir(ref mut dir) => {
                dir.readdir(request)
            },
            _ => {
                reply.error(ENOTDIR);
                return;
            },
        };

        match result {
            Ok(vec) => {
                let to_skip = if offset == 0 { offset } else { offset + 1 } as usize;
                for (i, entry) in vec.into_iter().enumerate().skip(to_skip) {

                    let rep = entry.to_reply();
                    reply.add(rep.0, i as i64,rep.1, rep.2);

                }
                reply.ok();
            },
            Err(error) => {
                reply.error(error.get_libc_code());
            }
        }
    }

}

#[derive(Debug)]
pub(crate) enum ChannelEvent {
    Read{req: Request, ino: u64, fh: u64, offset: i64, size: u32, reply: ReplyData},
}




