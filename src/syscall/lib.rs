// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_syscall"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(lrs_core_plugin)]
#![no_std]

extern crate lrs_saturating as saturating;
extern crate lrs_base    as base;
extern crate lrs_str_one    as str_one;
extern crate lrs_cty        as cty;
extern crate lrs_r_syscall  as r;

use base::prelude::*;
use core::{mem};
use base::{error};
use str_one::c_str::{CStr};
use saturating::{SaturatingCast};
use cty::{
    c_int, ssize_t, rlimit64, pid_t, uid_t, gid_t, c_char, size_t,
    timespec, dev_t, c_void, clockid_t, itimerspec, epoll_event, sigset_t, new_utsname,
    sysinfo, c_uint, c_ulong, umode_t, k_uint, loff_t, k_ulong, F_DUPFD_CLOEXEC, F_GETFL,
    F_SETFL, F_GETFD, F_SETFD, sockaddr, msghdr, mmsghdr, FUTEX_WAIT, FUTEX_WAKE,
    siginfo_t, rusage, SIOCGSTAMPNS, SIOCINQ, SIOCOUTQ, EPOLL_CLOEXEC, O_CLOEXEC,
    O_LARGEFILE, SOCK_CLOEXEC, MSG_CMSG_CLOEXEC, TFD_CLOEXEC, SFD_CLOEXEC, sigaction,
    F_SETPIPE_SZ, F_GETPIPE_SZ, IN_CLOEXEC, tms, clock_t, MFD_CLOEXEC, F_ADD_SEALS,
    F_GET_SEALS, PAGE_SIZE, TIOCGPTN, TIOCSPTLCK, TIOCGPTLCK, TIOCSIG, TIOCPKT, TIOCGPKT,
    TIOCSTI, winsize, TIOCGWINSZ, TIOCSWINSZ, TIOCSCTTY, TIOCNOTTY, TIOCGEXCL, TIOCNXCL,
    TIOCEXCL, TIOCCONS, TIOCGDEV, TIOCVHANGUP, TIOCSETD, TIOCGETD, TIOCGSID, TIOCSPGRP,
    TIOCGPGRP, TCFLSH, TIOCOUTQ, TCXONC, TCGETS2, termios2, TCSETS2, mq_attr, sched_attr,
    __user_cap_data_struct, __user_cap_header_struct, _LINUX_CAPABILITY_VERSION_3,
    PR_CAPBSET_READ, PR_CAPBSET_DROP, PR_GET_KEEPCAPS, PR_SET_KEEPCAPS,
    SECCOMP_SET_MODE_STRICT,
};

pub use r::{StatType, StatfsType};

mod std { pub use base::std::*; pub use cty; }

// XXX: iovec _MUST_ be the same as &mut [u8]
//
// TODO: Audit ioctl

/// Opens a file relative to a file descriptor.
///
/// [argument, dir]
/// The file descriptor relative to which relative paths are interpreted.
///
/// [argument, path]
/// The path of the file.
///
/// [argument, flags]
/// The flags used to open the file.
///
/// [argument, mode]
/// The mode used to create new files.
///
/// [return_value]
/// Rteruns an open file descriptor or an error value.
///
/// = Remarks
///
/// Unless lrs was compiled with the `no-auto-cloexec` flag, this function automatically
/// adds the `O_CLOEXEC` flag. This function automatically adds the `O_LARGEFILE` flag.
///
/// = See also
///
/// * link:man:openat(2)
pub fn openat(dir: c_int, path: &CStr, mut flags: c_int, mode: umode_t) -> c_int {
    if cfg!(not(no_auto_cloexec)) {
        flags |= O_CLOEXEC;
    }
    flags |= O_LARGEFILE;
    unsafe { r::openat(dir, path.as_ptr(), flags, mode) }
}

/// Closes a file descriptor.
///
/// [argument, fd]
/// The file descriptor to close.
///
/// [return_value]
/// Returns a success value or an error value.
///
/// = See also
///
/// * link:man:close(2)
pub fn close(fd: c_int) -> c_int {
    unsafe { r::close(fd as k_uint) }
}

/// Seeks in a file descriptor.
///
/// [argument, fd]
/// The file descriptor in which to seek.
///
/// [argument, offset]
/// The range to seek.
///
/// [argument, whence]
/// How to seek.
///
/// [return_value]
/// Returns the new position in the file descriptor or an error value.
///
/// = See also
///
/// * link:man:lseek(2)
pub fn lseek(fd: c_int, offset: loff_t, whence: c_uint) -> loff_t {
    unsafe { r::lseek(fd as k_uint, offset, whence) }
}

/// Duplicates a file descriptor.
///
/// [argument, fd]
/// The file descriptor to duplicate.
///
/// [argument, arg]
/// The smalest value of the new file descriptor.
///
/// [return_value]
/// Returns the new file descriptor or an error value.
///
/// = See also
///
/// * link:man:fcntl(2) and F_DUPFD_CLOEXEC therein
pub fn fcntl_dupfd_cloexec(fd: c_int, arg: c_int) -> c_int {
    unsafe { r::fcntl(fd as k_uint, F_DUPFD_CLOEXEC, arg as k_ulong) }
}

/// Retrieves the file access mode and file status flags of a file descriptor.
///
/// [argument, fd]
/// The file descriptor to inspect.
///
/// [return_value]
/// Returns the file access mode and file status flags or an error value.
///
/// = See also
///
/// * link:man:fcntl(2) and F_GETFL therein
pub fn fcntl_getfl(fd: c_int) -> c_int {
    unsafe { r::fcntl(fd as k_uint, F_GETFL, 0) }
}

/// Sets the file status flags of a file descriptor.
///
/// [argument, fd]
/// The file descriptor to modify.
///
/// [argument, arg]
/// The new file status flags.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:fcntl(2) and F_SETFL therein
pub fn fcntl_setfl(fd: c_int, arg: c_int) -> c_int {
    unsafe { r::fcntl(fd as k_uint, F_SETFL, arg as k_ulong) }
}

/// Retrieves the file descriptor flags of a file descriptor.
///
/// [argument, fd]
/// The file descriptor to inspect.
///
/// [return_value]
/// Returns the file descriptor flags or an error value.
///
/// = See also
///
/// * link:man:fcntl(2) and F_GETFD therein
pub fn fcntl_getfd(fd: c_int) -> c_int {
    unsafe { r::fcntl(fd as k_uint, F_GETFD, 0) }
}

/// Sets the file descriptor flags of a file descriptor.
///
/// [argument, fd]
/// The file descriptor to modify.
///
/// [argument, arg]
/// The new file descriptor flags.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:fcntl(2) and F_SETFD therein
pub fn fcntl_setfd(fd: c_int, arg: c_int) -> c_int {
    unsafe { r::fcntl(fd as k_uint, F_SETFD, arg as k_ulong) }
}

/// Truncates a file descriptor to a certain length.
///
/// [argument, fd]
/// The file descriptor to truncate.
///
/// [argument, offset]
/// The new length.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:ftruncate(2)
pub fn ftruncate(fd: c_int, offset: loff_t) -> c_int {
    unsafe { r::ftruncate(fd as k_uint, offset) }
}

/// Returns the process id of this process.
///
/// = See also
///
/// * link:man:getpid(2)
pub fn getpid() -> pid_t {
    unsafe { r::getpid() }
}

/// Returns the process id of the parent of this process.
///
/// = See also
///
/// * link:man:getppid(2)
pub fn getppid() -> pid_t {
    unsafe { r::getppid() }
}

/// Sets the real, effective, and saved user ids of this process.
///
/// [argument, ruid]
/// The real user id.
///
/// [argument, ruid]
/// The effective user id.
///
/// [argument, ruid]
/// The saved user id.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:setresuid(2)
pub fn setresuid(ruid: uid_t, euid: uid_t, suid: uid_t) -> c_int {
    unsafe { r::setresuid(ruid, euid, suid) }
}

/// Sets the real, effective, and saved group ids of this process.
///
/// [argument, ruid]
/// The real user id.
///
/// [argument, ruid]
/// The effective user id.
///
/// [argument, ruid]
/// The saved user id.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:setresgid(2)
pub fn setresgid(rgid: gid_t, egid: gid_t, sgid: gid_t) -> c_int {
    unsafe { r::setresgid(rgid, egid, sgid) }
}

/// Transfers the kernel state of a file descriptor to the disk.
///
/// [argument, fd]
/// The file descriptor to be synchronized.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:fsync(2)
pub fn fsync(fd: c_int) -> c_int {
    unsafe { r::fsync(fd as k_uint) }
}

/// Transfers most of the kernel state of a file descriptor to the disk.
///
/// [argument, fd]
/// The file descriptor to be synchronized.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:fdatasync(2)
pub fn fdatasync(fd: c_int) -> c_int {
    unsafe { r::fdatasync(fd as k_uint) }
}

/// Transfers the kernel state to disk.
///
/// = See also
///
/// * link:man:sync(2)
pub fn sync() {
    unsafe { r::sync() }
}

/// Transfers the kernel state of a filesystem to disk.
///
/// [argument, fd]
/// An open file descriptor in the filesystem.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:syncfs(2)
pub fn syncfs(fd: c_int) -> c_int {
    unsafe { r::syncfs(fd) }
}

/// Advises the kernel of a certain usage pattern of a file descriptor.
///
/// [argument, fd]
/// The file descriptor.
///
/// [argument, offset]
/// The start of the usage.
///
/// [argument, len]
/// The length of the usage.
///
/// [argument, advice]
/// The advice given to the kernel.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:fadvise(2)
pub fn fadvise(fd: c_int, offset: loff_t, len: loff_t, advice: c_int) -> c_int {
    unsafe { r::fadvise(fd, offset, len, advice) }
}

/// Changes the mode of an inode represented by a file descriptor.
///
/// [argument, fd]
/// An open file descriptor referring to an inode.
///
/// [argument, mode]
/// The new mode of the inode.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:fchmod(2)
pub fn fchmod(fd: c_int, mode: umode_t) -> c_int {
    unsafe { r::fchmod(fd as k_uint, mode) }
}

/// Allocates memory for a file descriptor.
///
/// [argument, fd]
/// The affected file descriptor.
///
/// [argument, mode]
/// The mode of the allocation.
///
/// [argument, base]
/// The base of the allocation.
///
/// [argument, len]
/// The length of the allocation.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:fallocate(2)
pub fn fallocate(fd: c_int, mode: c_int, base: loff_t, len: loff_t) -> c_int {
    unsafe { r::fallocate(fd, mode, base, len) }
}

/// Creates a new timerfd.
///
/// [argument, clock]
/// The clock to be used for timekeeping.
///
/// [argument, flags]
/// The flags to be used for creating the file descriptor.
///
/// [return_value]
/// Returns the file descriptor or an error value.
///
/// = Remarks
///
/// Unless lrs was compiled with the `no-auto-cloexec` flag, this function automatically
/// adds the `TFD_CLOEXEC` flag.
///
/// = See also
///
/// * link:man:timerfd_create(2)
pub fn timerfd_create(clock: c_int, mut flags: c_int) -> c_int {
    if cfg!(not(no_auto_cloexec)) {
        flags |= TFD_CLOEXEC;
    }
    unsafe { r::timerfd_create(clock, flags) }
}

/// Creates a new epoll instance.
///
/// [argument, flags]
/// The flags to be used for creating the file descriptor.
///
/// [return_value]
/// Returns the file descriptor or an error value.
///
/// = Remarks
///
/// Unless lrs was compiled with the `no-auto-cloexec` flag, this function automatically
/// adds the `EPOLL_CLOEXEC` flag.
///
/// = See also
///
/// * link:man:epoll_create1(2)
pub fn epoll_create(mut flags: c_int) -> c_int {
    if cfg!(not(no_auto_cloexec)) {
        flags |= EPOLL_CLOEXEC;
    }
    unsafe { r::epoll_create1(flags) }
}

/// Applies or removes an advisory lock on a file descriptor.
///
/// [argument, fd]
/// The affected file descriptor.
///
/// [argument, op]
/// The operation to be used.
///
/// [return_value]
/// Returns succcess or an error value.
///
/// = See also
///
/// * link:man:flock(2)
pub fn flock(fd: c_int, op: c_int) -> c_int {
    unsafe { r::flock(fd as k_uint, op as k_uint) }
}

/// Initiates readahead for a file descriptor in the kernel.
///
/// [argument, fd]
/// The affected file descriptor.
///
/// [argument, offset]
/// The start of the readahead.
///
/// [argument, count]
/// The number of bytes to read.
///
/// [return_value]
/// Returns succcess or an error value.
///
/// = See also
///
/// * link:man:readahead(2)
pub fn readahead(fd: c_int, offset: loff_t, count: size_t) -> ssize_t {
    unsafe { r::readahead(fd, offset, count) }
}

/// Reads from a file descriptor.
///
/// [argument, fd]
/// The affected file descriptor.
///
/// [argument, buf]
/// The buffer to read into.
///
/// [return_value]
/// Returns the number of bytes read or an error value.
///
/// = See also
///
/// * link:man:read(2)
pub fn read(fd: c_int, buf: &mut [u8]) -> ssize_t {
    unsafe {
        r::read(fd as k_uint, buf.as_mut_ptr() as *mut _, buf.len().saturating_cast())
    }
}

