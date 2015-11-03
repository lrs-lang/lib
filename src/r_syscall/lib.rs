// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_r_syscall"]
#![crate_type = "lib"]
#![feature(asm, plugin, no_std, slice_patterns)]
#![plugin(lrs_core_plugin)]
#![no_std]
#![allow(dead_code)]

extern crate lrs_cty as cty;

pub use ::arch::{
    accept, accept4, acct, add_key, adjtimex, bind, bpf, brk, capget,
    capset, chdir, chroot, clock_adjtime, clock_getres, clock_gettime,
    clock_nanosleep, clock_settime, close, connect, delete_module, dup, dup3,
    epoll_create1, epoll_ctl, epoll_pwait, eventfd2,
    execve, execveat, exit, exit_group, faccessat, fadvise, fallocate, fanotify_init,
    fanotify_mark, fchdir, fchmod, fchmodat, fchown, fchownat, fcntl, fdatasync,
    fgetxattr, finit_module, flistxattr, flock, fremovexattr, fsetxattr,
    fstatat, fstatfs, fsync, ftruncate, futex, getcpu, getcwd, getdents,
    getegid, geteuid, getgid, getgroups, getitimer, get_mempolicy, getpeername, getpgid,
    getpid, getppid, getpriority, getrandom, getresgid, getresuid, getrlimit,
    get_robust_list, getrusage, getsid, getsockname, getsockopt, gettid, gettimeofday,
    getuid, getxattr, init_module, inotify_add_watch, inotify_init1,
    inotify_rm_watch, io_cancel, ioctl, io_destroy, io_getevents, ioprio_get,
    ioprio_set, io_setup, io_submit, kcmp, kexec_load, keyctl, kill,
    lgetxattr, linkat, listen, listxattr, llistxattr, lookup_dcookie,
    lremovexattr, lseek, lsetxattr, madvise, mbind, memfd_create,
    mincore, mkdirat, mknodat, mlock, mlockall, mmap, mount, move_pages,
    mprotect, mq_getsetattr, mq_open, mq_timedreceive, mq_timedsend, mq_unlink, mremap,
    msgctl, msgget, msgrcv, msgsnd, msync, munlock, munlockall, munmap, name_to_handle_at,
    nanosleep, openat, open_by_handle_at, perf_event_open, personality,
    pipe2, pivot_root, ppoll, prctl, pread, preadv, prlimit, process_vm_readv,
    process_vm_writev, pselect6, ptrace, pwrite, pwritev, quotactl, read, readahead,
    readlinkat, readv, reboot, recvfrom, recvmmsg, recvmsg, remap_file_pages,
    removexattr, renameat, renameat2, request_key, restart_syscall,
    rt_sigaction, rt_sigpending, rt_sigprocmask, rt_sigqueueinfo, rt_sigreturn,
    rt_sigsuspend, rt_sigtimedwait, rt_tgsigqueueinfo, sched_getaffinity, sched_getattr,
    sched_getparam, sched_get_priority_max, sched_get_priority_min, sched_getscheduler,
    sched_rr_get_interval, sched_setaffinity, sched_setattr, sched_setparam,
    sched_setscheduler, sched_yield, seccomp, semget, semop, semtimedop, sendfile,
    sendmmsg, sendmsg, sendto, setdomainname, setfsgid, setfsuid, setgid, setgroups,
    sethostname, setitimer, set_mempolicy, setns, setpgid, setpriority, setregid,
    setresgid, setresuid, setreuid, setrlimit, set_robust_list, setsid, setsockopt,
    set_tid_address, settimeofday, setuid, setxattr, shmat, shmctl, shmdt, shmget,
    shutdown, sigaltstack, signalfd4, socket, socketpair, splice, statfs,
    swapoff, swapon, symlinkat, sync, sync_file_range, syncfs, sysinfo,
    syslog, tee, tgkill, timer_delete, timerfd_create, timerfd_gettime,
    timerfd_settime, timer_getoverrun, timer_gettime, timer_settime, times, tkill,
    truncate, umask, umount, uname, unlinkat, unshare, utimensat,
    vhangup, vmsplice, waitid, write, writev,
};

pub use ::arch::{
    StatType, StatfsType,
};

macro_rules! call {
    ($nr:expr) => {
        ::arch::syscall0($nr as SCT)
    };

    ($nr:expr, $a1:expr) => {
        ::arch::syscall1($nr as SCT, $a1 as SCT)
    };

    ($nr:expr, $a1:expr, $a2:expr) => {
        ::arch::syscall2($nr as SCT, $a1 as SCT, $a2 as SCT)
    };

    ($nr:expr, $a1:expr, $a2:expr, $a3:expr) => {
        ::arch::syscall3($nr as SCT, $a1 as SCT, $a2 as SCT, $a3 as SCT)
    };

    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {
        ::arch::syscall4($nr as SCT, $a1 as SCT, $a2 as SCT, $a3 as SCT, $a4 as SCT)
    };

    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {
        ::arch::syscall5($nr as SCT, $a1 as SCT, $a2 as SCT, $a3 as SCT, $a4 as SCT,
                         $a5 as SCT)
    };

    ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        ::arch::syscall6($nr as SCT, $a1 as SCT, $a2 as SCT, $a3 as SCT, $a4 as SCT,
                         $a5 as SCT, $a6 as SCT)
    };
}

mod common;

#[cfg(target_arch = "x86_64")]
#[path = "x86_64/mod.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;

#[cfg(target_arch = "arm")]
#[path = "arm.rs"]
mod arch;

#[cfg(target_arch = "aarch64")]
#[path = "aarch64.rs"]
mod arch;
