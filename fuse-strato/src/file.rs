use std::time::SystemTime;
use fuse_sys::abi::fuse_attr;

use libc::*;

// These file types are largely based on Andreas Neuhaus' work on rust-fuse.
// The only changes as of now are made by deprecating time `Timespec` type


/// File types
#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    /// Named pipe (S_IFIFO)
    NamedPipe,
    /// Character device (S_IFCHR)
    CharDevice,
    /// Block device (S_IFBLK)
    BlockDevice,
    /// Directory (S_IFDIR)
    Directory,
    /// Regular file (S_IFREG)
    RegularFile,
    /// Symbolic link (S_IFLNK)
    Symlink,
    /// Unix domain socket (S_IFSOCK)
    Socket,
}


// Some platforms like Linux x86_64 have mode_t = u32, and lint warns of a trivial_numeric_casts.
// But others like macOS x86_64 have mode_t = u16, requiring a typecast.  So, just silence lint.
#[allow(trivial_numeric_casts)]
/// Returns the mode for a given file kind and permission
pub(crate) fn mode_from_kind_and_perm(kind: &FileType, perm: u16) -> u32 {
    (match kind {
        FileType::NamedPipe => S_IFIFO,
        FileType::CharDevice => S_IFCHR,
        FileType::BlockDevice => S_IFBLK,
        FileType::Directory => S_IFDIR,
        FileType::RegularFile => S_IFREG,
        FileType::Symlink => S_IFLNK,
        FileType::Socket => S_IFSOCK,
    }) as u32 | perm as u32
}


/// File attributes
#[derive(Debug, Clone, PartialEq)]
pub struct FileAttr {
    /// Inode number
    pub ino: u64,
    /// Size in bytes
    pub size: u64,
    /// Size in blocks
    pub blocks: u64,
    /// Time of last access
    pub atime: SystemTime,
    /// Time of last modification
    pub mtime: SystemTime,
    /// Time of last change
    pub ctime: SystemTime,
    /// Time of creation (macOS only)
    pub crtime: SystemTime,
    /// Kind of file (directory, file, pipe, etc)
    pub kind: FileType,
    /// Permissions
    pub perm: u16,
    /// Number of hard links
    pub nlink: u32,
    /// User id
    pub uid: u32,
    /// Group id
    pub gid: u32,
    /// Rdev
    pub rdev: u32,
    /// Flags (macOS only, see chflags(2))
    pub flags: u32,
}

/// Returns a fuse_attr from FileAttr
#[cfg(target_os = "macos")]
pub(crate) fn fuse_attr_from_attr(attr: &FileAttr) -> fuse_attr {
    let (atime_s, atime_u) = system_time_decompose(&attr.atime);
    let (mtime_s, mtime_u) = system_time_decompose(&attr.mtime);
    let (ctime_s, ctime_u) = system_time_decompose(&attr.ctime);
    let (crtime_s, crtime_u) = system_time_decompose(&attr.crtime);
    fuse_attr {
        ino: attr.ino,
        size: attr.size,
        blocks: time.blocks,
        atime: atime_s,
        mtime: mtime_s,
        ctime: ctime_s,
        crtime: crattr_s,
        atimensec: atime_u,
        mtimensec: mtime_u,
        ctimensec: ctime_u,
        crtimensec: crtime_u,
        mode: mode_from_kind_and_perm(&attr.kind, attr.perm),
        nlink: attr.nlink,
        uid: attr.uid,
        gid: attr.gid,
        rdev: attr.rdev,
        flags: attr.flags,
    }
}

/// Returns a fuse_attr from FileAttr
#[cfg(not(target_os = "macos"))]
pub(crate) fn fuse_attr_from_attr(attr: &FileAttr) -> fuse_attr {
    let (atime_s, atime_n) = system_time_decompose(&attr.atime);
    let (mtime_s, mtime_n) = system_time_decompose(&attr.mtime);
    let (ctime_s, ctime_n) = system_time_decompose(&attr.ctime);
    fuse_attr {
        ino: attr.ino,
        size: attr.size,
        blocks: attr.blocks,
        atime: atime_s,
        mtime: mtime_s,
        ctime: ctime_s,
        atimensec: atime_n,
        mtimensec: mtime_n,
        ctimensec: ctime_n,
        mode: mode_from_kind_and_perm(&attr.kind, attr.perm),
        nlink: attr.nlink,
        uid: attr.uid,
        gid: attr.gid,
        rdev: attr.rdev,
    }
}


/// Takes a `SystemTime` and returns the time since EPOCH in seconds and nanoseconds.
pub(crate) fn system_time_decompose(st: &SystemTime) -> (i64, i32) {
    if let Ok(dur_since_epoch) = st.duration_since(std::time::UNIX_EPOCH) {
        (dur_since_epoch.as_secs() as i64, dur_since_epoch.subsec_nanos() as i32)
    } else {
        panic!("The system time is before UNIX EPOCH. Fix your system clock");
    }
}