/// Writes to a file descriptor.
///
/// [argument, fd]
/// The affected file descriptor.
///
/// [argument, buf]
/// The buffer to write.
///
/// [return_value]
/// Returns the number of bytes written or an error value.
///
/// = See also
///
/// * link:man:write(2)
pub fn write(fd: c_int, buf: &[u8]) -> ssize_t {
    unsafe {
        r::write(fd as k_uint, buf.as_ptr() as *const _, buf.len().saturating_cast())
    }
}

/// Reads from an offset in a file descriptor.
///
/// [argument, fd]
/// The affected file descriptor.
///
/// [argument, buf]
/// The buffer to read into.
///
/// [argument, offset]
/// The offset from which to read.
///
/// [return_value]
/// Returns the number of bytes read or an error value.
///
/// = See also
///
/// * link:man:pread(2)
pub fn pread(fd: c_int, buf: &mut [u8], offset: loff_t) -> ssize_t {
    unsafe {
        r::pread(fd as k_uint, buf.as_mut_ptr() as *mut _, buf.len().saturating_cast(),
                 offset)
    }
}


/// Writes to an offset in a file descriptor.
///
/// [argument, fd]
/// The affected file descriptor.
///
/// [argument, buf]
/// The buffer to write.
///
/// [argument, offset]
/// The offset at which to write.
///
/// [return_value]
/// Returns the number of bytes written or an error value.
///
/// = See also
///
/// * link:man:pwrite(2)
pub fn pwrite(fd: c_int, buf: &[u8], offset: loff_t) -> ssize_t {
    unsafe {
        r::pwrite(fd as k_uint, buf.as_ptr() as *const _, buf.len().saturating_cast(),
                  offset)
    }
}

/// Reads from a file descriptor into multiple buffers.
///
/// [argument, fd]
/// The affected file descriptor.
///
/// [argument, bufs]
/// The buffers to read into.
///
/// [return_value]
/// Returns the number of bytes read or an error value.
///
/// = See also
///
/// * link:man:readv(2)
pub fn readv(fd: c_int, bufs: &mut [&mut [u8]]) -> ssize_t {
    unsafe {
        r::readv(fd as k_ulong, bufs.as_mut_ptr() as *mut _, bufs.len().saturating_cast())
    }
}

/// Writes to a file descriptor from multiple buffers.
///
/// [argument, fd]
/// The affected file descriptor.
///
/// [argument, bufs]
/// The buffers to write.
///
/// [return_value]
/// Returns the number of bytes written or an error value.
///
/// = See also
///
/// * link:man:writev(2)
pub fn writev(fd: c_int, bufs: &[&[u8]]) -> ssize_t {
    unsafe {
        r::writev(fd as k_ulong, bufs.as_ptr() as *const _, bufs.len().saturating_cast())
    }
}

/// Reads from an offset in a file descriptor into multiple buffers.
///
/// [argument, fd]
/// The affected file descriptor.
///
/// [argument, bufs]
/// The buffers to read into.
///
/// [argument, offset]
/// The offset from which to read.
///
/// [return_value]
/// Returns the number of bytes read or an error value.
///
/// = See also
///
/// * link:man:preadv(2)
pub fn preadv(fd: c_int, bufs: &mut [&mut [u8]], offset: loff_t) -> ssize_t {
    let lo = ((offset as u64) & 0xFFFF_FFFF) as k_ulong;
    let hi = ((offset as u64) > 32) as k_ulong;
    unsafe {
        r::preadv(fd as k_ulong, bufs.as_mut_ptr() as *mut _,
                  bufs.len().saturating_cast(), lo, hi)
    }
}

/// Writes to an offset in a file descriptor from multiple buffers.
///
/// [argument, fd]
/// The affected file descriptor.
///
/// [argument, bufs]
/// The buffers to write.
///
/// [argument, offset]
/// The offset at which to write.
///
/// [return_value]
/// Returns the number of bytes written or an error value.
///
/// = See also
///
/// * link:man:pwritev(2)
pub fn pwritev(fd: c_int, bufs: &[&[u8]], offset: loff_t) -> ssize_t {
    let lo = ((offset as u64) & 0xFFFF_FFFF) as k_ulong;
    let hi = ((offset as u64) > 32) as k_ulong;
    unsafe {
        r::pwritev(fd as k_ulong, bufs.as_ptr() as *const _, bufs.len().saturating_cast(),
                   lo, hi)
    }
}

/// Retrieves the real, effective, and saved user ids of the process.
///
/// [argument, ruid]
/// The place where the real id will be stored.
///
/// [argument, euid]
/// The place where the effective id will be stored.
///
/// [argument, suid]
/// The place where the saved id will be stored.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:getresuid(2)
pub fn getresuid(ruid: &mut uid_t, euid: &mut uid_t, suid: &mut uid_t) -> c_int {
    unsafe { r::getresuid(ruid, euid, suid) }
}

/// Retrieves the real, effective, and saved group ids of the process.
///
/// [argument, ruid]
/// The place where the real id will be stored.
///
/// [argument, euid]
/// The place where the effective id will be stored.
///
/// [argument, suid]
/// The place where the saved id will be stored.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:getresgid(2)
pub fn getresgid(rgid: &mut gid_t, egid: &mut gid_t, sgid: &mut gid_t) -> c_int {
    unsafe { r::getresgid(rgid, egid, sgid) }
}

/// Retrieves the supplementary groups of this process.
///
/// [argument, buf]
/// The buffer in which the groups will be stored.
///
/// [return_value]
/// Returns the number of groups stored or an error value.
///
/// = See also
///
/// * link:man:getgroups(2)
pub fn getgroups(buf: &mut [gid_t]) -> c_int {
    unsafe { r::getgroups(buf.len().saturating_cast(), buf.as_mut_ptr()) }
}

/// Sets the supplementary groups of this process.
///
/// [argument, buf]
/// The buffer that contains the groups.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:setgroups(2)
pub fn setgroups(buf: &[gid_t]) -> c_int {
    unsafe { r::setgroups(buf.len().saturating_cast(), buf.as_ptr() as *mut _) }
}

/// Retrieves filesystem statistics from a path.
///
/// [argument, path]
/// A path in a mountpoint of the filesystem.
///
/// [argument, buf]
/// The buffer in which the statistics will be stored.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:statfs(2)
pub fn statfs(path: &CStr, buf: &mut StatfsType) -> c_int {
    unsafe { r::statfs(path.as_ptr(), buf) }
}

/// Retrieves filesystem statistics from a file descriptor.
///
/// [argument, fd]
/// An open file descriptor in the filesystem.
///
/// [argument, buf]
/// The buffer in which the statistics will be stored.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:fstatfs(2)
pub fn fstatfs(fd: c_int, buf: &mut StatfsType) -> c_int {
    unsafe { r::fstatfs(fd as k_uint, buf) }
}

/// Retrieves or sets resource limits of a process.
///
/// [argument, pid]
/// The affected process.
///
/// [argument, res]
/// The affected resource.
///
/// [argument, new]
/// The (optional) new value of the resource.
///
/// [argument, old]
/// A place where the previous value of the object will be stored.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:prlimit(2)
pub fn prlimit(pid: pid_t, res: c_int, new: Option<&rlimit64>,
               mut old: Option<&mut rlimit64>) -> c_int {
    let new_p = new.map(|v| v as *const _).unwrap_or(0 as *const _);
    let old_p = old.as_mut().map(|v| *v as *mut _).unwrap_or(0 as *mut _);
    unsafe { r::prlimit(pid, res as k_uint, new_p, old_p) }
}

/// Retrieves entries in a opened directory.
///
/// [argument, fd]
/// An open directory file descriptor.
///
/// [argument, buf]
/// The buffer in which the entries will be stored.
///
/// [return_value]
/// Returns the number of bytes read or an error value.
///
/// = See also
///
/// * link:man:getdents(2)
pub fn getdents(fd: c_int, buf: &mut [u8]) -> c_int {
    unsafe {
        r::getdents(fd as k_uint, buf.as_mut_ptr() as *mut _, buf.len().saturating_cast())
    }
}

/// Retrieves information about a file relative to a file descriptor.
///
/// [argument, dir]
/// The directory relative to which relative paths will be interpreted.
///
/// [argument, file]
/// The path of the file.
///
/// [argument, buf]
/// Where the information will be stored.
///
/// [argument, flags]
/// Flags to use while retrieving the information.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:fstatat(2)
pub fn fstatat(dir: c_int, file: &CStr, buf: &mut StatType, flags: c_int) -> c_int {
    unsafe { r::fstatat(dir, file.as_ptr(), buf, flags) }
}

/// Checks whether a file relative to a file descriptor can be accessed.
///
/// [argument, dir]
/// The directory relative to which relative paths will be interpreted.
///
/// [argument, file]
/// The path of the file.
///
/// [argument, mode]
/// The mode to access the file with.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:faccessat(2)
pub fn faccessat(dir: c_int, file: &CStr, mode: umode_t) -> c_int {
    unsafe { r::faccessat(dir, file.as_ptr(), mode as c_int) }
}

/// Truncates a file.
///
/// [argument, file]
/// The path of the file to truncate.
///
/// [argument, len]
/// The new length of the file.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:truncate(2)
pub fn truncate(file: &CStr, len: loff_t) -> c_int {
    unsafe { r::truncate(file.as_ptr(), len) }
}

/// Creates a hardlink relative to directories.
///
/// [argument, olddir]
/// The directory relative to which relative oldfile paths will be interpreted.
///
/// [argument, oldfile]
/// The path of the existing file.
///
/// [argument, newdir]
/// The directory relative to which relative newfile paths will be interpreted.
///
/// [argument, newfile]
/// The path of the new link.
///
/// [argument, flags]
/// Flags to use while creating the link.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:linkat(2)
pub fn linkat(olddir: c_int, oldfile: &CStr, newdir: c_int, newfile: &CStr,
              flags: c_int) -> c_int {
    unsafe { r::linkat(olddir, oldfile.as_ptr(), newdir, newfile.as_ptr(), flags) }
}

/// Changes the access and modification times of a file relative to a directory.
///
/// [argument, dir]
/// The directory relative to which relative paths will be interpreted.
///
/// [argument, file]
/// The path of the file.
///
/// [argument, times]
/// The new times of the file.
///
/// [argument, flags]
/// Flags to use while modifying the file.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:utimensat(2)
pub fn utimensat(dir: c_int, file: Option<&CStr>, times: &[timespec; 2],
                 flags: c_int) -> c_int {
    let file = file.map(|f| f.as_ptr()).unwrap_or(0 as *const _);
    unsafe { r::utimensat(dir, file, times.as_ptr(), flags) }
}

/// Renames a file relative to a directory.
///
/// [argument, olddir]
/// The directory relative to which relative oldfile paths will be interpreted.
///
/// [argument, oldfile]
/// The path of the existing file.
///
/// [argument, newdir]
/// The directory relative to which relative newfile paths will be interpreted.
///
/// [argument, newfile]
/// The path of the new file.
///
/// [argument, flags]
/// Flags to use while renaming the file.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:renameat2(2)
pub fn renameat2(olddir: c_int, oldfile: &CStr, newdir: c_int, newfile: &CStr,
                 flags: c_int) -> c_int {
    unsafe {
        if flags == 0 {
            r::renameat(olddir, oldfile.as_ptr(), newdir, newfile.as_ptr())
        } else {
            r::renameat2(olddir, oldfile.as_ptr(), newdir, newfile.as_ptr(),
                         flags as k_uint)
        }
    }
}

/// Creates a directory relative to a directory.
///
/// [argument, dir]
/// The directory relative to which relative paths will be interpreted.
///
/// [argument, file]
/// The path of the new directory.
///
/// [argument, mode]
/// The mode of the new directory.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:mkdirat(2)
pub fn mkdirat(dir: c_int, file: &CStr, mode: umode_t) -> c_int {
    unsafe {  r::mkdirat(dir, file.as_ptr(), mode) }
}

/// Unlinks a file relative to a directory.
///
/// [argument, dir]
/// The directory relative to which relative paths will be interpreted.
///
/// [argument, file]
/// The path of the file.
///
/// [argument, flags]
/// The flags used while unlinking the file.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:unlinkat(2)
pub fn unlinkat(dir: c_int, file: &CStr, flags: c_int) -> c_int {
    unsafe { r::unlinkat(dir, file.as_ptr(), flags) }
}

/// Creates a symbolic link relative to a directory.
///
/// [argument, target]
/// The target of the link.
///
/// [argument, dir]
/// The directory relative to which the `link` argument is interpreted.
///
/// [argument, link]
/// The path of the new link.
///
/// [argument, flags]
/// The flags used while unlinking the file.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:symlinkat(2)
pub fn symlinkat(target: &CStr, dir: c_int, link: &CStr) -> c_int {
    unsafe { r::symlinkat(target.as_ptr(), dir, link.as_ptr()) }
}

