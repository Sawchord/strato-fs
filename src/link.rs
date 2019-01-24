use time::Timespec;

use fuse::{FileType, FileAttr};

use crate::handler::{ProtectedHandle, HandleDispatcher};

#[derive (Clone)]
pub struct DirectoryEntry {
    name : String,
    handle : ProtectedHandle,

    atime: Option<Timespec>,
    mtime: Option<Timespec>,
    ctime: Option<Timespec>,
    crtime: Option<Timespec>,

    ttl: Option<Timespec>,
}


impl DirectoryEntry {

    /// Creates a directory entry
    pub fn new(name : String, handle : ProtectedHandle) -> Self {
        DirectoryEntry {
            name,
            handle : handle.clone(),

            atime: None,
            mtime: None,
            ctime: None,
            crtime: None,

            ttl: None,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn mtime(&mut self, time:Timespec) -> &Self {
        self.mtime = Some(time);
        self
    }



    /// Set the time, this `Directory Entry` is considered valid.
    /// This OS will cache the attributes for this time.
    /// Afterwards, the OS will query the `getattr` again.
    pub fn ttl(&mut self, ttl:Timespec) -> &Self {
        self.ttl = Some(ttl);
        self
    }

    pub fn get_ttl(&self) -> Timespec {
        self.ttl.unwrap_or(Timespec::new(1,0))
    }


    pub(crate) fn to_attr(&self) -> FileAttr{

        let reader = self.handle.read();

        let file_type = match reader.dispatch_ref() {
            HandleDispatcher::Dir(_) => FileType::Directory,
            HandleDispatcher::File(_) => FileType::RegularFile,
        };

        let time_none = Timespec::new(0, 0);

        // TODO: Let these values either be user settable or find a way to set them programatically
        FileAttr {
            ino: reader.get_ino(),
            size: 4096,
            blocks: 1,
            atime: self.atime.unwrap_or(time_none),
            mtime: self.mtime.unwrap_or(time_none),
            ctime: self.ctime.unwrap_or(time_none),
            crtime: self.crtime.unwrap_or(time_none),
            kind: file_type,
            perm: 0o744,
            nlink: 1,
            uid: 1000,
            gid: 1000,
            rdev: 0,
            flags: 0,
        }

    }

    pub(crate) fn to_reply(&self) -> (u64, FileType, String) {
        match self.handle.read().dispatch_ref() {
            HandleDispatcher::Dir(_) => {
                (self.handle.read().get_ino(), FileType::Directory, self.name.clone())
            }
            HandleDispatcher::File(_) => {
                (self.handle.read().get_ino(), FileType::RegularFile, self.name.clone())
            }
        }

    }

}