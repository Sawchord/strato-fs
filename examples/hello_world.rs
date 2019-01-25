extern crate env_logger;

use std::sync::Arc;
use std::ops::Deref;
use std::path::Path;
use std::thread;
use std::env;
use std::process::Command;

use parking_lot::RwLock;

use time;

use strato::{Node, Directory, File, Request};
use strato::handler::Handle;
use strato::engine::Engine;
use strato::controller::Controller;
use strato::link::DirectoryEntry;

struct StaticDirInner {
    handle: Option<Handle>,
    links : Vec<DirectoryEntry>
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

    fn add(&mut self, link: DirectoryEntry) {
        self.write().links.push(link);
    }

}

impl Node for StaticDir {

    fn init(&mut self, controller: Controller) {
        println!("Init on static dir");
        self.write().handle = Some(controller.get_handle());
    }

    fn read_attributes(&mut self, _req: &Request,
                       mut attr: DirectoryEntry) -> Option<DirectoryEntry> {
        println!("Requested attributes on static dir");

        attr.mtime(time::get_time());
        attr.ttl(time::get_time() + time::Duration::seconds(20));

        Some(attr)
    }

}

impl Directory for StaticDir {

    fn readdir(&mut self, _req: &Request) -> Option<Vec<DirectoryEntry>> {
        println!("Readdir on static dir");
        let mut vec = vec!{
                DirectoryEntry::new(".".to_string(), self.read().handle.clone().unwrap()),
                DirectoryEntry::new("..".to_string(), self.read().handle.clone().unwrap()),
            };
        vec.append(&mut self.read().links.clone());
        Some(vec)
    }

    fn lookup(&mut self, _req: &Request, name: String)
        -> Option<DirectoryEntry> {
        println!("Lookup on static dir, name: {}", name);
        if name == "." || name == ".." {
            return Some(DirectoryEntry::new(name, self.read().handle.clone().unwrap()))
        } else {
            for x in self.read().links.iter() {
                if x.get_name() == name {
                    return Some(x.clone());
                }
            }
        };
        None
    }
}


struct StaticFile {
    handle: Option<Handle>,
    text: String,
}

impl StaticFile {

    fn new(text: String) -> Self {
        StaticFile{
            handle: None,
            text,
        }
    }

}

impl Node for StaticFile {

    fn init(&mut self, controller: Controller) {
        println!("Init on static file");
        self.handle = Some(controller.get_handle());
    }

    fn read_attributes(&mut self, _req: &Request,
                       mut attr: DirectoryEntry) -> Option<DirectoryEntry> {
        println!("Requested attributes on static file");

        attr.mtime(time::get_time());
        attr.ttl(time::get_time() + time::Duration::seconds(20));

        attr.size(self.text.len() as u64);

        Some(attr)
    }

}

impl File for StaticFile {

    fn read(&mut self, _req: &Request) -> Option<Vec<u8>> {
        println!("Request read on static file");
        Some(self.text.clone().into_bytes())
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


    let text_handle = engine.add_file(StaticFile::new("Hello World\n".to_string()));
    root.add(DirectoryEntry::new("hello.txt".to_string(), text_handle));

    let text_handle = engine.add_file(StaticFile::new("Goodbeye World\n".to_string()));
    root.add(DirectoryEntry::new("goodbye.txt".to_string(), text_handle));


    match engine.start() {
        Err(error) => println!("{}", error),
        _ => {
            println!("Engine started");
            thread::park();
            ()
        }
    };

}