/// Reads the target of a symbolic link relative to a directory.
///
/// [argument, dir]
/// The directory relative to which relative paths are interpreted.
///
/// [argument, path]
/// The path of the link.
///
/// [argument, buf]
/// The buffer in which the target will be placed.
///
/// [return_value]
/// Returns the length of the target or an error value.
///
/// = See also
///
/// * link:man:readlinkat(2)
pub fn readlinkat(dir: c_int, path: &CStr, buf: &mut [u8]) -> ssize_t {
    unsafe { r::readlinkat(dir, path.as_ptr(), buf.as_mut_ptr() as *mut c_char,
                          buf.len().saturating_cast()) }
}

/// Changes the owner of a file relative to a directory.
///
/// [argument, dir]
/// The directory relative to which relative paths are interpreted.
///
/// [argument, path]
/// The path of the file.
///
/// [argument, user]
/// The new user owner.
///
/// [argument, user]
/// The new user group.
///
/// [argument, flags]
/// Flags to use while changing the owner.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:fchownat(2)
pub fn fchownat(dir: c_int, path: &CStr, user: uid_t, group: gid_t,
                flags: c_int) -> c_int {
    unsafe { r::fchownat(dir, path.as_ptr(), user, group, flags) }
}

/// Changes the mode of a file relative to a directory.
///
/// [argument, dir]
/// The directory relative to which relative paths are interpreted.
///
/// [argument, path]
/// The path of the file.
///
/// [argument, mode]
/// The new mode of the file.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:fchmodat(2)
pub fn fchmodat(dir: c_int, path: &CStr, mode: umode_t) -> c_int {
    unsafe { r::fchmodat(dir, path.as_ptr(), mode) }
}

/// Creates a file relative to a directory.
///
/// [argument, dir]
/// The directory relative to which relative paths are interpreted.
///
/// [argument, path]
/// The path of the new file.
///
/// [argument, mode]
/// The mode of a new file.
///
/// [argument, dev]
/// The device type of a new device.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:mknodat(2)
pub fn mknodat(dir: c_int, path: &CStr, mode: umode_t, dev: dev_t) -> c_int {
    unsafe { r::mknodat(dir, path.as_ptr(), mode, dev) }
}

/// Sets an extended attribute of a file.
///
/// [argument, path]
/// The path of the file.
///
/// [argument, name]
/// The name of the attribute.
///
/// [argument, val]
/// The value of the attribute.
///
/// [argument, flags]
/// The flags used while setting the attribute.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:setxattr(2)
pub fn setxattr(path: &CStr, name: &CStr, val: &[u8], flags: c_int) -> c_int {
    unsafe { r::setxattr(path.as_ptr(), name.as_ptr(), val.as_ptr() as *const c_void,
                        val.len().saturating_cast(), flags) }
}

/// Sets an extended attribute of a file without following symlinks.
///
/// [argument, path]
/// The path of the file.
///
/// [argument, name]
/// The name of the attribute.
///
/// [argument, val]
/// The value of the attribute.
///
/// [argument, flags]
/// The flags used while setting the attribute.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:lsetxattr(2)
pub fn lsetxattr(path: &CStr, name: &CStr, val: &[u8], flags: c_int) -> c_int {
    unsafe { r::lsetxattr(path.as_ptr(), name.as_ptr(), val.as_ptr() as *const c_void,
                         val.len().saturating_cast(), flags) }
}

/// Sets an extended attribute of a file descriptor.
///
/// [argument, fd]
/// The file descriptor.
///
/// [argument, name]
/// The name of the attribute.
///
/// [argument, val]
/// The value of the attribute.
///
/// [argument, flags]
/// The flags used while setting the attribute.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:fsetxattr(2)
pub fn fsetxattr(fd: c_int, name: &CStr, val: &[u8], flags: c_int) -> c_int {
    unsafe { r::fsetxattr(fd, name.as_ptr(), val.as_ptr() as *const c_void,
                         val.len().saturating_cast(), flags) }
}

/// Retrieves an extended attribute of a file.
///
/// [argument, path]
/// The path of the file.
///
/// [argument, name]
/// The name of the attribute.
///
/// [argument, val]
/// The buffer in which the value will be placed.
///
/// [return_value]
/// Returns the size of the value or an error value.
///
/// = See also
///
/// * link:man:getxattr(2)
pub fn getxattr(path: &CStr, name: &CStr, val: &mut [u8]) -> ssize_t {
    unsafe { r::getxattr(path.as_ptr(), name.as_ptr(), val.as_mut_ptr() as *mut c_void,
                        val.len().saturating_cast()) }
}

/// Retrieves an extended attribute of a file without following symlinks.
///
/// [argument, path]
/// The path of the file.
///
/// [argument, name]
/// The name of the attribute.
///
/// [argument, val]
/// The buffer in which the value will be placed.
///
/// [return_value]
/// Returns the size of the value or an error value.
///
/// = See also
///
/// * link:man:lgetxattr(2)
pub fn lgetxattr(path: &CStr, name: &CStr, val: &mut [u8]) -> ssize_t {
    unsafe { r::lgetxattr(path.as_ptr(), name.as_ptr(), val.as_mut_ptr() as *mut c_void,
                         val.len().saturating_cast()) }
}

/// Retrieves an extended attribute of a file descriptor.
///
/// [argument, fd]
/// The file descriptor.
///
/// [argument, name]
/// The name of the attribute.
///
/// [argument, val]
/// The buffer in which the value will be placed.
///
/// [return_value]
/// Returns the size of the value or an error value.
///
/// = See also
///
/// * link:man:fgetxattr(2)
pub fn fgetxattr(fd: c_int, name: &CStr, val: &mut [u8]) -> ssize_t {
    unsafe { r::fgetxattr(fd, name.as_ptr(), val.as_mut_ptr() as *mut c_void,
                         val.len().saturating_cast()) }
}

/// Removes an extended attribute of a file.
///
/// [argument, path]
/// The path of the file.
///
/// [argument, name]
/// The name of the attribute.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:removexattr(2)
pub fn removexattr(path: &CStr, name: &CStr) -> c_int {
    unsafe { r::removexattr(path.as_ptr(), name.as_ptr()) }
}

/// Removes an extended attribute of a file without following symlinks.
///
/// [argument, path]
/// The path of the file.
///
/// [argument, name]
/// The name of the attribute.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:lremovexattr(2)
pub fn lremovexattr(path: &CStr, name: &CStr) -> c_int {
    unsafe { r::lremovexattr(path.as_ptr(), name.as_ptr()) }
}

/// Removes an extended attribute of a file descriptor.
///
/// [argument, fd]
/// The file descriptor.
///
/// [argument, name]
/// The name of the attribute.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:fremovexattr(2)
pub fn fremovexattr(fd: c_int, name: &CStr) -> c_int {
    unsafe { r::fremovexattr(fd, name.as_ptr()) }
}

/// Retrieves all extended attribute of a file.
///
/// [argument, path]
/// The path of the file.
///
/// [argument, list]
/// The buffer in which the attributes will be placed.
///
/// [return_value]
/// Returns the size of all attributes or an error value.
///
/// = See also
///
/// * link:man:listxattr(2)
pub fn listxattr(path: &CStr, list: &mut [u8]) -> ssize_t {
    unsafe { r::listxattr(path.as_ptr(), list.as_mut_ptr() as *mut c_char,
                         list.len().saturating_cast()) }
}

/// Retrieves all extended attribute of a file without following symlinks.
///
/// [argument, path]
/// The path of the file.
///
/// [argument, list]
/// The buffer in which the attributes will be placed.
///
/// [return_value]
/// Returns the size of all attributes or an error value.
///
/// = See also
///
/// * link:man:llistxattr(2)
pub fn llistxattr(path: &CStr, list: &mut [u8]) -> ssize_t {
    unsafe { r::llistxattr(path.as_ptr(), list.as_mut_ptr() as *mut c_char,
                          list.len().saturating_cast()) }
}

/// Retrieves all extended attribute of a file descriptor.
///
/// [argument, fd]
/// The file descriptor.
///
/// [argument, list]
/// The buffer in which the attributes will be placed.
///
/// [return_value]
/// Returns the size of all attributes or an error value.
///
/// = See also
///
/// * link:man:flistxattr(2)
pub fn flistxattr(fd: c_int, list: &mut [u8]) -> ssize_t {
    unsafe {
        r::flistxattr(fd, list.as_mut_ptr() as *mut c_char, list.len().saturating_cast())
    }
}

/// Retrieves the resolution of a clock.
///
/// [argument, clock]
/// The id of the clock.
///
/// [argument, res]
/// The place in which the resolution will be stored.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:clock_getres(2)
pub fn clock_getres(clock: clockid_t, res: &mut timespec) -> c_int {
    unsafe { r::clock_getres(clock, res) }
}

/// Retrieves the time of a clock.
///
/// [argument, clock]
/// The id of the clock.
///
/// [argument, res]
/// The place in which the time will be stored.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:clock_gettime(2)
pub fn clock_gettime(clock: clockid_t, res: &mut timespec) -> c_int {
    unsafe { r::clock_gettime(clock, res) }
}

/// Sets the time of a clock.
///
/// [argument, clock]
/// The id of the clock.
///
/// [argument, res]
/// The new time of the clock.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:clock_settime(2)
pub fn clock_settime(clock: clockid_t, res: &timespec) -> c_int {
    unsafe { r::clock_settime(clock, res) }
}

/// Sleeps for a certain amount of time.
///
/// [argument, clock]
/// The clock used for timekeeping.
///
/// [argument, flags]
/// Flags used for sleeping.
///
/// [argument, req]
/// The requested amount of sleep.
///
/// [argument, rem]
/// The place where the remaining amount of time is placed.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:clock_nanosleep(2)
pub fn clock_nanosleep(clock: clockid_t, flags: c_int, req: &timespec,
                       rem: &mut timespec) -> c_int {
    unsafe { r::clock_nanosleep(clock, flags, req, rem) }
}

/// Arms or disarms a timerfd.
///
/// [argument, fd]
/// The timerfd.
///
/// [argument, flags]
/// Flags used to disarm or arm the timerfd.
///
/// [argument, new]
/// The new settings of the timer.
///
/// [argument, old]
/// An optional place where the old settings will be stored.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:timerfd_settime(2)
pub fn timerfd_settime(fd: c_int, flags: c_int, new: &itimerspec,
                       old: Option<&mut itimerspec>) -> c_int {
    let old = match old {
        Some(old) => old as *mut _,
        _ => 0 as *mut _,
    };
    unsafe { r::timerfd_settime(fd, flags, new, old) }
}

/// Retrieves the current settings of a timerfd.
///
/// [argument, fd]
/// The timerfd.
///
/// [argument, cur]
/// A place where the settings of the timerfd will be stored.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:timerfd_gettime(2)
pub fn timerfd_gettime(fd: c_int, cur: &mut itimerspec) -> c_int {
    unsafe { r::timerfd_gettime(fd, cur) }
}

/// Modifies a file descriptor in an epoll instance.
///
/// [argument, fd]
/// The epoll instance.
///
/// [argument, op]
/// The operation to perform on the `fd` argument.
///
/// [argument, fd]
/// The file descriptor on which to operate.
///
/// [argument, event]
/// An argument used by some operations.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:epoll_ctl(2)
pub fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int,
                 event: Option<&mut epoll_event>) -> c_int {
    let event = match event {
        Some(event) => event as *mut _,
        _ => 0 as *mut _,
    };
    unsafe { r::epoll_ctl(epfd, op, fd, event) }
}

/// Waits on an epoll instance.
///
/// [argument, fd]
/// The epoll instance.
///
/// [argument, events]
/// The buffer into which events will be placed.
///
/// [argument, timeout]
/// The timeout in milliseconds.
///
/// [argument, sigmask]
/// A set of signals that will be masked during the operation.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:epoll_pwait(2)
pub fn epoll_pwait(epfd: c_int, events: &mut [epoll_event], timeout: c_int,
                   sigmask: Option<&sigset_t>) -> c_int {
    let sigmask = match sigmask {
        Some(sigmask) => sigmask as *const _,
        _ => 0 as *const _,
    };
    unsafe { r::epoll_pwait(epfd, events.as_mut_ptr(), events.len().saturating_cast(),
                           timeout, sigmask, mem::size_of::<sigset_t>() as size_t) }
}

/// Retrieves a thread's CPU affinity mask.
///
/// [argument, tid]
/// The id of the thread.
///
/// [argument, set]
/// The buffer into which the mask will be stored.
///
/// [return_value]
/// Returns success or an error value.
///
/// = Remarks
///
/// The set size must be a multiple of the size of `k_long`.
///
/// = See also
///
/// * link:man:sched_getaffinity(2)
pub fn sched_getaffinity(tid: pid_t, set: &mut [u8]) -> c_int {
    unsafe {
        r::sched_getaffinity(tid, set.len().saturating_cast(), set.as_mut_ptr() as *mut _)
    }
}

