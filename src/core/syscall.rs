// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{mem};

use c_str::{CStr};
use cty::{c_int, mode_t, ssize_t, rlimit64, pid_t, uid_t, gid_t, stat, c_char,
          size_t, statfs, timespec, dev_t, c_void, clockid_t, itimerspec, epoll_event,
          sigset_t, new_utsname, sysinfo, c_uint, c_ulong, umode_t, k_uint, loff_t,
          k_ulong};
use ext::{SaturatingCast};
use raw_syscall as r;

// XXX: iovec _MUST_ be the same as &mut [u8]

pub fn openat(dir: c_int, path: &CStr, flags: c_int, mode: mode_t) -> c_int {
    unsafe { r::openat(dir, path.as_ptr(), flags, mode as umode_t) }
}

pub fn read(fd: c_int, buf: &mut [u8]) -> ssize_t {
    unsafe {
        r::read(fd as k_uint, buf.as_mut_ptr() as *mut _, buf.len().saturating_cast())
    }
}

pub fn write(fd: c_int, buf: &[u8]) -> ssize_t {
    unsafe {
        r::write(fd as k_uint, buf.as_ptr() as *const _, buf.len().saturating_cast())
    }
}

pub fn pread(fd: c_int, buf: &mut [u8], offset: loff_t) -> ssize_t {
    unsafe {
        r::pread(fd as k_uint, buf.as_mut_ptr() as *mut _, buf.len().saturating_cast(),
                 offset)
    }
}

pub fn pwrite(fd: c_int, buf: &[u8], offset: loff_t) -> ssize_t {
    unsafe {
        r::pwrite(fd as k_uint, buf.as_ptr() as *const _, buf.len().saturating_cast(),
                  offset)
    }
}

pub fn readv(fd: c_int, bufs: &mut [&mut [u8]]) -> ssize_t {
    unsafe {
        r::readv(fd as k_ulong, bufs.as_mut_ptr() as *mut _, bufs.len().saturating_cast())
    }
}

pub fn writev(fd: c_int, bufs: &[&[u8]]) -> ssize_t {
    unsafe {
        r::writev(fd as k_ulong, bufs.as_ptr() as *const _, bufs.len().saturating_cast())
    }
}

pub fn preadv(fd: c_int, bufs: &mut [&mut [u8]], offset: loff_t) -> ssize_t {
    let lo = ((offset as u64) & 0xFFFF_FFFF) as k_ulong;
    let hi = ((offset as u64) > 32) as k_ulong;
    unsafe {
        r::preadv(fd as k_ulong, bufs.as_mut_ptr() as *mut _,
                  bufs.len().saturating_cast(), lo, hi)
    }
}

pub fn pwritev(fd: c_int, bufs: &[&[u8]], offset: loff_t) -> ssize_t {
    let lo = ((offset as u64) & 0xFFFF_FFFF) as k_ulong;
    let hi = ((offset as u64) > 32) as k_ulong;
    unsafe {
        r::pwritev(fd as k_ulong, bufs.as_ptr() as *const _, bufs.len().saturating_cast(),
                   lo, hi)
    }
}

pub fn getresuid(ruid: &mut uid_t, euid: &mut uid_t, suid: &mut uid_t) -> c_int {
    unsafe { r::getresuid(ruid, euid, suid) }
}

pub fn getresgid(rgid: &mut gid_t, egid: &mut gid_t, sgid: &mut gid_t) -> c_int {
    unsafe { r::getresgid(rgid, egid, sgid) }
}

pub fn getgroups(buf: &mut [gid_t]) -> c_int {
    unsafe { r::getgroups(buf.len().saturating_cast(), buf.as_mut_ptr()) }
}

pub fn setgroups(buf: &[gid_t]) -> c_int {
    unsafe { r::setgroups(buf.len().saturating_cast(), buf.as_ptr() as *mut _) }
}

pub fn statfs(path: &CStr, buf: &mut statfs) -> c_int {
    unsafe { r::statfs(path.as_ptr(), buf) }
}

pub fn fstatfs(fd: c_int, buf: &mut statfs) -> c_int {
    unsafe { r::fstatfs(fd as k_uint, buf) }
}

pub fn prlimit(pid: pid_t, res: c_int, new: Option<&rlimit64>,
               old: Option<&mut rlimit64>) -> c_int {
    let new_p = new.map(|v| v as *const _).unwrap_or(0 as *const _);
    let old_p = old.as_ref().map(|v| *v as *mut _).unwrap_or(0 as *mut _);
    unsafe { r::prlimit(pid, res as k_uint, new_p, old_p) }
}

pub fn getdents(fd: c_int, buf: &mut [u8]) -> c_int {
    unsafe {
        r::getdents(fd as k_uint, buf.as_mut_ptr() as *mut _, buf.len().saturating_cast())
    }
}

