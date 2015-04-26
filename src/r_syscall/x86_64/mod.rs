// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use ::arch::abi::{
    syscall0, syscall1, syscall2, syscall3, syscall4, syscall5, syscall6, SCT,
};

pub use ::common::{
    read, write, open, close, poll, lseek, mprotect, munmap, brk, rt_sigprocmask, access,
    pipe, select, sched_yield, mremap, msync, mincore, madvise, shmget, shmat, shmctl,
    dup, dup2, pause, nanosleep, getitimer, alarm, setitimer, getpid, socket, connect,
    accept, sendto, shutdown, bind, listen, getsockname, getpeername, socketpair,
    fork, vfork, exit, wait4, kill, semget, semop, shmdt, msgget, msgsnd, msgrcv,
    msgctl, fcntl, flock, fsync, fdatasync, truncate, ftruncate, getcwd, chdir, fchdir,
    rename, mkdir, rmdir, creat, link, unlink, symlink, readlink, chmod, fchmod, chown,
    fchown, lchown, umask, gettimeofday, getrlimit, getrusage, sysinfo, times, getuid,
    syslog, getgid, setuid, setgid, geteuid, getegid, setpgid, getppid, getpgrp, setsid,
    setreuid, setregid, getgroups, setgroups, setresuid, getresuid, setresgid, getresgid,
    getpgid, setfsuid, setfsgid, getsid, capget, capset, rt_sigsuspend, utime, mknod,
    personality, ustat, statfs, fstatfs, sysfs, getpriority, setpriority, sched_setparam,
    sched_getparam, sched_setscheduler, sched_getscheduler, sched_get_priority_max,
    sched_get_priority_min, sched_rr_get_interval, mlock, munlock, mlockall, munlockall,
    vhangup, pivot_root, prctl, adjtimex, setrlimit, chroot, sync,
    acct, settimeofday, mount, umount, swapon, swapoff, reboot, sethostname,
    setdomainname, init_module, delete_module, quotactl,
    gettid, readahead, setxattr, lsetxattr, fsetxattr, waitid,
    getxattr, lgetxattr, fgetxattr, listxattr, llistxattr, flistxattr, removexattr,
    lremovexattr, fremovexattr, tkill, time, futex, sched_setaffinity, sched_getaffinity,
    io_destroy, io_getevents, io_cancel, lookup_dcookie, epoll_create, remap_file_pages,
    set_tid_address, restart_syscall, semtimedop, timer_settime, timer_gettime,
    timer_getoverrun, timer_delete, clock_settime, clock_gettime, clock_getres,
    clock_nanosleep, exit_group, epoll_wait, epoll_ctl, tgkill, utimes, mbind,
    set_mempolicy, get_mempolicy, mq_open, mq_unlink, mq_timedsend, mq_timedreceive,
    mq_getsetattr, add_key, request_key, keyctl, ioprio_set, ioprio_get, inotify_init,
    inotify_add_watch, inotify_rm_watch, migrate_pages, openat, mkdirat, mknodat,
    fchownat, futimesat, unlinkat, linkat, symlinkat, readlinkat, fchmodat, faccessat,
    pselect6, ppoll, unshare, splice, tee, sync_file_range, utimensat, epoll_pwait,
    signalfd, timerfd_create, eventfd, fallocate, timerfd_settime, timerfd_gettime,
    accept4, signalfd4, eventfd2, epoll_create1, dup3, pipe2, inotify_init1,
    perf_event_open, fanotify_init, fanotify_mark, name_to_handle_at, open_by_handle_at,
    clock_adjtime, syncfs, setns, getcpu, kcmp, finit_module, sched_setattr,
    sched_getattr, seccomp, getrandom, memfd_create, kexec_file_load, bpf, rt_sigaction,
    ioctl, readv, writev, recvfrom, sendmsg, recvmsg, execve, ptrace,
    rt_sigpending, sigaltstack,
    kexec_load, set_robust_list, get_robust_list, vmsplice, move_pages, preadv,
    pwritev, recvmmsg, sendmmsg, process_vm_readv, process_vm_writev,
    setsockopt, getsockopt, io_setup, io_submit, execveat,
};