/// Sets a thread's CPU affinity mask.
///
/// [argument, tid]
/// The id of the thread.
///
/// [argument, set]
/// The cpu affinity of the thread.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:sched_setaffinity(2)
pub fn sched_setaffinity(tid: pid_t, set: &[u8]) -> c_int {
    unsafe {
        r::sched_setaffinity(tid, set.len().saturating_cast(), set.as_ptr() as *mut _)
    }
}

/// Retrieves string-style information about the system.
///
/// [argument, buf]
/// The place in which the information will be stored.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:uname(2)
pub fn uname(buf: &mut new_utsname) -> c_int {
    unsafe { r::uname(buf) }
}

/// Retrieves integer-style information about the system.
///
/// [argument, buf]
/// The place in which the information will be stored.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:sysinfo(2)
pub fn sysinfo(buf: &mut sysinfo) -> c_int {
    unsafe { r::sysinfo(buf) }
}

/// Retrieves random bytes from the system.
///
/// [argument, buf]
/// The buffer in which the bytes will be stored.
///
/// [argument, flags]
/// Flags used while retrieving the data.
///
/// [return_value]
/// Returns the number of bytes read or an error value.
///
/// = See also
///
/// * link:man:getrandom(2)
pub fn getrandom(buf: &mut [u8], flags: c_uint) -> c_int {
    unsafe { r::getrandom(buf.as_ptr() as *mut c_char, buf.len() as size_t, flags) }
}

/// Enables or disables process accounting.
///
/// [argument, path]
/// The path into which accounting information will be written.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:acct(2)
pub fn acct(path: Option<&CStr>) -> c_int {
    let ptr = path.map(|p| p.as_ptr()).unwrap_or(0 as *const _);
    unsafe { r::acct(ptr) }
}

/// Mounts a filesystem.
///
/// [argument, src]
/// The filesystem to mount.
///
/// [argument, dst]
/// Where to mount it.
///
/// [argument, ty]
/// The type of the filesystem.
///
/// [argument, flags]
/// Flags used when mounting the filesystem.
///
/// [argument, data]
/// Filesystem dependent data.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:mount(2)
pub fn mount(src: &CStr, dst: &CStr, ty: &CStr, flags: c_ulong, data: &CStr) -> c_int {
    unsafe {
        r::mount(src.as_ptr() as *mut _, dst.as_ptr() as *mut _, ty.as_ptr() as *mut _,
                 flags, data.as_ptr() as *mut _)
    }
}

/// Unmounts a filesystem.
///
/// [argument, dst]
/// Where the filesystem is mounted.
///
/// [argument, flags]
/// Flags used when unmounting the filesystem.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:mount(2)
pub fn umount(dst: &CStr, flags: c_int) -> c_int {
    unsafe { r::umount(dst.as_ptr() as *mut _, flags) }
}

/// Sets the hostname of the system.
///
/// [argument, name]
/// The new hostname of the system.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:sethostname(2)
pub fn sethostname(name: &[u8]) -> c_int {
    unsafe { r::sethostname(name.as_ptr() as *mut c_char, name.len().saturating_cast()) }
}

/// Sets the domain name of the system.
///
/// [argument, name]
/// The new domain of the system.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:setdomainname(2)
pub fn setdomainname(name: &[u8]) -> c_int {
    unsafe { r::setdomainname(name.as_ptr() as *mut c_char, name.len().saturating_cast()) }
}

/// Creates a socket.
///
/// [argument, domain]
/// The domain of the socket.
///
/// [argument, ty]
/// The type of the socket.
///
/// [argument, proto]
/// The protocol of the socket.
///
/// [return_value]
/// Returns the socket or an error value.
///
/// = Remarks
///
/// Unless lrs was compiled with the `no-auto-cloexec` flag, this function automatically
/// adds the `SOCK_CLOEXEC` flag.
///
/// = See also
///
/// * link:man:socket(2)
pub fn socket(domain: c_int, mut ty: c_int, proto: c_int) -> c_int {
    if cfg!(not(no_auto_cloexec)) {
        ty |= SOCK_CLOEXEC;
    }
    unsafe { r::socket(domain, ty, proto) }
}

/// Connects a socket to an address.
///
/// [argument, sockfd]
/// The socket.
///
/// [argument, addr]
/// The address to connect to.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:connect(2)
pub fn connect(sockfd: c_int, addr: &[u8]) -> c_int {
    unsafe {
        r::connect(sockfd, addr.as_ptr() as *mut sockaddr, addr.len().saturating_cast())
    }
}

/// Accepts a connection request on a socket.
///
/// [argument, sockfd]
/// The socket.
///
/// [argument, addr]
/// Optional space in which the address of the peer will be stored.
///
/// [argument, addrlen]
/// The length of the peer's address.
///
/// [argument, flags]
/// Flags used for accepting a request.
///
/// [return_value]
/// Returns the new socket.
///
/// = See also
///
/// * link:man:accept4(2)
pub fn accept4(sockfd: c_int, addr: Option<&mut [u8]>, addrlen: &mut usize,
               flags: c_int) -> c_int {
    let addr = addr.unwrap_or(&mut []);
    let mut len = addr.len().saturating_cast();
    let res = unsafe {
        r::accept4(sockfd, addr.as_mut_ptr() as *mut sockaddr, &mut len, flags)
    };
    *addrlen = len as usize;
    res
}

/// Receives data on a socket.
///
/// [argument, sockfd]
/// The socket on which to receive.
///
/// [argument, buf]
/// The buffer into which the received data is placed.
///
/// [argument, flags]
/// Flags used while receiving.
///
/// [argument, src_addr]
/// An optional place where the address of the sender is placed.
///
/// [argument, addrlen]
/// A place where the length of the senders address is placed.
///
/// [return_value]
/// Returns the number of bytes received or an error value.
///
/// = See also
///
/// * link:man:recvfrom(2)
pub fn recvfrom(sockfd: c_int, buf: &mut [u8], flags: c_int, src_addr: Option<&mut [u8]>,
                addrlen: &mut usize) -> ssize_t {
    let src_addr = src_addr.unwrap_or(&mut []);
    let mut len = src_addr.len().saturating_cast();
    let res = unsafe {
        r::recvfrom(sockfd, buf.as_mut_ptr() as *mut c_void, buf.len().saturating_cast(),
                    flags as k_uint, src_addr.as_mut_ptr() as *mut sockaddr, &mut len)
    };
    *addrlen = len as usize;
    res
}

/// Receives a message on a socket.
///
/// [argument, sockfd]
/// The socket on which to receive.
///
/// [argument, msghdr]
/// The message buffer.
///
/// [argument, flags]
/// Flags used while receiving.
///
/// [return_value]
/// Returns the number of bytes received or an error value.
///
/// = Remarks
///
/// Unless lrs was compiled with the `no-auto-cloexec` flag, this function automatically
/// adds the `MSG_CMSG_CLOEXEC` flag.
///
/// = See also
///
/// * link:man:recvmsg(2)
pub fn recvmsg(sockfd: c_int, msg: &mut msghdr, mut flags: c_int) -> ssize_t {
    if cfg!(not(no_auto_cloexec)) {
        flags |= MSG_CMSG_CLOEXEC;
    }
    unsafe { r::recvmsg(sockfd, msg, flags as k_uint) }
}

/// Receives multiple messages on a socket.
///
/// [argument, sockfd]
/// The socket on which to receive.
///
/// [argument, msgvec]
/// A vector of message buffers.
///
/// [argument, flags]
/// Flags used while receiving.
///
/// [argument, timeout]
/// A timeout for the operation.
///
/// [return_value]
/// Returns the number of messages received or an error value.
///
/// = See also
///
/// * link:man:recvmmsg(2)
pub fn recvmmsg(sockfd: c_int, msgvec: &mut [mmsghdr], flags: c_uint,
                timeout: Option<&mut timespec>) -> c_int {
    let timeout = timeout.map(|t| t as *mut timespec).unwrap_or(0 as *mut timespec);
    unsafe {
        r::recvmmsg(sockfd, msgvec.as_mut_ptr(), msgvec.len().saturating_cast(), flags,
                    timeout) as c_int
    }
}

/// Sends data to an address.
///
/// [argument, sockfd]
/// The socket over which to send.
///
/// [argument, buf]
/// The buffer to send.
///
/// [argument, flags]
/// Flags used while sending.
///
/// [argument, dst_addr]
/// An optional destination of the message.
///
/// [return_value]
/// Returns the number of bytes sent or an error value.
///
/// = See also
///
/// * link:man:sendto(2)
pub fn sendto(sockfd: c_int, buf: &[u8], flags: c_int,
              dst_addr: Option<&[u8]>) -> ssize_t {
    let (dst_ptr, dst_len) = match dst_addr {
        Some(addr) => (addr.as_ptr(), addr.len()),
        _ => (0 as *const u8, 0),
    };
    unsafe {
        r::sendto(sockfd, buf.as_ptr() as *mut c_void, buf.len().saturating_cast(),
                  flags as k_uint, dst_ptr as *mut sockaddr, dst_len.saturating_cast())
    }
}

/// Sends a message on a socket.
///
/// [argument, sockfd]
/// The socket over which to send.
///
/// [argument, msghdr]
/// The message buffer.
///
/// [argument, flags]
/// Flags used while sending.
///
/// [return_value]
/// Returns the number of bytes sent or an error value.
///
/// = See also
///
/// * link:man:sendmsg(2)
pub fn sendmsg(sockfd: c_int, msg: &msghdr, flags: c_int) -> ssize_t {
    unsafe { r::sendmsg(sockfd, msg as *const _ as *mut _, flags as k_uint) }
}

/// Sends multiple messages on a socket.
///
/// [argument, sockfd]
/// The socket over which to send.
///
/// [argument, msgvec]
/// A vector of message buffers.
///
/// [argument, flags]
/// Flags used while sending.
///
/// [return_value]
/// Returns the number of messages sent or an error value.
///
/// = See also
///
/// * link:man:sendmmsg(2)
pub fn sendmmsg(sockfd: c_int, msgvec: &[mmsghdr], flags: c_uint) -> c_int {
    unsafe {
        r::sendmmsg(sockfd, msgvec.as_ptr() as *mut mmsghdr,
                    msgvec.len().saturating_cast(), flags) as c_int
    }
}

/// Shuts down (part of) a socket.
///
/// [argument, sockfd]
/// The socket.
///
/// [argument, how]
/// Which parts to shut down.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:shutdown(2)
pub fn shutdown(sockfd: c_int, how: c_int) -> c_int {
    unsafe { r::shutdown(sockfd, how) }
}

/// Binds a socket to an address.
///
/// [argument, sockfd]
/// The socket.
///
/// [argument, addr]
/// The address to bind to.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:bind(2)
pub fn bind(sockfd: c_int, addr: &[u8]) -> c_int {
    unsafe {
        r::bind(sockfd, addr.as_ptr() as *mut sockaddr, addr.len().saturating_cast())
    }
}

/// Marks a socket as accepting connections.
///
/// [argument, sockfd]
/// The socket.
///
/// [argument, backlog]
/// The maximum number of pending connections.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:listen(2)
pub fn listen(sockfd: c_int, backlog: u32) -> c_int {
    unsafe { r::listen(sockfd, backlog.saturating_cast()) }
}

/// Retrieves the address a socket is bound to.
///
/// [argument, sockfd]
/// The socket.
///
/// [argument, addr]
/// The buffer into which the address is placed.
///
/// [argument, addrlen]
/// A place into which the length of the address is placed.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:getsockname(2)
pub fn getsockname(sockfd: c_int, addr: &mut [u8], addrlen: &mut usize) -> c_int {
    let mut len = addr.len().saturating_cast();
    let res = unsafe {
        r::getsockname(sockfd, addr.as_mut_ptr() as *mut sockaddr, &mut len)
    };
    *addrlen = len as usize;
    res
}

/// Retrieves the address a socket is connected to.
///
/// [argument, sockfd]
/// The socket.
///
/// [argument, addr]
/// The buffer into which the address is placed.
///
/// [argument, addrlen]
/// A place into which the length of the address is placed.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:getpeername(2)
pub fn getpeername(sockfd: c_int, addr: &mut [u8], addrlen: &mut usize) -> c_int {
    let mut len = addr.len().saturating_cast();
    let res = unsafe {
        r::getpeername(sockfd, addr.as_mut_ptr() as *mut sockaddr, &mut len)
    };
    *addrlen = len as usize;
    res
}

/// Creates a pair of connected sockets.
///
/// [argument, domain]
/// The domain of the sockets.
///
/// [argument, ty]
/// The type of the sockets.
///
/// [argument, proto]
/// The protocol of the sockets.
///
/// [argument, sv]
/// The place where the sockets will be stored.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:socketpair(2)
pub fn socketpair(domain: c_int, ty: c_int, proto: c_int, sv: &mut [c_int; 2]) -> c_int {
    unsafe { r::socketpair(domain, ty, proto, sv.as_mut_ptr()) }
}

/// Sets a socket option.
///
/// [argument, sockfd]
/// The socket.
///
/// [argument, level]
/// The level of the option.
///
/// [argument, optname]
/// The name of the option.
///
/// [argument, optval]
/// The value to set.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:setsockopt(2)
pub fn setsockopt(sockfd: c_int, level: c_int, optname: c_int, optval: &[u8]) -> c_int {
    unsafe {
        r::setsockopt(sockfd, level, optname, optval.as_ptr() as *mut c_char,
                      optval.len().saturating_cast())
    }
}

