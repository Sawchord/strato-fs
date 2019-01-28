extern crate env_logger;

use std::sync::Arc;
use std::ops::Deref;
use std::path::Path;
use std::thread;
use std::env;
use std::process::Command;

use parking_lot::RwLock;

//use futures::future;
//use futures::future::Future;

use tokio::prelude::*;

use time;

use strato::{Node, Directory, File, Request};
use strato::error::{FileError, DirError, NodeError};
use strato::Handle;
use strato::Engine;
use strato::Controller;
use strato::link::NodeEntry;

struct StaticDirInner {
    handle: Option<Handle>,
    links : Vec<NodeEntry>
}

#[derive(Clone)]
struct StaticDir(Arc<RwLock<StaticDirInner>>);

impl Deref for StaticDir {
    type Target = Arc<RwLock<StaticDirInner>>;
    fn deref(&self) -> &Arc<RwLock<StaticDirInner>> {
        &self.0
    }
}

impl StaticDir {

    fn new() -> Self {
        StaticDir(Arc::new(RwLock::new(
            StaticDirInner{
                handle: None,
                links : Vec::new(),
            }
        )))
    }

    fn add(&mut self, link: NodeEntry) {
        self.write().links.push(link);
    }

}

impl Node for StaticDir {

    fn init(&mut self, controller: Controller) {
        println!("Init on static dir");
        self.write().handle = Some(controller.get_handle());
    }

    fn read_attributes(&mut self, _req: Request,
                       mut attr: NodeEntry) -> Result<NodeEntry, NodeError> {
        //println!("Requested attributes on static dir");

        attr.mtime(time::get_time());
        attr.ttl(time::get_time() + time::Duration::seconds(20));

        Ok(attr)
    }

}

impl Directory for StaticDir {

    fn readdir(&mut self, _req: Request) -> Result<Vec<NodeEntry>, DirError> {
        println!("Readdir on static dir");
        let mut vec = vec!{
                NodeEntry::new(".".to_string(), self.read().handle.clone().unwrap()),
                NodeEntry::new("..".to_string(), self.read().handle.clone().unwrap()),
            };
        vec.append(&mut self.read().links.clone());
        Ok(vec)
    }

    fn lookup(&mut self, _req: Request, name: String) -> Result<NodeEntry, NodeError> {
        println!("Lookup on static dir, name: {}", name);
        if name == "." || name == ".." {
            return Ok(NodeEntry::new(name, self.read().handle.clone().unwrap()))
        } else {
            for x in self.read().links.iter() {
                if x.get_name() == name {
                    return Ok(x.clone());
                }
            }
        };
        Err(NodeError::new(NodeError::NoSuchEntry))
    }
}

#[derive(Clone, Debug)]
struct StaticFile {
    handle: Option<Handle>,
    text: String,
    delay: u32,
}

impl StaticFile {

    fn new(text: String, delay: u32) -> Self {
        StaticFile{
            handle: None,
            text,
            delay
        }
    }

}

impl Node for StaticFile {

    fn init(&mut self, controller: Controller) {
        println!("Init on static file");
        self.handle = Some(controller.get_handle());
    }

    fn read_attributes(&mut self, _req: Request, mut attr: NodeEntry)
            -> Result<NodeEntry, NodeError> {

        //println!("Requested attributes on static file");

        attr.mtime(time::get_time() - time::Duration::seconds(20));
        attr.ttl(time::get_time() + time::Duration::seconds(1));

        attr.size(self.text.len() as u64);

        Ok(attr)
    }

}

impl File for StaticFile {

    fn read(&mut self, _req: Request) -> Box<Future<Item=Vec<u8>, Error=FileError> + Send> {
        println!("Request read on static file");

        let del = std::time::Instant::now() + std::time::Duration::from_secs(self.delay as u64);
        let cl = self.clone();

        Box::new(tokio::timer::Delay::new(del)
            .then( move |_| {
                future::ok(cl.text.clone().into_bytes())
            }))

    }

}



fn main() {
    env_logger::init();

    let mountpoint = Path::new(&env::args_os().nth(1).unwrap()).to_owned();
    let mountpoint_str = mountpoint.to_str().unwrap();
    println!("File will be mounted system at {}", mountpoint_str);


    // The mountpoint might be in an erroneous state, due to a prior test run of an example.
    // We unmount it beforehand to ensure that it is in a known state.
    let status = Command::new("fusermount")
        .arg("-u")
        .arg(mountpoint_str)
        .status()
        .expect("Failed to execute fusermount");


    if status.success() {
        println!("Unmounted old {}", mountpoint_str);
    } else {
        println!("{} was already unmounted", mountpoint_str);
    }



    // This is the actual user code
    // This should make a nice API someday

    let mut root = StaticDir::new();
    let mut engine = Engine::new(&mountpoint, root.clone());


    let text_handle = engine.add_file(StaticFile::new("Hello World\n".to_string(), 10));
    root.add(NodeEntry::new("hello.txt".to_string(), text_handle));

    let text_handle = engine.add_file(StaticFile::new("Goodbeye World\n".to_string(), 5));
    root.add(NodeEntry::new("goodbye.txt".to_string(), text_handle));


    match engine.start() {
        Err(error) => println!("{}", error),
        _ => {
            println!("Engine started");
            thread::park();
            ()
        }
    };

}