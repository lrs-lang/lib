// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{mem};
use arch::syscall::{nr, syscall0, syscall1, syscall2, syscall3, syscall4, syscall5,
                    syscall6, SCT};
use cty::{c_int, mode_t, size_t, ssize_t, uid_t, gid_t, F_DUPFD_CLOEXEC, F_GETFD,
          F_GETFL, F_SETFD, F_SETFL, statfs, pid_t, c_char, off_t, iovec, c_void,
          rlimit, linux_dirent64, stat, timespec, dev_t, clockid_t, itimerspec,
          epoll_event, sigset_t, utsname, sysinfo, c_uint, c_ulong};

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

pub unsafe fn __fstatat(dir: c_int, file: *const c_char, buf: *mut stat,
                        flags: c_int) -> c_int {
    call!(nr::FSTATAT, dir, file, buf, flags) as c_int
}

pub unsafe fn __faccessat(dir: c_int, file: *const c_char, mode: c_int) -> c_int {
    call!(nr::FACCESSAT, dir, file, mode) as c_int
}

pub unsafe fn __truncate(file: *const c_char, len: off_t) -> c_int {
    call!(nr::TRUNCATE, file, len) as c_int
}

pub unsafe fn __linkat(olddir: c_int, oldfile: *const c_char, newdir: c_int,
                       newfile: *const c_char, flags: c_int) -> c_int {
    call!(nr::LINKAT, olddir, oldfile, newdir, newfile, flags) as c_int
}

pub unsafe fn __utimensat(dir: c_int, file: *const c_char, times: *const timespec,
                          flags: c_int) -> c_int {
    call!(nr::UTIMENSAT, dir, file, times, flags) as c_int
}

pub unsafe fn __renameat2(olddir: c_int, oldfile: *const c_char, newdir: c_int,
                          newfile: *const c_char, flags: c_int) -> c_int {
    call!(nr::RENAMEAT2, olddir, oldfile, newdir, newfile, flags) as c_int
}

pub unsafe fn __mkdirat(dir: c_int, file: *const c_char, mode: mode_t) -> c_int {
    call!(nr::MKDIRAT, dir, file, mode) as c_int
}

pub unsafe fn __unlinkat(dir: c_int, file: *const c_char, flags: c_int) -> c_int {
    call!(nr::UNLINKAT, dir, file, flags) as c_int
}

pub unsafe fn __symlinkat(target: *const c_char, dir: c_int,
                          link: *const c_char) -> c_int {
    call!(nr::SYMLINKAT, target, dir, link) as c_int
}

pub unsafe fn __readlinkat(dir: c_int, path: *const c_char, buf: *mut c_char,
                           size: size_t) -> ssize_t {
    call!(nr::READLINKAT, dir, path, buf, size) as ssize_t
}

pub unsafe fn __fchownat(dir: c_int, path: *const c_char, user: uid_t, group: gid_t,
                         flags: c_int) -> c_int {
    call!(nr::FCHOWNAT, dir, path, user, group, flags) as c_int
}

pub fn fchmod(fd: c_int, mode: mode_t) -> c_int {
    unsafe { call!(nr::FCHMOD, fd, mode) as c_int }
}

pub unsafe fn __fchmodat(dir: c_int, path: *const c_char, mode: mode_t) -> c_int {
    call!(nr::FCHMODAT, dir, path, mode) as c_int
}

pub unsafe fn __mknodat(dir: c_int, path: *const c_char, mode: mode_t,
                        dev: dev_t) -> c_int {
    call!(nr::MKNODAT, dir, path, mode, dev) as c_int
}

#[cfg(target_arch = "x86_64")]
pub fn readahead(fd: c_int, offset: off_t, count: size_t) -> ssize_t {
    unsafe { call!(nr::READAHEAD, fd, offset, count) as ssize_t }
}

#[cfg(target_arch = "x86_64")]
pub fn fallocate(fd: c_int, mode: c_int, base: off_t, len: off_t) -> c_int {
    unsafe { call!(nr::FALLOCATE, fd, mode, base, len) as c_int }
}

pub unsafe fn __setxattr(path: *const c_char, name: *const c_char, val: *const c_void,
                         size: size_t, flags: c_int) -> c_int {
    call!(nr::SETXATTR, path, name, val, size, flags) as c_int
}

pub unsafe fn __lsetxattr(path: *const c_char, name: *const c_char, val: *const c_void,
                         size: size_t, flags: c_int) -> c_int {
    call!(nr::LSETXATTR, path, name, val, size, flags) as c_int
}

pub unsafe fn __fsetxattr(fd: c_int, name: *const c_char, val: *const c_void,
                         size: size_t, flags: c_int) -> c_int {
    call!(nr::FSETXATTR, fd, name, val, size, flags) as c_int
}

pub unsafe fn __getxattr(path: *const c_char, name: *const c_char, val: *mut c_void,
                         size: size_t) -> ssize_t {
    call!(nr::GETXATTR, path, name, val, size) as ssize_t
}

pub unsafe fn __lgetxattr(path: *const c_char, name: *const c_char, val: *mut c_void,
                         size: size_t) -> ssize_t {
    call!(nr::LGETXATTR, path, name, val, size) as ssize_t
}