/// Retrieves a socket option.
///
/// [argument, sockfd]
/// The socket.
///
/// [argument, level]
/// The level of the option.
///
/// [argument, optname]
/// The name of the option.
///
/// [argument, optval]
/// The buffer in which the value will be stored.
///
/// [argument, optlen]
/// A place into which the length of the value will be stored.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:getsockopt(2)
pub fn getsockopt(sockfd: c_int, level: c_int, optname: c_int, optval: &mut [u8],
                  optlen: &mut usize) -> c_int {
    let mut len = optval.len().saturating_cast();
    let res = unsafe {
        r::getsockopt(sockfd, level, optname, optval.as_mut_ptr() as *mut c_char,
                      &mut len)
    };
    *optlen = len as usize;
    res
}

/// Waits on a futex.
///
/// [argument, addr]
/// The address of the futex.
///
/// [argument, val]
/// The expected value of the futex.
///
/// [argument, timeout]
/// A timeout of the wait operation.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:futex(2) and FUTEX_WAIT therein
pub fn futex_wait(addr: &mut c_int, val: c_int, timeout: Option<&timespec>) -> c_int {
    let timeout = timeout.map(|t| t as *const _ as *mut _).unwrap_or(0 as *mut _);
    unsafe {
        r::futex(addr as *mut _ as *mut c_uint, FUTEX_WAIT, val as c_uint, timeout,
                 0 as *mut _, 0)
    }
}

/// Wakes processes sleeping on a futex.
///
/// [argument, addr]
/// The address of the futex.
///
/// [argument, num]
/// The number of processes to wake.
///
/// [return_value]
/// Returns the number of processes woken or an error value.
///
/// = See also
///
/// * link:man:futex(2) and FUTEX_WAKE therein
pub fn futex_wake(addr: &mut c_int, num: usize) -> c_int {
    let num: c_int = num.saturating_cast();
    unsafe {
        r::futex(addr as *mut _ as *mut c_uint, FUTEX_WAKE, num as c_uint, 0 as *mut _,
                 0 as *mut _, 0)
    }
}

/// Terminates the thread.
///
/// [argument, val]
/// The exit value of the thread.
///
/// = See also
///
/// * link:man:exit(2)
pub fn exit(val: c_int) -> ! {
    unsafe { r::exit(val); }
    loop { }
}

/// Terminates the process.
///
/// [argument, val]
/// The exit value of the process.
///
/// = See also
///
/// * link:man:exit_group(2)
pub fn exit_group(val: c_int) -> ! {
    unsafe { r::exit_group(val); }
    loop { }
}

/// Executes a file relative to a directory.
///
/// [argument, fd]
/// The directory relative to which relative paths will be interpreted.
///
/// [argument, filename]
/// The file to execute.
///
/// [argument, argv]
/// The argument pointer.
///
/// [argument, envp]
/// The environment pointer.
///
/// [argument, flags]
/// Flags used when executing a process.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:execveat(2)
pub fn execveat(fd: c_int, filename: &CStr, argv: *const *const c_char,
                envp: *const *const c_char, flags: c_int) -> c_int {
    unsafe { r::execveat(fd, filename.as_ptr(), argv, envp, flags) }
}

/// Maps a file into memory.
///
/// [argument, addr]
/// The address at which to map the file.
///
/// [argument, len]
/// The length of the map.
///
/// [argument, prot]
/// How the memory will be protected.
///
/// [argument, flags]
/// Flags used when mapping a file.
///
/// [argument, fd]
/// The file to map.
///
/// [argument, off]
/// The offset of the file at which the map is started.
///
/// [return_value]
/// Returns a pointer to the map or an error value.
///
/// = See also
///
/// * link:man:mmap(2)
pub unsafe fn mmap(addr: usize, len: usize, prot: c_int, flags: c_int, fd: c_int,
                   off: u64) -> isize {
    r::mmap(addr as k_ulong, len as k_ulong, prot as k_ulong, flags as k_ulong,
            fd as k_ulong, off) as isize
}

/// Unmaps a file.
///
/// [argument, addr]
/// The address of the map.
///
/// [argument, len]
/// The length of the map.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:munmap(2)
pub unsafe fn munmap(addr: usize, len: usize) -> c_int {
    r::munmap(addr as k_ulong, len as size_t)
}

/// Remaps a file in memory.
///
/// [argument, addr]
/// The address of the map.
///
/// [argument, old_len]
/// The current length of the map.
///
/// [argument, new_len]
/// The new length of the map.
///
/// [argument, flags]
/// Flags used to remap the memory.
///
/// [argument, new_addr]
/// The new address at which the memory will be mapped.
///
/// [return_value]
/// Returns a pointer to the new map or an error value.
///
/// = See also
///
/// * link:man:mremap(2)
pub unsafe fn mremap(addr: usize, old_len: usize, new_len: usize, flags: c_int,
              new_addr: usize) -> isize {
    r::mremap(addr as k_ulong, old_len as k_ulong, new_len as k_ulong,
              flags as k_ulong, new_addr as k_ulong) as isize
}

/// Waits for a child process.
///
/// [argument, which]
/// The type of process to wait for.
///
/// [argument, upid]
/// The id to wait for.
///
/// [argument, infop]
/// A place into which the process information will be placed.
///
/// [argument, options]
/// What changes to wait for.
///
/// [argument, ru]
/// An optional place where resource usage of the process will be placed.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:waitid(2)
pub fn waitid(which: c_int, upid: pid_t, infop: &mut siginfo_t, options: c_int,
              ru: Option<&mut rusage>) -> c_int {
    let ru = ru.map(|r| r as *mut _).unwrap_or(0 as *mut _);
    unsafe { r::waitid(which, upid, infop, options, ru) }
}

/// Retrieves the current working directory.
///
/// [argument, buf]
/// The buffer into which the directory will be placed.
///
/// [return_value]
/// Returns the length of the current working directory or an error value.
///
/// = See also
///
/// * link:man:getcwd(2)
pub fn getcwd(buf: &mut [u8]) -> c_int {
    unsafe { r::getcwd(buf.as_mut_ptr() as *mut c_char, buf.len() as k_ulong) }
}

/// Changes the current working directory.
///
/// [argument, path]
/// The new working directory.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:chdir(2)
pub fn chdir(path: &CStr) -> c_int {
    unsafe { r::chdir(path.as_ptr()) }
}

/// Executes ioctl with the SIOCGSTAMPNS option.
///
/// [argument, fd]
/// The file descriptor on which to operate.
///
/// [argument, time]
/// A place into which the retrieved time will be placed.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:ioctl(2)
/// * link:man:socket(7) and SIOCGSTAMP therein
pub fn ioctl_siocgstampns(fd: c_int, time: &mut timespec) -> c_int {
    unsafe { r::ioctl(fd as k_uint, SIOCGSTAMPNS as k_uint, time as *mut _ as k_ulong) }
}

/// Executes ioctl with the SIOCINQ option.
///
/// [argument, fd]
/// The file descriptor on which to operate.
///
/// [argument, unread]
/// A place into which number of unread bytes will be placed.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:ioctl(2)
/// * link:man:tcp(7) and SIOCINQ therein
pub fn ioctl_siocinq(fd: c_int, unread: &mut usize) -> c_int {
    let mut u: c_int = 0;
    let rv = unsafe {
        r::ioctl(fd as k_uint, SIOCINQ as k_uint, &mut u as *mut _ as k_ulong)
    };
    *unread = u as usize;
    rv
}

/// Executes ioctl with the SIOCOUTQ option.
///
/// [argument, fd]
/// The file descriptor on which to operate.
///
/// [argument, unread]
/// A place into which number of unread bytes will be placed.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:ioctl(2)
/// * link:man:tcp(7) and SIOCOUTQ therein
pub fn ioctl_siocoutq(fd: c_int, unread: &mut usize) -> c_int {
    let mut u: c_int = 0;
    let rv = unsafe {
        r::ioctl(fd as k_uint, SIOCOUTQ as k_uint, &mut u as *mut _ as k_ulong)
    };
    *unread = u as usize;
    rv
}

/// Modifies or inspects the process signal mask.
///
/// [argument, how]
/// How the mask will be modified.
///
/// [argument, set]
/// The argument for modification.
///
/// [argument, old]
/// Optional place in which the old set will be stored.
///
/// = See also
///
/// * link:man:rt_sigprocmask(2)
pub fn rt_sigprocmask(how: c_int, set: Option<&sigset_t>,
                      old: Option<&mut sigset_t>) -> c_int {
    let set = set.map(|v| v as *const _ as *mut _).unwrap_or(0 as *mut _);
    let old = old.map(|v| v as *mut _).unwrap_or(0 as *mut _);
    unsafe { r::rt_sigprocmask(how, set, old, mem::size_of::<sigset_t>() as size_t) }
}

/// Examines the pending signals.
///
/// [argument, set]
/// The place in which the pending signals will be set.
///
/// = See also
///
/// * link:man:rt_sigpending(2)
pub fn rt_sigpending(set: &mut sigset_t) -> c_int {
    unsafe { r::rt_sigpending(set, mem::size_of::<sigset_t>() as size_t) }
}

/// Temporarily replace the signal mask and waits for a signal to arrive.
///
/// [argument, set]
/// The temporary signal mask.
///
/// = See also
///
/// * link:man:rt_sigsuspend(2)
pub fn rt_sigsuspend(set: &sigset_t) -> c_int {
    let set = set as *const _ as *mut _;
    unsafe { r::rt_sigsuspend(set, mem::size_of::<sigset_t>() as size_t) }
}

/// Creates or modified a signalfd.
///
/// [argument, fd]
/// The file descriptor to modify.
///
/// [argument, set]
/// The set of signals to monitor.
///
/// [argument, flags]
/// The flags used to create the fd.
///
/// = Remarks
///
/// Unless lrs was compiled with the `no-auto-cloexec` flag, this function automatically
/// adds the `SFD_CLOEXEC` flag.
///
/// = See also
///
/// * link:man:signalfd4(2)
pub fn signalfd4(fd: c_int, set: &sigset_t, mut flags: c_int) -> c_int {
    if cfg!(not(no_auto_cloexec)) {
        flags |= SFD_CLOEXEC;
    }
    unsafe { r::signalfd4(fd, set, mem::size_of::<sigset_t>() as size_t, flags) }
}

/// Suspends the thread until a certain signal occurs.
///
/// [argument, set]
/// The set of signals to wait for.
///
/// [argument, info]
/// Place where information about the signal will be stored.
///
/// [argument, timeout]
/// Optional timeout.
///
/// = See also
///
/// * link:man:rt_sigtimedwait(2)
pub fn rt_sigtimedwait(set: &sigset_t, info: &mut siginfo_t,
                       timeout: Option<&timespec>) -> c_int {
    let timeout = timeout.map(|t| t as *const _).unwrap_or(0 as *const _);
    unsafe {
        r::rt_sigtimedwait(set, info, timeout, mem::size_of::<sigset_t>() as size_t)
    }
}

/// Changes or inspects the handler of a signal.
///
/// [argument, signum]
/// The signal to modify.
///
/// [argument, act]
/// The new handler.
///
/// [argument, old]
/// The old handler.
///
/// = See also
///
/// * link:man:rt_sigaction(2)
pub fn rt_sigaction(signum: c_int, act: Option<&sigaction>,
                    old: Option<&mut sigaction>) -> c_int {
    let act = act.map(|a| a as *const _).unwrap_or(0 as *const _);
    let old = old.map(|a| a as *mut _).unwrap_or(0 as *mut _);
    unsafe { r::rt_sigaction(signum, act, old, mem::size_of::<sigset_t>() as size_t) }
}

// pub fn rt_sigreturn() {
//     unsafe { r::rt_sigreturn() }
// }

/// Creates a new pipe.
///
/// [argument, fds]
/// The buffer in which the ends of the pipe will be placed.
///
/// [argument, flags]
/// Flags for creating the pipe.
///
/// = Remarks
///
/// Unless lrs was compiled with the `no-auto-cloexec` flag, this function automatically
/// adds the `O_CLOEXEC` flag.
///
/// = See also
///
/// * link:man:pipe2(2)
pub fn pipe2(fds: &mut [c_int; 2], mut flags: c_int) -> c_int {
    if cfg!(not(no_auto_cloexec)) {
        flags |= O_CLOEXEC;
    }
    unsafe { r::pipe2(fds.as_mut_ptr(), flags) }
}

/// Sets the capacity of a pipe.
///
/// [argument, fd]
/// A pipe file descriptor.
///
/// [argument, size]
/// The new capacity of the pipe.
///
/// = See also
///
/// * link:man:fcntl(2) and F_SETPIPE_SZ therein
pub fn fcntl_setpipe_sz(fd: c_int, size: c_int) -> c_int {
    unsafe { r::fcntl(fd as c_uint, F_SETPIPE_SZ, size as k_ulong) }
}

