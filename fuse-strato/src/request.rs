use std::path::PathBuf;

use std::ffi::OsString;

use fuse_sys::abi::*;
use fuse_sys::abi::consts::*;
use fuse_sys::abi::fuse_opcode::*;

#[derive(Debug)]
pub(crate) struct FuseRequest  {
    header: fuse_in_header,
    body: FuseRequestBody,
}


#[derive(Debug)]
pub(crate) enum FuseRequestBody {
    Init(fuse_init_in),
    Destroy(),
    // TODO: Unimplemented
    Interrupt(),
    Lookup(OsString),
    Forget(fuse_forget_in),
    GetAttr(),
    SetAttr(fuse_setattr_in),
    ReadLink(),
    MkNod(fuse_mknod_in),
    MkDir(OsString),
    Unlink(OsString),
    RmDir(OsString),
    // TODO: Implement Path
    Symlink(OsString, PathBuf),
    Rename(fuse_rename_in, OsString, OsString),
    Link(fuse_link_in, OsString),
    Open(fuse_open_in),
    Read(fuse_read_in),
    Write(fuse_write_in, Vec<u8>),
    Flush(fuse_flush_in),
    Release(fuse_release_in),
    FSync(fuse_fsync_in),
    OpenDir(fuse_open_in),
    ReadDir(fuse_read_in),
    ReleaseDir(fuse_release_in),
    FSyncDir(fuse_fsync_in),
    StatFS(),
    SetXAttr(fuse_setxattr_in),
    GetXAttr(fuse_getxattr_in),
    ListXAttr(fuse_getxattr_in),
    RemoveXAttr(OsString),
    Access(fuse_access_in),
    Create(fuse_open_in),
    GetLock(fuse_lk_in),
    SetLock(fuse_lk_in),
    Bmap(fuse_bmap_in),

    #[cfg(target_os = "macos")]
    SetVolumeName(OsString),

    #[cfg(target_os = "macos")]
    Exchange(fuse_exchange_in, OsString, OsString),

    #[cfg(target_os = "macos")]
    GetXTimes(),

}