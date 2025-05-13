//! This module utilises the [fuse](https://github.com/libfuse/libfuse) API to mock a filesystem
//! that is stalling, say, due to network latency.
//!
//! As documented
//! [here](https://github.com/libfuse/libfuse?tab=readme-ov-file#supported-platforms),
//! libfuse targets Linux, BSD and OSX to some extent.
//! For now, we limit to Linux. libfuse package must be installed.

use fuser::Filesystem;
use fuser::ReplyAttr;
use fuser::ReplyData;
use fuser::ReplyDirectory;
use fuser::ReplyEmpty;
use fuser::ReplyEntry;
use fuser::ReplyOpen;
use fuser::Request;
use libc::{ENOENT, ENOSYS};
use std::ffi::OsStr;
use std::thread::sleep;
use std::time::Duration;

#[derive(Clone)]
pub struct StallingFilesystem {
    /// Duration before a reply is given.
    stall_dur: Duration,
}

impl Filesystem for StallingFilesystem {
    fn lookup(&mut self, _req: &Request, _parent_ino: u64, _name: &OsStr, reply: ReplyEntry) {
        sleep(self.stall_dur);
        reply.error(ENOENT);
    }

    fn getattr(&mut self, _req: &Request, _ino: u64, _fh: Option<u64>, reply: ReplyAttr) {
        sleep(self.stall_dur);
        reply.error(ENOENT);
    }

    fn readdir(
        &mut self,
        _req: &Request,
        _ino: u64,
        _fh: u64,
        _offset: i64,
        reply: ReplyDirectory,
    ) {
        sleep(self.stall_dur);
        reply.error(ENOENT);
    }

    fn open(&mut self, _req: &Request<'_>, _ino: u64, _flags: i32, reply: ReplyOpen) {
        sleep(self.stall_dur);
        reply.error(ENOENT); // Or EISDIR if it's a directory
    }

    fn read(
        &mut self,
        _req: &Request<'_>,
        _ino: u64,
        _fh: u64,
        _offset: i64,
        _size: u32,
        _flags: i32,
        _lock_owner: Option<u64>,
        reply: ReplyData,
    ) {
        sleep(self.stall_dur);
        reply.error(ENOSYS);
    }

    fn release(
        &mut self,
        _req: &Request<'_>,
        _ino: u64,
        _fh: u64,
        _flags: i32,
        _lock_owner: Option<u64>,
        _flush: bool,
        reply: ReplyEmpty,
    ) {
        reply.ok();
    }

    fn opendir(&mut self, _req: &Request<'_>, _ino: u64, _fh: i32, reply: ReplyOpen) {
        reply.error(ENOENT);
    }

    fn releasedir(
        &mut self,
        _req: &Request<'_>,
        _ino: u64,
        _fh: u64,
        _flags: i32,
        reply: ReplyEmpty,
    ) {
        reply.ok();
    }
}
