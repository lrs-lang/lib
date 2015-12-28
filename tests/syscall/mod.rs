// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{cty, syscall, mem, process};
use std::string::{CStr};

macro_rules! rv {
    ($e:expr) => {
        match $e {
            n if n < 0 => abort!(),
            n => n,
        }
    }
}

// NOTE: Every test is run in its own process. Hence we don't bother with cleaning up.

#[test]
fn iovec() {
    // Tests that iovec == &[T] so that we don't have to convert between those two in
    // syscalls.

    let slice: &[u8] = &[0u8; 2];
    let iovec = cty::iovec {
        iov_base: &slice[0] as *const _ as *mut _,
        iov_len: 2,
    };
    unsafe {
        let slice_bytes = mem::as_data::<&[u8]>(&slice).as_bytes();
        let iovec_bytes = mem::as_data(&iovec).as_bytes();
        test!(slice_bytes == iovec_bytes);
    }
}

fn generic_file() -> cty::c_int {
    rv!(syscall::openat(cty::AT_FDCWD, "lib.rs\0".try_as_ref().unwrap():&CStr, 0, 0))
}

#[test]
fn openat() {
    let dir = rv!(syscall::openat(cty::AT_FDCWD, "syscall\0".try_as_ref().unwrap():&CStr, 0, 0));
    let file = rv!(syscall::openat(dir, "mod.rs\0".try_as_ref().unwrap():&CStr, 0, 0));
    let fd_flags = rv!(syscall::fcntl_getfd(file));
    if cfg!(not(no_auto_cloexec)) {
        test!(fd_flags & cty::FD_CLOEXEC == cty::FD_CLOEXEC);
    }
    let fs_flags = rv!(syscall::fcntl_getfl(file));

    // XXX: This fails in qemu-arm. fs_flags is null. but we can see in strace that the
    // syscall does return O_LARGEFILE. probably a qemu bug.
    test!(fs_flags & cty::O_LARGEFILE == cty::O_LARGEFILE);
}

#[test]
fn close() {
    let file = generic_file();
    test!(syscall::close(file) >= 0);

    let file2 = generic_file();
    test!(file == file2);
}

#[test]
fn lseek() {
    let file = generic_file();
    test!(syscall::lseek(file, 10, cty::SEEK_SET) == 10);
    test!(syscall::lseek(file, 11, cty::SEEK_SET) == 11);
    test!(syscall::lseek(file, 9, cty::SEEK_CUR) == 20);
    test!(syscall::lseek(file, -20, cty::SEEK_CUR) == 0);
    let len = rv!(syscall::lseek(file, 0, cty::SEEK_END));
    test!(syscall::lseek(file, -10, cty::SEEK_END) == len - 10);
}

#[test]
fn fcntl_dupfd_cloexec() {
    let file = generic_file();
    let file2 = syscall::fcntl_dupfd_cloexec(file, 20);
    test!(file2 == 20);
    test!(syscall::lseek(file2, 10, cty::SEEK_SET) == 10);
    test!(syscall::lseek(file, 0, cty::SEEK_CUR) == 10);
}

#[test]
fn fcntl_getfl_setfl() {
    let file = generic_file();
    let fl1 = rv!(syscall::fcntl_getfl(file));
    test!(fl1 & cty::O_APPEND == 0);
    rv!(syscall::fcntl_setfl(file, fl1 | cty::O_APPEND));
    let fl2 = rv!(syscall::fcntl_getfl(file));
    test!(fl2 & cty::O_APPEND == cty::O_APPEND);
}

#[test]
fn fcntl_getfd_setfd() {
    let file = generic_file();
    let fd1 = rv!(syscall::fcntl_getfd(file));
    if cfg!(not(no_auto_cloexec)) {
        test!(fd1 & cty::FD_CLOEXEC == cty::FD_CLOEXEC);
        rv!(syscall::fcntl_setfd(file, fd1 & !cty::FD_CLOEXEC));
        let fd2 = rv!(syscall::fcntl_getfd(file));
        test!(fd2 & cty::FD_CLOEXEC == 0);
    } else {
        test!(fd1 & cty::FD_CLOEXEC == 0);
        rv!(syscall::fcntl_setfd(file, fd1 | cty::FD_CLOEXEC));
        let fd2 = rv!(syscall::fcntl_getfd(file));
        test!(fd2 & cty::FD_CLOEXEC == cty::FD_CLOEXEC);
    }
}

