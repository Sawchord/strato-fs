use time::Timespec;

use fuse::{FileType, FileAttr};

use crate::handler::{ProtectedHandle, HandleDispatcher::*};

macro_rules! getter {
    ($a: ident, $b:ident, $c:ty, $doc:tt) => {
        #[doc = $doc]
        pub fn $a(&self) -> $c {
            self.$b
        }
    }
}

macro_rules! setter {
    ($a: ident, $b:ty, $doc:tt) => [
        #[doc = $doc]
        pub fn $a(&mut self, val: $b) -> &Self {
            self.$a = val;
            self
        }
    ]
}


#[derive (Clone, Debug)]
pub struct DirectoryEntry {
    name : String,
    handle : ProtectedHandle,

    size: u64,

    atime: Timespec,
    mtime: Timespec,
    ctime: Timespec,
    crtime: Timespec,

    ttl: Timespec,
}


impl DirectoryEntry {

    /// Creates a directory entry
    pub fn new(name : String, handle : ProtectedHandle) -> Self {

        let epoch = Timespec::new(0, 0);

        DirectoryEntry {
            name,
            handle : handle.clone(),

            size: 0,

            atime: epoch,
            mtime: epoch,
            ctime: epoch,
            crtime: epoch,

            ttl: epoch,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }


    getter!(get_size, size, u64, "Returns the size of the entry");
    setter!(size, u64, "Set the size of the entry");

    getter!(get_atime, atime, Timespec, "Returns the last accessed time");
    setter!(atime, Timespec, "Set the time the file was last accessed.");

    getter!(get_mtime, mtime, Timespec, "Returns the last modified time");
    setter!(mtime, Timespec, "Set the time the file was last modified.");

    getter!(get_ctime, ctime, Timespec, "Returns the last changed time");
    setter!(ctime, Timespec, "Set the time the file was last changed.");

    getter!(get_crtime, crtime, Timespec, "Returns the created time");
    setter!(crtime, Timespec, "Set the time the file was created.");


    getter!(get_ttl, ttl, Timespec, "Set the time, this `Directory Entry` is considered valid.\
        This OS will cache the attributes for this time. Afterwards, the OS will query the \
        `getattr` again.");
    setter!(ttl, Timespec, "Get the time, this `Directory Entry` is considered valid.");
    

    pub(crate) fn to_attr(&self) -> FileAttr{

        let reader = self.handle.read();

        let file_type = match reader.dispatch_ref() {
            Dir(_) => FileType::Directory,
            RegularFile(_) => FileType::RegularFile,
        };

        // TODO: Let these values either be user settable or find a way to set them programmatically
        FileAttr {
            ino: reader.get_ino(),
            size: self.size,
            blocks: 1,
            atime: self.atime,
            mtime: self.mtime,
            ctime: self.ctime,
            crtime: self.crtime,
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
            Dir(_) => {
                (self.handle.read().get_ino(), FileType::Directory, self.name.clone())
            }
            RegularFile(_) => {
                (self.handle.read().get_ino(), FileType::RegularFile, self.name.clone())
            }
        }

    }

}