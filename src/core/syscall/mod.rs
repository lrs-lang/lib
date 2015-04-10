// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{mem};

use c_str::{CStr};
use cty::{c_int, mode_t, ssize_t, off_t, rlimit, pid_t, uid_t, gid_t, stat, c_char,
          size_t, SYSCALL_RLIM_INFINITY, RLIM_INFINITY, statfs, timespec, dev_t, c_void,
          clockid_t, itimerspec, epoll_event, sigset_t};
use ext::{SaturatingCast};

pub use self::raw::*;

pub mod raw;

pub fn openat(dir: c_int, path: &CStr, flags: c_int, mode: mode_t) -> c_int {
    unsafe { __openat(dir, path.as_ptr(), flags, mode) }
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

pub fn fstatat(dir: c_int, file: &CStr, buf: &mut stat, flags: c_int) -> c_int {
    unsafe { __fstatat(dir, file.as_ptr(), buf, flags) }
}

pub fn faccessat(dir: c_int, file: &CStr, mode: c_int) -> c_int {
    unsafe { __faccessat(dir, file.as_ptr(), mode) }
}

pub fn truncate(file: &CStr, len: off_t) -> c_int {
    unsafe { __truncate(file.as_ptr(), len) }
}

pub fn linkat(olddir: c_int, oldfile: &CStr, newdir: c_int, newfile: &CStr,
              flags: c_int) -> c_int {
    unsafe { __linkat(olddir, oldfile.as_ptr(), newdir, newfile.as_ptr(), flags) }
}

pub fn utimensat(dir: c_int, file: Option<&CStr>, times: &[timespec; 2],
                 flags: c_int) -> c_int {
    let file = file.map(|f| f.as_ptr()).unwrap_or(0 as *const _);
    unsafe { __utimensat(dir, file, times.as_ptr(), flags) }
}

pub fn renameat2(olddir: c_int, oldfile: &CStr, newdir: c_int, newfile: &CStr,
                 flags: c_int) -> c_int {
    unsafe { __renameat2(olddir, oldfile.as_ptr(), newdir, newfile.as_ptr(), flags) }
}

pub fn mkdirat(dir: c_int, file: &CStr, mode: mode_t) -> c_int {
    unsafe {  __mkdirat(dir, file.as_ptr(), mode) }
}

pub fn unlinkat(dir: c_int, file: &CStr, flags: c_int) -> c_int {
    unsafe { __unlinkat(dir, file.as_ptr(), flags) }
}

pub fn symlinkat(target: &CStr, dir: c_int, link: &CStr) -> c_int {
    unsafe { __symlinkat(target.as_ptr(), dir, link.as_ptr()) }
}

pub fn readlinkat(dir: c_int, path: &CStr, buf: &mut [u8]) -> ssize_t {
    unsafe { __readlinkat(dir, path.as_ptr(), buf.as_mut_ptr() as *mut c_char,
                          buf.len().saturating_cast()) }
}

pub fn fchownat(dir: c_int, path: &CStr, user: uid_t, group: gid_t,
                flags: c_int) -> c_int {
    unsafe { __fchownat(dir, path.as_ptr(), user, group, flags) }
}

pub fn fchmodat(dir: c_int, path: &CStr, mode: mode_t) -> c_int {
    unsafe { __fchmodat(dir, path.as_ptr(), mode) }
}

pub fn mknodat(dir: c_int, path: &CStr, mode: mode_t, dev: dev_t) -> c_int {
    unsafe { __mknodat(dir, path.as_ptr(), mode, dev) }
}

pub fn setxattr(path: &CStr, name: &CStr, val: &[u8], flags: c_int) -> c_int {
    unsafe { __setxattr(path.as_ptr(), name.as_ptr(), val.as_ptr() as *const c_void,
                        val.len().saturating_cast(), flags) }
}

pub fn lsetxattr(path: &CStr, name: &CStr, val: &[u8], flags: c_int) -> c_int {
    unsafe { __lsetxattr(path.as_ptr(), name.as_ptr(), val.as_ptr() as *const c_void,
                         val.len().saturating_cast(), flags) }
}

pub fn fsetxattr(fd: c_int, name: &CStr, val: &[u8], flags: c_int) -> c_int {
    unsafe { __fsetxattr(fd, name.as_ptr(), val.as_ptr() as *const c_void,
                         val.len().saturating_cast(), flags) }
}

pub fn getxattr(path: &CStr, name: &CStr, val: &mut [u8]) -> ssize_t {
    unsafe { __getxattr(path.as_ptr(), name.as_ptr(), val.as_mut_ptr() as *mut c_void,
                        val.len().saturating_cast()) }
}

pub fn lgetxattr(path: &CStr, name: &CStr, val: &mut [u8]) -> ssize_t {
    unsafe { __lgetxattr(path.as_ptr(), name.as_ptr(), val.as_mut_ptr() as *mut c_void,
                         val.len().saturating_cast()) }
}

pub fn fgetxattr(fd: c_int, name: &CStr, val: &mut [u8]) -> ssize_t {
    unsafe { __fgetxattr(fd, name.as_ptr(), val.as_mut_ptr() as *mut c_void,
                         val.len().saturating_cast()) }
}

pub fn removexattr(path: &CStr, name: &CStr) -> c_int {
    unsafe { __removexattr(path.as_ptr(), name.as_ptr()) }
}

pub fn lremovexattr(path: &CStr, name: &CStr) -> c_int {
    unsafe { __lremovexattr(path.as_ptr(), name.as_ptr()) }
}

pub fn fremovexattr(fd: c_int, name: &CStr) -> c_int {
    unsafe { __fremovexattr(fd, name.as_ptr()) }
}

pub fn listxattr(path: &CStr, list: &mut [u8]) -> ssize_t {
    unsafe { __listxattr(path.as_ptr(), list.as_mut_ptr() as *mut c_char,
                         list.len().saturating_cast()) }
}

pub fn llistxattr(path: &CStr, list: &mut [u8]) -> ssize_t {
    unsafe { __llistxattr(path.as_ptr(), list.as_mut_ptr() as *mut c_char,
                          list.len().saturating_cast()) }
}

pub fn flistxattr(fd: c_int, list: &mut [u8]) -> ssize_t {
    unsafe { __flistxattr(fd, list.as_mut_ptr() as *mut c_char, list.len().saturating_cast()) }
}

pub fn clock_getres(clock: clockid_t, res: &mut timespec) -> c_int {
    unsafe { __clock_getres(clock, res) }
}

pub fn clock_gettime(clock: clockid_t, res: &mut timespec) -> c_int {
    unsafe { __clock_gettime(clock, res) }
}

pub fn clock_settime(clock: clockid_t, res: &timespec) -> c_int {
    unsafe { __clock_settime(clock, res) }
}

pub fn clock_nanosleep(clock: clockid_t, flags: c_int, req: &timespec,
                       rem: &mut timespec) -> c_int {
    unsafe { __clock_nanosleep(clock, flags, req, rem) }
}

pub fn timerfd_settime(fd: c_int, flags: c_int, new: &itimerspec,
                       old: Option<&mut itimerspec>) -> c_int {
    let old = match old {
        Some(old) => old as *mut _,
        _ => 0 as *mut _,
    };
    unsafe { __timerfd_settime(fd, flags, new, old) }
}

pub fn timerfd_gettime(fd: c_int, cur: &mut itimerspec) -> c_int {
    unsafe { __timerfd_gettime(fd, cur) }
}

pub fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int,
                 event: Option<&mut epoll_event>) -> c_int {
    let event = match event {
        Some(event) => event as *mut _,
        _ => 0 as *mut _,
    };
    unsafe { __epoll_ctl(epfd, op, fd, event) }
}

pub fn epoll_pwait(epfd: c_int, events: &mut [epoll_event], timeout: c_int,
                   sigmask: Option<&sigset_t>) -> c_int {
    let sigmask = match sigmask {
        Some(sigmask) => sigmask as *const _,
        _ => 0 as *const _,
    };
    unsafe { __epoll_pwait(epfd, events.as_mut_ptr(), events.len().saturating_cast(),
                           timeout, sigmask, mem::size_of::<sigset_t>() as size_t) }
}

pub fn sched_getaffinity(tid: pid_t, set: &mut [u8]) -> c_int {
    unsafe { __sched_getaffinity(tid, set.len().saturating_cast(), set.as_mut_ptr()) }
}