#[test]
fn ftruncate() {
    // TODO
}

#[test]
fn getpid_getppid() {
    let pid = rv!(syscall::getpid());
    let child = process::fork(|| {
        test!(syscall::getppid() == pid);
    }).unwrap();
    test!(process::wait_id(child, process::WAIT_EXITED).unwrap()
                                        == process::ChildStatus::Exited(0));
}

#[test]
fn setresuid_getresuid() {
    // TODO
}

#[test]
fn setresgid_getresgid() {
    // TODO
}

#[test]
fn fsync() {
    // TODO
}

#[test]
fn fdatasync() {
    // TODO
}

#[test]
fn sync() {
    // TODO
}

#[test]
fn syncfs() {
    // TODO
}

#[test]
fn fadvise() {
    // TODO
}

#[test]
fn fchmod() {
    // TODO
}

#[test]
fn fallocate() {
    // TODO
}

#[test]
fn timerfd_create_settime_gettime() {
    let timer = rv!(syscall::timerfd_create(cty::CLOCK_MONOTONIC, 0));
    let fdflags = rv!(syscall::fcntl_getfd(timer));
    test!(fdflags & cty::FD_CLOEXEC == cty::FD_CLOEXEC);
    let time = cty::timespec { tv_sec: 1, tv_nsec: 0 };
    let time2 = cty::itimerspec { it_interval: time, it_value: time };
    rv!(syscall::timerfd_settime(timer, 0, &time2, None));
    let mut time3 = mem::zeroed();
    rv!(syscall::timerfd_gettime(timer, &mut time3));
    test!(time3.it_interval == time2.it_interval);
    let mut time4 = mem::zeroed();
    rv!(syscall::timerfd_settime(timer, 0, &time2, Some(&mut time4)));
    test!(time4.it_interval == time2.it_interval);
}

#[test]
fn epoll_create() {
    // TODO
}

#[test]
fn flock() {
    // TODO
}

#[test]
fn readahead() {
    // TODO
}

#[test]
fn read() {
    // TODO
}

#[test]
fn write() {
    // TODO
}

#[test]
fn pread() {
    // TODO
}

#[test]
fn pwrite() {
    // TODO
}

#[test]
fn readv() {
    // TODO
}

#[test]
fn writev() {
    // TODO
}

#[test]
fn preadv() {
    // TODO
}

#[test]
fn pwritev() {
    // TODO
}

#[test]
fn getresuid() {
    // TODO
}

#[test]
fn getresgid() {
    // TODO
}

#[test]
fn getgroups() {
    // TODO
}

#[test]
fn setgroups() {
    // TODO
}

#[test]
fn statfs() {
    // TODO
}

#[test]
fn fstatfs() {
    // TODO
}

#[test]
fn prlimit() {
    // TODO
}

#[test]
fn getdents() {
    // TODO
}

#[test]
fn fstatat() {
    // TODO
}

#[test]
fn faccessat() {
    // TODO
}

#[test]
fn truncate() {
    // TODO
}

#[test]
fn linkat() {
    // TODO
}

#[test]
fn utimensat() {
    // TODO
}

#[test]
fn renameat2() {
    // TODO
}

#[test]
fn mkdirat() {
    // TODO
}

#[test]
fn unlinkat() {
    // TODO
}

#[test]
fn symlinkat() {
    // TODO
}

#[test]
fn readlinkat() {
    // TODO
}

#[test]
fn fchownat() {
    // TODO
}

#[test]
fn fchmodat() {
    // TODO
}

#[test]
fn mknodat() {
    // TODO
}

#[test]
fn setxattr() {
    // TODO
}

#[test]
fn lsetxattr() {
    // TODO
}

#[test]
fn fsetxattr() {
    // TODO
}

#[test]
fn getxattr() {
    // TODO
}

#[test]
fn lgetxattr() {
    // TODO
}

#[test]
fn fgetxattr() {
    // TODO
}

#[test]
fn removexattr() {
    // TODO
}

#[test]
fn lremovexattr() {
    // TODO
}

#[test]
fn fremovexattr() {
    // TODO
}

#[test]
fn listxattr() {
    // TODO
}

#[test]
fn llistxattr() {
    // TODO
}

#[test]
fn flistxattr() {
    // TODO
}

#[test]
fn clock_getres() {
    // TODO
}