pub unsafe fn __fgetxattr(fd: c_int, name: *const c_char, val: *mut c_void,
                         size: size_t) -> ssize_t {
    call!(nr::FGETXATTR, fd, name, val, size) as ssize_t
}

pub unsafe fn __removexattr(path: *const c_char, name: *const c_char) -> c_int {
    call!(nr::REMOVEXATTR, path, name) as c_int
}

pub unsafe fn __lremovexattr(path: *const c_char, name: *const c_char) -> c_int {
    call!(nr::LREMOVEXATTR, path, name) as c_int
}

pub unsafe fn __fremovexattr(fd: c_int, name: *const c_char) -> c_int {
    call!(nr::FREMOVEXATTR, fd, name) as c_int
}

pub unsafe fn __listxattr(path: *const c_char, list: *mut c_char,
                          size: size_t) -> ssize_t {
    call!(nr::LISTXATTR, path, list, size) as ssize_t
}

pub unsafe fn __llistxattr(path: *const c_char, list: *mut c_char,
                          size: size_t) -> ssize_t {
    call!(nr::LLISTXATTR, path, list, size) as ssize_t
}

pub unsafe fn __flistxattr(fd: c_int, list: *mut c_char, size: size_t) -> ssize_t {
    call!(nr::FLISTXATTR, fd, list, size) as ssize_t
}

pub fn flock(fd: c_int, op: c_int) -> c_int {
    unsafe { call!(nr::FLOCK, fd, op) as c_int }
}

pub unsafe fn __clock_getres(clock: clockid_t, res: *mut timespec) -> c_int {
    call!(nr::CLOCK_GETRES, clock, res) as c_int
}

pub unsafe fn __clock_gettime(clock: clockid_t, res: *mut timespec) -> c_int {
    call!(nr::CLOCK_GETTIME, clock, res) as c_int
}

pub unsafe fn __clock_settime(clock: clockid_t, res: *const timespec) -> c_int {
    call!(nr::CLOCK_SETTIME, clock, res) as c_int
}

pub unsafe fn __clock_nanosleep(clock: clockid_t, flags: c_int, req: *const timespec,
                                rem: *mut timespec) -> c_int {
    call!(nr::CLOCK_NANOSLEEP, clock, flags, req, rem) as c_int
}

pub fn timerfd_create(clock: c_int, flags: c_int) -> c_int {
    unsafe { call!(nr::TIMERFD_CREATE, clock, flags) as c_int }
}

pub unsafe fn __timerfd_settime(fd: c_int, flags: c_int, new: *const itimerspec,
                                old: *mut itimerspec) -> c_int {
    call!(nr::TIMERFD_SETTIME, fd, flags, new, old) as c_int
}

pub unsafe fn __timerfd_gettime(fd: c_int, cur: *mut itimerspec) -> c_int {
    call!(nr::TIMERFD_GETTIME, fd, cur) as c_int
}

pub fn epoll_create1(flags: c_int) -> c_int {
    unsafe { call!(nr::EPOLL_CREATE1, flags) as c_int }
}

pub unsafe fn __epoll_ctl(epfd: c_int, op: c_int, fd: c_int,
                          event: *mut epoll_event) -> c_int {
    call!(nr::EPOLL_CTL, epfd, op, fd, event) as c_int
}

pub unsafe fn __epoll_pwait(epfd: c_int, events: *mut epoll_event, num: c_int,
                            timeout: c_int, sigmask: *const sigset_t,
                            sigsetsize: size_t) -> c_int {
    call!(nr::EPOLL_PWAIT, epfd, events, num, timeout, sigmask, sigsetsize) as c_int
}

pub unsafe fn __sched_getaffinity(tid: pid_t, size: size_t, set: *mut u8) -> c_int {
    call!(nr::SCHED_GETAFFINITY, tid, size, set) as c_int
}

pub unsafe fn __uname(buf: *mut utsname) -> c_int {
    call!(nr::UNAME, buf) as c_int
}

pub unsafe fn __sysinfo(buf: *mut sysinfo) -> c_int {
    call!(nr::SYSINFO, buf) as c_int
}

pub unsafe fn __getrandom(buf: *mut c_void, buflen: size_t, flags: c_uint) -> c_int {
    call!(nr::GETRANDOM, buf, buflen, flags) as c_int
}

pub unsafe fn __acct(filename: *const c_char) -> c_int {
    call!(nr::ACCT, filename) as c_int
}

pub unsafe fn __mount(src: *const c_char, dst: *const c_char, ty: *const c_char,
                      flags: c_ulong, data: *const c_void) -> c_int {
    call!(nr::MOUNT, src, dst, ty, flags, data) as c_int
}

pub unsafe fn __umount2(dst: *const c_char, flags: c_int) -> c_int {
    call!(nr::UMOUNT2, dst, flags) as c_int
}

pub unsafe fn __sethostname(name: *const c_char, len: size_t) -> c_int {
    call!(nr::SETHOSTNAME, name, len) as c_int
}

pub unsafe fn __setdomainname(name: *const c_char, len: size_t) -> c_int {
    call!(nr::SETDOMAINNAME, name, len) as c_int
}
