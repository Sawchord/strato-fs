use std::sync::Arc;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::io;
use std::thread;

use parking_lot::RwLock;

use fuse::BackgroundSession;

use libc::*;

use futures::sync::mpsc::{UnboundedSender, UnboundedReceiver};
use futures::stream::Stream;
use futures::future;
use futures::future::Future;
use tokio;

use crate::{File, Directory, Registry};
use crate::handler::{Handle, HandleDispatcher::*};
use crate::controller::Controller;
use crate::driver::{Driver, ChannelEvent};
use crate::utils::InoGenerator;
use crate::error::FileError;

macro_rules! get_handle {
    ($registry: ident, $ino: ident, $reply:ident) => [
        match $registry.read().get(&$ino) {
            None => {
                $reply.error(ENOENT);
                return future::ok(());
            }
            Some(i) => i
        }.clone()
    ];
}


#[derive(Debug)]
pub struct Engine<'a> {
    mount_point : PathBuf,
    registry : Registry,
    ino_generator : Arc<InoGenerator>,
    event_channel : Option<UnboundedSender<ChannelEvent>>,
    fuse_session : Option<BackgroundSession<'a>>,
}

impl<'a> Engine<'a> {

    pub fn new<T: 'static>(path: &Path, root: T) -> Self
    where T: Directory + Send + Sync {

        let mut engine = Engine{
            mount_point : path.to_path_buf(),
            registry : Arc::new(RwLock::new(BTreeMap::new())),
            ino_generator : Arc::new(InoGenerator::new()),
            event_channel : None,
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

        let (sender, receiver) = futures::sync::mpsc::unbounded::<ChannelEvent>();

        let driver = Driver::new(self.registry.clone(), self.ino_generator.clone(), sender.clone());
        let session = unsafe {fuse::spawn_mount(driver, &mount_point, &options[..])}?;

        // TODO: Give Fuse Session to tokio session? This way we can end tokio session and automatically end Fuse session
        self.event_channel = Some(sender);
        self.fuse_session = Some(session);

        Engine::start_tokio_runtime(self.registry.clone(), receiver);

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




    fn start_tokio_runtime(registry: Registry, channel: UnboundedReceiver<ChannelEvent>) {

        let runtime = channel.for_each(move |event| {
            println!("Inside the event channel {:?}", event);

            match event {

                ChannelEvent::Read{req, ino, fh, offset, size, reply} => {
                    let handle = get_handle!(registry, ino, reply);
                    let file_op = match handle.write().dispatch() {
                        RegularFile(ref mut file) => {
                            file.read(req)
                        }
                        _ => {
                            reply.error(EISDIR);
                            return future::ok(());
                        }
                    };
                    
                    let finish: Box<dyn Future<Item=(), Error=()> + Send>
                    = Box::new(file_op.then(move |result|{

                        match result {
                            Ok(vec) => {
                                println!("Request params: Offset {} Size {}", offset, size);
                                reply.data(&vec[offset as usize..]);
                            }
                            Err(error) => {
                                reply.error(error.get_libc_code());
                            }
                        }

                        future::ok(())
                    }));

                    tokio::executor::spawn(finish);
                },


            }
            future::ok(())
        });


        thread::spawn(move || {
            tokio::run(runtime);
        });

    }
}