#[test]
fn clock_gettime() {
    // TODO
}

#[test]
fn clock_settime() {
    // TODO
}

#[test]
fn clock_nanosleep() {
    // TODO
}

#[test]
fn epoll_ctl() {
    // TODO
}

#[test]
fn epoll_pwait() {
    // TODO
}

#[test]
fn sched_getaffinity() {
    // TODO
}

#[test]
fn sched_setaffinity() {
    // TODO
}

#[test]
fn uname() {
    // TODO
}

#[test]
fn sysinfo() {
    // TODO
}

#[test]
fn getrandom() {
    // TODO
}

#[test]
fn acct() {
    // TODO
}

#[test]
fn mount() {
    // TODO
}

#[test]
fn umount() {
    // TODO
}

#[test]
fn sethostname() {
    // TODO
}

#[test]
fn setdomainname() {
    // TODO
}

#[test]
fn socket() {
    // TODO
}

#[test]
fn connect() {
    // TODO
}

#[test]
fn accept4() {
    // TODO
}

#[test]
fn recvfrom() {
    // TODO
}

#[test]
fn recvmsg() {
    // TODO
}

#[test]
fn recvmmsg() {
    // TODO
}

#[test]
fn sendto() {
    // TODO
}

#[test]
fn sendmsg() {
    // TODO
}

#[test]
fn sendmmsg() {
    // TODO
}

#[test]
fn shutdown() {
    // TODO
}

#[test]
fn bind() {
    // TODO
}

#[test]
fn listen() {
    // TODO
}

#[test]
fn getsockname() {
    // TODO
}

#[test]
fn getpeername() {
    // TODO
}

#[test]
fn socketpair() {
    // TODO
}

#[test]
fn setsockopt() {
    // TODO
}

#[test]
fn getsockopt() {
    // TODO
}

#[test]
fn futex_wait() {
    // TODO
}

#[test]
fn futex_wake() {
    // TODO
}

#[test]
fn exit() {
    // TODO
}

#[test]
fn exit_group() {
    // TODO
}

#[test]
fn execveat() {
    // TODO
}

#[test]
fn mmap() {
    // TODO
}

#[test]
fn munmap() {
    // TODO
}

#[test]
fn mremap() {
    // TODO
}

#[test]
fn waitid() {
    // TODO
}

#[test]
fn getcwd() {
    // TODO
}

#[test]
fn chdir() {
    // TODO
}

#[test]
fn ioctl_siocgstampns() {
    // TODO
}

#[test]
fn ioctl_siocinq() {
    // TODO
}

#[test]
fn ioctl_siocoutq() {
    // TODO
}

#[test]
fn rt_sigprocmask() {
    // TODO
}

#[test]
fn rt_sigpending() {
    // TODO
}

#[test]
fn rt_sigsuspend() {
    // TODO
}

#[test]
fn signalfd4() {
    // TODO
}

#[test]
fn rt_sigtimedwait() {
    // TODO
}

#[test]
fn rt_sigaction() {
    // TODO
}

#[test]
fn pipe2() {
    // TODO
}

#[test]
fn fcntl_setpipe_sz() {
    // TODO
}

#[test]
fn fcntl_getpipe_sz() {
    // TODO
}

#[test]
fn ioctl_fionread() {
    // TODO
}

#[test]
fn tee() {
    // TODO
}

#[test]
fn splice() {
    // TODO
}

#[test]
fn inotify_init1() {
    // TODO
}

#[test]
fn inotify_add_watch() {
    // TODO
}

#[test]
fn inotify_rm_watch() {
    // TODO
}

#[test]
fn dup3() {
    // TODO
}

#[test]
fn umask() {
    // TODO
}

#[test]
fn eventfd2() {
    // TODO
}

#[test]
fn times() {
    // TODO
}

#[test]
fn reboot() {
    // TODO
}

#[test]
fn memfd_create() {
    // TODO
}

#[test]
fn fcntl_add_seals() {
    // TODO
}

#[test]
fn fcntl_get_seals() {
    // TODO
}

#[test]
fn msync() {
    // TODO
}

#[test]
fn madvise() {
    // TODO
}

#[test]
fn mprotect() {
    // TODO
}

#[test]
fn mlock() {
    // TODO
}

#[test]
fn munlock() {
    // TODO
}

#[test]
fn mlockall() {
    // TODO
}