/// Gets the capacity of a pipe.
///
/// [argument, fd]
/// A pipe file descriptor.
///
/// [return_value]
/// Returns the capacity of the pipe.
///
/// = See also
///
/// * link:man:fcntl(2) and F_GETPIPE_SZ therein
pub fn fcntl_getpipe_sz(fd: c_int) -> c_int {
    unsafe { r::fcntl(fd as c_uint, F_GETPIPE_SZ, 0) }
}

/// Executes ioctl with the FIONREAD option.
///
/// [argument, fd]
/// The file descriptor on which to operate.
///
/// [argument, unread]
/// A place into which number of unread bytes will be placed.
///
/// [return_value]
/// Returns success or an error value.
///
/// = See also
///
/// * link:man:ioctl(2)
pub fn ioctl_fionread(fd: c_int, unread: &mut usize) -> c_int {
    ioctl_siocinq(fd, unread)
}

/// Copies data from one pipe to another.
///
/// [argument, fd_in]
/// The pipe to copy from.
///
/// [argument, fd_out]
/// The pipe to copy to.
///
/// [argument, len]
/// The number of bytes to copy.
///
/// [argument, flags]
/// Flags to use while copying.
///
/// [return_value]
/// Returns the number of bytes copied.
///
/// = See also
///
/// * link:man:tee(2)
pub fn tee(fd_in: c_int, fd_out: c_int, len: usize, flags: c_uint) -> ssize_t {
    unsafe { r::tee(fd_in, fd_out, len as size_t, flags) }
}

/// Copies data between two file descriptors.
///
/// [argument, fd_in]
/// The file to copy from.
///
/// [argument, off_in]
/// The position to copy from.
///
/// [argument, fd_out]
/// The file to copy to.
///
/// [argument, off_out]
/// The position to copy to.
///
/// [argument, len]
/// The number of bytes to copy.
///
/// [argument, flags]
/// Flags to use while copying.
///
/// [return_value]
/// Returns the number of bytes copied.
///
/// = See also
///
/// * link:man:splice(2)
pub fn splice(fd_in: c_int, mut off_in: Option<&mut u64>, fd_out: c_int,
              mut off_out: Option<&mut u64>, len: usize, flags: c_uint) -> ssize_t {
    let mut loff_in = 0;
    let mut loff_out = 0;
    let mut poff_in = 0 as *mut loff_t;
    let mut poff_out = 0 as *mut loff_t;
    if let Some(ref mut p) = off_in {
        loff_in = **p as loff_t;
        poff_in = &mut loff_in;
    }
    if let Some(ref mut p) = off_out {
        loff_out = **p as loff_t;
        poff_out = &mut loff_out;
    }
    let rv = unsafe { r::splice(fd_in, poff_in, fd_out, poff_out, len as size_t, flags) };
    if let Some(p) = off_in {
        *p = loff_in as u64;
    }
    if let Some(p) = off_out {
        *p = loff_out as u64;
    }
    rv
}

/// Creates a new inotify object.
///
/// [argument, flags]
/// Flags to use when creating the object.
///
/// = Remarks
///
/// Unless lrs was compiled with the `no-auto-cloexec` flag, this function automatically
/// adds the `IN_CLOEXEC` flag.
///
/// = See also
///
/// * link:man:inotify_init1(2)
pub fn inotify_init1(mut flags: c_int) -> c_int {
    if cfg!(not(no_auto_cloexec)) {
        flags |= IN_CLOEXEC;
    }
    unsafe { r::inotify_init1(flags) }
}

/// Adds or modifies an inotify watch.
///
/// [argument, fd]
/// The fd of the inotify object.
///
/// [argument, path]
/// The path to watch.
///
/// [argument, mask]
/// The events to watch.
///
/// = See also
///
/// * link:man:inotify_add_watch(2)
pub fn inotify_add_watch(fd: c_int, path: &CStr, mask: u32) -> c_int {
    unsafe { r::inotify_add_watch(fd, path.as_ptr(), mask) }
}

/// Removes an inotify watch.
///
/// [argument, fd]
/// The fd of the inotify object.
///
/// [argument, wd]
/// The watch descriptor to remove.
///
/// = See also
///
/// * link:man:inotify_rm_watch(2)
pub fn inotify_rm_watch(fd: c_int, wd: c_int) -> c_int {
    unsafe { r::inotify_rm_watch(fd, wd) }
}

/// Duplicates a file descriptor by replacing another one.
///
/// [argument, oldfd]
/// The file descriptor to duplicate.
///
/// [argument, newfd]
/// The file descriptor to replace.
///
/// [argument, flags]
/// Flags to use when creating the new file descriptor.
///
/// = Remarks
///
/// Unless lrs was compiled with the `no-auto-cloexec` flag, this function automatically
/// adds the `O_CLOEXEC` flag.
///
/// = See also
///
/// * link:man:dup3(2)
pub fn dup3(oldfd: c_int, newfd: c_int, mut flags: c_int) -> c_int {
    if cfg!(not(no_auto_cloexec)) {
        flags |= O_CLOEXEC;
    }
    unsafe { r::dup3(oldfd as c_uint, newfd as c_uint, flags) }
}

/// Sets the file mode creating mask of the process.
///
/// [argument, mode]
/// The mode to be masked.
///
/// [return_value]
/// Returns the previous mask.
///
/// = See also
///
/// * link:man:umask(2)
pub fn umask(mode: umode_t) -> umode_t {
    unsafe { r::umask(mode as c_int) }
}

/// Creates a new eventfd.
///
/// [argument, init]
/// The initial value of the eventfd.
///
/// [argument, flags]
/// Flags to use when creating the new file descriptor.
///
/// = See also
///
/// * link:man:eventfd2(2)
pub fn eventfd2(init: c_uint, flags: c_int) -> c_int {
    unsafe { r::eventfd2(init, flags) }
}

/// Retrieves the CPU times used by this process and its children.
///
/// [argument, buf]
/// Place where the times will be stored.
///
/// = See also
///
/// * link:man:times(2)
pub fn times(buf: *mut tms) -> clock_t {
    unsafe { r::times(buf) }
}

// /// Suspends the thread until a signal handler is invoked.
// ///
// /// = See also
// ///
// /// * link:man:pause(2)
// pub fn pause() -> c_int {
//     unsafe { r::pause() }
// }

/// Performs reboot-related operations.
///
/// [argument, cmd]
/// The command to be executed.
///
/// [argument, arg]
/// An optional argument.
///
/// = Remarks
///
/// The argument is actually a void* but currently only used as a string in one case.
///
/// = See also
///
/// * link:man:reboot(2)
pub fn reboot(cmd: c_uint, arg: &CStr) -> c_int {
    #![allow(overflowing_literals)]
    // Trivia: The following magic numbers can be used in the second field:
    //
    //  0x28121969
    //  0x05121996
    //  0x16041998
    //  0x20112000
    //
    // The are the birthdays of Linus Torvalds and his children.
    unsafe { r::reboot(0xfee1dead, 0x28121969, cmd, arg.as_ptr() as *mut c_void) }
}

/// Creates a memfd.
///
/// [argument, name]
/// The name of the pseudo-file.
///
/// [argument, flags]
/// Flags to use for the new file.
///
/// = Remarks
///
/// Unless lrs was compiled with the `no-auto-cloexec` flag, this function automatically
/// adds the `MFD_CLOEXEC` flag.
///
/// = See also
///
/// * link:man:memfd_create(2)
pub fn memfd_create(name: &CStr, mut flags: c_uint) -> c_int {
    if cfg!(not(no_auto_cloexec)) {
        flags |= MFD_CLOEXEC;
    }
    unsafe { r::memfd_create(name.as_ptr(), flags) }
}

/// Adds file seals to an inode.
///
/// [argument, fd]
/// A file descriptor refering to the inode.
///
/// [argument, seals]
/// The seals to add.
///
/// = Remarks
///
/// == Kernel versions
///
/// The required kernel version is 3.17.
///
/// = See also
///
/// * link:man:fcntl(2) and F_ADD_SEALS therein
pub fn fcntl_add_seals(fd: c_int, seals: c_uint) -> c_int {
    unsafe { r::fcntl(fd as c_uint, F_ADD_SEALS, seals as k_ulong) }
}

/// Returns the seals of an inode.
///
/// [argument, fd]
/// A file descriptor refering to the inode.
///
/// = Remarks
///
/// == Kernel versions
///
/// The required kernel version is 3.17.
///
/// = See also
///
/// * link:man:fcntl(2) and F_GET_SEALS therein
pub fn fcntl_get_seals(fd: c_int) -> c_int {
    unsafe { r::fcntl(fd as c_uint, F_GET_SEALS, 0) }
}

/// Synchronizes a memory mapping with the filesystem.
///
/// [argument, addr]
/// The start of the range to be synchronized.
///
/// [argument, len]
/// The length of the range to be synchronized.
///
/// [argument, flags]
/// Flags to used for synchronization.
///
/// = See also
///
/// * link:man:msync(2)
pub fn msync(addr: usize, len: usize, flags: c_int) -> c_int {
    unsafe { r::msync(addr as k_ulong, len as size_t, flags) }
}

/// Advise the kernel of a certain memory usage pattern.
///
/// [argument, addr]
/// The start of the range to be advised.
///
/// [argument, len]
/// The length of the range to be advised.
///
/// [argument, advice]
/// The advice given.
///
/// = See also
///
/// * link:man:madvise(2)
pub unsafe fn madvise(addr: usize, len: usize, advice: c_int) -> c_int {
    r::madvise(addr as k_ulong, len as k_ulong, advice)
}

/// Change the memory protection of a region.
///
/// [argument, addr]
/// The start of the region.
///
/// [argument, len]
/// The length of the region.
///
/// [argument, protection]
/// The new protection.
///
/// = See also
///
/// * link:man:mprotect(2)
pub fn mprotect(addr: usize, len: usize, protection: c_int) -> c_int {
    unsafe { r::mprotect(addr as k_ulong, len as k_ulong, protection as k_ulong) }
}

/// Lock a memory range in memory.
///
/// [argument, addr]
/// The base address of the range.
///
/// [argument, len]
/// The length of the range.
///
/// = See also
///
/// * link:man:mlock(2)
pub fn mlock(addr: usize, len: usize) -> c_int {
    unsafe { r::mlock(addr as k_ulong, len as k_ulong) }
}

/// Unlock a memory range.
///
/// [argument, addr]
/// The base address of the range.
///
/// [argument, len]
/// The length of the range.
///
/// = See also
///
/// * link:man:munlock(2)
pub fn munlock(addr: usize, len: usize) -> c_int {
    unsafe { r::munlock(addr as k_ulong, len as k_ulong) }
}

/// Lock all pages in memory.
///
/// [argument, flags]
/// Flags to used for locking.
///
/// = See also
///
/// * link:man:mlockall(2)
pub fn mlockall(flags: c_int) -> c_int {
    unsafe { r::mlockall(flags) }
}

/// Unlock all pages in memory.
///
/// = See also
///
/// * link:man:munlockall(2)
pub fn munlockall() -> c_int {
    unsafe { r::munlockall() }
}

/// Checks whether pages are in memory or swapped out.
///
/// [argument, addr]
/// The base address of the range to check.
///
/// [argument, length]
/// The length of the range to check.
///
/// [argument, buf]
/// The buffer in which the result will be stored.
///
/// = See also
///
/// * link:man:mincore(2)
pub fn mincore(addr: usize, length: usize, buf: &mut [u8]) -> c_int {
    let pages = (buf.len() + PAGE_SIZE - 1) / PAGE_SIZE;
    if pages > buf.len() {
        return -error::InvalidArgument.0;
    }
    unsafe { r::mincore(addr as k_ulong, length as size_t, buf.as_mut_ptr()) }
}

/// Create a new session.
///
/// = See also
///
/// * link:man:setsid(2)
pub fn setsid() -> pid_t {
    unsafe { r::setsid() }
}

/// Get the session id of a process.
///
/// [argument, pid]
/// The process whose session id to return.
///
/// = See also
///
/// * link:man:getsid(2)
pub fn getsid(pid: pid_t) -> pid_t {
    unsafe { r::getsid(pid) }
}

/// Set the current working directory to the directory referenced by a file descriptor.
///
/// [argument, fd]
/// The file descriptor pointing to the new working directory.
///
/// = See also
///
/// * link:man:fchdir(2)
pub fn fchdir(fd: c_int) -> c_int {
    unsafe { r::fchdir(fd as c_uint) }
}

/// Sets the process group of a process.
///
/// [argument, process]
/// The process to move to the process group.
///
/// [argument, group]
/// The new group of the process.
///
/// = See also
///
/// * link:man:setpgid(2)
pub fn setpgid(pid: pid_t, pgid: pid_t) -> c_int {
    unsafe { r::setpgid(pid, pgid) }
}

/// Get the process group of a process.
///
/// [argument, pid]
/// The process whose process group to return.
///
/// = See also
///
/// * link:man:getpgid(2)
pub fn getpgid(pid: pid_t) -> pid_t {
    unsafe { r::getpgid(pid) }
}

