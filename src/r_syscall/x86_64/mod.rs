// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use ::arch::abi::{
    syscall0, syscall1, syscall2, syscall3, syscall4, syscall5, syscall6, SCT,
};

pub use ::common::{
    accept, accept4, access, acct, add_key, adjtimex, alarm, bind, bpf, brk, capget,
    capset, chdir, chmod, chown, chroot, clock_adjtime, clock_getres, clock_gettime,
    clock_nanosleep, clock_settime, close, connect, creat, delete_module, dup, dup2, dup3,
    epoll_create, epoll_create1, epoll_ctl, epoll_pwait, epoll_wait, eventfd, eventfd2,
    execve, execveat, exit, exit_group, faccessat, fallocate, fanotify_init,
    fanotify_mark, fchdir, fchmod, fchmodat, fchown, fchownat, fcntl, fdatasync,
    fgetxattr, finit_module, flistxattr, flock, fork, fremovexattr, fsetxattr, fstatfs,
    fsync, ftruncate, futex, futimesat, getcpu, getcwd, getegid, geteuid, getgid,
    getgroups, getitimer, get_mempolicy, getpeername, getpgid, getpgrp, getpid, getppid,
    getpriority, getrandom, getresgid, getresuid, getrlimit, get_robust_list, getrusage,
    getsid, getsockname, getsockopt, gettid, gettimeofday, getuid, getxattr, init_module,
    inotify_add_watch, inotify_init, inotify_init1, inotify_rm_watch, io_cancel, ioctl,
    io_destroy, io_getevents, ioprio_get, ioprio_set, io_setup, io_submit, kcmp,
    kexec_file_load, kexec_load, keyctl, kill, lchown, lgetxattr, link, linkat, listen,
    listxattr, llistxattr, lookup_dcookie, lremovexattr, lseek, lsetxattr, madvise, mbind,
    memfd_create, migrate_pages, mincore, mkdir, mkdirat, mknod, mknodat, mlock, mlockall,
    mount, move_pages, mprotect, mq_getsetattr, mq_open, mq_timedreceive, mq_timedsend,
    mq_unlink, mremap, msgctl, msgget, msgrcv, msgsnd, msync, munlock, munlockall, munmap,
    name_to_handle_at, nanosleep, open, openat, open_by_handle_at, pause, perf_event_open,
    personality, pipe, pipe2, pivot_root, poll, ppoll, prctl, preadv, process_vm_readv,
    process_vm_writev, pselect6, ptrace, pwritev, quotactl, read, readahead, readlink,
    readlinkat, readv, reboot, recvfrom, recvmmsg, recvmsg, remap_file_pages, removexattr,
    rename, renameat, renameat2, request_key, restart_syscall, rmdir, rt_sigaction,
    rt_sigpending, rt_sigprocmask, rt_sigqueueinfo, rt_sigsuspend, rt_sigreturn,
    rt_sigtimedwait, rt_tgsigqueueinfo, sched_getaffinity, sched_getattr, sched_getparam,
    sched_get_priority_max, sched_get_priority_min, sched_getscheduler,
    sched_rr_get_interval, sched_setaffinity, sched_setattr, sched_setparam,
    sched_setscheduler, sched_yield, seccomp, select, semget, semop, semtimedop, sendmmsg,
    sendmsg, sendto, setdomainname, setfsgid, setfsuid, setgid, setgroups, sethostname,
    setitimer, set_mempolicy, setns, setpgid, setpriority, setregid, setresgid, setresuid,
    setreuid, setrlimit, set_robust_list, setsid, setsockopt, set_tid_address,
    settimeofday, setuid, setxattr, shmat, shmctl, shmdt, shmget, shutdown, sigaltstack,
    signalfd4, socket, socketpair, splice, statfs, swapoff, swapon, symlink,
    symlinkat, sync, sync_file_range, syncfs, sysfs, sysinfo, syslog, tee, tgkill, time,
    timer_delete, timerfd_create, timerfd_gettime, timerfd_settime, timer_getoverrun,
    timer_gettime, timer_settime, times, tkill, truncate, umask, umount, unlink, unlinkat,
    unshare, ustat, utime, utimensat, utimes, vfork, vhangup, vmsplice, waitid,
    write, writev,
};

use cty::{
    self,
    c_uint, k_int, k_long, k_ulong, c_char, k_uint, linux_dirent64, loff_t,
    new_utsname, pid_t, rlimit64, size_t, ssize_t, stat,

    __NR_iopl, __NR_mmap,
};

