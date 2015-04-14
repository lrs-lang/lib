// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_syscall"]
#![crate_type = "lib"]
#![feature(asm, plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]
#![allow(dead_code)]

extern crate linux_core as core;
extern crate linux_cty as cty;

pub use ::arch::{
    read, write, open, close, stat, fstat, lstat, poll, lseek, mmap, mprotect, munmap,
    brk, rt_sigprocmask, pread, pwrite, access, pipe, select, sched_yield, mremap, msync,
    mincore, madvise, shmget, shmat, shmctl, dup, dup2, pause, nanosleep, getitimer,
    alarm, setitimer, getpid, sendfile, socket, connect, accept, sendto, shutdown, bind,
    listen, getsockname, getpeername, socketpair, fork, vfork, exit, wait4, kill,
    uname, semget, semop, shmdt, msgget, msgsnd, msgrcv, msgctl, fcntl, flock,
    fsync, fdatasync, truncate, ftruncate, getdents, getcwd, chdir, fchdir, rename, mkdir,
    rmdir, creat, link, unlink, symlink, readlink, chmod, fchmod, chown, fchown, lchown,
    umask, gettimeofday, getrlimit, getrusage, sysinfo, times, getuid, syslog, getgid,
    setuid, setgid, geteuid, getegid, setpgid, getppid, getpgrp, setsid, setreuid,
    setregid, getgroups, setgroups, setresuid, getresuid, setresgid, getresgid, getpgid,
    setfsuid, setfsgid, getsid, capget, capset, rt_sigsuspend, utime, mknod, personality,
    ustat, statfs, fstatfs, sysfs, getpriority, setpriority, sched_setparam,
    sched_getparam, sched_setscheduler, sched_getscheduler, sched_get_priority_max,
    sched_get_priority_min, sched_rr_get_interval, mlock, munlock, mlockall, munlockall,
    vhangup, pivot_root, prctl, adjtimex, setrlimit, chroot, sync,
    acct, settimeofday, mount, umount, swapon, swapoff, reboot, sethostname,
    setdomainname, iopl, init_module, delete_module, quotactl, 
    gettid, readahead, setxattr, lsetxattr, fsetxattr,
    getxattr, lgetxattr, fgetxattr, listxattr, llistxattr, flistxattr, removexattr,
    lremovexattr, fremovexattr, tkill, time, futex, sched_setaffinity, sched_getaffinity,
    io_destroy, io_getevents, io_cancel, lookup_dcookie, epoll_create, remap_file_pages,
    set_tid_address, restart_syscall, semtimedop, fadvise, timer_settime,
    timer_gettime, timer_getoverrun, timer_delete, clock_settime, clock_gettime,
    clock_getres, clock_nanosleep, exit_group, epoll_wait, epoll_ctl, tgkill, utimes,
    mbind, set_mempolicy, get_mempolicy, mq_open, mq_unlink, mq_timedsend,
    mq_timedreceive, mq_getsetattr, add_key, request_key, keyctl, ioprio_set, ioprio_get,
    inotify_init, inotify_add_watch, inotify_rm_watch, migrate_pages, openat, mkdirat,
    mknodat, fchownat, futimesat, fstatat, unlinkat, renameat, linkat, symlinkat,
    readlinkat, fchmodat, faccessat, pselect6, ppoll, unshare, splice, tee,
    sync_file_range, utimensat, epoll_pwait, signalfd, timerfd_create, eventfd, fallocate,
    timerfd_settime, timerfd_gettime, accept4, signalfd4, eventfd2, epoll_create1, dup3,
    pipe2, inotify_init1, perf_event_open, fanotify_init, fanotify_mark, prlimit,
    name_to_handle_at, open_by_handle_at, clock_adjtime, syncfs, setns, getcpu, kcmp,
    finit_module, sched_setattr, sched_getattr, seccomp, getrandom, memfd_create,
    kexec_file_load, bpf, rt_sigaction, ioctl, readv, writev, recvfrom,
    sendmsg, recvmsg, execve, ptrace,
    sigaltstack, kexec_load, set_robust_list,
    get_robust_list, vmsplice, move_pages, preadv, pwritev, recvmmsg,
    sendmmsg, process_vm_readv, process_vm_writev, setsockopt, getsockopt, io_setup,
    io_submit, execveat,
};

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

mod common;

#[cfg(target_arch = "x86_64")]
#[path = "x86_64/mod.rs"]
mod arch;
