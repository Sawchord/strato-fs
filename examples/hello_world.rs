extern crate env_logger;

use std::path::Path;
use std::thread;
use std::time::Duration;
use std::env;
use std::process::Command;

use strato::{Directory, Request};
use strato::handler::ProtectedHandle;
use strato::engine::Engine;
use strato::controller::Controller;
use strato::link::DirectoryEntry;

struct StaticDir {
    handle: Option<ProtectedHandle>,
    links : Vec<DirectoryEntry>
}

impl StaticDir {
    fn new() -> Self {
        StaticDir{
            handle: None,
            links : Vec::new(),
        }
    }
}

impl Directory for StaticDir {

    fn init(&mut self, controller: Controller) {
        println!("Init on static dir");
        self.handle = Some(controller.get_handle());
    }

    fn readdir(&mut self, controller: Controller, req: &Request) -> Option<Vec<DirectoryEntry>> {
        println!("Readdir on static dir");
        Some(vec!{
            DirectoryEntry::new(".".to_string(), self.handle.clone().unwrap()),
            DirectoryEntry::new("..".to_string(), self.handle.clone().unwrap()),
        })
    }

    fn lookup(&mut self, controller: Controller, req: &Request, name: String)
        -> Option<(DirectoryEntry, Duration)> {
        println!("Lookup on static dir");
        if name == "." || name == ".." {
            Some((DirectoryEntry::new(name, self.handle.clone().unwrap()), Duration::new(1, 0)))
        } else {
            None
        }

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

    let mut engine = Engine::new(&mountpoint);

    let dir_handle = Box::new(StaticDir::new());
    engine.add_directory_handle(dir_handle);

    match engine.start() {
        Err(error) => println!("{}", error),
        _ => {
            println!("Engine started");
            thread::park();
            ()
        }
    };

}