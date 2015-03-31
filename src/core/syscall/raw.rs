// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{mem};
use arch::syscall::{nr, syscall0, syscall1, syscall2, syscall3, syscall4, SCT};
use cty::{c_int, mode_t, size_t, ssize_t, uid_t, gid_t, F_DUPFD_CLOEXEC, F_GETFD,
          F_GETFL, F_SETFD, F_SETFL, statfs, pid_t, c_char, off_t, iovec, c_void,
          rlimit, linux_dirent64, stat};

macro_rules! call {
    ($nr:expr) => {
        syscall0($nr as SCT)
    };

    ($nr:expr, $a1:expr) => {
        syscall1($nr as SCT, $a1 as SCT)
    };

    ($nr:expr, $a1:expr, $a2:expr) => {
        syscall2($nr as SCT, $a1 as SCT, $a2 as SCT)
    };

    ($nr:expr, $a1:expr, $a2:expr, $a3:expr) => {
        syscall3($nr as SCT, $a1 as SCT, $a2 as SCT, $a3 as SCT)
    };

    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {
        syscall4($nr as SCT, $a1 as SCT, $a2 as SCT, $a3 as SCT, $a4 as SCT)
    };

    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {
        syscall5($nr as SCT, $a1 as SCT, $a2 as SCT, $a3 as SCT, $a4 as SCT,
                 $a5 as SCT)
    };

    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        syscall6($nr as SCT, $a1 as SCT, $a2 as SCT, $a3 as SCT, $a4 as SCT, $a5 as SCT,
                 $a6 as SCT)
    };
}

pub unsafe fn __openat(dir: c_int, fd: *const c_char, flags: c_int,
                       mode: mode_t) -> c_int {
    call!(nr::OPENAT, dir, fd, flags, mode) as c_int
}

pub unsafe fn __read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t {
    call!(nr::READ, fd, buf, count) as ssize_t
}

pub unsafe fn __write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t {
    call!(nr::WRITE, fd, buf, count) as ssize_t
}

pub fn close(fd: c_int) -> c_int {
    unsafe { call!(nr::CLOSE, fd) as c_int }
}

#[cfg(target_arch = "x86_64")]
pub fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t {
    unsafe { call!(nr::LSEEK, fd, offset, whence) as off_t }
}

pub fn fcntl_dupfd_cloexec(fd: c_int, arg: c_int) -> c_int {
    unsafe { call!(nr::FCNTL, fd, F_DUPFD_CLOEXEC, arg) as c_int }
}

pub fn fcntl_getfl(fd: c_int) -> c_int {
    unsafe { call!(nr::FCNTL, fd, F_GETFL) as c_int }
}

pub fn fcntl_setfl(fd: c_int, arg: c_int) -> c_int {
    unsafe { call!(nr::FCNTL, fd, F_SETFL, arg) as c_int }
}

pub fn fcntl_getfd(fd: c_int) -> c_int {
    unsafe { call!(nr::FCNTL, fd, F_GETFD) as c_int }
}

pub fn fcntl_setfd(fd: c_int, arg: c_int) -> c_int {
    unsafe { call!(nr::FCNTL, fd, F_SETFD, arg) as c_int }
}

pub unsafe fn __pread(fd: c_int, buf: *mut c_void, count: size_t,
                    offset: off_t) -> ssize_t {
    call!(nr::PREAD, fd, buf, count, offset) as ssize_t
}

pub unsafe fn __pwrite(fd: c_int, buf: *const c_void, count: size_t,
                     offset: off_t) -> ssize_t {
    call!(nr::PWRITE, fd, buf, count, offset) as ssize_t
}

pub unsafe fn __readv(fd: c_int, iovec: *const iovec, count: c_int) -> ssize_t {
    call!(nr::READV, fd, iovec, count) as ssize_t
}

pub unsafe fn __writev(fd: c_int, iovec: *const iovec, count: c_int) -> ssize_t {
    call!(nr::WRITEV, fd, iovec, count) as ssize_t
}

pub unsafe fn __preadv(fd: c_int, iovec: *const iovec, count: c_int,
                     offset: off_t) -> ssize_t {
    call!(nr::PREADV, fd, iovec, count, offset) as ssize_t
}

pub unsafe fn __pwritev(fd: c_int, iovec: *const iovec, count: c_int,
                      offset: off_t) -> ssize_t {
    call!(nr::PWRITEV, fd, iovec, count, offset) as ssize_t
}

pub fn ftruncate(fd: c_int, offset: off_t) -> c_int {
    unsafe { call!(nr::FTRUNCATE, fd, offset) as c_int }
}

pub fn getpid() -> pid_t {
    unsafe { call!(nr::GETPID) as pid_t }
}

pub fn getppid() -> pid_t {
    unsafe { call!(nr::GETPPID) as pid_t }
}

pub unsafe fn __getresuid(ruid: *mut uid_t, euid: *mut uid_t, suid: *mut uid_t) -> c_int {
    call!(nr::GETRESUID, ruid, euid, suid) as c_int
}

pub unsafe fn __getresgid(rgid: *mut gid_t, egid: *mut gid_t, sgid: *mut gid_t) -> c_int {
    call!(nr::GETRESGID, rgid, egid, sgid) as c_int
}

pub fn setresuid(ruid: uid_t, euid: uid_t, suid: uid_t) -> c_int {
    unsafe { call!(nr::SETRESUID, ruid, euid, suid) as c_int }
}

pub fn setresgid(rgid: gid_t, egid: gid_t, sgid: gid_t) -> c_int {
    unsafe { call!(nr::SETRESGID, rgid, egid, sgid) as c_int }
}

pub unsafe fn __getgroups(size: c_int, list: *mut gid_t) -> c_int {
    call!(nr::GETGROUPS, size, list) as c_int
}

pub unsafe fn __setgroups(size: size_t, list: *const gid_t) -> c_int {
    call!(nr::SETGROUPS, size, list) as c_int
}

pub fn fsync(fd: c_int) -> c_int {
    unsafe { call!(nr::FSYNC, fd) as c_int }
}

pub fn fdatasync(fd: c_int) -> c_int {
    unsafe { call!(nr::FDATASYNC, fd) as c_int }
}

pub fn sync() {
    unsafe { call!(nr::SYNC); }
}

pub fn syncfs(fd: c_int) -> c_int {
    unsafe { call!(nr::SYNCFS, fd) as c_int }
}

#[cfg(target_arch = "x86_64")]
pub fn fadvise(fd: c_int, offset: off_t, len: off_t, advise: c_int) -> c_int {
    unsafe { call!(nr::FADVISE, fd, offset, len, advise) as c_int }
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn __statfs(file: *const c_char, buf: *mut statfs) -> c_int {
    *buf = mem::zeroed();
    call!(nr::STATFS, file, buf) as c_int
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn __fstatfs(fd: c_int, buf: *mut statfs) -> c_int {
    *buf = mem::zeroed();
    call!(nr::FSTATFS, fd, buf) as c_int
}

pub unsafe fn __prlimit(pid: pid_t, res: c_int, new: *const rlimit,
                        old: *mut rlimit) -> c_int {
    call!(nr::PRLIMIT64, pid, res, new, old) as c_int
}

pub unsafe fn __getdents(fd: c_int, dirp: *mut linux_dirent64, count: c_int) -> c_int {
    call!(nr::GETDENTS, fd, dirp, count) as c_int
}

pub unsafe fn __fstat(fd: c_int, buf: *mut stat) -> c_int {
    call!(nr::FSTAT, fd, buf) as c_int
}
