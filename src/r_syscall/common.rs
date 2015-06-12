// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// This file contains the kernel syscalls as they are defined in the kernel source with
// some minor exceptions which are documented below.

use cty::{
    self,
    bpf_attr, cap_user_data_t, cap_user_header_t, clockid_t, c_void, epoll_event, gid_t,
    key_serial_t, k_int, k_uint, k_ulong, loff_t, sigset_t, size_t, timespec, timex,
    uid_t, umode_t, c_char, aio_context_t, clock_t, fd_set, file_handle, getcpu_cache,
    iocb, io_event, iovec, itimerspec, itimerval, kexec_segment, k_long, linux_dirent,
    linux_dirent64, mq_attr, mqd_t, new_utsname, off_t,
    perf_event_attr, pid_t, pollfd, qid_t, rlimit, rlimit64,
    robust_list_head, rusage, __s32, sched_attr, sched_param, sigaction, siginfo_t,
    ssize_t, stack_t, statfs, sysinfo, timer_t, time_t,
    timeval, timezone, tms, ustat, utimbuf, k_uchar,
};

use super::arch::{SCT};

pub unsafe fn access(filename: *const c_char, mode: k_int) -> k_int {
    call!(cty::__NR_access, filename, mode) as k_int
}

pub unsafe fn acct(name: *const c_char) -> k_int {
    call!(cty::__NR_acct, name) as k_int
}

pub unsafe fn add_key(_type: *const c_char, _description: *const c_char,
                      _payload: *const c_void, plen: size_t,
                      ringid: key_serial_t) -> key_serial_t {
    call!(cty::__NR_add_key, _type, _description, _payload, plen, ringid) as key_serial_t
}

pub unsafe fn adjtimex(txc_p: *mut timex) -> k_int {
    call!(cty::__NR_adjtimex, txc_p) as k_int
}

pub unsafe fn alarm(seconds: k_uint) -> k_uint {
    call!(cty::__NR_alarm, seconds) as k_uint
}

pub unsafe fn bpf(cmd: k_int, uattr: *mut bpf_attr, size: k_uint) -> k_int {
    call!(cty::__NR_bpf, cmd, uattr, size) as k_int
}

pub unsafe fn brk(brk: k_ulong) -> k_int {
    call!(cty::__NR_brk, brk) as k_int
}

pub unsafe fn capget(header: cap_user_header_t, dataptr: cap_user_data_t) -> k_int {
    call!(cty::__NR_capget, header, dataptr) as k_int
}

pub unsafe fn capset(header: cap_user_header_t, data: cap_user_data_t) -> k_int {
    call!(cty::__NR_capset, header, data) as k_int
}

pub unsafe fn chdir(filename: *const c_char) -> k_int {
    call!(cty::__NR_chdir, filename) as k_int
}

pub unsafe fn chmod(filename: *const c_char, mode: umode_t) -> k_int {
    call!(cty::__NR_chmod, filename, mode) as k_int
}

pub unsafe fn chown(filename: *const c_char, user: uid_t, group: gid_t) -> k_int {
    call!(cty::__NR_chown, filename, user, group) as k_int
}

pub unsafe fn chroot(filename: *const c_char) -> k_int {
    call!(cty::__NR_chroot, filename) as k_int
}

pub unsafe fn clock_adjtime(which_clock: clockid_t, utx: *mut timex) -> k_int {
    call!(cty::__NR_clock_adjtime, which_clock, utx) as k_int
}

pub unsafe fn clock_getres(which_clock: clockid_t, tp: *mut timespec) -> k_int {
    call!(cty::__NR_clock_getres, which_clock, tp) as k_int
}

pub unsafe fn clock_gettime(which_clock: clockid_t, tp: *mut timespec) -> k_int {
    call!(cty::__NR_clock_gettime, which_clock, tp) as k_int
}

pub unsafe fn clock_nanosleep(which_clock: clockid_t, flags: k_int, rqtp: *const timespec,
                              rmtp: *mut timespec) -> k_int {
    call!(cty::__NR_clock_nanosleep, which_clock, flags, rqtp, rmtp) as k_int
}

pub unsafe fn clock_settime(which_clock: clockid_t, tp: *const timespec) -> k_int {
    call!(cty::__NR_clock_settime, which_clock, tp) as k_int
}

pub unsafe fn close(fd: k_uint) -> k_int {
    call!(cty::__NR_close, fd) as k_int
}

pub unsafe fn creat(pathname: *const c_char, mode: umode_t) -> k_int {
    call!(cty::__NR_creat, pathname, mode) as k_int
}

pub unsafe fn delete_module(name_user: *const c_char, flags: k_uint) -> k_int {
    call!(cty::__NR_delete_module, name_user, flags) as k_int
}

pub unsafe fn dup2(oldfd: k_uint, newfd: k_uint) -> k_int {
    call!(cty::__NR_dup2, oldfd, newfd) as k_int
}

pub unsafe fn dup3(oldfd: k_uint, newfd: k_uint, flags: k_int) -> k_int {
    call!(cty::__NR_dup3, oldfd, newfd, flags) as k_int
}

pub unsafe fn dup(fildes: k_uint) -> k_int {
    call!(cty::__NR_dup, fildes) as k_int
}

pub unsafe fn epoll_create1(flags: k_int) -> k_int {
    call!(cty::__NR_epoll_create1, flags) as k_int
}

pub unsafe fn epoll_create(size: k_int) -> k_int {
    call!(cty::__NR_epoll_create, size) as k_int
}

pub unsafe fn epoll_ctl(epfd: k_int, op: k_int, fd: k_int,
                        event: *mut epoll_event) -> k_int {
    call!(cty::__NR_epoll_ctl, epfd, op, fd, event) as k_int
}

pub unsafe fn epoll_pwait(epfd: k_int, events: *mut epoll_event, maxevents: k_int,
                          timeout: k_int, sigmask: *const sigset_t,
                          sigsetsize: size_t) -> k_int {
    call!(cty::__NR_epoll_pwait, epfd, events, maxevents, timeout, sigmask,
          sigsetsize) as k_int
}

pub unsafe fn epoll_wait(epfd: k_int, events: *mut epoll_event, maxevents: k_int,
                         timeout: k_int) -> k_int {
    call!(cty::__NR_epoll_wait, epfd, events, maxevents, timeout) as k_int
}

pub unsafe fn eventfd2(count: k_uint, flags: k_int) -> k_int {
    call!(cty::__NR_eventfd2, count, flags) as k_int
}

pub unsafe fn eventfd(count: k_uint) -> k_int {
    call!(cty::__NR_eventfd, count) as k_int
}

pub unsafe fn execveat(fd: k_int, filename: *const c_char, argv: *const *const c_char,
                       envp: *const *const c_char, flags: k_int) -> k_int {
    call!(cty::__NR_execveat, fd, filename, argv, envp, flags) as k_int
}

pub unsafe fn execve(filename: *const c_char, argv: *const *const c_char,
                     envp: *const *mut c_char) -> k_int {
    call!(cty::__NR_execve, filename, argv, envp) as k_int
}

pub unsafe fn exit(error_code: k_int) {
    call!(cty::__NR_exit, error_code);
}

pub unsafe fn exit_group(error_code: k_int) {
    call!(cty::__NR_exit_group, error_code);
}

pub unsafe fn faccessat(dfd: k_int, filename: *const c_char, mode: k_int) -> k_int {
    call!(cty::__NR_faccessat, dfd, filename, mode) as k_int
}

pub unsafe fn fanotify_init(flags: k_uint, event_f_flags: k_uint) -> k_int {
    call!(cty::__NR_fanotify_init, flags, event_f_flags) as k_int
}

pub unsafe fn fchdir(fd: k_uint) -> k_int {
    call!(cty::__NR_fchdir, fd) as k_int
}

pub unsafe fn fchmodat(dfd: k_int, filename: *const c_char, mode: umode_t) -> k_int {
    call!(cty::__NR_fchmodat, dfd, filename, mode) as k_int
}

pub unsafe fn fchmod(fd: k_uint, mode: umode_t) -> k_int {
    call!(cty::__NR_fchmod, fd, mode) as k_int
}

pub unsafe fn fchownat(dfd: k_int, filename: *const c_char, user: uid_t, group: gid_t,
                       flag: k_int) -> k_int {
    call!(cty::__NR_fchownat, dfd, filename, user, group, flag) as k_int
}

pub unsafe fn fchown(fd: k_uint, user: uid_t, group: gid_t) -> k_int {
    call!(cty::__NR_fchown, fd, user, group) as k_int
}

pub unsafe fn fcntl(fd: k_uint, cmd: k_uint, arg: k_ulong) -> k_int {
    call!(cty::__NR_fcntl, fd, cmd, arg) as k_int
}

pub unsafe fn fdatasync(fd: k_uint) -> k_int {
    call!(cty::__NR_fdatasync, fd) as k_int
}

pub unsafe fn fgetxattr(fd: k_int, name: *const c_char, value: *mut c_void,
                        size: size_t) -> ssize_t {
    call!(cty::__NR_fgetxattr, fd, name, value, size) as ssize_t
}

pub unsafe fn finit_module(fd: k_int, uargs: *const c_char, flags: k_int) -> k_int {
    call!(cty::__NR_finit_module, fd, uargs, flags) as k_int
}

pub unsafe fn flistxattr(fd: k_int, list: *mut c_char, size: size_t) -> ssize_t {
    call!(cty::__NR_flistxattr, fd, list, size) as ssize_t
}

pub unsafe fn flock(fd: k_uint, cmd: k_uint) -> k_int {
    call!(cty::__NR_flock, fd, cmd) as k_int
}

pub unsafe fn fork() -> pid_t {
    call!(cty::__NR_fork) as pid_t 
}

pub unsafe fn fremovexattr(fd: k_int, name: *const c_char) -> k_int {
    call!(cty::__NR_fremovexattr, fd, name) as k_int
}

pub unsafe fn fsetxattr(fd: k_int, name: *const c_char, value: *const c_void,
                        size: size_t, flags: k_int) -> k_int {
    call!(cty::__NR_fsetxattr, fd, name, value, size, flags) as k_int
}

pub unsafe fn fstatfs(fd: k_uint, buf: *mut statfs) -> k_int {
    call!(cty::__NR_fstatfs, fd, buf) as k_int
}

pub unsafe fn fsync(fd: k_uint) -> k_int {
    call!(cty::__NR_fsync, fd) as k_int
}

// length is k_ulong in the kernel but loff_t here for easier cross platform code
pub unsafe fn ftruncate(fd: k_uint, length: loff_t) -> k_int {
    call!(cty::__NR_ftruncate, fd, length) as k_int
}

