// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::{mem};

pub use ::common::{
    accept, accept4, acct, add_key, adjtimex, bind, bpf, brk, capget,
    capset, chdir, chroot, clock_adjtime, clock_getres, clock_gettime,
    clock_nanosleep, clock_settime, close, connect, delete_module, dup, dup3,
    epoll_create1, epoll_ctl, epoll_pwait, eventfd2,
    execve, execveat, exit, exit_group, faccessat, fanotify_init,
    fchdir, fchmod, fchmodat, fchown, fchownat, fdatasync,
    fgetxattr, finit_module, flistxattr, flock, fremovexattr, fsetxattr,
    fsync, futex, getcpu, getcwd, getegid, geteuid, getgid,
    getgroups, getitimer, get_mempolicy, getpeername, getpgid, getpid, getppid,
    getpriority, getrandom, getresgid, getresuid, getrlimit, get_robust_list, getrusage,
    getsid, getsockname, getsockopt, gettid, gettimeofday, getuid, getxattr, init_module,
    inotify_add_watch, inotify_init1, inotify_rm_watch, io_cancel, ioctl,
    io_destroy, io_getevents, ioprio_get, ioprio_set, io_setup, io_submit, kcmp,
    kexec_load, keyctl, kill, lgetxattr, linkat, listen,
    listxattr, llistxattr, lremovexattr, lsetxattr, madvise, mbind,
    memfd_create, mincore, mkdirat, mknodat, mlock, mlockall,
    mount, move_pages, mprotect, mq_getsetattr, mq_open, mq_timedreceive, mq_timedsend,
    mq_unlink, mremap, msgctl, msgget, msgrcv, msgsnd, msync, munlock, munlockall, munmap,
    name_to_handle_at, nanosleep, openat, open_by_handle_at, perf_event_open,
    personality, pipe2, pivot_root, ppoll, prctl, preadv, process_vm_readv,
    process_vm_writev, pselect6, ptrace, pwritev, quotactl, read,
    readlinkat, readv, reboot, recvfrom, recvmmsg, recvmsg, remap_file_pages, removexattr,
    renameat, renameat2, request_key, restart_syscall, rt_sigaction,
    rt_sigpending, rt_sigprocmask, rt_sigqueueinfo, rt_sigsuspend, rt_sigreturn,
    rt_sigtimedwait, rt_tgsigqueueinfo, sched_getaffinity, sched_getattr, sched_getparam,
    sched_get_priority_max, sched_get_priority_min, sched_getscheduler,
    sched_rr_get_interval, sched_setaffinity, sched_setattr, sched_setparam,
    sched_setscheduler, sched_yield, seccomp, semget, semop, semtimedop, sendmmsg,
    sendmsg, sendto, setdomainname, setfsgid, setfsuid, setgid, setgroups, sethostname,
    setitimer, set_mempolicy, setns, setpgid, setpriority, setregid, setresgid, setresuid,
    setreuid, setrlimit, set_robust_list, setsid, setsockopt, set_tid_address,
    settimeofday, setuid, setxattr, shmat, shmctl, shmdt, shmget, shutdown, sigaltstack,
    signalfd4, socket, socketpair, splice, swapoff, swapon,
    symlinkat, sync, syncfs, sysinfo, syslog, tee, tgkill,
    timer_delete, timerfd_create, timerfd_gettime, timerfd_settime, timer_getoverrun,
    timer_gettime, timer_settime, times, tkill, umask, umount, unlinkat,
    unshare, utimensat, vhangup, vmsplice, waitid,
    write, writev,
};

use cty::{
    self,
    c_uint, k_int, k_long, k_ulong, user_desc, c_char, k_uint, linux_dirent64, loff_t,
    new_utsname, pid_t, rlimit64, size_t, ssize_t, statfs64, stat64, EINVAL, c_long,
    __u64, __NR_clone, c_int,
};

pub type SCT = c_long;