pub fn fstatat(dir: c_int, file: &CStr, buf: &mut stat, flags: c_int) -> c_int {
    unsafe { r::fstatat(dir, file.as_ptr(), buf, flags) }
}

pub fn faccessat(dir: c_int, file: &CStr, mode: c_int) -> c_int {
    unsafe { r::faccessat(dir, file.as_ptr(), mode) }
}

pub fn truncate(file: &CStr, len: loff_t) -> c_int {
    unsafe { r::truncate(file.as_ptr(), len) }
}

pub fn linkat(olddir: c_int, oldfile: &CStr, newdir: c_int, newfile: &CStr,
              flags: c_int) -> c_int {
    unsafe { r::linkat(olddir, oldfile.as_ptr(), newdir, newfile.as_ptr(), flags) }
}

pub fn utimensat(dir: c_int, file: Option<&CStr>, times: &[timespec; 2],
                 flags: c_int) -> c_int {
    let file = file.map(|f| f.as_ptr()).unwrap_or(0 as *const _);
    unsafe { r::utimensat(dir, file, times.as_ptr(), flags) }
}

pub fn renameat(olddir: c_int, oldfile: &CStr, newdir: c_int, newfile: &CStr,
                 flags: c_int) -> c_int {
    unsafe {
        r::renameat(olddir, oldfile.as_ptr(), newdir, newfile.as_ptr(), flags as k_uint)
    }
}

pub fn mkdirat(dir: c_int, file: &CStr, mode: mode_t) -> c_int {
    unsafe {  r::mkdirat(dir, file.as_ptr(), mode as umode_t) }
}

pub fn unlinkat(dir: c_int, file: &CStr, flags: c_int) -> c_int {
    unsafe { r::unlinkat(dir, file.as_ptr(), flags) }
}

pub fn symlinkat(target: &CStr, dir: c_int, link: &CStr) -> c_int {
    unsafe { r::symlinkat(target.as_ptr(), dir, link.as_ptr()) }
}

pub fn readlinkat(dir: c_int, path: &CStr, buf: &mut [u8]) -> ssize_t {
    unsafe { r::readlinkat(dir, path.as_ptr(), buf.as_mut_ptr() as *mut c_char,
                          buf.len().saturating_cast()) }
}

pub fn fchownat(dir: c_int, path: &CStr, user: uid_t, group: gid_t,
                flags: c_int) -> c_int {
    unsafe { r::fchownat(dir, path.as_ptr(), user, group, flags) }
}

pub fn fchmodat(dir: c_int, path: &CStr, mode: mode_t) -> c_int {
    unsafe { r::fchmodat(dir, path.as_ptr(), mode as umode_t) }
}

pub fn mknodat(dir: c_int, path: &CStr, mode: mode_t, dev: dev_t) -> c_int {
    unsafe { r::mknodat(dir, path.as_ptr(), mode as umode_t, dev) }
}

pub fn setxattr(path: &CStr, name: &CStr, val: &[u8], flags: c_int) -> c_int {
    unsafe { r::setxattr(path.as_ptr(), name.as_ptr(), val.as_ptr() as *const c_void,
                        val.len().saturating_cast(), flags) }
}

pub fn lsetxattr(path: &CStr, name: &CStr, val: &[u8], flags: c_int) -> c_int {
    unsafe { r::lsetxattr(path.as_ptr(), name.as_ptr(), val.as_ptr() as *const c_void,
                         val.len().saturating_cast(), flags) }
}

pub fn fsetxattr(fd: c_int, name: &CStr, val: &[u8], flags: c_int) -> c_int {
    unsafe { r::fsetxattr(fd, name.as_ptr(), val.as_ptr() as *const c_void,
                         val.len().saturating_cast(), flags) }
}

pub fn getxattr(path: &CStr, name: &CStr, val: &mut [u8]) -> ssize_t {
    unsafe { r::getxattr(path.as_ptr(), name.as_ptr(), val.as_mut_ptr() as *mut c_void,
                        val.len().saturating_cast()) }
}

pub fn lgetxattr(path: &CStr, name: &CStr, val: &mut [u8]) -> ssize_t {
    unsafe { r::lgetxattr(path.as_ptr(), name.as_ptr(), val.as_mut_ptr() as *mut c_void,
                         val.len().saturating_cast()) }
}

pub fn fgetxattr(fd: c_int, name: &CStr, val: &mut [u8]) -> ssize_t {
    unsafe { r::fgetxattr(fd, name.as_ptr(), val.as_mut_ptr() as *mut c_void,
                         val.len().saturating_cast()) }
}