pub unsafe fn futex(uaddr: *mut u32, op: k_int, val: u32, utime: *mut timespec,
                    uaddr2: *mut u32, val3: u32) -> k_int {
    call!(cty::__NR_futex, uaddr, op, val, utime, uaddr2, val3) as k_int
}

pub unsafe fn futimesat(dfd: k_int, filename: *const c_char,
                        utimes: *mut timeval) -> k_int {
    call!(cty::__NR_futimesat, dfd, filename, utimes) as k_int
}

pub unsafe fn getcpu(cpup: *mut k_uint, nodep: *mut k_uint,
                     unused: *mut getcpu_cache) -> k_int {
    call!(cty::__NR_getcpu, cpup, nodep, unused) as k_int
}

pub unsafe fn getcwd(buf: *mut c_char, size: k_ulong) -> k_int {
    call!(cty::__NR_getcwd, buf, size) as k_int
}

pub unsafe fn getdents64(fd: k_uint, dirent: *mut linux_dirent64,
                         count: k_uint) -> k_int {
    call!(cty::__NR_getdents64, fd, dirent, count) as k_int
}

pub unsafe fn getdents(fd: k_uint, dirent: *mut linux_dirent, count: k_uint) -> k_int {
    call!(cty::__NR_getdents, fd, dirent, count) as k_int
}

pub unsafe fn getegid() -> gid_t {
    call!(cty::__NR_getegid) as gid_t
}

pub unsafe fn geteuid() -> uid_t {
    call!(cty::__NR_geteuid) as uid_t 
}

pub unsafe fn getgid() -> gid_t {
    call!(cty::__NR_getgid) as gid_t 
}

pub unsafe fn getgroups(gidsetsize: k_int, grouplist: *mut gid_t) -> k_int {
    call!(cty::__NR_getgroups, gidsetsize, grouplist) as k_int
}

pub unsafe fn getitimer(which: k_int, value: *mut itimerval) -> k_int {
    call!(cty::__NR_getitimer, which, value) as k_int
}

pub unsafe fn get_mempolicy(policy: *mut k_int, nmask: *mut k_ulong, maxnode: k_ulong,
                            addr: k_ulong, flags: k_ulong) -> k_int {
    call!(cty::__NR_get_mempolicy, policy, nmask, maxnode, addr, flags) as k_int
}

pub unsafe fn getpgid(pid: pid_t) -> pid_t {
    call!(cty::__NR_getpgid, pid) as pid_t
}

pub unsafe fn getpgrp() -> pid_t {
    call!(cty::__NR_getpgrp) as pid_t 
}

pub unsafe fn getpid() -> pid_t {
    call!(cty::__NR_getpid) as pid_t 
}

pub unsafe fn getppid() -> pid_t {
    call!(cty::__NR_getppid) as pid_t 
}

pub unsafe fn getpriority(which: k_int, who: k_int) -> k_int {
    call!(cty::__NR_getpriority, which, who) as k_int
}

pub unsafe fn getrandom(buf: *mut c_char, count: size_t, flags: k_uint) -> k_int {
    call!(cty::__NR_getrandom, buf, count, flags) as k_int
}

pub unsafe fn getresgid(rgidp: *mut gid_t, egidp: *mut gid_t,
                        sgidp: *mut gid_t) -> k_int {
    call!(cty::__NR_getresgid, rgidp, egidp, sgidp) as k_int
}

pub unsafe fn getresuid(ruidp: *mut uid_t, euidp: *mut uid_t,
                        suidp: *mut uid_t) -> k_int {
    call!(cty::__NR_getresuid, ruidp, euidp, suidp) as k_int
}

pub unsafe fn getrlimit(resource: k_uint, rlim: *mut rlimit) -> k_int {
    call!(cty::__NR_getrlimit, resource, rlim) as k_int
}

pub unsafe fn get_robust_list(pid: k_int, head_ptr: *mut *mut robust_list_head,
                              len_ptr: *mut size_t) -> k_long {
    call!(cty::__NR_get_robust_list, pid, head_ptr, len_ptr) as k_long
}

pub unsafe fn getrusage(who: k_int, ru: *mut rusage) -> k_int {
    call!(cty::__NR_getrusage, who, ru) as k_int
}

pub unsafe fn getsid(pid: pid_t) -> pid_t {
    call!(cty::__NR_getsid, pid) as pid_t
}

pub unsafe fn gettid() -> pid_t {
    call!(cty::__NR_gettid) as pid_t 
}

pub unsafe fn gettimeofday(tv: *mut timeval, tz: *mut timezone) -> k_int {
    call!(cty::__NR_gettimeofday, tv, tz) as k_int
}

pub unsafe fn getuid() -> uid_t {
    call!(cty::__NR_getuid) as uid_t 
}

pub unsafe fn getxattr(pathname: *const c_char, name: *const c_char, value: *mut c_void,
                       size: size_t) -> ssize_t {
    call!(cty::__NR_getxattr, pathname, name, value, size) as ssize_t
}

pub unsafe fn init_module(umod: *mut c_void, len: k_ulong,
                          uargs: *const c_char) -> k_int {
    call!(cty::__NR_init_module, umod, len, uargs) as k_int
}

pub unsafe fn inotify_add_watch(fd: k_int, pathname: *const c_char, mask: u32) -> k_int {
    call!(cty::__NR_inotify_add_watch, fd, pathname, mask) as k_int
}

pub unsafe fn inotify_init() -> k_int {
    call!(cty::__NR_inotify_init) as k_int 
}

pub unsafe fn inotify_init1(flags: k_int) -> k_int {
    call!(cty::__NR_inotify_init1, flags) as k_int
}

pub unsafe fn inotify_rm_watch(fd: k_int, wd: __s32) -> k_int {
    call!(cty::__NR_inotify_rm_watch, fd, wd) as k_int
}

pub unsafe fn io_cancel(ctx_id: aio_context_t, iocb: *mut iocb,
                        result: *mut io_event) -> k_int {
    call!(cty::__NR_io_cancel, ctx_id, iocb, result) as k_int
}

pub unsafe fn ioctl(fd: k_uint, cmd: k_uint, arg: k_ulong) -> k_int {
    call!(cty::__NR_ioctl, fd, cmd, arg) as k_int
}

pub unsafe fn io_destroy(ctx: aio_context_t) -> k_int {
    call!(cty::__NR_io_destroy, ctx) as k_int
}

pub unsafe fn io_getevents(ctx_id: aio_context_t, min_nr: k_long, nr: k_long,
                           events: *mut io_event, timeout: *mut timespec) -> k_int {
    call!(cty::__NR_io_getevents, ctx_id, min_nr, nr, events, timeout) as k_int
}

pub unsafe fn ioprio_get(which: k_int, who: k_int) -> k_int {
    call!(cty::__NR_ioprio_get, which, who) as k_int
}

pub unsafe fn ioprio_set(which: k_int, who: k_int, ioprio: k_int) -> k_int {
    call!(cty::__NR_ioprio_set, which, who, ioprio) as k_int
}

pub unsafe fn io_setup(nr_events: k_uint, ctxp: *mut aio_context_t) -> k_int {
    call!(cty::__NR_io_setup, nr_events, ctxp) as k_int
}

pub unsafe fn io_submit(ctx_id: aio_context_t, nr: k_long,
                        iocbpp: *mut *mut iocb) -> k_int {
    call!(cty::__NR_io_submit, ctx_id, nr, iocbpp) as k_int
}