#[inline(always)]
pub unsafe fn syscall0(n: SCT) -> SCT {
    let ret: SCT;
    asm!("int $$0x80" : "={eax}"(ret)
                      : "{eax}"(n)
                      : "memory", "cc"
                      : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall1(n: SCT, a1: SCT) -> SCT {
    let ret: SCT;
    asm!("int $$0x80" : "={eax}"(ret)
                      : "{eax}"(n), "{ebx}"(a1)
                      : "memory", "cc"
                      : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall2(n: SCT, a1: SCT, a2: SCT) -> SCT {
    let ret: SCT;
    asm!("int $$0x80" : "={eax}"(ret)
                      : "{eax}"(n), "{ebx}"(a1), "{ecx}"(a2)
                      : "memory", "cc"
                      : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall3(n: SCT, a1: SCT, a2: SCT, a3: SCT) -> SCT {
    let ret: SCT;
    asm!("int $$0x80" : "={eax}"(ret)
                      : "{eax}"(n), "{ebx}"(a1), "{ecx}"(a2), "{edx}"(a3)
                      : "memory", "cc"
                      : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall4(n: SCT, a1: SCT, a2: SCT, a3: SCT, a4: SCT) -> SCT {
    let ret: SCT;
    asm!("int $$0x80" : "={eax}"(ret)
                      : "{eax}"(n), "{ebx}"(a1), "{ecx}"(a2), "{edx}"(a3), "{esi}"(a4)
                      : "memory", "cc"
                      : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall5(n: SCT, a1: SCT, a2: SCT, a3: SCT, a4: SCT, a5: SCT) -> SCT {
    let ret: SCT;
    asm!("int $$0x80" : "={eax}"(ret)
                      : "{eax}"(n), "{ebx}"(a1), "{ecx}"(a2), "{edx}"(a3), "{esi}"(a4),
                        "{edi}"(a5)
                      : "memory", "cc"
                      : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall6(n: SCT, a1: SCT, a2: SCT, a3: SCT, a4: SCT, a5: SCT,
                       a6: SCT) -> SCT {
    let ret: SCT;
    asm!("int $$0x80" : "={eax}"(ret)
                      : "{eax}"(n), "{ebx}"(a1), "{ecx}"(a2), "{edx}"(a3), "{esi}"(a4),
                        "{edi}"(a5), "{ebp}"(a6)
                      : "memory", "cc"
                      : "volatile");
    ret
}

fn split_u64(val: u64) -> [u32; 2] {
    unsafe { mem::cast(val) }
}

fn split_i64(val: i64) -> [u32; 2] {
    unsafe { mem::cast(val) }
}

pub type StatType = stat64;
pub type StatfsType = statfs64;

pub unsafe fn statfs(pathname: *const c_char, buf: *mut statfs64) -> k_int {
    ::common::statfs64(pathname, mem::size_of::<statfs64>() as size_t, buf)
}

pub unsafe fn fstatfs(fd: k_uint, buf: *mut statfs64) -> k_int {
    ::common::fstatfs64(fd,  mem::size_of::<statfs64>() as size_t, buf)
}

pub unsafe fn fcntl(fd: k_uint, cmd: k_uint, arg: k_ulong) -> k_int {
    ::common::fcntl64(fd, cmd, arg)
}

pub unsafe fn pread(fd: k_uint, buf: *mut c_char, count: size_t, pos: loff_t) -> ssize_t {
    let [pos_lo, pos_hi] = split_i64(pos);
    call!(cty::__NR_pread64, fd, buf, count, pos_lo, pos_hi) as ssize_t
}

pub unsafe fn pwrite(fd: k_uint, buf: *const c_char, count: size_t,
                     pos: loff_t) -> ssize_t {
    let [pos_lo, pos_hi] = split_i64(pos);
    call!(cty::__NR_pwrite64, fd, buf, count, pos_lo, pos_hi) as ssize_t
}

pub unsafe fn sendfile(out_fd: k_int, in_fd: k_int, offset: *mut loff_t,
                       count: size_t) -> ssize_t {
    ::common::sendfile64(out_fd, in_fd, offset, count)
}

pub unsafe fn uname(name: *mut new_utsname) -> k_int {
    ::common::newuname(name)
}

pub unsafe fn getdents(fd: k_uint, dirent: *mut linux_dirent64, count: k_uint) -> k_int {
    ::common::getdents64(fd, dirent, count)
}

pub unsafe fn fadvise(fd: k_int, offset: loff_t, len: loff_t, advice: k_int) -> k_int {
    let [offset_lo, offset_hi] = split_i64(offset);
    let [len_lo, len_hi] = split_i64(len);
    call!(cty::__NR_fadvise64_64, fd, offset_lo, offset_hi, len_lo, len_hi,
          advice) as k_int
}

pub unsafe fn fstatat(dfd: k_int, filename: *const c_char, statbuf: *mut stat64,
                      flag: k_int) -> k_int {
    ::common::fstatat64(dfd, filename, statbuf, flag)
}

pub unsafe fn prlimit(pid: pid_t, resource: k_uint, new_rlim: *const rlimit64,
                      old_rlim: *mut rlimit64) -> k_int {
    ::common::prlimit64(pid, resource, new_rlim, old_rlim)
}

pub unsafe fn ftruncate(fd: k_uint, length: loff_t) -> k_int {
    let [length_lo, length_hi] = split_i64(length);
    call!(cty::__NR_ftruncate64, fd, length_lo, length_hi) as k_int
}

pub unsafe fn truncate(path: *const c_char, length: loff_t) -> k_int {
    let [length_lo, length_hi] = split_i64(length);
    call!(cty::__NR_truncate64, path, length_lo, length_hi) as k_int
}

pub unsafe fn lseek(fd: k_uint, offset: loff_t, whence: k_uint) -> loff_t {
    let mut res = 0;
    let rv = ::common::llseek(fd, (offset >> 32) as k_ulong, offset as k_ulong,
                              &mut res, whence);
    if rv < 0 {
        rv as loff_t
    } else {
        res
    }
}

pub unsafe fn sync_file_range(fd: k_int, offset: loff_t, nbytes: loff_t,
                              flags: k_uint) -> k_int {
    let [offset_lo, offset_hi] = split_i64(offset);
    let [nbytes_lo, nbytes_hi] = split_i64(nbytes);
    call!(cty::__NR_sync_file_range, fd, offset_lo, offset_hi, nbytes_lo,
          nbytes_hi, flags) as k_int
}

pub unsafe fn fallocate(fd: k_int, mode: k_int, offset: loff_t,
                        len: loff_t) -> k_int {
    let [offset_lo, offset_hi] = split_i64(offset);
    let [len_lo, len_hi] = split_i64(len);
    call!(cty::__NR_fallocate, fd, mode, offset_lo, offset_hi, len_lo, len_hi) as k_int
}

pub unsafe fn fanotify_mark(fanotify_fd: k_int, flags: k_uint, mask: __u64,
                            dfd: k_int, pathname: *const c_char) -> k_int {
    let [mask_lo, mask_hi] = split_u64(mask);
    call!(cty::__NR_fanotify_mark, fanotify_fd, flags, mask_lo, mask_hi, dfd,
          pathname) as k_int
}

pub unsafe fn lookup_dcookie(cookie: u64, buf: *mut c_char, len: size_t) -> k_int {
    let [cookie_lo, cookie_hi] = split_u64(cookie);
    call!(cty::__NR_lookup_dcookie, cookie_lo, cookie_hi, buf, len) as k_int
}

pub unsafe fn readahead(fd: k_int, offset: loff_t, count: size_t) -> ssize_t {
    let [offset_lo, offset_hi] = split_i64(offset);
    call!(cty::__NR_readahead, fd, offset_lo, offset_hi, count) as ssize_t
}

pub unsafe fn iopl(level: c_uint) -> k_int {
    call!(cty::__NR_iopl, level) as k_int
}

pub unsafe fn set_thread_area(u_info: *mut user_desc) -> k_int {
    call!(cty::__NR_set_thread_area, u_info) as k_int
}

pub unsafe fn get_thread_area(u_info: *mut user_desc) -> k_int {
    call!(cty::__NR_get_thread_area, u_info) as k_int
}

pub unsafe fn mmap(addr: k_ulong, len: k_ulong, prot: k_ulong, flags: k_ulong,
                   fd: k_ulong, off: u64) -> k_long {
    if off & (4096 - 1) != 0 {
        return -EINVAL;
    }
    call!(cty::__NR_mmap_pgoff, addr, len, prot, flags, fd, off >> 12) as k_long
}

pub unsafe fn clone(flags: k_ulong, newsp: *mut u8, parent_tidptr: *mut c_int,
                    child_tidptr: *mut c_int, tls: *mut u8) -> k_long {
    call!(__NR_clone, flags, newsp, parent_tidptr, tls, child_tidptr) as k_long
}
