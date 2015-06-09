// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Syscall wrappers.

pub use lrs_syscall::{
    openat, close, lseek, fcntl_dupfd_cloexec, fcntl_getfl, fcntl_setfl, fcntl_getfd,
    fcntl_setfd, ftruncate, getpid, getppid, setresuid, setresgid, fsync, fdatasync, sync,
    syncfs, fadvise, fchmod, fallocate, timerfd_create, epoll_create, flock, readahead,
    read, write, pread, pwrite, readv, writev, preadv, pwritev, getresuid, getresgid,
    getgroups, setgroups, statfs, fstatfs, prlimit, getdents, fstatat, faccessat,
    truncate, linkat, utimensat, renameat2, mkdirat, unlinkat, symlinkat, readlinkat,
    fchownat, fchmodat, mknodat, setxattr, lsetxattr, fsetxattr, getxattr, lgetxattr,
    fgetxattr, removexattr, lremovexattr, fremovexattr, listxattr, llistxattr, flistxattr,
    clock_getres, clock_gettime, clock_settime, clock_nanosleep, timerfd_settime,
    timerfd_gettime, epoll_ctl, epoll_pwait, sched_getaffinity, uname, sysinfo, getrandom,
    acct, mount, umount, sethostname, setdomainname, socket, connect, accept4, recvfrom,
    recvmsg, recvmmsg, sendto, sendmsg, sendmmsg, shutdown, bind, listen, getsockname,
    getpeername, socketpair, setsockopt, getsockopt, futex_wait, futex_wake, exit,
    exit_group, execveat, mmap, munmap, mremap, waitid, getcwd, chdir, ioctl_siocgstampns,
    ioctl_siocinq, ioctl_siocoutq, rt_sigprocmask, rt_sigpending, rt_sigsuspend,
    signalfd4, rt_sigtimedwait, rt_sigaction, pipe2, fcntl_setpipe_sz, fcntl_getpipe_sz,
    ioctl_fionread, tee, splice, inotify_init1, inotify_add_watch, inotify_rm_watch, dup3,
    umask, eventfd2, times, pause, reboot, memfd_create, fcntl_add_seals, fcntl_get_seals,
    madvise, mprotect, mlock, munlock, mlockall, munlockall, mincore, setsid, getsid,
    fchdir, setpgid, getpgid, kill, tgkill, gettid, getrusage, ioctl_tiocgptn,
    ioctl_tiocsptlck, ioctl_tiocgptlck, ioctl_tiocsig, ioctl_tiocpkt, ioctl_tiocgpkt,
    ioctl_tiocsti, ioctl_tiocgwinsz, ioctl_tiocswinsz, ioctl_tioccons, ioctl_tiocexcl,
    ioctl_tiocnxcl, ioctl_tiocgexcl, ioctl_tiocnotty, ioctl_tiocsctty, ioctl_tiocgpgrp,
    ioctl_tiocspgrp, ioctl_tiocgsid, ioctl_tiocgetd, ioctl_tiocsetd, ioctl_tiocvhangup,
    ioctl_tiocgdev, ioctl_tcflsh, ioctl_tiocoutq, ioctl_tcxonc, ioctl_tcgets2,
    ioctl_tcsets2, vhangup, mq_open, mq_unlink, mq_timedsend, mq_timedreceive,
    mq_getsetattr, sched_setattr, sched_getattr, sched_yield, sched_get_priority_max,
    sched_get_priority_min, sched_rr_get_interval, getpriority, setpriority,
};