#[test]
fn munlockall() {
    // TODO
}

#[test]
fn mincore() {
    // TODO
}

#[test]
fn setsid() {
    // TODO
}

#[test]
fn getsid() {
    // TODO
}

#[test]
fn fchdir() {
    // TODO
}

#[test]
fn setpgid() {
    // TODO
}

#[test]
fn getpgid() {
    // TODO
}

#[test]
fn kill() {
    // TODO
}

#[test]
fn tgkill() {
    // TODO
}

#[test]
fn gettid() {
    // TODO
}

#[test]
fn getrusage() {
    // TODO
}

#[test]
fn ioctl_tiocgptn() {
    // TODO
}

#[test]
fn ioctl_tiocsptlck() {
    // TODO
}

#[test]
fn ioctl_tiocgptlck() {
    // TODO
}

#[test]
fn ioctl_tiocsig() {
    // TODO
}

#[test]
fn ioctl_tiocpkt() {
    // TODO
}

#[test]
fn ioctl_tiocgpkt() {
    // TODO
}

#[test]
fn ioctl_tiocsti() {
    // TODO
}

#[test]
fn ioctl_tiocgwinsz() {
    // TODO
}

#[test]
fn ioctl_tiocswinsz() {
    // TODO
}

#[test]
fn ioctl_tioccons() {
    // TODO
}

#[test]
fn ioctl_tiocexcl() {
    // TODO
}

#[test]
fn ioctl_tiocnxcl() {
    // TODO
}

#[test]
fn ioctl_tiocgexcl() {
    // TODO
}

#[test]
fn ioctl_tiocnotty() {
    // TODO
}

#[test]
fn ioctl_tiocsctty() {
    // TODO
}

#[test]
fn ioctl_tiocgpgrp() {
    // TODO
}

#[test]
fn ioctl_tiocspgrp() {
    // TODO
}

#[test]
fn ioctl_tiocgsid() {
    // TODO
}

#[test]
fn ioctl_tiocgetd() {
    // TODO
}

#[test]
fn ioctl_tiocsetd() {
    // TODO
}

#[test]
fn ioctl_tiocvhangup() {
    // TODO
}

#[test]
fn ioctl_tiocgdev() {
    // TODO
}

#[test]
fn ioctl_tcflsh() {
    // TODO
}

#[test]
fn ioctl_tiocoutq() {
    // TODO
}

#[test]
fn ioctl_tcxonc() {
    // TODO
}

#[test]
fn ioctl_tcgets2() {
    // TODO
}

#[test]
fn ioctl_tcsets2() {
    // TODO
}

#[test]
fn vhangup() {
    // TODO
}

#[test]
fn mq_open() {
    // TODO
}

#[test]
fn mq_unlink() {
    // TODO
}

#[test]
fn mq_timedsend() {
    // TODO
}

#[test]
fn mq_timedreceive() {
    // TODO
}

#[test]
fn mq_getsetattr() {
    // TODO
}

#[test]
fn sched_setattr() {
    // TODO
}

#[test]
fn sched_getattr() {
    // TODO
}

#[test]
fn sched_yield() {
    // TODO
}

#[test]
fn sched_get_priority_max() {
    // TODO
}

#[test]
fn sched_get_priority_min() {
    // TODO
}

#[test]
fn sched_rr_get_interval() {
    // TODO
}

#[test]
fn getpriority() {
    // TODO
}

#[test]
fn setpriority() {
    // TODO
}

#[test]
fn capget_v3() {
    // TODO
}

#[test]
fn capset_v3() {
    // TODO
}

#[test]
fn prctl_pr_capbset_read() {
    // TODO
}

#[test]
fn prctl_pr_capbset_drop() {
    // TODO
}

#[test]
fn prctl_pr_get_keepcaps() {
    // TODO
}

#[test]
fn prctl_pr_set_keepcaps() {
    // TODO
}

#[test]
fn unshare() {
    // TODO
}

#[test]
fn getcpu() {
    // TODO
}

#[test]
fn setns() {
    // TODO
}

#[test]
fn seccomp_seccomp_set_mode_strict() {
    // TODO
}

#[test]
fn swapon() {
    // TODO
}

#[test]
fn swapoff() {
    // TODO
}

#[test]
fn chroot() {
    // TODO
}

#[test]
fn pivot_root() {
    // TODO
}