#[cfg(target_pointer_width = "32")]
#[path = "x32.rs"]
mod abi;

#[cfg(target_pointer_width = "64")]
#[path = "x64.rs"]
mod abi;

mod common {
    use ::arch::abi::{SCT};

    #[inline(always)]
    pub unsafe fn syscall0(n: SCT) -> SCT {
        let mut ret: SCT;
        asm!("syscall" : "={rax}"(ret)
                       : "{rax}"(n)
                       : "rcx", "r11", "memory"
                       : "volatile");
        ret
    }

    #[inline(always)]
    pub unsafe fn syscall1(n: SCT, a1: SCT) -> SCT {
        let mut ret: SCT;
        asm!("syscall" : "={rax}"(ret)
                       : "{rax}"(n), "{rdi}"(a1)
                       : "rcx", "r11", "memory"
                       : "volatile");
        ret
    }

    #[inline(always)]
    pub unsafe fn syscall2(n: SCT, a1: SCT, a2: SCT) -> SCT {
        let mut ret: SCT;
        asm!("syscall" : "={rax}"(ret)
                       : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2)
                       : "rcx", "r11", "memory"
                       : "volatile");
        ret
    }

    #[inline(always)]
    pub unsafe fn syscall3(n: SCT, a1: SCT, a2: SCT, a3: SCT) -> SCT {
        let mut ret: SCT;
        asm!("syscall" : "={rax}"(ret)
                       : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3)
                       : "rcx", "r11", "memory"
                       : "volatile");
        ret
    }

    #[inline(always)]
    pub unsafe fn syscall4(n: SCT, a1: SCT, a2: SCT, a3: SCT, a4: SCT) -> SCT {
        let mut ret: SCT;
        asm!("syscall" : "={rax}"(ret)
                       : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3),
                         "{r10}"(a4)
                       : "rcx", "r11", "memory"
                       : "volatile");
        ret
    }

    #[inline(always)]
    pub unsafe fn syscall5(n: SCT, a1: SCT, a2: SCT, a3: SCT, a4: SCT, a5: SCT) -> SCT {
        let mut ret: SCT;
        asm!("syscall" : "={rax}"(ret)
                       : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3),
                         "{r10}"(a4), "{r8}"(a5)
                       : "rcx", "r11", "memory"
                       : "volatile");
        ret
    }

    #[inline(always)]
    pub unsafe fn syscall6(n: SCT, a1: SCT, a2: SCT, a3: SCT, a4: SCT, a5: SCT,
                           a6: SCT) -> SCT {
        let mut ret: SCT;
        asm!("syscall" : "={rax}"(ret)
                       : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3),
                         "{r10}"(a4), "{r8}"(a5), "{r9}"(a6)
                       : "rcx", "r11", "memory"
                       : "volatile");
        ret
    }
}

// cross platform unification:

pub type StatType = stat;
pub type StatfsType = cty::statfs;

pub unsafe fn stat(filename: *const c_char, statbuf: *mut stat) -> k_int {
    ::common::newstat(filename, statbuf)
}

pub unsafe fn fstat(fd: k_uint, statbuf: *mut stat) -> k_int {
    ::common::newfstat(fd, statbuf)
}

pub unsafe fn lstat(filename: *const c_char, statbuf: *mut stat) -> k_int {
    ::common::newlstat(filename, statbuf)
}

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
    ::common::fadvise64(fd, offset, len as size_t, advice)
}

pub unsafe fn fstatat(dfd: k_int, filename: *const c_char, statbuf: *mut stat,
                      flag: k_int) -> k_int {
    ::common::newfstatat(dfd, filename, statbuf, flag)
}

pub unsafe fn prlimit(pid: pid_t, resource: k_uint, new_rlim: *const rlimit64,
                      old_rlim: *mut rlimit64) -> k_int {
    ::common::prlimit64(pid, resource, new_rlim, old_rlim)
}


// x86_64 specific

pub unsafe fn iopl(level: c_uint) -> k_int {
    call!(__NR_iopl, level) as k_int
}

pub unsafe fn mmap(addr: k_ulong, len: k_ulong, prot: k_ulong, flags: k_ulong,
                   fd: k_ulong, off: k_ulong) -> k_long {
    call!(__NR_mmap, addr, len, prot, flags, fd, off) as k_long
}