/// Sends a signal to a process.
///
/// [argument, pid]
/// The process to send the signal to.
///
/// [argument, sig]
/// The signal to send.
///
/// = See also
///
/// * link:man:kill(2)
pub fn kill(pid: pid_t, sig: c_int) -> c_int {
    unsafe { r::kill(pid, sig) }
}

/// Sends a signal to a thread.
///
/// [argument, tgid]
/// The thread group of the thread.
///
/// [argument, tid]
/// The thread id.
///
/// [argument, sig]
/// The signal to send.
///
/// = See also
///
/// * link:man:tgkill(2)
pub fn tgkill(tgid: c_int, tid: c_int, sig: c_int) -> c_int {
    unsafe { r::tgkill(tgid, tid, sig) }
}

/// Returns the thread id of the calling thread.
///
/// = See also
///
/// * link:man:gettid(2)
pub fn gettid() -> pid_t {
    unsafe { r::gettid() }
}

/// Returns the resource usage of this thread, this process, or its children.
///
/// [argument, who]
/// Whose resource usage to get.
///
/// [argument, usage]
/// Place where the usage will be stored.
///
/// = See also
///
/// * link:man:getrusage(2)
pub fn getrusage(who: c_int, usage: &mut rusage) -> c_int {
    unsafe { r::getrusage(who, usage) }
}

/// Executes ioctl with the TIOCGPTN option.
///
/// [argument, fd]
/// The file descriptor on which to operate.
///
/// [argument, id]
/// A place into which the slave id will be stored.
///
/// = See also
///
/// * link:man:tty_ioctl(2)
pub fn ioctl_tiocgptn(fd: c_int, id: &mut u32) -> c_int {
    let mut u: c_int = 0;
    let rv = unsafe {
        r::ioctl(fd as k_uint, TIOCGPTN(), &mut u as *mut _ as k_ulong)
    };
    *id = u as u32;
    rv
}

/// Executes ioctl with the TIOCSPTLCK option.
///
/// [argument, fd]
/// The file descriptor on which to operate.
///
/// [argument, locked]
/// Whether the slave is locked.
///
/// = See also
///
/// * link:man:tty_ioctl(2)
pub fn ioctl_tiocsptlck(fd: c_int, locked: bool) -> c_int {
    let mut locked = locked as c_int;
    unsafe {
        r::ioctl(fd as k_uint, TIOCSPTLCK(), &mut locked as *mut _ as k_ulong)
    }
}

/// Executes ioctl with the TIOCGPTLCK option.
///
/// [argument, fd]
/// The file descriptor on which to operate.
///
/// [argument, locked]
/// Place where the lock status will be stored.
///
/// = See also
///
/// * link:man:tty_ioctl(2)
pub fn ioctl_tiocgptlck(fd: c_int, locked: &mut bool) -> c_int {
    let mut t: c_int = 0;
    let rv = unsafe { r::ioctl(fd as k_uint, TIOCGPTLCK(), &mut t as *mut _ as k_ulong) };
    *locked = t != 0;
    rv
}

/// Executes ioctl with the TIOCSIG option.
///
/// [argument, fd]
/// The file descriptor on which to operate.
///
/// [argument, sig]
/// The signal to send.
///
/// = Remarks
///
/// This ioctl is undocumented but see drivers/tty/pty.c.
pub fn ioctl_tiocsig(fd: c_int, mut sig: c_int) -> c_int {
    unsafe { r::ioctl(fd as k_uint, TIOCSIG(), &mut sig as *mut _ as k_ulong) }
}

/// Executes ioctl with the TIOCPKT option.
///
/// [argument, fd]
/// The file descriptor on which to operate.
///
/// [argument, packet]
/// Whether packet mode is enabled.
///
/// = See also
///
/// * link:man:tty_ioctl(2)
pub fn ioctl_tiocpkt(fd: c_int, packet: bool) -> c_int {
    let mut packet = packet as c_int;
    unsafe { r::ioctl(fd as k_uint, TIOCPKT, &mut packet as *mut _ as k_ulong) }
}

/// Executes ioctl with the TIOCGPKT option.
///
/// [argument, fd]
/// The file descriptor on which to operate.
///
/// [argument, packet]
/// Place where the status of packet mode will be stored.
///
/// = See also
///
/// * link:man:tty_ioctl(2)
pub fn ioctl_tiocgpkt(fd: c_int, packet: &mut bool) -> c_int {
    let mut p: c_int = 0;
    let rv = unsafe { r::ioctl(fd as k_uint, TIOCGPKT(), &mut p as *mut _ as k_ulong) };
    *packet = p != 0;
    rv
}

/// Executes ioctl with the TIOCSTI option.
///
/// [argument, fd]
/// The file descriptor on which to operate.
///
/// [argument, byte]
/// The byte to insert into the input queue.
///
/// = See also
///
/// * link:man:tty_ioctl(2)
pub fn ioctl_tiocsti(fd: c_int, mut byte: u8) -> c_int {
    unsafe { r::ioctl(fd as k_uint, TIOCSTI, &mut byte as *mut _ as k_ulong) }
}

/// Executes ioctl with the TIOCGWINSZ option.
///
/// [argument, fd]
/// The file descriptor on which to operate.
///
/// [argument, size]
/// Place where the window size will be stored.
///
/// = See also
///
/// * link:man:tty_ioctl(2)
pub fn ioctl_tiocgwinsz(fd: c_int, size: &mut winsize) -> c_int {
    unsafe { r::ioctl(fd as k_uint, TIOCGWINSZ, size as *mut _ as k_ulong) }
}

/// Executes ioctl with the TIOCSWINSZ option.
///
/// [argument, fd]
/// The file descriptor on which to operate.
///
/// [argument, size]
/// The new window size.
///
/// = See also
///
/// * link:man:tty_ioctl(2)
pub fn ioctl_tiocswinsz(fd: c_int, size: &winsize) -> c_int {
    unsafe { r::ioctl(fd as k_uint, TIOCSWINSZ, size as *const _ as k_ulong) }
}

/// Executes ioctl with the TIOCCONS option.
///
/// [argument, fd]
/// The file descriptor on which to operate.
///
/// = See also
///
/// * link:man:tty_ioctl(2)
pub fn ioctl_tioccons(fd: c_int) -> c_int {
    unsafe { r::ioctl(fd as k_uint, TIOCCONS, 0) }
}

/// Executes ioctl with the TIOCEXCL option.
///
/// [argument, fd]
/// The file descriptor on which to operate.
///
/// = See also
///
/// * link:man:tty_ioctl(2)
pub fn ioctl_tiocexcl(fd: c_int) -> c_int {
    unsafe { r::ioctl(fd as k_uint, TIOCEXCL, 0) }
}

/// Executes ioctl with the TIOCNXCL option.
///
/// [argument, fd]
/// The file descriptor on which to operate.
///
/// = See also
///
/// * link:man:tty_ioctl(2)
pub fn ioctl_tiocnxcl(fd: c_int) -> c_int {
    unsafe { r::ioctl(fd as k_uint, TIOCNXCL, 0) }
}

/// Executes ioctl with the TIOCGEXCL option.
///
/// [argument, fd]
/// The file descriptor on which to operate.
///
/// [argument, packet]
/// Place where the status of exclusize mode will be stored.
///
/// = Remarks
///
/// == Kernel versions
///
/// The required kernel version is 3.8.
///
/// = See also
///
/// * link:man:tty_ioctl(2)
pub fn ioctl_tiocgexcl(fd: c_int, exclusive: &mut bool) -> c_int {
    let mut p: c_int = 0;
    let rv = unsafe { r::ioctl(fd as k_uint, TIOCGEXCL(), &mut p as *mut _ as k_ulong) };
    *exclusive = p != 0;
    rv
}

/// Executes ioctl with the TIOCNOTTY option.
///
/// [argument, fd]
/// The file descriptor on which to operate.
///
/// = See also
///
/// * link:man:tty_ioctl(2)
pub fn ioctl_tiocnotty(fd: c_int) -> c_int {
    unsafe { r::ioctl(fd as k_uint, TIOCNOTTY, 0) }
}

/// Executes ioctl with the TIOCSCTTY option.
///
/// [argument, fd]
/// The file descriptor on which to operate.
///
/// [argument, steal]
/// Whether to steal the terminal.
///
/// = See also
///
/// * link:man:tty_ioctl(2)
pub fn ioctl_tiocsctty(fd: c_int, steal: bool) -> c_int {
    unsafe { r::ioctl(fd as k_uint, TIOCSCTTY, steal as k_ulong) }
}

/// Executes ioctl with the TIOCGPGRP option.
///
/// [argument, fd]
/// The file descriptor on which to operate.
///
/// [argument, id]
/// Place where the process group id of the foreground process group will be stored.
///
/// = See also
///
/// * link:man:tty_ioctl(2)
pub fn ioctl_tiocgpgrp(fd: c_int, id: &mut pid_t) -> c_int {
    unsafe { r::ioctl(fd as k_uint, TIOCGPGRP, id as *mut _ as k_ulong) }
}

/// Executes ioctl with the TIOCSPGRP option.
///
/// [argument, fd]
/// The file descriptor on which to operate.
///
/// [argument, id]
/// The process group id of the foreground process group.
///
/// = See also
///
/// * link:man:tty_ioctl(2)
pub fn ioctl_tiocspgrp(fd: c_int, id: pid_t) -> c_int {
    unsafe { r::ioctl(fd as k_uint, TIOCSPGRP, &id as *const _ as k_ulong) }
}

/// Executes ioctl with the TIOCGSID option.
///
/// [argument, fd]
/// The file descriptor on which to operate.
///
/// [argument, id]
/// Place where the session id of the terminal will be stored.
///
/// = See also
///
/// * link:man:tty_ioctl(2)
pub fn ioctl_tiocgsid(fd: c_int, id: &mut pid_t) -> c_int {
    unsafe { r::ioctl(fd as k_uint, TIOCGSID, id as *mut _ as k_ulong) }
}

/// Executes ioctl with the TIOCGETD option.
///
/// [argument, fd]
/// The file descriptor on which to operate.
///
/// [argument, discipline]
/// Place where the line discipline of the terminal will be stored.
///
/// = See also
///
/// * link:man:tty_ioctl(2)
pub fn ioctl_tiocgetd(fd: c_int, discipline: &mut c_int) -> c_int {
    unsafe { r::ioctl(fd as k_uint, TIOCGETD, discipline as *mut _ as k_ulong) }
}

/// Executes ioctl with the TIOCSETD option.
///
/// [argument, fd]
/// The file descriptor on which to operate.
///
/// [argument, discipline]
/// The line discipline of the terminal.
///
/// = See also
///
/// * link:man:tty_ioctl(2)
pub fn ioctl_tiocsetd(fd: c_int, discipline: c_int) -> c_int {
    unsafe { r::ioctl(fd as k_uint, TIOCSETD, &discipline as *const _ as k_ulong) }
}

/// Executes ioctl with the TIOCVHANGUP option.
///
/// [argument, fd]
/// The file descriptor on which to operate.
///
/// = Remarks
///
/// This ioctl is undocumented but see drivers/tty/tty_io.c.
pub fn ioctl_tiocvhangup(fd: c_int) -> c_int {
    unsafe { r::ioctl(fd as k_uint, TIOCVHANGUP, 0) }
}

/// Executes ioctl with the TIOCGDEV option.
///
/// [argument, fd]
/// The file descriptor on which to operate.
///
/// [argument, device]
/// Place where the device of the terminal will be stored.
///
/// = Remarks
///
/// This ioctl is undocumented but see drivers/tty/tty_io.c.
pub fn ioctl_tiocgdev(fd: c_int, dev: &mut c_uint) -> c_int {
    unsafe { r::ioctl(fd as k_uint, TIOCGDEV(), dev as *mut _ as k_ulong) }
}

/// Executes ioctl with the TCFLSH option.
///
/// [argument, fd]
/// The file descriptor on which to operate.
///
/// [argument, how]
/// What to discard.
///
/// = See also
///
/// * link:man:tty_ioctl(2)
pub fn ioctl_tcflsh(fd: c_int, how: c_uint) -> c_int {
    unsafe { r::ioctl(fd as k_uint, TCFLSH, how as k_ulong) }
}

/// Executes ioctl with the TIOCOUTQ option.
///
/// [argument, fd]
/// The file descriptor on which to operate.
///
/// [argument, buf]
/// Place where the number of pending output bytes will be stored.
///
/// = See also
///
/// * link:man:tty_ioctl(2)
pub fn ioctl_tiocoutq(fd: c_int, buf: &mut usize) -> c_int {
    let mut tmp: c_int = 0;
    let rv = unsafe { r::ioctl(fd as k_uint, TIOCOUTQ, &mut tmp as *mut _ as k_ulong) };
    *buf = tmp as usize;
    rv
}

/// Executes ioctl with the TCXONC option.
///
/// [argument, fd]
/// The file descriptor on which to operate.
///
/// [argument, how]
/// What to disable/enable.
///
/// = See also
///
/// * link:man:tty_ioctl(2)
pub fn ioctl_tcxonc(fd: c_int, how: c_uint) -> c_int {
    unsafe { r::ioctl(fd as k_uint, TCXONC, how as k_ulong) }
}