use cty::{
    c_uint, k_int, k_long, k_ulong, user_desc, c_char, k_uint, linux_dirent64, loff_t,
    new_utsname, pid_t, rlimit64, size_t, ssize_t, stat,
};

use cty::{
    __NR_iopl, __NR_set_thread_area, __NR_get_thread_area, __NR_mmap,
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
        let mut ret : SCT;
        asm!("syscall" : "={rax}"(ret)
                       : "{rax}"(n)
                       : "rcx", "r11", "memory"
                       : "volatile");
        ret
    }

    #[inline(always)]
    pub unsafe fn syscall1(n: SCT, a1: SCT) -> SCT {
        let mut ret : SCT;
        asm!("syscall" : "={rax}"(ret)
                       : "{rax}"(n), "{rdi}"(a1)
                       : "rcx", "r11", "memory"
                       : "volatile");
        ret
    }

    #[inline(always)]
    pub unsafe fn syscall2(n: SCT, a1: SCT, a2: SCT) -> SCT {
        let mut ret : SCT;
        asm!("syscall" : "={rax}"(ret)
                       : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2)
                       : "rcx", "r11", "memory"
                       : "volatile");
        ret
    }

    #[inline(always)]
    pub unsafe fn syscall3(n: SCT, a1: SCT, a2: SCT, a3: SCT) -> SCT {
        let mut ret : SCT;
        asm!("syscall" : "={rax}"(ret)
                       : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3)
                       : "rcx", "r11", "memory"
                       : "volatile");
        ret
    }

    #[inline(always)]
    pub unsafe fn syscall4(n: SCT, a1: SCT, a2: SCT, a3: SCT, a4: SCT) -> SCT {
        let mut ret : SCT;
        asm!("syscall" : "={rax}"(ret)
                       : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3),
                         "{r10}"(a4)
                       : "rcx", "r11", "memory"
                       : "volatile");
        ret
    }

    #[inline(always)]
    pub unsafe fn syscall5(n: SCT, a1: SCT, a2: SCT, a3: SCT, a4: SCT, a5: SCT) -> SCT {
        let mut ret : SCT;
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
        let mut ret : SCT;
        asm!("syscall" : "={rax}"(ret)
                       : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3),
                         "{r10}"(a4), "{r8}"(a5), "{r9}"(a6)
                       : "rcx", "r11", "memory"
                       : "volatile");
        ret
    }
}


// cross platform unification:

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

pub unsafe fn fadvise(fd: k_int, offset: loff_t, len: size_t, advice: k_int) -> k_int {
    ::common::fadvise64(fd, offset, len, advice)
}

pub unsafe fn fstatat(dfd: k_int, filename: *const c_char, statbuf: *mut stat,
                      flag: k_int) -> k_int {
    ::common::newfstatat(dfd, filename, statbuf, flag)
}

pub unsafe fn prlimit(pid: pid_t, resource: k_uint, new_rlim: *const rlimit64,
                      old_rlim: *mut rlimit64) -> k_int {
    ::common::prlimit64(pid, resource, new_rlim, old_rlim)
}

pub unsafe fn renameat(olddfd: k_int, oldname: *const c_char, newdfd: k_int,
                       newname: *const c_char, flags: k_uint) -> k_int {
    ::common::renameat2(olddfd, oldname, newdfd, newname, flags)
}



// x86_64 specific

pub unsafe fn iopl(level: c_uint) -> k_int {
    call!(__NR_iopl, level) as k_int
}

pub unsafe fn set_thread_area(u_info: *mut user_desc) -> k_int {
    call!(__NR_set_thread_area, u_info) as k_int
}

pub unsafe fn get_thread_area(u_info: *mut user_desc) -> k_int {
    call!(__NR_get_thread_area, u_info) as k_int
}

pub unsafe fn mmap(addr: k_ulong, len: k_ulong, prot: k_ulong, flags: k_ulong,
                   fd: k_ulong, off: k_ulong) -> k_long {
    call!(__NR_mmap, addr, len, prot, flags, fd, off) as k_long
}