pub fn removexattr(path: &CStr, name: &CStr) -> c_int {
    unsafe { r::removexattr(path.as_ptr(), name.as_ptr()) }
}

pub fn lremovexattr(path: &CStr, name: &CStr) -> c_int {
    unsafe { r::lremovexattr(path.as_ptr(), name.as_ptr()) }
}

pub fn fremovexattr(fd: c_int, name: &CStr) -> c_int {
    unsafe { r::fremovexattr(fd, name.as_ptr()) }
}

pub fn listxattr(path: &CStr, list: &mut [u8]) -> ssize_t {
    unsafe { r::listxattr(path.as_ptr(), list.as_mut_ptr() as *mut c_char,
                         list.len().saturating_cast()) }
}

pub fn llistxattr(path: &CStr, list: &mut [u8]) -> ssize_t {
    unsafe { r::llistxattr(path.as_ptr(), list.as_mut_ptr() as *mut c_char,
                          list.len().saturating_cast()) }
}

pub fn flistxattr(fd: c_int, list: &mut [u8]) -> ssize_t {
    unsafe {
        r::flistxattr(fd, list.as_mut_ptr() as *mut c_char, list.len().saturating_cast())
    }
}

pub fn clock_getres(clock: clockid_t, res: &mut timespec) -> c_int {
    unsafe { r::clock_getres(clock, res) }
}

pub fn clock_gettime(clock: clockid_t, res: &mut timespec) -> c_int {
    unsafe { r::clock_gettime(clock, res) }
}

pub fn clock_settime(clock: clockid_t, res: &timespec) -> c_int {
    unsafe { r::clock_settime(clock, res) }
}

pub fn clock_nanosleep(clock: clockid_t, flags: c_int, req: &timespec,
                       rem: &mut timespec) -> c_int {
    unsafe { r::clock_nanosleep(clock, flags, req, rem) }
}

pub fn timerfd_settime(fd: c_int, flags: c_int, new: &itimerspec,
                       old: Option<&mut itimerspec>) -> c_int {
    let old = match old {
        Some(old) => old as *mut _,
        _ => 0 as *mut _,
    };
    unsafe { r::timerfd_settime(fd, flags, new, old) }
}

pub fn timerfd_gettime(fd: c_int, cur: &mut itimerspec) -> c_int {
    unsafe { r::timerfd_gettime(fd, cur) }
}

pub fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int,
                 event: Option<&mut epoll_event>) -> c_int {
    let event = match event {
        Some(event) => event as *mut _,
        _ => 0 as *mut _,
    };
    unsafe { r::epoll_ctl(epfd, op, fd, event) }
}

pub fn epoll_pwait(epfd: c_int, events: &mut [epoll_event], timeout: c_int,
                   sigmask: Option<&sigset_t>) -> c_int {
    let sigmask = match sigmask {
        Some(sigmask) => sigmask as *const _,
        _ => 0 as *const _,
    };
    unsafe { r::epoll_pwait(epfd, events.as_mut_ptr(), events.len().saturating_cast(),
                           timeout, sigmask, mem::size_of::<sigset_t>() as size_t) }
}

pub fn sched_getaffinity(tid: pid_t, set: &mut [u8]) -> c_int {
    unsafe {
        r::sched_getaffinity(tid, set.len().saturating_cast(), set.as_mut_ptr() as *mut _)
    }
}

pub fn uname(buf: &mut new_utsname) -> c_int {
    unsafe { r::uname(buf) }
}

pub fn sysinfo(buf: &mut sysinfo) -> c_int {
    unsafe { r::sysinfo(buf) }
}

pub fn getrandom(buf: &mut [u8], flags: c_uint) -> c_int {
    unsafe { r::getrandom(buf.as_ptr() as *mut c_char, buf.len() as size_t, flags) }
}

pub fn acct(filename: &CStr) -> c_int {
    unsafe { r::acct(filename.as_ptr()) }
}

pub fn mount(src: &CStr, dst: &CStr, ty: &CStr, flags: c_ulong, data: &CStr) -> c_int {
    unsafe {
        r::mount(src.as_ptr() as *mut _, dst.as_ptr() as *mut _, ty.as_ptr() as *mut _,
                 flags, data.as_ptr() as *mut _)
    }
}

pub fn umount(dst: &CStr, flags: c_int) -> c_int {
    unsafe { r::umount(dst.as_ptr() as *mut _, flags) }
}

pub fn sethostname(name: &[u8]) -> c_int {
    unsafe { r::sethostname(name.as_ptr() as *mut c_char, name.len().saturating_cast()) }
}

pub fn setdomainname(name: &[u8]) -> c_int {
    unsafe { r::setdomainname(name.as_ptr() as *mut c_char, name.len().saturating_cast()) }
}

pub fn sync() {
    unsafe { r::sync() }
}