/// Executes ioctl with the TCGETS2 option.
///
/// [argument, fd]
/// The file descriptor on which to operate.
///
/// [argument, attrs]
/// Place where the tty attributes will be stored.
///
/// = See also
///
/// * link:man:tty_ioctl(2) and TCGETS therein
pub fn ioctl_tcgets2(fd: c_int, attrs: &mut termios2) -> c_int {
    unsafe { r::ioctl(fd as k_uint, TCGETS2(), attrs as *mut _ as k_ulong) }
}

/// Executes ioctl with the TCSETS2 option.
///
/// [argument, fd]
/// The file descriptor on which to operate.
///
/// [argument, attrs]
/// The tty attributes.
///
/// = See also
///
/// * link:man:tty_ioctl(2) and TCSETS therein
pub fn ioctl_tcsets2(fd: c_int, attrs: &termios2) -> c_int {
    unsafe { r::ioctl(fd as k_uint, TCSETS2(), attrs as *const _ as k_ulong) }
}

/// Simulates a hangup on the current terminal.
///
/// = See also
///
/// * link:man:vhangup(2)
pub fn vhangup() -> c_int {
    unsafe { r::vhangup() }
}

/// Opens a message queue.
///
/// [argument, name]
/// The name of the queue.
///
/// [argument, flags]
/// Flags used when opening the queue.
///
/// [argument, mode]
/// Mode used on a new queue.
///
/// [argument, attr]
/// Attributes set on a new queue.
///
/// = Remarks
///
/// Unless lrs was compiled with the `no-auto-cloexec` flag, this function automatically
/// adds the `O_CLOEXEC` flag.
///
/// = See also
///
/// * link:man:mq_open(2)
pub fn mq_open(name: &CStr, mut flags: c_int, mode: umode_t,
               attr: Option<&mq_attr>) -> c_int {
    if cfg!(not(no_auto_cloexec)) {
        flags |= O_CLOEXEC;
    }
    let attr = attr.map(|a| a as *const _ as *mut _).unwrap_or(0 as *mut _);
    unsafe { r::mq_open(name.as_ptr(), flags, mode, attr) }
}

/// Destroys a queue.
///
/// [argument, name]
/// The name of the queue to destroy.
///
/// = See also
///
/// * link:man:mq_unlink(2)
pub fn mq_unlink(name: &CStr) -> c_int {
    unsafe { r::mq_unlink(name.as_ptr()) }
}

/// Sends a message over a message queue.
///
/// [argument, mq]
/// The message queue.
///
/// [argument, msg]
/// The message to send.
///
/// [argument, prio]
/// The priority of the message.
///
/// [argument, timeout]
/// The timeout of the operation.
///
/// = See also
///
/// * link:man:mq_timedsend(2)
pub fn mq_timedsend(mq: c_int, msg: &[u8], prio: c_uint,
                    timeout: Option<&timespec>) -> c_int {
    let timeout = timeout.map(|a| a as *const _).unwrap_or(0 as *const _);
    unsafe {
        r::mq_timedsend(mq, msg.as_ptr() as *const c_char, msg.len() as size_t, prio,
                        timeout)
    }
}

/// Receives a message over a message queue.
///
/// [argument, mq]
/// The message queue.
///
/// [argument, msg]
/// A place where the message will be stored.
///
/// [argument, prio]
/// A place where the priority will be stored.
///
/// [argument, timeout]
/// The timeout of the operation.
///
/// = See also
///
/// * link:man:mq_timedreceive(2)
pub fn mq_timedreceive(mq: c_int, msg: &mut [u8], prio: Option<&mut c_uint>,
                       timeout: Option<&timespec>) -> ssize_t {
    let prio = prio.map(|a| a as *mut _).unwrap_or(0 as *mut _);
    let timeout = timeout.map(|a| a as *const _).unwrap_or(0 as *const _);
    unsafe {
        r::mq_timedreceive(mq, msg.as_mut_ptr() as *mut c_char, msg.len() as size_t, prio,
                           timeout)
    }
}

/// Sets or retrieves attributes of a message queue.
///
/// [argument, mq]
/// The message queue.
///
/// [argument, new]
/// The attributes to set.
///
/// [argument, old]
/// A place where the old attributes will be stored.
///
/// = See also
///
/// * link:man:mq_getsetattr(2)
pub fn mq_getsetattr(mq: c_int, new: Option<&mq_attr>,
                     old: Option<&mut mq_attr>) -> c_int {
    let new = new.map(|a| a as *const _).unwrap_or(0 as *const _);
    let old = old.map(|a| a as *mut _).unwrap_or(0 as *mut _);
    unsafe { r::mq_getsetattr(mq, new, old) }
}

/// Sets the scheduling policy of a thread.
///
/// [argument, pid]
/// The thread to modify.
///
/// [argument, attr]
/// The policy to set.
///
/// [argument, flags]
/// Unused.
///
/// = See also
///
/// * link:man:sched_setattr(2)
pub fn sched_setattr(pid: pid_t, attr: &mut sched_attr, flags: c_uint) -> c_int {
    unsafe { r::sched_setattr(pid, attr, flags) }
}

/// Gets the scheduling policy of a thread.
///
/// [argument, pid]
/// The thread whose policy to get.
///
/// [argument, attr]
/// A place where the policy will be stored.
///
/// [argument, flags]
/// Unused.
///
/// = See also
///
/// * link:man:sched_getattr(2)
pub fn sched_getattr(pid: pid_t, attr: &mut sched_attr, flags: c_uint) -> c_int {
    unsafe {
        r::sched_getattr(pid, attr, mem::size_of::<sched_attr>() as c_uint, flags)
    }
}

/// Relinquish the CPU.
///
/// = See also
///
/// * link:man:sched_yield(2)
pub fn sched_yield() -> c_int {
    unsafe { r::sched_yield() }
}

/// Returns the maximum priority of a scheduling policy.
///
/// [argument, policy]
/// The policy.
///
/// = See also
///
/// * link:man:sched_get_priority_max(2)
pub fn sched_get_priority_max(policy: c_int) -> c_int {
    unsafe { r::sched_get_priority_max(policy) }
}

/// Returns the minimum priority of a scheduling policy.
///
/// [argument, policy]
/// The policy.
///
/// = See also
///
/// * link:man:sched_get_priority_min(2)
pub fn sched_get_priority_min(policy: c_int) -> c_int {
    unsafe { r::sched_get_priority_min(policy) }
}

/// Returns the round-robin time slice of a thread.
///
/// [argument, pid]
/// The thread to inspect.
///
/// [argument, tp]
/// Place where the time slice will be stored.
///
/// = See also
///
/// * link:man:sched_rr_get_interval(2)
pub fn sched_rr_get_interval(pid: pid_t, tp: &mut timespec) -> c_int {
    unsafe { r::sched_rr_get_interval(pid, tp) }
}

/// Returns the scheduling priority of a process, process group, or user.
///
/// [argument, which]
/// The id type of `who`.
///
/// [argument, who]
/// Whose priority to return.
///
/// = See also
///
/// * link:man:getpriority(2)
pub fn getpriority(which: c_int, who: c_int) -> c_int {
    unsafe { r::getpriority(which, who) }
}

/// Sets the scheduling priority of a process, process group, or user.
///
/// [argument, which]
/// The id type of `who`.
///
/// [argument, who]
/// Whose priority to set.
///
/// [argument, prio]
/// The scheduling priority.
///
/// = See also
///
/// * link:man:setpriority(2)
pub fn setpriority(which: c_int, who: c_int, prio: c_int) -> c_int {
    unsafe { r::setpriority(which, who, prio) }
}

/// Retrieves the capabilities of a thread.
///
/// [argument, tid]
/// The thread id.
///
/// [argument, buf]
/// The buffer in which the capabilities will be stored.
///
/// = See also
///
/// * link:man:capget(2)
pub fn capget_v3(tid: c_int, buf: &mut [__user_cap_data_struct; 2]) -> c_int {
    let mut header = __user_cap_header_struct {
        version: _LINUX_CAPABILITY_VERSION_3 as u32,
        pid: tid,
    };
    unsafe { r::capget(&mut header, buf.as_mut_ptr()) }
}

/// Sets the capabilities of this thread.
///
/// [argument, caps]
/// The capabilities.
///
/// = See also
///
/// * link:man:capset(2)
pub fn capset_v3(caps: &[__user_cap_data_struct; 2]) -> c_int {
    let mut header = __user_cap_header_struct {
        version: _LINUX_CAPABILITY_VERSION_3 as u32,
        pid: 0,
    };
    unsafe { r::capset(&mut header, caps.as_ptr() as *mut _) }
}

/// Checks whether a capability is in the bounding set of this thread.
///
/// [argument, cap]
/// The capability to check.
///
/// = See also
///
/// * link:man:prctl(2) and PR_CAPBSET_READ therein
pub fn prctl_pr_capbset_read(cap: c_int) -> c_int {
    unsafe { r::prctl(PR_CAPBSET_READ, cap as k_ulong, 0, 0, 0) }
}

/// Removes a capability from this thread's bounding set.
///
/// [argument, cap]
/// The capability to remove.
///
/// = See also
///
/// * link:man:prctl(2) and PR_CAPBSET_DROP therein
pub fn prctl_pr_capbset_drop(cap: c_int) -> c_int {
    unsafe { r::prctl(PR_CAPBSET_DROP, cap as k_ulong, 0, 0, 0) }
}

/// Checks whether capabilities are dropped when all root user ids are dropped.
///
/// = See also
///
/// * link:man:prctl(2) and PR_GET_KEEPCAPS therein
pub fn prctl_pr_get_keepcaps() -> c_int {
    unsafe { r::prctl(PR_GET_KEEPCAPS, 0, 0, 0, 0) }
}

/// Sets whether capabilities are dropped when all root user ids are dropped.
///
/// [argument, keep]
/// Whether the capabilities are kept.
///
/// = See also
///
/// * link:man:prctl(2) and PR_SET_KEEPCAPS therein
pub fn prctl_pr_set_keepcaps(keep: bool) -> c_int {
    unsafe { r::prctl(PR_SET_KEEPCAPS, keep as k_ulong, 0, 0, 0) }
}

/// Disassociate parts of the thread's execution context.
///
/// [argument, flags]
/// What to disassociate.
///
/// = See also
///
/// * link:man:unshare(2)
pub fn unshare(flags: c_int) -> c_int {
    unsafe { r::unshare(flags as k_ulong) }
}

/// Retrieves the CPU and NUMA node this thread is running one.
///
/// [argument, cpu]
/// Place where the CPU will be stored.
///
/// [argument, node]
/// Place where the NUMA node will be stored.
///
/// = See also
///
/// * link:man:getcpu(2)
pub fn getcpu(cpu: Option<&mut c_uint>, node: Option<&mut c_uint>) -> c_int {
    let cpu = cpu.map(|c| c as *mut _).unwrap_or(0 as *mut _);
    let node = node.map(|c| c as *mut _).unwrap_or(0 as *mut _);
    unsafe { r::getcpu(cpu, node, 0 as *mut _) }
}

/// Associate a thread with a namespace.
///
/// [argument, fd]
/// A file descriptor referring to the namespace.
///
/// [argument, nstype]
/// Which namespace types can be joined.
///
/// = See also
///
/// * link:man:setns(2)
pub fn setns(fd: c_int, nstype: c_int) -> c_int {
    unsafe { r::setns(fd, nstype) }
}

/// Enables strict seccomp mode for this thread.
///
/// = See also
///
/// * link:man:seccomp(2) and SECCOMP_SET_MODE_STRICT therein
pub fn seccomp_seccomp_set_mode_strict() -> c_int {
    unsafe { r::seccomp(SECCOMP_SET_MODE_STRICT, 0, 0 as *mut _) }
}

/// Adds a swap file/device.
///
/// [argument, path]
/// The path of the file/device.
///
/// [argument, flags]
/// Flags to modify the swap behavior.
///
/// = See also
///
/// * link:man:swapon(2)
pub fn swapon(path: &CStr, flags: c_int) -> c_int {
    unsafe { r::swapon(path.as_ptr(), flags) }
}

/// Removes a swap file/device.
///
/// [argument, path]
/// The path of the file/device.
///
/// = See also
///
/// * link:man:swapoff(2)
pub fn swapoff(path: &CStr) -> c_int {
    unsafe { r::swapoff(path.as_ptr()) }
}

/// Changes the root directory of the process.
///
/// [argument, path]
/// The new root directory.
///
/// = See also
///
/// * link:man:chroot(2)
pub fn chroot(path: &CStr) -> c_int {
    unsafe { r::chroot(path.as_ptr()) }
}

/// Moves the current root directory and sets a new one.
///
/// [argument, new]
/// The path of the new root directory.
///
/// [argument, old]
/// Where the old root directory will me moved to.
///
/// = See also
///
/// * link:man:pivot_root(2)
pub fn pivot_root(new: &CStr, old: &CStr) -> c_int {
    unsafe { r::pivot_root(new.as_ptr(), old.as_ptr()) }
}