pub unsafe fn kcmp(pid1: pid_t, pid2: pid_t, ty: k_int, idx1: k_ulong,
                   idx2: k_ulong) -> k_int {
    call!(cty::__NR_kcmp, pid1, pid2, ty, idx1, idx2) as k_int
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn kexec_file_load(kernel_fd: k_int, initrd_fd: k_int, cmdline_len: k_ulong,
                              cmdline_ptr: *const c_char, flags: k_ulong) -> k_long {
    call!(cty::__NR_kexec_file_load, kernel_fd, initrd_fd, cmdline_len, cmdline_ptr,
          flags) as k_long
}

pub unsafe fn kexec_load(entry: k_ulong, nr_segments: k_ulong,
                         segments: *mut kexec_segment, flags: k_ulong) -> k_long {
    call!(cty::__NR_kexec_load, entry, nr_segments, segments, flags) as k_long
}

pub unsafe fn keyctl(option: k_int, arg2: k_ulong, arg3: k_ulong, arg4: k_ulong,
                     arg5: k_ulong) -> k_long {
    call!(cty::__NR_keyctl, option, arg2, arg3, arg4, arg5) as k_long
}

pub unsafe fn kill(pid: pid_t, sig: k_int) -> k_int {
    call!(cty::__NR_kill, pid, sig) as k_int
}

pub unsafe fn lchown(filename: *const c_char, user: uid_t, group: gid_t) -> k_int {
    call!(cty::__NR_lchown, filename, user, group) as k_int
}

pub unsafe fn lgetxattr(pathname: *const c_char, name: *const c_char, value: *mut c_void,
                        size: size_t) -> ssize_t {
    call!(cty::__NR_lgetxattr, pathname, name, value, size) as ssize_t
}

pub unsafe fn linkat(olddfd: k_int, oldname: *const c_char, newdfd: k_int,
                     newname: *const c_char, flags: k_int) -> k_int {
    call!(cty::__NR_linkat, olddfd, oldname, newdfd, newname, flags) as k_int
}

pub unsafe fn link(oldname: *const c_char, newname: *const c_char) -> k_int {
    call!(cty::__NR_link, oldname, newname) as k_int
}

pub unsafe fn listxattr(pathname: *const c_char, list: *mut c_char,
                        size: size_t) -> ssize_t {
    call!(cty::__NR_listxattr, pathname, list, size) as ssize_t
}

pub unsafe fn llistxattr(pathname: *const c_char, list: *mut c_char,
                         size: size_t) -> ssize_t {
    call!(cty::__NR_llistxattr, pathname, list, size) as ssize_t
}

pub unsafe fn lremovexattr(pathname: *const c_char, name: *const c_char) -> k_int {
    call!(cty::__NR_lremovexattr, pathname, name) as k_int
}

pub unsafe fn lseek(fd: k_uint, offset: off_t, whence: k_uint) -> off_t {
    call!(cty::__NR_lseek, fd, offset, whence) as off_t
}

pub unsafe fn lsetxattr(pathname: *const c_char, name: *const c_char,
                        value: *const c_void, size: size_t, flags: k_int) -> k_int {
    call!(cty::__NR_lsetxattr, pathname, name, value, size, flags) as k_int
}

pub unsafe fn madvise(start: k_ulong, len_in: size_t, behavior: k_int) -> k_int {
    call!(cty::__NR_madvise, start, len_in, behavior) as k_int
}

pub unsafe fn mbind(start: k_ulong, len: k_ulong, mode: k_ulong, nmask: *const k_ulong,
                    maxnode: k_ulong, flags: k_uint) -> k_long {
    call!(cty::__NR_mbind, start, len, mode, nmask, maxnode, flags) as k_long
}

pub unsafe fn memfd_create(uname: *const c_char, flags: k_uint) -> k_int {
    call!(cty::__NR_memfd_create, uname, flags) as k_int
}

pub unsafe fn migrate_pages(pid: pid_t, maxnode: k_ulong, old_nodes: *const k_ulong,
                            new_nodes: *const k_ulong) -> k_long {
    call!(cty::__NR_migrate_pages, pid, maxnode, old_nodes, new_nodes) as k_long
}

pub unsafe fn mincore(start: k_ulong, len: size_t, vec: *mut k_uchar) -> k_int {
    call!(cty::__NR_mincore, start, len, vec) as k_int
}

pub unsafe fn mkdirat(dfd: k_int, pathname: *const c_char, mode: umode_t) -> k_int {
    call!(cty::__NR_mkdirat, dfd, pathname, mode) as k_int
}

pub unsafe fn mkdir(pathname: *const c_char, mode: umode_t) -> k_int {
    call!(cty::__NR_mkdir, pathname, mode) as k_int
}

pub unsafe fn mknodat(dfd: k_int, filename: *const c_char, mode: umode_t,
                      dev: k_uint) -> k_int {
    call!(cty::__NR_mknodat, dfd, filename, mode, dev) as k_int
}

pub unsafe fn mknod(filename: *const c_char, mode: umode_t, dev: k_uint) -> k_int {
    call!(cty::__NR_mknod, filename, mode, dev) as k_int
}

pub unsafe fn mlockall(flags: k_int) -> k_int {
    call!(cty::__NR_mlockall, flags) as k_int
}

pub unsafe fn mlock(start: k_ulong, len: size_t) -> k_int {
    call!(cty::__NR_mlock, start, len) as k_int
}

// pub unsafe fn mmap_pgoff(addr: k_ulong, len: k_ulong, prot: k_ulong, flags: k_ulong,
//                          fd: k_ulong, pgoff: k_ulong) -> k_long {
//     call!(cty::__NR_mmap_pgoff, addr, len, prot, flags, fd, pgoff) as k_long
// }

pub unsafe fn mount(dev_name: *mut c_char, dir_name: *mut c_char, ty: *mut c_char,
                    flags: k_ulong, data: *mut c_void) -> k_int {
    call!(cty::__NR_mount, dev_name, dir_name, ty, flags, data) as k_int
}

pub unsafe fn move_pages(pid: pid_t, nr_pages: k_ulong, pages: *const *mut c_void,
                         nodes: *const k_int, status: *mut k_int,
                         flags: k_int) -> k_long {
    call!(cty::__NR_move_pages, pid, nr_pages, pages, nodes, status, flags) as k_long
}

pub unsafe fn mprotect(start: k_ulong, len: size_t, prot: k_ulong) -> k_int {
    call!(cty::__NR_mprotect, start, len, prot) as k_int
}

pub unsafe fn mq_getsetattr(mqdes: mqd_t, u_mqstat: *const mq_attr,
                            u_omqstat: *mut mq_attr) -> k_int {
    call!(cty::__NR_mq_getsetattr, mqdes, u_mqstat, u_omqstat) as k_int
}

//pub unsafe fn mq_notify(mqdes: mqd_t, u_notification: *const sigevent) -> k_int {
//    call!(cty::__NR_mq_notify, mqdes, u_notification) as k_int
//}

pub unsafe fn mq_open(u_name: *const c_char, oflag: k_int, mode: umode_t,
                      u_attr: *mut mq_attr) -> mqd_t {
    call!(cty::__NR_mq_open, u_name, oflag, mode, u_attr) as mqd_t
}

pub unsafe fn mq_timedreceive(mqdes: mqd_t, u_msg_ptr: *mut c_char, msg_len: size_t,
                              u_msg_prio: *mut k_uint,
                              u_abs_timeout: *const timespec) -> ssize_t {
    call!(cty::__NR_mq_timedreceive, mqdes, u_msg_ptr, msg_len, u_msg_prio,
          u_abs_timeout) as ssize_t
}

pub unsafe fn mq_timedsend(mqdes: mqd_t, u_msg_ptr: *const c_char, msg_len: size_t,
                           msg_prio: k_uint, u_abs_timeout: *const timespec) -> k_int {
    call!(cty::__NR_mq_timedsend, mqdes, u_msg_ptr, msg_len, msg_prio,
          u_abs_timeout) as k_int
}

pub unsafe fn mq_unlink(u_name: *const c_char) -> k_int {
    call!(cty::__NR_mq_unlink, u_name) as k_int
}

pub unsafe fn mremap(addr: k_ulong, old_len: k_ulong, new_len: k_ulong, flags: k_ulong,
                     new_addr: k_ulong) -> *mut c_void {
    call!(cty::__NR_mremap, addr, old_len, new_len, flags, new_addr) as *mut c_void
}

pub unsafe fn msync(start: k_ulong, len: size_t, flags: k_int) -> k_int {
    call!(cty::__NR_msync, start, len, flags) as k_int
}

pub unsafe fn munlockall() -> k_int {
    call!(cty::__NR_munlockall) as k_int 
}

pub unsafe fn munlock(start: k_ulong, len: size_t) -> k_int {
    call!(cty::__NR_munlock, start, len) as k_int
}

pub unsafe fn munmap(addr: k_ulong, len: size_t) -> k_int {
    call!(cty::__NR_munmap, addr, len) as k_int
}

pub unsafe fn name_to_handle_at(dfd: k_int, name: *const c_char, handle: *mut file_handle,
                                mnt_id: *mut k_int, flag: k_int) -> k_int {
    call!(cty::__NR_name_to_handle_at, dfd, name, handle, mnt_id, flag) as k_int
}

pub unsafe fn nanosleep(rqtp: *mut timespec, rmtp: *mut timespec) -> k_int {
    call!(cty::__NR_nanosleep, rqtp, rmtp) as k_int
}

pub unsafe fn newuname(name: *mut new_utsname) -> k_int {
    call!(cty::__NR_newuname, name) as k_int
}

pub unsafe fn openat(dfd: k_int, filename: *const c_char, flags: k_int,
                     mode: umode_t) -> k_int {
    call!(cty::__NR_openat, dfd, filename, flags, mode) as k_int
}

pub unsafe fn open_by_handle_at(mountdirfd: k_int, handle: *mut file_handle,
                                flags: k_int) -> k_int {
    call!(cty::__NR_open_by_handle_at, mountdirfd, handle, flags) as k_int
}

pub unsafe fn open(filename: *const c_char, flags: k_int, mode: umode_t) -> k_int {
    call!(cty::__NR_open, filename, flags, mode) as k_int
}

pub unsafe fn pause() -> k_int {
    call!(cty::__NR_pause) as k_int 
}

// pub unsafe fn pciconfig_read(bus: k_ulong, dfn: k_ulong, off: k_ulong, len: k_ulong,
//                              buf: *mut c_void) -> k_int {
//     call!(cty::__NR_pciconfig_read, bus, dfn, off, len, buf) as k_int
// }
// 
// pub unsafe fn pciconfig_write(bus: k_ulong, dfn: k_ulong, off: k_ulong, len: k_ulong,
//                               buf: *mut c_void) -> k_int {
//     call!(cty::__NR_pciconfig_write, bus, dfn, off, len, buf) as k_int
// }

pub unsafe fn perf_event_open(attr_uptr: *mut perf_event_attr, pid: pid_t, cpu: k_int,
                              group_fd: k_int, flags: k_ulong) -> k_int {
    call!(cty::__NR_perf_event_open, attr_uptr, pid, cpu, group_fd, flags) as k_int
}

pub unsafe fn personality(personality: k_uint) -> k_int {
    call!(cty::__NR_personality, personality) as k_int
}

pub unsafe fn pipe2(fildes: *mut k_int, flags: k_int) -> k_int {
    call!(cty::__NR_pipe2, fildes, flags) as k_int
}

pub unsafe fn pipe(fildes: *mut k_int) -> k_int {
    call!(cty::__NR_pipe, fildes) as k_int
}

pub unsafe fn pivot_root(new_root: *const c_char, put_old: *const c_char) -> k_int {
    call!(cty::__NR_pivot_root, new_root, put_old) as k_int
}

pub unsafe fn poll(ufds: *mut pollfd, nfds: k_uint, timeout_msecs: k_int) -> k_int {
    call!(cty::__NR_poll, ufds, nfds, timeout_msecs) as k_int
}

pub unsafe fn ppoll(ufds: *mut pollfd, nfds: k_uint, tsp: *mut timespec,
                    sigmask: *const sigset_t, sigsetsize: size_t) -> k_int {
    call!(cty::__NR_ppoll, ufds, nfds, tsp, sigmask, sigsetsize) as k_int
}

pub unsafe fn prctl(option: k_int, arg2: k_ulong, arg3: k_ulong, arg4: k_ulong,
                    arg5: k_ulong) -> k_int {
    call!(cty::__NR_prctl, option, arg2, arg3, arg4, arg5) as k_int
}

pub unsafe fn preadv(fd: k_ulong, vec: *const iovec, vlen: k_ulong, pos_l: k_ulong,
                     pos_h: k_ulong) -> ssize_t {
    call!(cty::__NR_preadv, fd, vec, vlen, pos_l, pos_h) as ssize_t
}

pub unsafe fn prlimit64(pid: pid_t, resource: k_uint, new_rlim: *const rlimit64,
                        old_rlim: *mut rlimit64) -> k_int {
    call!(cty::__NR_prlimit64, pid, resource, new_rlim, old_rlim) as k_int
}

pub unsafe fn process_vm_readv(pid: pid_t, lvec: *const iovec, liovcnt: k_ulong,
                               rvec: *const iovec, riovcnt: k_ulong,
                               flags: k_ulong) -> ssize_t {
    call!(cty::__NR_process_vm_readv, pid, lvec, liovcnt, rvec, riovcnt, flags) as ssize_t
}

pub unsafe fn process_vm_writev(pid: pid_t, lvec: *const iovec, liovcnt: k_ulong,
                                rvec: *const iovec, riovcnt: k_ulong,
                                flags: k_ulong) -> ssize_t {
    call!(cty::__NR_process_vm_writev, pid, lvec, liovcnt, rvec, riovcnt,
          flags) as ssize_t
}

pub unsafe fn pselect6(n: k_int, inp: *mut fd_set, outp: *mut fd_set, exp: *mut fd_set,
                       tsp: *mut timespec, sig: *mut c_void) -> k_int {
    call!(cty::__NR_pselect6, n, inp, outp, exp, tsp, sig) as k_int
}

pub unsafe fn ptrace(request: k_long, pid: k_long, addr: k_ulong,
                     data: k_ulong) -> k_long {
    call!(cty::__NR_ptrace, request, pid, addr, data) as k_long
}

pub unsafe fn pwritev(fd: k_ulong, vec: *const iovec, vlen: k_ulong, pos_l: k_ulong,
                      pos_h: k_ulong) -> ssize_t {
    call!(cty::__NR_pwritev, fd, vec, vlen, pos_l, pos_h) as ssize_t
}

pub unsafe fn quotactl(cmd: k_uint, special: *const c_char, id: qid_t,
                       addr: *mut c_void) -> k_int {
    call!(cty::__NR_quotactl, cmd, special, id, addr) as k_int
}

pub unsafe fn read(fd: k_uint, buf: *mut c_char, count: size_t) -> ssize_t {
    call!(cty::__NR_read, fd, buf, count) as ssize_t
}

pub unsafe fn readlinkat(dfd: k_int, pathname: *const c_char, buf: *mut c_char,
                         bufsiz: k_int) -> ssize_t {
    call!(cty::__NR_readlinkat, dfd, pathname, buf, bufsiz) as ssize_t
}

pub unsafe fn readlink(path: *const c_char, buf: *mut c_char, bufsiz: k_int) -> ssize_t {
    call!(cty::__NR_readlink, path, buf, bufsiz) as ssize_t
}

pub unsafe fn readv(fd: k_ulong, vec: *const iovec, vlen: k_ulong) -> ssize_t {
    call!(cty::__NR_readv, fd, vec, vlen) as ssize_t
}

pub unsafe fn reboot(magic1: k_int, magic2: k_int, cmd: k_uint,
                     arg: *mut c_void) -> k_int {
    call!(cty::__NR_reboot, magic1, magic2, cmd, arg) as k_int
}

pub unsafe fn remap_file_pages(start: k_ulong, size: k_ulong, prot: k_ulong,
                               pgoff: k_ulong, flags: k_ulong) -> k_int {
    call!(cty::__NR_remap_file_pages, start, size, prot, pgoff, flags) as k_int
}

pub unsafe fn removexattr(pathname: *const c_char, name: *const c_char) -> k_int {
    call!(cty::__NR_removexattr, pathname, name) as k_int
}

pub unsafe fn renameat2(olddfd: k_int, oldname: *const c_char, newdfd: k_int,
                        newname: *const c_char, flags: k_uint) -> k_int {
    call!(cty::__NR_renameat2, olddfd, oldname, newdfd, newname, flags) as k_int
}

pub unsafe fn renameat(olddfd: k_int, oldname: *const c_char, newdfd: k_int,
                       newname: *const c_char) -> k_int {
    call!(cty::__NR_renameat, olddfd, oldname, newdfd, newname) as k_int
}

pub unsafe fn rename(oldname: *const c_char, newname: *const c_char) -> k_int {
    call!(cty::__NR_rename, oldname, newname) as k_int
}

pub unsafe fn request_key(_type: *const c_char, _description: *const c_char,
                          _callout_info: *const c_char,
                          destringid: key_serial_t) -> key_serial_t {
    call!(cty::__NR_request_key, _type, _description, _callout_info,
          destringid) as key_serial_t
}

pub unsafe fn restart_syscall() -> k_int {
    call!(cty::__NR_restart_syscall) as k_int 
}

pub unsafe fn rmdir(pathname: *const c_char) -> k_int {
    call!(cty::__NR_rmdir, pathname) as k_int
}

pub unsafe fn rt_sigaction(sig: k_int, act: *const sigaction, oact: *mut sigaction,
                           sigsetsize: size_t) -> k_int {
    call!(cty::__NR_rt_sigaction, sig, act, oact, sigsetsize) as k_int
}

pub unsafe fn rt_sigpending(uset: *mut sigset_t, sigsetsize: size_t) -> k_int {
    call!(cty::__NR_rt_sigpending, uset, sigsetsize) as k_int
}

pub unsafe fn rt_sigprocmask(how: k_int, nset: *mut sigset_t, oset: *mut sigset_t,
                             sigsetsize: size_t) -> k_int {
    call!(cty::__NR_rt_sigprocmask, how, nset, oset, sigsetsize) as k_int
}

pub unsafe fn rt_sigqueueinfo(pid: pid_t, sig: k_int, uinfo: *mut siginfo_t) -> k_int {
    call!(cty::__NR_rt_sigqueueinfo, pid, sig, uinfo) as k_int
}

pub unsafe fn rt_sigsuspend(unewset: *mut sigset_t, sigsetsize: size_t) -> k_int {
    call!(cty::__NR_rt_sigsuspend, unewset, sigsetsize) as k_int
}

pub unsafe fn rt_sigtimedwait(uthese: *const sigset_t, uinfo: *mut siginfo_t,
                              uts: *const timespec, sigsetsize: size_t) -> k_int {
    call!(cty::__NR_rt_sigtimedwait, uthese, uinfo, uts, sigsetsize) as k_int
}

pub unsafe fn rt_tgsigqueueinfo(tgid: pid_t, pid: pid_t, sig: k_int,
                                uinfo: *mut siginfo_t) -> k_int {
    call!(cty::__NR_rt_tgsigqueueinfo, tgid, pid, sig, uinfo) as k_int
}

pub unsafe fn rt_sigreturn() {
    call!(cty::__NR_rt_sigreturn);
}

pub unsafe fn sched_getaffinity(pid: pid_t, len: k_uint,
                                user_mask_ptr: *mut k_ulong) -> k_int {
    call!(cty::__NR_sched_getaffinity, pid, len, user_mask_ptr) as k_int
}

pub unsafe fn sched_getattr(pid: pid_t, uattr: *mut sched_attr, size: k_uint,
                            flags: k_uint) -> k_int {
    call!(cty::__NR_sched_getattr, pid, uattr, size, flags) as k_int
}

pub unsafe fn sched_getparam(pid: pid_t, param: *mut sched_param) -> k_int {
    call!(cty::__NR_sched_getparam, pid, param) as k_int
}

pub unsafe fn sched_get_priority_max(policy: k_int) -> k_int {
    call!(cty::__NR_sched_get_priority_max, policy) as k_int
}

pub unsafe fn sched_get_priority_min(policy: k_int) -> k_int {
    call!(cty::__NR_sched_get_priority_min, policy) as k_int
}

pub unsafe fn sched_getscheduler(pid: pid_t) -> k_int {
    call!(cty::__NR_sched_getscheduler, pid) as k_int
}

pub unsafe fn sched_rr_get_interval(pid: pid_t, k_interval: *mut timespec) -> k_int {
    call!(cty::__NR_sched_rr_get_interval, pid, k_interval) as k_int
}

pub unsafe fn sched_setaffinity(pid: pid_t, len: k_uint,
                                user_mask_ptr: *mut k_ulong) -> k_int {
    call!(cty::__NR_sched_setaffinity, pid, len, user_mask_ptr) as k_int
}

pub unsafe fn sched_setattr(pid: pid_t, uattr: *mut sched_attr, flags: k_uint) -> k_int {
    call!(cty::__NR_sched_setattr, pid, uattr, flags) as k_int
}

pub unsafe fn sched_setparam(pid: pid_t, param: *mut sched_param) -> k_int {
    call!(cty::__NR_sched_setparam, pid, param) as k_int
}

pub unsafe fn sched_setscheduler(pid: pid_t, policy: k_int,
                                 param: *mut sched_param) -> k_int {
    call!(cty::__NR_sched_setscheduler, pid, policy, param) as k_int
}

pub unsafe fn sched_yield() -> k_int {
    call!(cty::__NR_sched_yield) as k_int 
}

pub unsafe fn seccomp(op: k_uint, flags: k_uint, uargs: *const c_char) -> k_int {
    call!(cty::__NR_seccomp, op, flags, uargs) as k_int
}

pub unsafe fn select(n: k_int, inp: *mut fd_set, outp: *mut fd_set, exp: *mut fd_set,
                     tvp: *mut timeval) -> k_int {
    call!(cty::__NR_select, n, inp, outp, exp, tvp) as k_int
}

pub unsafe fn setdomainname(name: *mut c_char, len: k_int) -> k_int {
    call!(cty::__NR_setdomainname, name, len) as k_int
}

pub unsafe fn setfsgid(gid: gid_t) -> k_int {
    call!(cty::__NR_setfsgid, gid) as k_int
}

pub unsafe fn setfsuid(uid: uid_t) -> k_int {
    call!(cty::__NR_setfsuid, uid) as k_int
}

pub unsafe fn setgid(gid: gid_t) -> k_int {
    call!(cty::__NR_setgid, gid) as k_int
}

pub unsafe fn setgroups(gidsetsize: k_int, grouplist: *mut gid_t) -> k_int {
    call!(cty::__NR_setgroups, gidsetsize, grouplist) as k_int
}

pub unsafe fn sethostname(name: *mut c_char, len: k_int) -> k_int {
    call!(cty::__NR_sethostname, name, len) as k_int
}

pub unsafe fn setitimer(which: k_int, value: *mut itimerval,
                        ovalue: *mut itimerval) -> k_int {
    call!(cty::__NR_setitimer, which, value, ovalue) as k_int
}

pub unsafe fn set_mempolicy(mode: k_int, nmask: *const k_ulong,
                            maxnode: k_ulong) -> k_long {
    call!(cty::__NR_set_mempolicy, mode, nmask, maxnode) as k_long
}

pub unsafe fn setns(fd: k_int, nstype: k_int) -> k_int {
    call!(cty::__NR_setns, fd, nstype) as k_int
}

pub unsafe fn setpgid(pid: pid_t, pgid: pid_t) -> k_int {
    call!(cty::__NR_setpgid, pid, pgid) as k_int
}

pub unsafe fn setpriority(which: k_int, who: k_int, niceval: k_int) -> k_int {
    call!(cty::__NR_setpriority, which, who, niceval) as k_int
}

pub unsafe fn setregid(rgid: gid_t, egid: gid_t) -> k_int {
    call!(cty::__NR_setregid, rgid, egid) as k_int
}

pub unsafe fn setresgid(rgid: gid_t, egid: gid_t, sgid: gid_t) -> k_int {
    call!(cty::__NR_setresgid, rgid, egid, sgid) as k_int
}

pub unsafe fn setresuid(ruid: uid_t, euid: uid_t, suid: uid_t) -> k_int {
    call!(cty::__NR_setresuid, ruid, euid, suid) as k_int
}

pub unsafe fn setreuid(ruid: uid_t, euid: uid_t) -> k_int {
    call!(cty::__NR_setreuid, ruid, euid) as k_int
}

pub unsafe fn setrlimit(resource: k_uint, rlim: *mut rlimit) -> k_int {
    call!(cty::__NR_setrlimit, resource, rlim) as k_int
}

pub unsafe fn set_robust_list(head: *mut robust_list_head, len: size_t) -> k_long {
    call!(cty::__NR_set_robust_list, head, len) as k_long
}

pub unsafe fn setsid() -> pid_t {
    call!(cty::__NR_setsid) as pid_t
}

pub unsafe fn set_tid_address(tidptr: *mut k_int) -> k_long {
    call!(cty::__NR_set_tid_address, tidptr) as k_long
}

pub unsafe fn settimeofday(tv: *mut timeval, tz: *mut timezone) -> k_int {
    call!(cty::__NR_settimeofday, tv, tz) as k_int
}

pub unsafe fn setuid(uid: uid_t) -> k_int {
    call!(cty::__NR_setuid, uid) as k_int
}

pub unsafe fn setxattr(pathname: *const c_char, name: *const c_char, value: *const c_void,
                       size: size_t, flags: k_int) -> k_int {
    call!(cty::__NR_setxattr, pathname, name, value, size, flags) as k_int
}

pub unsafe fn signalfd4(ufd: k_int, user_mask: *const sigset_t, sizemask: size_t,
                        flags: k_int) -> k_int {
    call!(cty::__NR_signalfd4, ufd, user_mask, sizemask, flags) as k_int
}

pub unsafe fn sigaltstack(uss: *const stack_t, uoss: *mut stack_t) -> k_int {
    call!(cty::__NR_sigaltstack, uss, uoss) as k_int
}

// There are two of the following and I don't know which one is the correct one. Both are
// deprecated anyway.

//pub unsafe fn sigsuspend(mask: old_sigset_t) -> k_int {
//    call!(cty::__NR_sigsuspend, mask) as k_int
//}
//
//pub unsafe fn sigsuspend(unused1: k_int, unused2: k_int, mask: old_sigset_t) -> k_int {
//    call!(cty::__NR_sigsuspend, unused1, unused2, mask) as k_int
//}

pub unsafe fn splice(fd_in: k_int, off_in: *mut loff_t, fd_out: k_int,
                     off_out: *mut loff_t, len: size_t, flags: k_uint) -> ssize_t {
    call!(cty::__NR_splice, fd_in, off_in, fd_out, off_out, len, flags) as ssize_t
}

pub unsafe fn statfs(pathname: *const c_char, buf: *mut statfs) -> k_int {
    call!(cty::__NR_statfs, pathname, buf) as k_int
}

pub unsafe fn swapoff(specialfile: *const c_char) -> k_int {
    call!(cty::__NR_swapoff, specialfile) as k_int
}

pub unsafe fn swapon(specialfile: *const c_char, swap_flags: k_int) -> k_int {
    call!(cty::__NR_swapon, specialfile, swap_flags) as k_int
}

pub unsafe fn symlinkat(oldname: *const c_char, newdfd: k_int,
                        newname: *const c_char) -> k_int {
    call!(cty::__NR_symlinkat, oldname, newdfd, newname) as k_int
}

pub unsafe fn symlink(oldname: *const c_char, newname: *const c_char) -> k_int {
    call!(cty::__NR_symlink, oldname, newname) as k_int
}

pub unsafe fn sync() {
    call!(cty::__NR_sync);
}

pub unsafe fn syncfs(fd: k_int) -> k_int {
    call!(cty::__NR_syncfs, fd) as k_int
}

pub unsafe fn sysfs(option: k_int, arg1: k_ulong, arg2: k_ulong) -> k_int {
    call!(cty::__NR_sysfs, option, arg1, arg2) as k_int
}

pub unsafe fn sysinfo(info: *mut sysinfo) -> k_int {
    call!(cty::__NR_sysinfo, info) as k_int
}

pub unsafe fn syslog(ty: k_int, buf: *mut c_char, len: k_int) -> k_int {
    call!(cty::__NR_syslog, ty, buf, len) as k_int
}

pub unsafe fn tee(fdin: k_int, fdout: k_int, len: size_t, flags: k_uint) -> ssize_t {
    call!(cty::__NR_tee, fdin, fdout, len, flags) as ssize_t
}

pub unsafe fn tgkill(tgid: pid_t, pid: pid_t, sig: k_int) -> k_int {
    call!(cty::__NR_tgkill, tgid, pid, sig) as k_int
}

//pub unsafe fn timer_create(which_clock: clockid_t, timer_event_spec: *mut sigevent,
//                           created_timer_id: *mut timer_t) -> k_int {
//    call!(cty::__NR_timer_create, which_clock, timer_event_spec,
//          created_timer_id) as k_int
//}

pub unsafe fn timer_delete(timer_id: timer_t) -> k_int {
    call!(cty::__NR_timer_delete, timer_id) as k_int
}

pub unsafe fn timerfd_create(clockid: k_int, flags: k_int) -> k_int {
    call!(cty::__NR_timerfd_create, clockid, flags) as k_int
}

pub unsafe fn timerfd_gettime(ufd: k_int, otmr: *mut itimerspec) -> k_int {
    call!(cty::__NR_timerfd_gettime, ufd, otmr) as k_int
}

pub unsafe fn timerfd_settime(ufd: k_int, flags: k_int, utmr: *const itimerspec,
                              otmr: *mut itimerspec) -> k_int {
    call!(cty::__NR_timerfd_settime, ufd, flags, utmr, otmr) as k_int
}

pub unsafe fn timer_getoverrun(timer_id: timer_t) -> k_int {
    call!(cty::__NR_timer_getoverrun, timer_id) as k_int
}

pub unsafe fn timer_gettime(timer_id: timer_t, setting: *mut itimerspec) -> k_int {
    call!(cty::__NR_timer_gettime, timer_id, setting) as k_int
}

pub unsafe fn timer_settime(timer_id: timer_t, flags: k_int,
                            new_setting: *const itimerspec,
                            old_setting: *mut itimerspec) -> k_int {
    call!(cty::__NR_timer_settime, timer_id, flags, new_setting, old_setting) as k_int
}

pub unsafe fn times(tbuf: *mut tms) -> clock_t {
    call!(cty::__NR_times, tbuf) as clock_t
}

pub unsafe fn time(tloc: *mut time_t) -> time_t {
    call!(cty::__NR_time, tloc) as time_t
}

pub unsafe fn tkill(pid: pid_t, sig: k_int) -> k_int {
    call!(cty::__NR_tkill, pid, sig) as k_int
}

pub unsafe fn truncate(path: *const c_char, length: k_long) -> k_int {
    call!(cty::__NR_truncate, path, length) as k_int
}

pub unsafe fn umask(mask: k_int) -> umode_t {
    call!(cty::__NR_umask, mask) as umode_t
}

pub unsafe fn umount(name: *mut c_char, flags: k_int) -> k_int {
    call!(cty::__NR_umount, name, flags) as k_int
}

pub unsafe fn unlinkat(dfd: k_int, pathname: *const c_char, flag: k_int) -> k_int {
    call!(cty::__NR_unlinkat, dfd, pathname, flag) as k_int
}

pub unsafe fn unlink(pathname: *const c_char) -> k_int {
    call!(cty::__NR_unlink, pathname) as k_int
}

pub unsafe fn unshare(unshare_flags: k_ulong) -> k_int {
    call!(cty::__NR_unshare, unshare_flags) as k_int
}

pub unsafe fn ustat(dev: k_uint, ubuf: *mut ustat) -> k_int {
    call!(cty::__NR_ustat, dev, ubuf) as k_int
}

pub unsafe fn utime(filename: *const c_char, times: *const utimbuf) -> k_int {
    call!(cty::__NR_utime, filename, times) as k_int
}

pub unsafe fn utimensat(dfd: k_int, filename: *const c_char, utimes: *const timespec,
                        flags: k_int) -> k_int {
    call!(cty::__NR_utimensat, dfd, filename, utimes, flags) as k_int
}

pub unsafe fn utimes(filename: *const c_char, utimes: *const timeval) -> k_int {
    call!(cty::__NR_utimes, filename, utimes) as k_int
}

pub unsafe fn sendfile64(out_fd: k_int, in_fd: k_int, offset: *mut loff_t,
                         count: size_t) -> ssize_t {
    call!(cty::__NR_sendfile64, out_fd, in_fd, offset, count) as ssize_t
}

pub unsafe fn vfork() -> pid_t {
    call!(cty::__NR_vfork) as pid_t 
}

pub unsafe fn vhangup() -> k_int {
    call!(cty::__NR_vhangup) as k_int 
}

pub unsafe fn vmsplice(fd: k_int, iov: *const iovec, nr_segs: k_ulong,
                       flags: k_uint) -> ssize_t {
    call!(cty::__NR_vmsplice, fd, iov, nr_segs, flags) as ssize_t
}

pub unsafe fn waitid(which: k_int, upid: pid_t, infop: *mut siginfo_t, options: k_int,
                     ru: *mut rusage) -> k_int {
    call!(cty::__NR_waitid, which, upid, infop, options, ru) as k_int
}

pub unsafe fn write(fd: k_uint, buf: *const c_char, count: size_t) -> ssize_t {
    call!(cty::__NR_write, fd, buf, count) as ssize_t
}

pub unsafe fn writev(fd: k_ulong, vec: *const iovec, vlen: k_ulong) -> ssize_t {
    call!(cty::__NR_writev, fd, vec, vlen) as ssize_t
}

// 64 bit calls for 32 bit systems
#[cfg(target_arch = "x86")]
pub use self::_64_calls::*;

#[cfg(target_arch = "x86")]
mod _64_calls {
    use cty::{
        self, k_int, c_char, k_ulong, size_t, k_uint, stat64, statfs64, loff_t,
    };
    use ::arch::{SCT};

    pub unsafe fn fcntl64(fd: k_uint, cmd: k_uint, arg: k_ulong) -> k_int {
        call!(cty::__NR_fcntl64, fd, cmd, arg) as k_int
    }

    pub unsafe fn fstat64(fd: k_ulong, statbuf: *mut stat64) -> k_int {
        call!(cty::__NR_fstat64, fd, statbuf) as k_int
    }

    pub unsafe fn fstatat64(dfd: k_int, filename: *const c_char, statbuf: *mut stat64,
                            flag: k_int) -> k_int {
        call!(cty::__NR_fstatat64, dfd, filename, statbuf, flag) as k_int
    }

    pub unsafe fn fstatfs64(fd: k_uint, sz: size_t, buf: *mut statfs64) -> k_int {
        call!(cty::__NR_fstatfs64, fd, sz, buf) as k_int
    }

    pub unsafe fn lstat64(filename: *const c_char, statbuf: *mut stat64) -> k_int {
        call!(cty::__NR_lstat64, filename, statbuf) as k_int
    }

    pub unsafe fn stat64(filename: *const c_char, statbuf: *mut stat64) -> k_int {
        call!(cty::__NR_stat64, filename, statbuf) as k_int
    }

    pub unsafe fn statfs64(pathname: *const c_char, sz: size_t,
                           buf: *mut statfs64) -> k_int {
        call!(cty::__NR_statfs64, pathname, sz, buf) as k_int
    }

    pub unsafe fn llseek(fd: k_uint, offset_high: k_ulong, offset_low: k_ulong,
                         result: *mut loff_t, whence: k_uint) -> k_int {
        call!(cty::__NR_llseek, fd, offset_high, offset_low, result, whence) as k_int
    }
}

// 64 bit calls for 64 bit systems
#[cfg(target_arch = "x86_64")]
pub use self::new_calls::*;

#[cfg(target_arch = "x86_64")]
mod new_calls {
    use cty::{
        self, k_int, c_char, k_uint, stat,
    };
    use ::arch::{SCT};

    pub unsafe fn newfstatat(dfd: k_int, filename: *const c_char, statbuf: *mut stat,
                             flag: k_int) -> k_int {
        call!(cty::__NR_newfstatat, dfd, filename, statbuf, flag) as k_int
    }
    
    pub unsafe fn newfstat(fd: k_uint, statbuf: *mut stat) -> k_int {
        call!(cty::__NR_newfstat, fd, statbuf) as k_int
    }
    
    pub unsafe fn newlstat(filename: *const c_char, statbuf: *mut stat) -> k_int {
        call!(cty::__NR_newlstat, filename, statbuf) as k_int
    }
    
    pub unsafe fn newstat(filename: *const c_char, statbuf: *mut stat) -> k_int {
        call!(cty::__NR_newstat, filename, statbuf) as k_int
    }
}

// ipc interface
//
// XXX: This interface is quite complex. See the code in ipc/syscall.c. This MUST be
// audited before it is used.
#[cfg(target_arch = "x86")]
pub use self::ipc_one::*;

#[cfg(target_arch = "x86")]
mod ipc_one {
    use cty::{
        self, k_int, c_char, c_void, IPC_64, msgbuf, k_long, ssize_t, timespec,
        key_t, sembuf, k_ulong, size_t, c_long, msqid64_ds, shmid64_ds, k_uint,
    };
    use ::arch::{SCT};

    pub unsafe fn semop(semid: k_int, tsops: *mut sembuf, nsops: k_uint) -> k_int {
        call!(cty::__NR_ipc, cty::SEMOP, semid, nsops, 0, tsops) as k_int
    }

    pub unsafe fn semtimedop(semid: k_int, tsops: *mut sembuf, nsops: k_uint,
                             timeout: *const timespec) -> k_int {
        call!(cty::__NR_ipc, cty::SEMTIMEDOP, semid, nsops, 0, tsops, timeout) as k_int
    }

    pub unsafe fn semget(key: key_t, nsems: k_int, semflg: k_int) -> k_int {
        call!(cty::__NR_ipc, cty::SEMGET, key, nsems, semflg) as k_int
    }

    pub unsafe fn semctl(semid: k_int, semnum: k_int, cmd: k_int, arg: k_ulong) -> k_int {
        // I kid you not: The last argument goes as a pointer that is immediately
        // dereferenced in the kernel.
        call!(cty::__NR_ipc, cty::SEMCTL, semid, semnum, cmd | IPC_64,
              &arg as *const _) as k_int
    }

    pub unsafe fn msgsnd(msqid: k_int, msgp: *mut msgbuf, msgsz: size_t,
                         msgflg: k_int) -> k_int {
        call!(cty::__NR_ipc, cty::MSGSND, msqid, msgsz, msgflg, msgp) as k_int
    }

    pub unsafe fn msgrcv(msqid: k_int, msgp: *mut msgbuf, msgsz: size_t, msgtyp: k_long,
                         msgflg: k_int) -> ssize_t {
        let mut data = [msgp as c_long, msgtyp];
        call!(cty::__NR_ipc, cty::MSGRCV, msqid, msgsz, msgflg,
              data.as_mut_ptr()) as ssize_t
    }

    pub unsafe fn msgget(key: key_t, msgflg: k_int) -> k_int {
        call!(cty::__NR_ipc, cty::MSGGET, key, msgflg) as k_int
    }

    // See shmctl comment
    pub unsafe fn msgctl(msqid: k_int, cmd: k_int, buf: *mut msqid64_ds) -> k_int {
        call!(cty::__NR_ipc, cty::MSGCTL, msqid, cmd | IPC_64, 0, buf, 0) as k_int
    }

    pub unsafe fn shmdt(shmaddr: *mut c_char) -> k_int {
        call!(cty::__NR_ipc, cty::SHMDT, 0, 0, 0, shmaddr) as k_int
    }

    // here the kernel defines shmid_ds instead of shmid64_ds. But shmid_ds is deprecated
    // and glibc and friends will always use shmid64_ds. We have to add the IPC_64 flag to
    // tell the kernel that we're using shmid64_ds.
    pub unsafe fn shmctl(shmid: k_int, cmd: k_int, buf: *mut shmid64_ds) -> k_int {
        call!(cty::__NR_ipc, cty::SHMCTL, shmid, cmd | IPC_64, 0, buf, 0) as k_int
    }

    pub unsafe fn shmget(key: key_t, size: size_t, shmflg: k_int) -> k_int {
        call!(cty::__NR_ipc, cty::SHMGET, key, size, shmflg) as k_int
    }

    pub unsafe fn shmat(shmid: k_int, mut shmaddr: *mut c_char,
                        shmflg: k_int) -> *mut c_void {
        call!(cty::__NR_ipc, cty::SHMAT, shmid, shmflg, &mut shmaddr as *mut _,
              shmaddr) as *mut c_void
    }
}

// separate ipc calls
#[cfg(target_arch = "x86_64")]
pub use self::ipc_sep::*;

#[cfg(target_arch = "x86_64")]
mod ipc_sep {
    use cty::{
        self, k_int, c_char, c_void, IPC_64, msgbuf, k_long, ssize_t, timespec,
        key_t, sembuf, k_ulong, size_t, msqid64_ds, shmid64_ds, k_uint,
    };
    use ::arch::{SCT};

    pub unsafe fn semop(semid: k_int, tsops: *mut sembuf, nsops: k_uint) -> k_int {
        call!(cty::__NR_semop, semid, tsops, nsops) as k_int
    }

    pub unsafe fn semtimedop(semid: k_int, tsops: *mut sembuf, nsops: k_uint,
                             timeout: *const timespec) -> k_int {
        call!(cty::__NR_semtimedop, semid, tsops, nsops, timeout) as k_int
    }

    pub unsafe fn semget(key: key_t, nsems: k_int, semflg: k_int) -> k_int {
        call!(cty::__NR_semget, key, nsems, semflg) as k_int
    }

    pub unsafe fn semctl(semid: k_int, semnum: k_int, cmd: k_int, arg: k_ulong) -> k_int {
        call!(cty::__NR_semctl, semid, semnum, cmd | IPC_64, arg) as k_int
    }

    pub unsafe fn msgsnd(msqid: k_int, msgp: *mut msgbuf, msgsz: size_t,
                         msgflg: k_int) -> k_int {
        call!(cty::__NR_msgsnd, msqid, msgp, msgsz, msgflg) as k_int
    }

    pub unsafe fn msgrcv(msqid: k_int, msgp: *mut msgbuf, msgsz: size_t, msgtyp: k_long,
                         msgflg: k_int) -> ssize_t {
        call!(cty::__NR_msgrcv, msqid, msgp, msgsz, msgtyp, msgflg) as ssize_t
    }

    pub unsafe fn msgget(key: key_t, msgflg: k_int) -> k_int {
        call!(cty::__NR_msgget, key, msgflg) as k_int
    }

    // See shmctl comment
    pub unsafe fn msgctl(msqid: k_int, cmd: k_int, buf: *mut msqid64_ds) -> k_int {
        call!(cty::__NR_msgctl, msqid, cmd | IPC_64, buf) as k_int
    }

    pub unsafe fn shmdt(shmaddr: *mut c_char) -> k_int {
        call!(cty::__NR_shmdt, shmaddr) as k_int
    }

    // here the kernel defines shmid_ds instead of shmid64_ds. But shmid_ds is deprecated
    // and glibc and friends will always use shmid64_ds. We have to add the IPC_64 flag to
    // tell the kernel that we're using shmid64_ds.
    pub unsafe fn shmctl(shmid: k_int, cmd: k_int, buf: *mut shmid64_ds) -> k_int {
        call!(cty::__NR_shmctl, shmid, cmd | IPC_64, buf) as k_int
    }

    pub unsafe fn shmget(key: key_t, size: size_t, shmflg: k_int) -> k_int {
        call!(cty::__NR_shmget, key, size, shmflg) as k_int
    }

    pub unsafe fn shmat(shmid: k_int, shmaddr: *mut c_char,
                        shmflg: k_int) -> *mut c_void {
        call!(cty::__NR_shmat, shmid, shmaddr, shmflg) as *mut c_void
    }
}

// socketcall interface
#[cfg(target_arch = "x86")]
pub use self::sock_one::*;

#[cfg(target_arch = "x86")]
mod sock_one {
    use cty::{
        self, k_int, sockaddr, c_char, c_void, size_t, k_uint, ssize_t, k_ulong, msghdr,
        timespec, mmsghdr,
    };
    use ::arch::{SCT};

    macro_rules! socketcall {
        ($nr:expr) => { socketcall!($nr, 0, 0, 0, 0, 0, 0) };
        ($nr:expr, $a1:expr) => { socketcall!($nr, $a1, 0, 0, 0, 0, 0) };
        ($nr:expr, $a1:expr, $a2:expr) => { socketcall!($nr, $a1, $a2, 0, 0, 0, 0) };
        ($nr:expr, $a1:expr, $a2:expr, $a3:expr) => {
            socketcall!($nr, $a1, $a2, $a3, 0, 0, 0)
        };
        ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {
            socketcall!($nr, $a1, $a2, $a3, $a4, 0, 0)
        };
        ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {
            socketcall!($nr, $a1, $a2, $a3, $a4, $a5, 0)
        };
        ($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {{
            let args = [$a1 as k_ulong,  $a2 as k_ulong, $a3 as k_ulong, 
                        $a4 as k_ulong,  $a5 as k_ulong, $a6 as k_ulong];
            call!($nr, args.as_ptr())
        }};
    }

    pub unsafe fn socket(family: k_int, ty: k_int, protocol: k_int) -> k_int {
        socketcall!(cty::SYS_SOCKET, family, ty, protocol) as k_int
    }

    pub unsafe fn bind(fd: k_int, umyaddr: *mut sockaddr, addrlen: k_int) -> k_int {
        socketcall!(cty::SYS_BIND, fd, umyaddr, addrlen) as k_int
    }

    pub unsafe fn connect(fd: k_int, uservaddr: *mut sockaddr, addrlen: k_int) -> k_int {
        socketcall!(cty::SYS_CONNECT, fd, uservaddr, addrlen) as k_int
    }

    pub unsafe fn listen(fd: k_int, backlog: k_int) -> k_int {
        socketcall!(cty::SYS_LISTEN, fd, backlog) as k_int
    }

    pub unsafe fn accept4(fd: k_int, upeer_sockaddr: *mut sockaddr,
                          upeer_addrlen: *mut k_int, flags: k_int) -> k_int {
        socketcall!(cty::SYS_ACCEPT4, fd, upeer_sockaddr, upeer_addrlen, flags) as k_int
    }

    pub unsafe fn accept(fd: k_int, upeer_sockaddr: *mut sockaddr,
                         upeer_addrlen: *mut k_int) -> k_int {
        socketcall!(cty::SYS_ACCEPT, fd, upeer_sockaddr, upeer_addrlen) as k_int
    }

    pub unsafe fn getsockname(fd: k_int, usockaddr: *mut sockaddr,
                              usockaddr_len: *mut k_int) -> k_int {
        socketcall!(cty::SYS_GETSOCKNAME, fd, usockaddr, usockaddr_len) as k_int
    }

    pub unsafe fn getsockopt(fd: k_int, level: k_int, optname: k_int,
                             optval: *mut c_char, optlen: *mut k_int) -> k_int {
        socketcall!(cty::SYS_GETSOCKOPT, fd, level, optname, optval, optlen) as k_int
    }

    pub unsafe fn getpeername(fd: k_int, usockaddr: *mut sockaddr,
                              usockaddr_len: *mut k_int) -> k_int {
        socketcall!(cty::SYS_GETPEERNAME, fd, usockaddr, usockaddr_len) as k_int
    }

    pub unsafe fn socketpair(family: k_int, ty: k_int, protocol: k_int,
                             usockvec: *mut k_int) -> k_int {
        socketcall!(cty::SYS_SOCKETPAIR, family, ty, protocol, usockvec) as k_int
    }

    pub unsafe fn recvfrom(fd: k_int, ubuf: *mut c_void, size: size_t, flags: k_uint,
                           addr: *mut sockaddr, addr_len: *mut k_int) -> ssize_t {
        socketcall!(cty::SYS_RECVFROM, fd, ubuf, size, flags, addr, addr_len) as ssize_t
    }

    pub unsafe fn recvmmsg(fd: k_int, mmsg: *mut mmsghdr, vlen: k_uint, flags: k_uint,
                           timeout: *mut timespec) -> ssize_t {
        socketcall!(cty::SYS_RECVMMSG, fd, mmsg, vlen, flags, timeout) as ssize_t
    }

    pub unsafe fn recvmsg(fd: k_int, msg: *mut msghdr, flags: k_uint) -> ssize_t {
        socketcall!(cty::SYS_RECVMSG, fd, msg, flags) as ssize_t
    }

    pub unsafe fn sendmmsg(fd: k_int, mmsg: *mut mmsghdr, vlen: k_uint,
                           flags: k_uint) -> ssize_t {
        socketcall!(cty::SYS_SENDMMSG, fd, mmsg, vlen, flags) as ssize_t
    }

    pub unsafe fn sendmsg(fd: k_int, msg: *mut msghdr, flags: k_uint) -> ssize_t {
        socketcall!(cty::SYS_SENDMSG, fd, msg, flags) as ssize_t
    }

    pub unsafe fn sendto(fd: k_int, buff: *mut c_void, len: size_t, flags: k_uint,
                         addr: *mut sockaddr, addr_len: k_int) -> ssize_t {
        socketcall!(cty::SYS_SENDTO, fd, buff, len, flags, addr, addr_len) as ssize_t
    }

    pub unsafe fn shutdown(fd: k_int, how: k_int) -> k_int {
        socketcall!(cty::SYS_SHUTDOWN, fd, how) as k_int
    }

    pub unsafe fn setsockopt(fd: k_int, level: k_int, optname: k_int,
                             optval: *mut c_char, optlen: k_int) -> k_int {
        socketcall!(cty::SYS_SETSOCKOPT, fd, level, optname, optval, optlen) as k_int
    }
}

// separate socket calls
#[cfg(target_arch = "x86_64")]
pub use self::sock_sep::*;

#[cfg(target_arch = "x86_64")]
mod sock_sep {
    use cty::{
        self, k_int, sockaddr, c_char, c_void, size_t, k_uint, ssize_t, msghdr,
        timespec, mmsghdr,
    };
    use ::arch::{SCT};

    pub unsafe fn socket(family: k_int, ty: k_int, protocol: k_int) -> k_int {
        call!(cty::__NR_socket, family, ty, protocol) as k_int
    }

    pub unsafe fn bind(fd: k_int, umyaddr: *mut sockaddr, addrlen: k_int) -> k_int {
        call!(cty::__NR_bind, fd, umyaddr, addrlen) as k_int
    }

    pub unsafe fn connect(fd: k_int, uservaddr: *mut sockaddr, addrlen: k_int) -> k_int {
        call!(cty::__NR_connect, fd, uservaddr, addrlen) as k_int
    }

    pub unsafe fn listen(fd: k_int, backlog: k_int) -> k_int {
        call!(cty::__NR_listen, fd, backlog) as k_int
    }

    pub unsafe fn accept4(fd: k_int, upeer_sockaddr: *mut sockaddr,
                          upeer_addrlen: *mut k_int, flags: k_int) -> k_int {
        call!(cty::__NR_accept4, fd, upeer_sockaddr, upeer_addrlen, flags) as k_int
    }

    pub unsafe fn accept(fd: k_int, upeer_sockaddr: *mut sockaddr,
                         upeer_addrlen: *mut k_int) -> k_int {
        call!(cty::__NR_accept, fd, upeer_sockaddr, upeer_addrlen) as k_int
    }

    pub unsafe fn getsockname(fd: k_int, usockaddr: *mut sockaddr,
                              usockaddr_len: *mut k_int) -> k_int {
        call!(cty::__NR_getsockname, fd, usockaddr, usockaddr_len) as k_int
    }

    pub unsafe fn getsockopt(fd: k_int, level: k_int, optname: k_int,
                             optval: *mut c_char, optlen: *mut k_int) -> k_int {
        call!(cty::__NR_getsockopt, fd, level, optname, optval, optlen) as k_int
    }

    pub unsafe fn getpeername(fd: k_int, usockaddr: *mut sockaddr,
                              usockaddr_len: *mut k_int) -> k_int {
        call!(cty::__NR_getpeername, fd, usockaddr, usockaddr_len) as k_int
    }

    pub unsafe fn socketpair(family: k_int, ty: k_int, protocol: k_int,
                             usockvec: *mut k_int) -> k_int {
        call!(cty::__NR_socketpair, family, ty, protocol, usockvec) as k_int
    }

    pub unsafe fn recvfrom(fd: k_int, ubuf: *mut c_void, size: size_t, flags: k_uint,
                           addr: *mut sockaddr, addr_len: *mut k_int) -> ssize_t {
        call!(cty::__NR_recvfrom, fd, ubuf, size, flags, addr, addr_len) as ssize_t
    }

    pub unsafe fn recvmmsg(fd: k_int, mmsg: *mut mmsghdr, vlen: k_uint, flags: k_uint,
                           timeout: *mut timespec) -> ssize_t {
        call!(cty::__NR_recvmmsg, fd, mmsg, vlen, flags, timeout) as ssize_t
    }

    pub unsafe fn recvmsg(fd: k_int, msg: *mut msghdr, flags: k_uint) -> ssize_t {
        call!(cty::__NR_recvmsg, fd, msg, flags) as ssize_t
    }

    pub unsafe fn sendmmsg(fd: k_int, mmsg: *mut mmsghdr, vlen: k_uint,
                           flags: k_uint) -> ssize_t {
        call!(cty::__NR_sendmmsg, fd, mmsg, vlen, flags) as ssize_t
    }

    pub unsafe fn sendmsg(fd: k_int, msg: *mut msghdr, flags: k_uint) -> ssize_t {
        call!(cty::__NR_sendmsg, fd, msg, flags) as ssize_t
    }

    pub unsafe fn sendto(fd: k_int, buff: *mut c_void, len: size_t, flags: k_uint,
                         addr: *mut sockaddr, addr_len: k_int) -> ssize_t {
        call!(cty::__NR_sendto, fd, buff, len, flags, addr, addr_len) as ssize_t
    }

    pub unsafe fn shutdown(fd: k_int, how: k_int) -> k_int {
        call!(cty::__NR_shutdown, fd, how) as k_int
    }

    pub unsafe fn setsockopt(fd: k_int, level: k_int, optname: k_int,
                             optval: *mut c_char, optlen: k_int) -> k_int {
        call!(cty::__NR_setsockopt, fd, level, optname, optval, optlen) as k_int
    }
}

// syscalls with 64-bit arguments
//
// these implementations cannot be used on 32 bit systems
#[cfg(target_arch = "x86_64")]
pub use self::wide::*;

#[cfg(target_arch = "x86_64")]
mod wide {
    use cty::{
        self, k_int, c_char, size_t, k_uint, ssize_t, loff_t, __u64,
    };
    use ::arch::{SCT};

    // pub unsafe fn fadvise64_64(fd: k_int, offset: loff_t, len: loff_t,
    //                            advice: k_int) -> k_int {
    //     call!(cty::__NR_fadvise64_64, fd, offset, len, advice) as k_int
    // }

    pub unsafe fn fadvise64(fd: k_int, offset: loff_t, len: size_t,
                            advice: k_int) -> k_int {
        call!(cty::__NR_fadvise64, fd, offset, len, advice) as k_int
    }

    pub unsafe fn fallocate(fd: k_int, mode: k_int, offset: loff_t,
                            len: loff_t) -> k_int {
        call!(cty::__NR_fallocate, fd, mode, offset, len) as k_int
    }

    pub unsafe fn fanotify_mark(fanotify_fd: k_int, flags: k_uint, mask: __u64,
                                dfd: k_int, pathname: *const c_char) -> k_int {
        call!(cty::__NR_fanotify_mark, fanotify_fd, flags, mask, dfd, pathname) as k_int
    }

    // pub unsafe fn ftruncate64(fd: k_uint, length: loff_t) -> k_int {
    //     call!(cty::__NR_ftruncate64, fd, length) as k_int
    // }

    pub unsafe fn lookup_dcookie(cookie64: u64, buf: *mut c_char, len: size_t) -> k_int {
        call!(cty::__NR_lookup_dcookie, cookie64, buf, len) as k_int
    }

    pub unsafe fn pread64(fd: k_uint, buf: *mut c_char, count: size_t,
                          pos: loff_t) -> ssize_t {
        call!(cty::__NR_pread64, fd, buf, count, pos) as ssize_t
    }

    pub unsafe fn pwrite64(fd: k_uint, buf: *const c_char, count: size_t,
                           pos: loff_t) -> ssize_t {
        call!(cty::__NR_pwrite64, fd, buf, count, pos) as ssize_t
    }

    pub unsafe fn readahead(fd: k_int, offset: loff_t, count: size_t) -> ssize_t {
        call!(cty::__NR_readahead, fd, offset, count) as ssize_t
    }

    // pub unsafe fn sync_file_range2(fd: k_int, flags: k_uint, offset: loff_t,
    //                                nbytes: loff_t) -> k_int {
    //     call!(cty::__NR_sync_file_range2, fd, flags, offset, nbytes) as k_int
    // }

    pub unsafe fn sync_file_range(fd: k_int, offset: loff_t, nbytes: loff_t,
                                  flags: k_uint) -> k_int {
        call!(cty::__NR_sync_file_range, fd, offset, nbytes, flags) as k_int
    }

    // pub unsafe fn truncate64(path: *const c_char, length: loff_t) -> k_int {
    //     call!(cty::__NR_truncate64, path, length) as k_int
    // }
}

#[cfg(target_os = "TODO: add something like cfg(false)")]
mod deprecated {
    pub unsafe fn chown16(filename: *const c_char, user: old_uid_t,
                          group: old_gid_t) -> k_int {
        call!(cty::__NR_chown16, filename, user, group) as k_int
    }

    pub unsafe fn fchown16(fd: k_uint, user: old_uid_t, group: old_gid_t) -> k_int {
        call!(cty::__NR_fchown16, fd, user, group) as k_int
    }

    pub unsafe fn getegid16() -> old_gid_t {
        call!(cty::__NR_getegid16) as old_gid_t
    }

    pub unsafe fn geteuid16() -> old_uid_t {
        call!(cty::__NR_geteuid16) as old_uid_t 
    }

    pub unsafe fn getgid16() -> old_gid_t {
        call!(cty::__NR_getgid16) as old_gid_t 
    }

    pub unsafe fn getgroups16(gidsetsize: k_int, grouplist: *mut old_gid_t) -> k_int {
        call!(cty::__NR_getgroups16, gidsetsize, grouplist) as k_int
    }

    pub unsafe fn getresgid16(rgidp: *mut old_gid_t, egidp: *mut old_gid_t,
                              sgidp: *mut old_gid_t) -> k_int {
        call!(cty::__NR_getresgid16, rgidp, egidp, sgidp) as k_int
    }

    pub unsafe fn getresuid16(ruidp: *mut old_uid_t, euidp: *mut old_uid_t,
                              suidp: *mut old_uid_t) -> k_int {
        call!(cty::__NR_getresuid16, ruidp, euidp, suidp) as k_int
    }

    pub unsafe fn getuid16() -> old_uid_t {
        call!(cty::__NR_getuid16) as old_uid_t 
    }

    pub unsafe fn lchown16(filename: *const c_char, user: old_uid_t,
                           group: old_gid_t) -> k_int {
        call!(cty::__NR_lchown16, filename, user, group) as k_int
    }

    pub unsafe fn setfsgid16(gid: old_gid_t) -> k_int {
        call!(cty::__NR_setfsgid16, gid) as k_int
    }

    pub unsafe fn setfsuid16(uid: old_uid_t) -> k_int {
        call!(cty::__NR_setfsuid16, uid) as k_int
    }

    pub unsafe fn setgid16(gid: old_gid_t) -> k_int {
        call!(cty::__NR_setgid16, gid) as k_int
    }

    pub unsafe fn setgroups16(gidsetsize: k_int, grouplist: *mut old_gid_t) -> k_int {
        call!(cty::__NR_setgroups16, gidsetsize, grouplist) as k_int
    }

    pub unsafe fn setregid16(rgid: old_gid_t, egid: old_gid_t) -> k_int {
        call!(cty::__NR_setregid16, rgid, egid) as k_int
    }

    pub unsafe fn setresgid16(rgid: old_gid_t, egid: old_gid_t,
                              sgid: old_gid_t) -> k_int {
        call!(cty::__NR_setresgid16, rgid, egid, sgid) as k_int
    }

    pub unsafe fn setresuid16(ruid: old_uid_t, euid: old_uid_t,
                              suid: old_uid_t) -> k_int {
        call!(cty::__NR_setresuid16, ruid, euid, suid) as k_int
    }

    pub unsafe fn setreuid16(ruid: old_uid_t, euid: old_uid_t) -> k_int {
        call!(cty::__NR_setreuid16, ruid, euid) as k_int
    }

    pub unsafe fn setuid16(uid: old_uid_t) -> k_int {
        call!(cty::__NR_setuid16, uid) as k_int
    }

    pub unsafe fn gethostname(name: *mut c_char, len: k_int) -> k_int {
        call!(cty::__NR_gethostname, name, len) as k_int
    }

    pub unsafe fn old_getrlimit(resource: k_uint, rlim: *mut rlimit) -> k_int {
        call!(cty::__NR_old_getrlimit, resource, rlim) as k_int
    }

    pub unsafe fn old_mmap(arg: *mut mmap_arg_struct) -> k_long {
        call!(cty::__NR_old_mmap, arg) as k_long
    }

    pub unsafe fn old_readdir(fd: k_uint, dirent: *mut old_linux_dirent,
                              count: k_uint) -> k_int {
        call!(cty::__NR_old_readdir, fd, dirent, count) as k_int
    }

    pub unsafe fn old_select(arg: *mut sel_arg_struct) -> k_int {
        call!(cty::__NR_old_select, arg) as k_int
    }

    pub unsafe fn oldumount(name: *mut c_char) -> k_int {
        call!(cty::__NR_oldumount, name) as k_int
    }

    pub unsafe fn olduname(name: *mut oldold_utsname) -> k_int {
        call!(cty::__NR_olduname, name) as k_int
    }

    pub unsafe fn sendfile(out_fd: k_int, in_fd: k_int, offset: *mut off_t,
                           count: size_t) -> ssize_t {
        call!(cty::__NR_sendfile, out_fd, in_fd, offset, count) as ssize_t
    }

    pub unsafe fn sigaction(sig: k_int, act: *const old_sigaction,
                            oact: *mut old_sigaction) -> k_int {
        call!(cty::__NR_sigaction, sig, act, oact) as k_int
    }

    pub unsafe fn sigpending(set: *mut old_sigset_t) -> k_int {
        call!(cty::__NR_sigpending, set) as k_int
    }

    pub unsafe fn sigprocmask(how: k_int, nset: *mut old_sigset_t,
                              oset: *mut old_sigset_t) -> k_int {
        call!(cty::__NR_sigprocmask, how, nset, oset) as k_int
    }

    pub unsafe fn signalfd(ufd: k_int, user_mask: *const sigset_t,
                           sizemask: size_t) -> k_int {
        call!(cty::__NR_signalfd, ufd, user_mask, sizemask) as k_int
    }

    pub unsafe fn signal(sig: k_int, handler: usize) -> usize {
        call!(cty::__NR_signal, sig, handler) as usize
    }

    pub unsafe fn bdflush(func: k_int, data: k_long) -> k_int {
        call!(cty::__NR_bdflush, func, data) as k_int
    }

    pub unsafe fn fstat(fd: k_uint, statbuf: *mut __old_kernel_stat) -> k_int {
        call!(cty::__NR_fstat, fd, statbuf) as k_int
    }

    pub unsafe fn lstat(filename: *const c_char,
                        statbuf: *mut __old_kernel_stat) -> k_int {
        call!(cty::__NR_lstat, filename, statbuf) as k_int
    }

    pub unsafe fn nice(increment: k_int) -> k_int {
        call!(cty::__NR_nice, increment) as k_int
    }

    pub unsafe fn sgetmask() -> k_long {
        call!(cty::__NR_sgetmask) as k_long 
    }

    pub unsafe fn ssetmask(newmask: k_int) -> k_long {
        call!(cty::__NR_ssetmask, newmask) as k_long
    }

    pub unsafe fn stat(filename: *const c_char,
                       statbuf: *mut __old_kernel_stat) -> k_int {
        call!(cty::__NR_stat, filename, statbuf) as k_int
    }

    pub unsafe fn stime(tptr: *mut time_t) -> k_int {
        call!(cty::__NR_stime, tptr) as k_int
    }

    pub unsafe fn sysctl(args: *mut __sysctl_args) -> k_int {
        call!(cty::__NR_sysctl, args) as k_int
    }

    pub unsafe fn uname(name: *mut old_utsname) -> k_int {
        call!(cty::__NR_uname, name) as k_int
    }

    pub unsafe fn uselib(library: *const c_char) -> k_int {
        call!(cty::__NR_uselib, library) as k_int
    }

    pub unsafe fn waitpid(pid: pid_t, stat_addr: *mut k_int, options: k_int) -> k_int {
        call!(cty::__NR_waitpid, pid, stat_addr, options) as k_int
    }

    pub unsafe fn wait4(upid: pid_t, stat_addr: *mut k_int, options: k_int,
                        ru: *mut rusage) -> pid_t {
        call!(cty::__NR_wait4, upid, stat_addr, options, ru) as pid_t
    }

    pub unsafe fn send(fd: k_int, buff: *mut c_void, len: size_t,
                       flags: k_uint) -> ssize_t {
        call!(cty::__NR_send, fd, buff, len, flags) as ssize_t
    }

    pub unsafe fn recv(fd: k_int, ubuf: *mut c_void, size: size_t,
                       flags: k_uint) -> ssize_t {
        call!(cty::__NR_recv, fd, ubuf, size, flags) as ssize_t
    }
}
