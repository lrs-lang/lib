// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(dead_code)]

use std::ffi::{CStr};

use imp::cty::{c_int, mode_t, ssize_t, off_t, rlimit, pid_t, uid_t, gid_t,
               SYSCALL_RLIM_INFINITY, RLIM_INFINITY, statfs};
use imp::rust::{SaturatingCast};

pub use self::raw::*;

pub mod raw;

pub fn open(path: &CStr, flags: c_int, mode: mode_t) -> c_int {
    unsafe { __open(path.as_ptr(), flags, mode) }
}

pub fn read(fd: c_int, buf: &mut [u8]) -> ssize_t {
    unsafe { __read(fd, buf.as_mut_ptr() as *mut _, buf.len().saturating_cast()) }
}

pub fn write(fd: c_int, buf: &[u8]) -> ssize_t {
    unsafe { __write(fd, buf.as_ptr() as *const _, buf.len().saturating_cast()) }
}

pub fn pread(fd: c_int, buf: &mut [u8], offset: off_t) -> ssize_t {
    unsafe {
        __pread(fd, buf.as_mut_ptr() as *mut _, buf.len().saturating_cast(), offset)
    }
}

pub fn pwrite(fd: c_int, buf: &[u8], offset: off_t) -> ssize_t {
    unsafe {
        __pwrite(fd, buf.as_ptr() as *const _, buf.len().saturating_cast(), offset)
    }
}

pub fn readv(fd: c_int, bufs: &mut [&mut [u8]]) -> ssize_t {
    // XXX: iovec _MUST_ be the same as &mut [u8]
    unsafe {
        __readv(fd, bufs.as_mut_ptr() as *mut _, bufs.len().saturating_cast())
    }
}

pub fn writev(fd: c_int, bufs: &[&[u8]]) -> ssize_t {
    // XXX: iovec _MUST_ be the same as &mut [u8]
    unsafe {
        __writev(fd, bufs.as_ptr() as *const _, bufs.len().saturating_cast())
    }
}

pub fn preadv(fd: c_int, bufs: &mut [&mut [u8]], offset: off_t) -> ssize_t {
    // XXX: iovec _MUST_ be the same as &mut [u8]
    unsafe {
        __preadv(fd, bufs.as_mut_ptr() as *mut _, bufs.len().saturating_cast(), offset)
    }
}

pub fn pwritev(fd: c_int, bufs: &[&[u8]], offset: off_t) -> ssize_t {
    // XXX: iovec _MUST_ be the same as &mut [u8]
    unsafe {
        __pwritev(fd, bufs.as_ptr() as *const _, bufs.len().saturating_cast(), offset)
    }
}

pub fn getresuid(ruid: &mut uid_t, euid: &mut uid_t, suid: &mut uid_t) -> c_int {
    unsafe { __getresuid(ruid, euid, suid) }
}

pub fn getresgid(rgid: &mut gid_t, egid: &mut gid_t, sgid: &mut gid_t) -> c_int {
    unsafe { __getresgid(rgid, egid, sgid) }
}

pub fn getgroups(buf: &mut [gid_t]) -> c_int {
    unsafe { __getgroups(buf.len().saturating_cast(), buf.as_mut_ptr()) }
}

pub fn setgroups(buf: &[gid_t]) -> c_int {
    unsafe { __setgroups(buf.len().saturating_cast(), buf.as_ptr()) }
}

pub fn statfs(path: &CStr, buf: &mut statfs) -> c_int {
    unsafe { __statfs(path.as_ptr(), buf) }
}

pub fn fstatfs(fd: c_int, buf: &mut statfs) -> c_int {
    unsafe { __fstatfs(fd, buf) }
}

pub fn prlimit(pid: pid_t, res: c_int, new: Option<&rlimit>,
               old: Option<&mut rlimit>) -> c_int {
    macro_rules! fix {
        ($val:expr) => {
            if $val >= SYSCALL_RLIM_INFINITY { RLIM_INFINITY } else { $val }
        }
    };
    let tmp;
    let mut new = new;
    if let Some(new_v) = new {
        if SYSCALL_RLIM_INFINITY != RLIM_INFINITY {
            tmp = rlimit {
                rlim_cur: fix!(new_v.rlim_cur),
                rlim_max: fix!(new_v.rlim_max),
            };
            new = Some(&tmp);
        }
    }
    let new_p = new.map(|v| v as *const _).unwrap_or(0 as *const _);
    let old_p = old.as_ref().map(|v| *v as *mut _).unwrap_or(0 as *mut _);
    let ret = unsafe { __prlimit(pid, res, new_p, old_p) };
    if ret == 0 && SYSCALL_RLIM_INFINITY != RLIM_INFINITY {
        if let Some(old_v) = old {
            old_v.rlim_cur = fix!(old_v.rlim_cur);
            old_v.rlim_max = fix!(old_v.rlim_max);
        }
    }
    ret
}

pub fn getdents(fd: c_int, buf: &mut [u8]) -> c_int {
    unsafe {
        __getdents(fd, buf.as_mut_ptr() as *mut _, buf.len().saturating_cast())
    }
}
