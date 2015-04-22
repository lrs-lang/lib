// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_syscall"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core       as core;
extern crate linux_saturating as saturating;
extern crate linux_base    as base;
extern crate linux_str_one    as str_one;
extern crate linux_cty        as cty;
extern crate linux_r_syscall  as r;

#[prelude_import] use base::prelude::*;
use core::{mem};
use str_one::c_str::{CStr};
use saturating::{SaturatingCast};
use base::rmo::{AsRef, AsMut};
use cty::{
    c_int, ssize_t, rlimit64, pid_t, uid_t, gid_t, stat, c_char, size_t, statfs,
    timespec, dev_t, c_void, clockid_t, itimerspec, epoll_event, sigset_t, new_utsname,
    sysinfo, c_uint, c_ulong, umode_t, k_uint, loff_t, k_ulong, F_DUPFD_CLOEXEC, F_GETFL,
    F_SETFL, F_GETFD, F_SETFD, sockaddr, user_msghdr, mmsghdr, FUTEX_WAIT, FUTEX_WAKE,
};

// XXX: iovec _MUST_ be the same as &mut [u8]

pub fn openat(dir: c_int, path: &CStr, flags: c_int, mode: umode_t) -> c_int {
    unsafe { r::openat(dir, path.as_ptr(), flags, mode) }
}

pub fn close(fd: c_int) -> c_int {
    unsafe { r::close(fd as k_uint) }
}

pub fn lseek(fd: c_int, offset: loff_t, whence: c_uint) -> loff_t {
    unsafe { r::lseek(fd as k_uint, offset, whence) }
}

pub fn fcntl_dupfd_cloexec(fd: c_int, arg: c_int) -> c_int {
    unsafe { r::fcntl(fd as k_uint, F_DUPFD_CLOEXEC, arg as k_ulong) }
}

pub fn fcntl_getfl(fd: c_int) -> c_int {
    unsafe { r::fcntl(fd as k_uint, F_GETFL, 0) }
}

pub fn fcntl_setfl(fd: c_int, arg: c_int) -> c_int {
    unsafe { r::fcntl(fd as k_uint, F_SETFL, arg as k_ulong) }
}

pub fn fcntl_getfd(fd: c_int) -> c_int {
    unsafe { r::fcntl(fd as k_uint, F_GETFD, 0) }
}

pub fn fcntl_setfd(fd: c_int, arg: c_int) -> c_int {
    unsafe { r::fcntl(fd as k_uint, F_SETFD, arg as k_ulong) }
}

pub fn ftruncate(fd: c_int, offset: loff_t) -> c_int {
    unsafe { r::ftruncate(fd as k_uint, offset as k_ulong) }
}

pub fn getpid() -> pid_t {
    unsafe { r::getpid() }
}

pub fn getppid() -> pid_t {
    unsafe { r::getppid() }
}

pub fn setresuid(ruid: uid_t, euid: uid_t, suid: uid_t) -> c_int {
    unsafe { r::setresuid(ruid, euid, suid) }
}

pub fn setresgid(rgid: gid_t, egid: gid_t, sgid: gid_t) -> c_int {
    unsafe { r::setresgid(rgid, egid, sgid) }
}

pub fn fsync(fd: c_int) -> c_int {
    unsafe { r::fsync(fd as k_uint) }
}

pub fn fdatasync(fd: c_int) -> c_int {
    unsafe { r::fdatasync(fd as k_uint) }
}

pub fn sync() {
    unsafe { r::sync() }
}

pub fn syncfs(fd: c_int) -> c_int {
    unsafe { r::syncfs(fd) }
}

pub fn fadvise(fd: c_int, offset: loff_t, len: loff_t, advise: c_int) -> c_int {
    unsafe { r::fadvise(fd, offset, len as k_ulong, advise) }
}

pub fn fchmod(fd: c_int, mode: umode_t) -> c_int {
    unsafe { r::fchmod(fd as k_uint, mode) }
}

pub fn fallocate(fd: c_int, mode: c_int, base: loff_t, len: loff_t) -> c_int {
    unsafe { r::fallocate(fd, mode, base, len) }
}

