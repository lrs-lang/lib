// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use ::common::{
    accept, accept4, acct, add_key, adjtimex, bind, bpf, brk, capget,
    capset, chdir, chroot, clock_adjtime, clock_getres, clock_gettime,
    clock_nanosleep, clock_settime, close, connect, delete_module, dup, dup3,
    epoll_create1, epoll_ctl, epoll_pwait, eventfd2,
    execve, execveat, exit, exit_group, faccessat, fallocate, fanotify_init,
    fanotify_mark, fchdir, fchmod, fchmodat, fchown, fchownat, fcntl, fdatasync,
    fgetxattr, finit_module, flistxattr, flock, fremovexattr, fsetxattr, fstatfs,
    fsync, ftruncate, futex, getcpu, getcwd, getegid, geteuid, getgid,
    getgroups, getitimer, get_mempolicy, getpeername, getpgid, getpid, getppid,
    getpriority, getrandom, getresgid, getresuid, getrlimit, get_robust_list, getrusage,
    getsid, getsockname, getsockopt, gettid, gettimeofday, getuid, getxattr, init_module,
    inotify_add_watch, inotify_init1, inotify_rm_watch, io_cancel, ioctl,
    io_destroy, io_getevents, ioprio_get, ioprio_set, io_setup, io_submit, kcmp,
    kexec_load, keyctl, kill, lgetxattr, linkat, listen,
    listxattr, llistxattr, lookup_dcookie, lremovexattr, lseek, lsetxattr, madvise, mbind,
    memfd_create, mincore, mkdirat, mknodat, mlock, mlockall,
    mount, move_pages, mprotect, mq_getsetattr, mq_open, mq_timedreceive, mq_timedsend,
    mq_unlink, mremap, msgctl, msgget, msgrcv, msgsnd, msync, munlock, munlockall, munmap,
    name_to_handle_at, nanosleep, openat, open_by_handle_at, perf_event_open,
    personality, pipe2, pivot_root, ppoll, prctl, preadv, process_vm_readv,
    process_vm_writev, pselect6, ptrace, pwritev, quotactl, read, readahead,
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
    signalfd4, socket, socketpair, splice, statfs, swapoff, swapon,
    symlinkat, sync, sync_file_range, syncfs, sysinfo, syslog, tee, tgkill,
    timer_delete, timerfd_create, timerfd_gettime, timerfd_settime, timer_getoverrun,
    timer_gettime, timer_settime, times, tkill, truncate, umask, umount, unlinkat,
    unshare, utimensat, vhangup, vmsplice, waitid,
    write, writev,
};

use cty::{
    self,
    k_int, k_long, k_ulong, c_char, k_uint, linux_dirent64, loff_t,
    new_utsname, pid_t, rlimit64, size_t, ssize_t, stat, c_int,

    __NR_mmap, __NR_clone,
};

pub type SCT = k_long;

#[inline(always)]
pub unsafe fn syscall0(n: SCT) -> SCT {
    let ret: SCT;
    asm!("svc 0" : "={x0}"(ret)
                 : "{x8}"(n)
                 : "memory", "cc"
                 : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall1(n: SCT, a1: SCT) -> SCT {
    let ret: SCT;
    asm!("svc 0" : "={x0}"(ret)
                 : "{x8}"(n), "{x0}"(a1)
                 : "memory", "cc"
                 : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall2(n: SCT, a1: SCT, a2: SCT) -> SCT {
    let ret: SCT;
    asm!("svc 0" : "={x0}"(ret)
                 : "{x8}"(n), "{x0}"(a1), "{x1}"(a2)
                 : "memory", "cc"
                 : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall3(n: SCT, a1: SCT, a2: SCT, a3: SCT) -> SCT {
    let ret: SCT;
    asm!("svc 0" : "={x0}"(ret)
                 : "{x8}"(n), "{x0}"(a1), "{x1}"(a2), "{x2}"(a3)
                 : "memory", "cc"
                 : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall4(n: SCT, a1: SCT, a2: SCT, a3: SCT, a4: SCT) -> SCT {
    let ret: SCT;
    asm!("svc 0" : "={x0}"(ret)
                 : "{x8}"(n), "{x0}"(a1), "{x1}"(a2), "{x2}"(a3), "{x3}"(a4)
                 : "memory", "cc"
                 : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall5(n: SCT, a1: SCT, a2: SCT, a3: SCT, a4: SCT, a5: SCT) -> SCT {
    let ret: SCT;
    asm!("svc 0" : "={x0}"(ret)
                 : "{x8}"(n), "{x0}"(a1), "{x1}"(a2), "{x2}"(a3), "{x3}"(a4), "{x4}"(a5)
                 : "memory", "cc"
                 : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall6(n: SCT, a1: SCT, a2: SCT, a3: SCT, a4: SCT, a5: SCT,
                       a6: SCT) -> SCT {
    let ret: SCT;
    asm!("svc 0" : "={x0}"(ret)
                 : "{x8}"(n), "{x0}"(a1), "{x1}"(a2), "{x2}"(a3), "{x3}"(a4),
                   "{x4}"(a5), "{x5}"(a6)
                 : "memory", "cc"
                 : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall7(n: SCT, a1: SCT, a2: SCT, a3: SCT, a4: SCT, a5: SCT, a6: SCT,
                       a7: SCT) -> SCT {
    let ret: SCT;
    asm!("svc 0" : "={x0}"(ret)
                 : "{x8}"(n), "{x0}"(a1), "{x1}"(a2), "{x2}"(a3), "{x3}"(a4),
                   "{x4}"(a5), "{x5}"(a6), "{x6}"(a7)
                 : "memory", "cc"
                 : "volatile");
    ret
}

pub type StatType = stat;
pub type StatfsType = cty::statfs;

pub unsafe fn pread(fd: k_uint, buf: *mut c_char, count: size_t, pos: loff_t) -> ssize_t {
    ::common::pread64(fd, buf, count, pos)
}

pub unsafe fn pwrite(fd: k_uint, buf: *const c_char, count: size_t,
                     pos: loff_t) -> ssize_t {
    ::common::pwrite64(fd, buf, count, pos)
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
    ::common::fadvise64_64(fd, offset, len, advice)
}

pub unsafe fn fstatat(dfd: k_int, filename: *const c_char, statbuf: *mut stat,
                      flag: k_int) -> k_int {
    ::common::newfstatat(dfd, filename, statbuf, flag)
}

pub unsafe fn prlimit(pid: pid_t, resource: k_uint, new_rlim: *const rlimit64,
                      old_rlim: *mut rlimit64) -> k_int {
    ::common::prlimit64(pid, resource, new_rlim, old_rlim)
}

pub unsafe fn mmap(addr: k_ulong, len: k_ulong, prot: k_ulong, flags: k_ulong,
                   fd: k_ulong, off: k_ulong) -> k_long {
    call!(__NR_mmap, addr, len, prot, flags, fd, off) as k_long
}

pub unsafe fn clone(flags: k_ulong, newsp: *mut u8, parent_tidptr: *mut c_int,
                    child_tidptr: *mut c_int, tls: *mut u8) -> k_long {
    call!(__NR_clone, flags, newsp, parent_tidptr, tls, child_tidptr) as k_long
}