pub fn timerfd_create(clock: c_int, flags: c_int) -> c_int {
    unsafe { r::timerfd_create(clock, flags) }
}

pub fn epoll_create(flags: c_int) -> c_int {
    unsafe { r::epoll_create1(flags) }
}

pub fn flock(fd: c_int, op: c_int) -> c_int {
    unsafe { r::flock(fd as k_uint, op as k_uint) }
}

pub fn readahead(fd: c_int, offset: loff_t, count: size_t) -> ssize_t {
    unsafe { r::readahead(fd, offset, count) }
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
               mut old: Option<&mut rlimit64>) -> c_int {
    let new_p = new.map(|v| v as *const _).unwrap_or(0 as *const _);
    let old_p = old.as_mut().map(|v| *v as *mut _).unwrap_or(0 as *mut _);
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

pub fn faccessat(dir: c_int, file: &CStr, mode: umode_t) -> c_int {
    unsafe { r::faccessat(dir, file.as_ptr(), mode as c_int) }
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

pub fn mkdirat(dir: c_int, file: &CStr, mode: umode_t) -> c_int {
    unsafe {  r::mkdirat(dir, file.as_ptr(), mode) }
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

pub fn fchmodat(dir: c_int, path: &CStr, mode: umode_t) -> c_int {
    unsafe { r::fchmodat(dir, path.as_ptr(), mode) }
}

pub fn mknodat(dir: c_int, path: &CStr, mode: umode_t, dev: dev_t) -> c_int {
    unsafe { r::mknodat(dir, path.as_ptr(), mode, dev) }
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

pub fn socket(domain: c_int, ty: c_int, proto: c_int) -> c_int {
    unsafe { r::socket(domain, ty, proto) }
}

pub fn connect<T>(sockfd: c_int, addr: T) -> c_int
    where T: AsRef<[u8]>
{
    let bytes = addr.as_ref();
    unsafe {
        r::connect(sockfd, bytes.as_ptr() as *mut sockaddr, bytes.len().saturating_cast())
    }
}

pub fn accept4<T: ?Sized>(sockfd: c_int, addr: Option<&mut T>, addrlen: &mut usize,
                          flags: c_int) -> c_int
    where T: AsMut<[u8]>
{
    let bytes = addr.map(|a| a.as_mut()).unwrap_or(&mut []);
    let mut len = bytes.len().saturating_cast();
    let res = unsafe {
        r::accept4(sockfd, bytes.as_mut_ptr() as *mut sockaddr, &mut len, flags)
    };
    *addrlen = len as usize;
    res
}

pub fn recvfrom<T: ?Sized>(sockfd: c_int, buf: &mut [u8], flags: c_int,
                           src_addr: Option<&mut T>, addrlen: &mut usize) -> ssize_t
    where T: AsMut<[u8]>
{
    let bytes = src_addr.map(|a| a.as_mut()).unwrap_or(&mut []);
    let mut len = bytes.len().saturating_cast();
    let res = unsafe {
        r::recvfrom(sockfd, buf.as_mut_ptr() as *mut c_void, buf.len().saturating_cast(),
                    flags as k_uint, bytes.as_mut_ptr() as *mut sockaddr, &mut len)
    };
    *addrlen = len as usize;
    res
}

pub fn recvmsg(sockfd: c_int, msg: &mut user_msghdr, flags: c_int) -> ssize_t {
    unsafe { r::recvmsg(sockfd, msg, flags as k_uint) }
}

pub fn recvmmsg(sockfd: c_int, msgvec: &mut [mmsghdr], flags: c_uint,
                timeout: Option<&mut timespec>) -> c_int {
    let timeout = timeout.map(|t| t as *mut timespec).unwrap_or(0 as *mut timespec);
    unsafe {
        r::recvmmsg(sockfd, msgvec.as_mut_ptr(), msgvec.len().saturating_cast(), flags,
                    timeout) as c_int
    }
}

pub fn sendto<T>(sockfd: c_int, buf: &[u8], flags: c_int, dst_addr: Option<&T>) -> ssize_t
    where T: AsRef<[u8]>
{
    let dst_addr = dst_addr.map(|a| a.as_ref()).unwrap_or(&[]);
    unsafe {
        r::sendto(sockfd, buf.as_ptr() as *mut c_void, buf.len().saturating_cast(),
                  flags as k_uint, dst_addr.as_ptr() as *mut sockaddr,
                  dst_addr.len().saturating_cast())
    }
}

pub fn sendmsg(sockfd: c_int, msg: &user_msghdr, flags: c_int) -> ssize_t {
    unsafe { r::sendmsg(sockfd, msg as *const _ as *mut _, flags as k_uint) }
}

pub fn sendmmsg(sockfd: c_int, msgvec: &[mmsghdr], flags: c_uint) -> c_int {
    unsafe {
        r::sendmmsg(sockfd, msgvec.as_ptr() as *mut mmsghdr,
                    msgvec.len().saturating_cast(), flags) as c_int
    }
}

pub fn shutdown(sockfd: c_int, how: c_int) -> c_int {
    unsafe { r::shutdown(sockfd, how) }
}

pub fn bind<T>(sockfd: c_int, addr: T) -> c_int
    where T: AsRef<[u8]>
{
    let bytes = addr.as_ref();
    unsafe {
        r::bind(sockfd, bytes.as_ptr() as *mut sockaddr, bytes.len().saturating_cast())
    }
}

pub fn listen(sockfd: c_int, backlog: c_int) -> c_int {
    unsafe { r::listen(sockfd, backlog) }
}

pub fn getsockname<T>(sockfd: c_int, mut addr: T, addrlen: &mut usize) -> c_int
    where T: AsMut<[u8]>
{
    let bytes = addr.as_mut();
    let mut len = bytes.len().saturating_cast();
    let res = unsafe {
        r::getsockname(sockfd, bytes.as_mut_ptr() as *mut sockaddr, &mut len)
    };
    *addrlen = len as usize;
    res
}

pub fn getpeername<T>(sockfd: c_int, mut addr: T, addrlen: &mut usize) -> c_int
    where T: AsMut<[u8]>
{
    let bytes = addr.as_mut();
    let mut len = bytes.len().saturating_cast();
    let res = unsafe {
        r::getpeername(sockfd, bytes.as_mut_ptr() as *mut sockaddr, &mut len)
    };
    *addrlen = len as usize;
    res
}

pub fn socketpair(domain: c_int, ty: c_int, proto: c_int, sv: &mut [c_int; 2]) -> c_int {
    unsafe { r::socketpair(domain, ty, proto, sv.as_mut_ptr()) }
}

pub fn setsockopt<T>(sockfd: c_int, level: c_int, optname: c_int, optval: T) -> c_int
    where T: AsRef<[u8]>,
{
    let bytes = optval.as_ref();
    unsafe {
        r::setsockopt(sockfd, level, optname, bytes.as_ptr() as *mut c_char,
                      bytes.len().saturating_cast())
    }
}

pub fn getsockopt<T>(sockfd: c_int, level: c_int, optname: c_int, mut optval: T,
                     optlen: &mut usize) -> c_int
    where T: AsMut<[u8]>,
{
    let bytes = optval.as_mut();
    let mut len = bytes.len().saturating_cast();
    let res = unsafe {
        r::getsockopt(sockfd, level, optname, bytes.as_mut_ptr() as *mut c_char,
                      &mut len)
    };
    *optlen = len as usize;
    res
}

pub fn futex_wait(addr: &mut c_int, val: c_int, timeout: Option<&timespec>) -> c_int {
    let timeout = timeout.map(|t| t as *const _ as *mut _).unwrap_or(0 as *mut _);
    unsafe {
        r::futex(addr as *mut _ as *mut c_uint, FUTEX_WAIT, val as c_uint, timeout,
                 0 as *mut _, 0)
    }
}

pub fn futex_wake(addr: &mut c_int, num: usize) -> c_int {
    let num: c_int = num.saturating_cast();
    unsafe {
        r::futex(addr as *mut _ as *mut c_uint, FUTEX_WAKE, num as c_uint, 0 as *mut _,
                 0 as *mut _, 0)
    }
}

pub fn exit_group(val: i32) -> ! {
    unsafe { r::exit_group(val as c_int); }
    loop { }
}
