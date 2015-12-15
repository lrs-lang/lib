// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(non_camel_case_types, raw_pointer_derive)]

#[repr(u8)]
#[derive(Copy, Eq)]
pub enum c_void {
    __variant1,
    __variant2,
}

pub type c_char       = u8;
pub type c_schar      = i8;
pub type c_uchar      = u8;
pub type c_short      = i16;
pub type c_ushort     = u16;
pub type c_int        = i32;
pub type c_uint       = u32;
pub type c_long       = i64;
pub type c_ulong      = u64;
pub type c_longlong   = i64;
pub type c_ulonglong  = u64;
pub type c_float      = f32;
pub type c_double     = f64;

// XXX: This is actually configurable and must not be correct. But musl just uses a
// constant too. The real page size is given to programs via the auxiliary vector.
pub const PAGE_SIZE: usize = 4096;

pub const INT_MAX: c_int = c_int::max();

// Bits per k_long, not c_long.
pub const __BITS_PER_LONG : usize = 64;
pub const BYTES_PER_LONG  : usize = 8;
pub const BYTES_PER_SHORT : usize = 2;
pub const BYTES_PER_INT   : usize = 4;

pub type timespec_tv_nsec_type = c_long;

// bitfield manipulation

#[cfg(target_endian = "little")]
pub fn bf32_get(f: u32, start: usize, width: usize) -> u32 {
    (f >> start) & ((1 << width) - 1)
}

#[cfg(target_endian = "little")]
pub fn bf32_set(f: u32, start: usize, width: usize, val: u32) -> u32 {
    let mask = (1 << width) - 1;
    (f & !(mask << start)) | ((val & mask) << start)
}

#[cfg(target_endian = "little")]
pub fn bf64_get(f: u64, start: usize, width: usize) -> u64 {
    (f >> start) & ((1 << width) - 1)
}

#[cfg(target_endian = "little")]
pub fn bf64_set(f: u64, start: usize, width: usize, val: u64) -> u64 {
    let mask = (1 << width) - 1;
    (f & !(mask << start)) | ((val & mask) << start)
}

#[cfg(target_endian = "big")]
pub fn bf32_get(f: u32, start: usize, width: usize) -> u32 {
    (f << start) >> (32 - width)
}

#[cfg(target_endian = "big")]
pub fn bf32_set(f: u32, start: usize, width: usize, val: u32) -> u32 {
    (f & !((!0 << (32 - width)) >> start)) | ((val << (32 - width)) >> start)
}

#[cfg(target_endian = "big")]
pub fn bf64_get(f: u64, start: usize, width: usize) -> u64 {
    (f << start) >> (64 - width)
}

#[cfg(target_endian = "big")]
pub fn bf64_set(f: u64, start: usize, width: usize, val: u64) -> u64 {
    (f & !((!0 << (64 - width)) >> start)) | ((val << (64 - width)) >> start)
}

pub const USER_POINTER_ALIGN : usize = 8;
pub const BITS_PER_C_ULONG : usize = 64;
pub type user_size_t = c_ulong;

/////////////////////////////////
// include/uapi/linux/eventpoll.h
/////////////////////////////////

pub use ::gen::{
    epoll_event,
};

///////////////////////////////
// include/uapi/linux/fadvise.h
///////////////////////////////

pub use ::gen::{
    POSIX_FADV_DONTNEED, POSIX_FADV_NOREUSE,
};

//////////////////////
// include/linux/net.h
//////////////////////

pub use ::gen::{
    SOCK_STREAM, SOCK_DGRAM, SOCK_RAW, SOCK_RDM, SOCK_SEQPACKET, SOCK_DCCP, SOCK_PACKET,
    SOCK_MAX, SOCK_TYPE_MASK, SOCK_CLOEXEC, SOCK_NONBLOCK,
};

//////////////////////////////////////
// arch/arm64/include/uapi/asm/fcntl.h
//////////////////////////////////////

pub const O_DIRECTORY : c_int = 0o40000;
pub const O_NOFOLLOW  : c_int = 0o100000;
pub const O_DIRECT    : c_int = 0o200000;
pub const O_LARGEFILE : c_int = 0o400000;

pub use ::gen::{
    O_ACCMODE, O_RDONLY, O_WRONLY, O_RDWR, O_CREAT, O_EXCL, O_NOCTTY, O_TRUNC, O_APPEND,
    O_NONBLOCK, O_DSYNC, FASYNC,
    O_NOATIME, O_CLOEXEC, __O_SYNC, O_SYNC, O_PATH, __O_TMPFILE, O_TMPFILE,
    O_TMPFILE_MASK, O_NDELAY,

    F_DUPFD, F_GETFD, F_SETFD, F_GETFL, F_SETFL, F_GETLK, F_SETLK, F_SETLKW, F_SETOWN,
    F_GETOWN, F_SETSIG, F_GETSIG, F_GETLK64, F_SETLK64, F_SETLKW64, F_SETOWN_EX,
    F_GETOWN_EX, F_GETOWNER_UIDS, F_OFD_GETLK, F_OFD_SETLK, F_OFD_SETLKW, F_OWNER_TID,
    F_OWNER_PID, F_OWNER_PGRP,

    f_owner_ex,

    FD_CLOEXEC, F_RDLCK, F_WRLCK, F_UNLCK, F_EXLCK, F_SHLCK, LOCK_SH, LOCK_EX, LOCK_NB,
    LOCK_UN, LOCK_MAND, LOCK_READ, LOCK_WRITE, LOCK_RW, F_LINUX_SPECIFIC_BASE,
};

//////////////////////////////////////
// include/uapi/asm-generic/int-ll64.h
//////////////////////////////////////

pub use ::gen::{
    __s8, __u8, __s16, __u16, __s32, __u32, __s64, __u64,
};

////////////////////////////////////
// arch/arm/include/uapi/asm/ioctl.h
////////////////////////////////////

pub use ::gen::{
    _IOC_NRBITS, _IOC_TYPEBITS, _IOC_SIZEBITS, _IOC_DIRBITS, _IOC_NRMASK, _IOC_TYPEMASK,
    _IOC_SIZEMASK, _IOC_DIRMASK, _IOC_NRSHIFT, _IOC_TYPESHIFT, _IOC_SIZESHIFT,
    _IOC_DIRSHIFT, _IOC_NONE, _IOC_WRITE, _IOC_READ,

    _IOC, _IOC_TYPECHECK, _IO, _IOR, _IOW, _IOWR, _IOR_BAD, _IOW_BAD, _IOWR_BAD, _IOC_DIR,
    _IOC_TYPE, _IOC_NR, _IOC_SIZE, IOC_IN, IOC_OUT, IOC_INOUT, IOCSIZE_MASK,
    IOCSIZE_SHIFT,
};

///////////////////////////////////////
// arch/arm64/include/uapi/asm/ioctls.h
///////////////////////////////////////

pub use ::gen::{
    TCGETS, TCSETS, TCSETSW, TCSETSF, TCGETA, TCSETA, TCSETAW, TCSETAF, TCSBRK, TCXONC,
    TCFLSH, TIOCEXCL, TIOCNXCL, TIOCSCTTY, TIOCGPGRP, TIOCSPGRP, TIOCOUTQ, TIOCSTI,
    TIOCGWINSZ, TIOCSWINSZ, TIOCMGET, TIOCMBIS, TIOCMBIC, TIOCMSET, TIOCGSOFTCAR,
    TIOCSSOFTCAR, FIONREAD, TIOCINQ, TIOCLINUX, TIOCCONS, TIOCGSERIAL, TIOCSSERIAL,
    TIOCPKT, FIONBIO, TIOCNOTTY, TIOCSETD, TIOCGETD, TCSBRKP, TIOCSBRK, TIOCCBRK,
    TIOCGSID, TIOCGRS485, TIOCSRS485, TCGETX, TCSETX, TCSETXF, TCSETXW, TIOCVHANGUP,
    FIONCLEX, FIOCLEX, FIOASYNC, TIOCSERCONFIG, TIOCSERGWILD, TIOCSERSWILD,
    TIOCGLCKTRMIOS, TIOCSLCKTRMIOS, TIOCSERGSTRUCT, TIOCSERGETLSR, TIOCSERGETMULTI,
    TIOCSERSETMULTI, TIOCMIWAIT, TIOCGICOUNT, FIOQSIZE, TIOCPKT_DATA, TIOCPKT_FLUSHREAD,
    TIOCPKT_FLUSHWRITE, TIOCPKT_STOP, TIOCPKT_START, TIOCPKT_NOSTOP, TIOCPKT_DOSTOP,
    TIOCPKT_IOCTL, TIOCSER_TEMT,

    TCGETS2, TCSETS2, TCSETSW2, TCSETSF2, TIOCGPTN, TIOCSPTLCK, TIOCGDEV, TIOCSIG,
    TIOCGPKT, TIOCGPTLCK, TIOCGEXCL,
};

///////////////////////////////////////
// arch/arm64/include/uapi/asm/ipcbuf.h
///////////////////////////////////////

pub use ::gen::{
    ipc64_perm,
};

/////////////////////////////////////////
// include/uapi/asm-generic/mman-common.h
/////////////////////////////////////////

pub use gen::{
    PROT_READ, PROT_WRITE, PROT_EXEC, PROT_SEM, PROT_NONE, PROT_GROWSDOWN, PROT_GROWSUP,
    MAP_SHARED, MAP_PRIVATE, MAP_TYPE, MAP_FIXED, MAP_ANONYMOUS, MAP_UNINITIALIZED,
    MS_ASYNC, MS_INVALIDATE, MS_SYNC, MADV_NORMAL, MADV_RANDOM, MADV_SEQUENTIAL,
    MADV_WILLNEED, MADV_DONTNEED, MADV_REMOVE, MADV_DONTFORK, MADV_DOFORK, MADV_HWPOISON,
    MADV_SOFT_OFFLINE, MADV_MERGEABLE, MADV_UNMERGEABLE, MADV_HUGEPAGE, MADV_NOHUGEPAGE,
    MADV_DONTDUMP, MADV_DODUMP, MAP_FILE, MAP_HUGE_SHIFT, MAP_HUGE_MASK,
};

/////////////////////////////////////
// arch/arm64/include/uapi/asm/mman.h
/////////////////////////////////////

pub use gen::{
    MAP_GROWSDOWN, MAP_DENYWRITE, MAP_EXECUTABLE, MAP_LOCKED, MAP_NORESERVE, MAP_POPULATE,
    MAP_NONBLOCK, MAP_STACK, MAP_HUGETLB, MCL_CURRENT, MCL_FUTURE,
};

///////////////////////////////////////
// arch/arm64/include/uapi/asm/msgbuf.h
///////////////////////////////////////

#[repr(C)]
#[derive(Pod, Eq)]
pub struct msqid64_ds {
    pub msg_perm:   ipc64_perm,
    pub msg_stime:  __kernel_time_t,
    pub msg_rtime:  __kernel_time_t,
    pub msg_ctime:  __kernel_time_t,
    pub msg_cbytes: __kernel_ulong_t,
    pub msg_qnum:   __kernel_ulong_t,
    pub msg_qbytes: __kernel_ulong_t,
    pub msg_lspid:  __kernel_pid_t,
    pub msg_lrpid:  __kernel_pid_t,
    pub __unused4:  __kernel_ulong_t,
    pub __unused5:  __kernel_ulong_t,
}

//////////////////////////////////////
// arch/arm64/include/uapi/asm/param.h
//////////////////////////////////////

pub const EXEC_PAGESIZE  : usize = 65536;

pub use gen::{
    HZ, NOGROUP, MAXHOSTNAMELEN,
};

/////////////////////////////////////
// arch/arm64/include/uapi/asm/poll.h
/////////////////////////////////////

pub use ::gen::{
    POLLIN, POLLPRI, POLLOUT, POLLERR, POLLHUP, POLLNVAL, POLLRDNORM, POLLRDBAND,
    POLLWRNORM, POLLWRBAND, POLLMSG, POLLREMOVE, POLLRDHUP, POLLFREE, POLL_BUSY_LOOP,
    pollfd,
};

////////////////////////////////////////////
// arch/arm64/include/uapi/asm/posix_types.h
////////////////////////////////////////////

pub type __kernel_old_uid_t = c_ushort;
pub type __kernel_old_gid_t = c_ushort;

pub use ::gen::{
    __kernel_long_t, __kernel_ulong_t, __kernel_ino_t, __kernel_mode_t, __kernel_pid_t,
    __kernel_ipc_pid_t, __kernel_uid_t, __kernel_gid_t, __kernel_suseconds_t,
    __kernel_daddr_t, __kernel_uid32_t, __kernel_gid32_t, __kernel_old_dev_t,
    __kernel_off_t, __kernel_loff_t, __kernel_time_t, __kernel_clock_t, __kernel_timer_t,
    __kernel_clockid_t, __kernel_caddr_t, __kernel_uid16_t, __kernel_gid16_t,
    __kernel_fsid_t, BYTES_PER_KERNEL_MODE_T,
};

pub type __kernel_size_t    = __kernel_ulong_t;
pub type __kernel_ssize_t   = __kernel_long_t;
pub type __kernel_ptrdiff_t = __kernel_long_t;

/////////////////////////////////////////
// arch/arm64/include/uapi/asm/resource.h
/////////////////////////////////////////

pub use ::gen::{
    RLIMIT_CPU, RLIMIT_FSIZE, RLIMIT_DATA, RLIMIT_STACK, RLIMIT_CORE, RLIMIT_RSS,
    RLIMIT_NPROC, RLIMIT_NOFILE, RLIMIT_MEMLOCK, RLIMIT_AS, RLIMIT_LOCKS,
    RLIMIT_SIGPENDING, RLIMIT_MSGQUEUE, RLIMIT_NICE, RLIMIT_RTPRIO, RLIMIT_RTTIME,
    RLIM_NLIMITS, RLIM_INFINITY,
};

///////////////////////////////////////
// arch/arm64/include/uapi/asm/sembuf.h
///////////////////////////////////////

#[repr(C)]
#[derive(Pod, Eq)]
pub struct semid64_ds {
    pub sem_perm:  ipc64_perm,
    pub sem_otime: __kernel_time_t,
    pub sem_ctime: __kernel_time_t,
    pub sem_nsems: c_ulong,
    pub __unused3: c_ulong,
    pub __unused4: c_ulong,
}

//////////////////////////////////////
// arch/arm64/include/uapi/asm/setup.h
//////////////////////////////////////

pub const COMMAND_LINE_SIZE : usize = 2048;

///////////////////////////////////////
// arch/arm64/include/uapi/asm/shmbuf.h
///////////////////////////////////////

#[repr(C)]
#[derive(Pod, Eq)]
pub struct shmid64_ds {
    pub shm_perm:   ipc64_perm,
    pub shm_segsz:  __kernel_size_t, // XXX: was: size_t
    pub shm_atime:  __kernel_time_t,
    pub shm_dtime:  __kernel_time_t,
    pub shm_ctime:  __kernel_time_t,
    pub shm_cpid:   __kernel_pid_t,
    pub shm_lpid:   __kernel_pid_t,
    pub shm_nattch: __kernel_ulong_t,
    pub __unused4:  __kernel_ulong_t,
    pub __unused5:  __kernel_ulong_t,
}

pub use ::gen::{
    shminfo64,
};

////////////////////////////////////
// arch/arm64/include/asm/shmparam.h
////////////////////////////////////

pub use ::gen::{
    SHMLBA,
};

////////////////////////////////////////
// arch/arm64/include/uapi/asm/siginfo.h
////////////////////////////////////////

#[repr(C)]
#[derive(Pod, Eq)]
pub struct siginfo_t {
    data: [u64; SI_MAX_SIZE / 8],
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct siginfo_sigfault {
    pub _addr: *mut c_void,
    pub _addr_lsb: c_short,
    pub _addr_bnd: siginfo_addr_bnd,
}

pub const __ARCH_SI_PREAMBLE_SIZE: usize = 4 * ::BYTES_PER_INT;

pub use ::gen::{
    __ARCH_SI_CLOCK_T,
    sigval_t, SI_MAX_SIZE, SI_PAD_SIZE, __ARCH_SI_UID_T, __ARCH_SI_BAND_T,
    siginfo_kill, BYTES_PER_ARCH_SI_UID_T, siginfo_timer, siginfo_rt,
    siginfo_sigchld, siginfo_addr_bnd, siginfo_sigpoll, siginfo_sigsys, SI_USER,
    SI_KERNEL, SI_QUEUE, SI_TIMER, SI_MESGQ, SI_ASYNCIO, SI_SIGIO, SI_TKILL, SI_DETHREAD,
    ILL_ILLOPC, ILL_ILLOPN, ILL_ILLADR, ILL_ILLTRP, ILL_PRVOPC, ILL_PRVREG, ILL_COPROC,
    ILL_BADSTK, NSIGILL, FPE_INTDIV, FPE_INTOVF, FPE_FLTDIV, FPE_FLTOVF, FPE_FLTUND,
    FPE_FLTRES, FPE_FLTINV, FPE_FLTSUB, NSIGFPE, SEGV_MAPERR, SEGV_ACCERR, SEGV_BNDERR,
    BUS_ADRALN, BUS_ADRERR, BUS_OBJERR, BUS_MCEERR_AR, BUS_MCEERR_AO, TRAP_BRKPT,
    TRAP_TRACE, TRAP_BRANCH, TRAP_HWBKPT, NSIGTRAP, CLD_EXITED, CLD_KILLED, CLD_DUMPED,
    CLD_TRAPPED, CLD_STOPPED, CLD_CONTINUED, NSIGCHLD, POLL_IN, POLL_OUT, POLL_MSG,
    POLL_ERR, POLL_PRI, POLL_HUP, NSIGPOLL, SYS_SECCOMP, NSIGSYS, SIGEV_SIGNAL,
    SIGEV_NONE, SIGEV_THREAD, SIGEV_THREAD_ID,
};

/////////////////////////////////////////
// include/uapi/asm-generic/signal-defs.h
/////////////////////////////////////////

pub use ::gen::{
    SIG_BLOCK, SIG_UNBLOCK, SIG_SETMASK, __sighandler_t, __sigrestore_t, SIG_DFL, SIG_IGN,
    SIG_ERR,
};

///////////////////////////////////////
// arch/arm64/include/uapi/asm/signal.h
///////////////////////////////////////

pub const SA_RESTORER : c_int = 0x04000000;

pub use ::gen::{
    _NSIG, _NSIG_BPW, _NSIG_WORDS, SIGHUP, SIGINT, SIGQUIT, SIGILL, SIGTRAP, SIGABRT,
    SIGIOT, SIGBUS, SIGFPE, SIGKILL, SIGUSR1, SIGSEGV, SIGUSR2, SIGPIPE, SIGALRM, SIGTERM,
    SIGSTKFLT, SIGCHLD, SIGCONT, SIGSTOP, SIGTSTP, SIGTTIN, SIGTTOU, SIGURG, SIGXCPU,
    SIGXFSZ, SIGVTALRM, SIGPROF, SIGWINCH, SIGIO, SIGPOLL, SIGPWR, SIGSYS, SIGUNUSED,
    SA_NOCLDSTOP, SA_NOCLDWAIT, SA_SIGINFO, SA_ONSTACK, SA_RESTART, SA_NODEFER,
    SA_RESETHAND, SA_NOMASK, SA_ONESHOT, MINSIGSTKSZ, SIGSTKSZ, SigsetVal, sigset_t,
    old_sigset_t, sigaltstack, stack_t, sigaction,
};

///////////////////////////////////////
// arch/arm64/include/uapi/asm/socket.h
///////////////////////////////////////

pub use ::gen::{
    SOL_SOCKET, SO_DEBUG, SO_REUSEADDR, SO_TYPE, SO_ERROR, SO_DONTROUTE, SO_BROADCAST,
    SO_SNDBUF, SO_RCVBUF, SO_SNDBUFFORCE, SO_RCVBUFFORCE, SO_KEEPALIVE, SO_OOBINLINE,
    SO_NO_CHECK, SO_PRIORITY, SO_LINGER, SO_BSDCOMPAT, SO_REUSEPORT, SO_PASSCRED,
    SO_PEERCRED, SO_RCVLOWAT, SO_SNDLOWAT, SO_RCVTIMEO, SO_SNDTIMEO,
    SO_SECURITY_AUTHENTICATION, SO_SECURITY_ENCRYPTION_TRANSPORT,
    SO_SECURITY_ENCRYPTION_NETWORK, SO_BINDTODEVICE, SO_ATTACH_FILTER, SO_DETACH_FILTER,
    SO_GET_FILTER, SO_PEERNAME, SO_TIMESTAMP, SCM_TIMESTAMP, SO_ACCEPTCONN, SO_PEERSEC,
    SO_PASSSEC, SO_TIMESTAMPNS, SCM_TIMESTAMPNS, SO_MARK, SO_TIMESTAMPING,
    SCM_TIMESTAMPING, SO_PROTOCOL, SO_DOMAIN, SO_RXQ_OVFL, SO_WIFI_STATUS,
    SCM_WIFI_STATUS, SO_PEEK_OFF, SO_NOFCS, SO_LOCK_FILTER, SO_SELECT_ERR_QUEUE,
    SO_BUSY_POLL, SO_MAX_PACING_RATE, SO_BPF_EXTENSIONS,
};

////////////////////////////////////////
// arch/arm64/include/uapi/asm/sockios.h
////////////////////////////////////////

pub use ::gen::{
    FIOSETOWN, SIOCSPGRP, FIOGETOWN, SIOCGPGRP, SIOCATMARK, SIOCGSTAMP, SIOCGSTAMPNS,
};

///////////////////////////////////////
// arch/arm64/include/uapi/asm/statfs.h
///////////////////////////////////////

pub type __statfs_word = __kernel_long_t;

// These are not defined in the kernel uapi but can be derived from the stat struct
pub type __fsword_t = __statfs_word;
pub type fsblkcnt_t = __statfs_word;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct statfs64 {
    pub f_type:     __statfs_word,
    pub f_bsize:    __statfs_word,
    pub f_blocks:   __u64,
    pub f_bfree:    __u64,
    pub f_bavail:   __u64,
    pub f_files:    __u64,
    pub f_ffree:    __u64,
    pub f_fsid:     __kernel_fsid_t,
    pub f_namelen:  __statfs_word,
    pub f_frsize:   __statfs_word,
    pub f_flags:    __statfs_word,
    pub f_spare: [__statfs_word; 4],
}

pub use ::gen::{
    statfs,
};

/////////////////////////////////////
// arch/arm64/include/uapi/asm/stat.h
/////////////////////////////////////

pub use ::gen::{
    STAT_HAVE_NSEC, stat,
};

/////////////////////////////////////////
// arch/arm64/include/uapi/asm/termbits.h
/////////////////////////////////////////

pub use ::gen::{
    cc_t, speed_t, tcflag_t, NCCS, termios, termios2, ktermios,

    VINTR, VQUIT, VERASE, VKILL, VEOF, VTIME, VMIN, VSWTC, VSTART, VSTOP, VSUSP, VEOL,
    VREPRINT, VDISCARD, VWERASE, VLNEXT, VEOL2, IGNBRK, BRKINT, IGNPAR, PARMRK, INPCK,
    ISTRIP, INLCR, IGNCR, ICRNL, IUCLC, IXON, IXANY, IXOFF, IMAXBEL, IUTF8, OPOST, OLCUC,
    ONLCR, OCRNL, ONOCR, ONLRET, OFILL, OFDEL, NLDLY, NL0, NL1, CRDLY, CR0, CR1, CR2, CR3,
    TABDLY, TAB0, TAB1, TAB2, TAB3, XTABS, BSDLY, BS0, BS1, VTDLY, VT0, VT1, FFDLY, FF0,
    FF1, CBAUD, B0, B50, B75, B110, B134, B150, B200, B300, B600, B1200, B1800, B2400,
    B4800, B9600, B19200, B38400, EXTA, EXTB, CSIZE, CS5, CS6, CS7, CS8, CSTOPB, CREAD,
    PARENB, PARODD, HUPCL, CLOCAL, CBAUDEX, BOTHER, B57600, B115200, B230400, B460800,
    B500000, B576000, B921600, B1000000, B1152000, B1500000, B2000000, B2500000, B3000000,
    B3500000, B4000000, CIBAUD, CMSPAR, CRTSCTS, IBSHIFT, ISIG, ICANON, XCASE, ECHO,
    ECHOE, ECHOK, ECHONL, NOFLSH, TOSTOP, ECHOCTL, ECHOPRT, ECHOKE, FLUSHO, PENDIN,
    IEXTEN, EXTPROC, TCOOFF, TCOON, TCIOFF, TCION, TCIFLUSH, TCOFLUSH, TCIOFLUSH, TCSANOW,
    TCSADRAIN, TCSAFLUSH,
};

////////////////////////////////////////
// arch/arm64/include/uapi/asm/termios.h
////////////////////////////////////////

pub use ::gen::{
    winsize, NCC, termio, TIOCM_LE, TIOCM_DTR, TIOCM_RTS, TIOCM_ST, TIOCM_SR, TIOCM_CTS,
    TIOCM_CAR, TIOCM_RNG, TIOCM_DSR, TIOCM_CD, TIOCM_RI, TIOCM_OUT1, TIOCM_OUT2,
    TIOCM_LOOP,
};

/////////////////////////////////////////
// arch/arm64/include/uapi/asm/ucontext.h
/////////////////////////////////////////

// const SIZE_OF_SIGSET: usize = 8;
//
// #[repr(C)]
// #[derive(Pod, Eq)]
// pub struct ucontext {
//     pub uc_flags:    c_ulong,
//     pub uc_link:     *mut ucontext,
//     pub uc_stack:    stack_t,
//     pub uc_sigmask:  sigset_t,
//     pub __unused:    [__u8; 1024 / 8 - SIZE_OF_SIGSET],
//     pub uc_mcontext: sigcontext,
// }

///////////////////////////////
// include/uapi/linux/sysinfo.h
///////////////////////////////

pub const SYSINFO_PADDING: usize = 0;

//////////////////////////////////
// include/uapi/asm-generic/uapi.h
//////////////////////////////////

pub const __NR_io_setup               : usize = 0;
pub const __NR_io_destroy             : usize = 1;
pub const __NR_io_submit              : usize = 2;
pub const __NR_io_cancel              : usize = 3;
pub const __NR_io_getevents           : usize = 4;
pub const __NR_setxattr               : usize = 5;
pub const __NR_lsetxattr              : usize = 6;
pub const __NR_fsetxattr              : usize = 7;
pub const __NR_getxattr               : usize = 8;
pub const __NR_lgetxattr              : usize = 9;
pub const __NR_fgetxattr              : usize = 10;
pub const __NR_listxattr              : usize = 11;
pub const __NR_llistxattr             : usize = 12;
pub const __NR_flistxattr             : usize = 13;
pub const __NR_removexattr            : usize = 14;
pub const __NR_lremovexattr           : usize = 15;
pub const __NR_fremovexattr           : usize = 16;
pub const __NR_getcwd                 : usize = 17;
pub const __NR_lookup_dcookie         : usize = 18;
pub const __NR_eventfd2               : usize = 19;
pub const __NR_epoll_create1          : usize = 20;
pub const __NR_epoll_ctl              : usize = 21;
pub const __NR_epoll_pwait            : usize = 22;
pub const __NR_dup                    : usize = 23;
pub const __NR_dup3                   : usize = 24;
pub const __NR_fcntl                  : usize = 25;
pub const __NR_inotify_init1          : usize = 26;
pub const __NR_inotify_add_watch      : usize = 27;
pub const __NR_inotify_rm_watch       : usize = 28;
pub const __NR_ioctl                  : usize = 29;
pub const __NR_ioprio_set             : usize = 30;
pub const __NR_ioprio_get             : usize = 31;
pub const __NR_flock                  : usize = 32;
pub const __NR_mknodat                : usize = 33;
pub const __NR_mkdirat                : usize = 34;
pub const __NR_unlinkat               : usize = 35;
pub const __NR_symlinkat              : usize = 36;
pub const __NR_linkat                 : usize = 37;
pub const __NR_renameat               : usize = 38;
pub const __NR_umount                 : usize = 39;
pub const __NR_mount                  : usize = 40;
pub const __NR_pivot_root             : usize = 41;
pub const __NR_ni_syscall             : usize = 42;
pub const __NR_statfs                 : usize = 43;
pub const __NR_fstatfs                : usize = 44;
pub const __NR_truncate               : usize = 45;
pub const __NR_ftruncate              : usize = 46;
pub const __NR_fallocate              : usize = 47;
pub const __NR_faccessat              : usize = 48;
pub const __NR_chdir                  : usize = 49;
pub const __NR_fchdir                 : usize = 50;
pub const __NR_chroot                 : usize = 51;
pub const __NR_fchmod                 : usize = 52;
pub const __NR_fchmodat               : usize = 53;
pub const __NR_fchownat               : usize = 54;
pub const __NR_fchown                 : usize = 55;
pub const __NR_openat                 : usize = 56;
pub const __NR_close                  : usize = 57;
pub const __NR_vhangup                : usize = 58;
pub const __NR_pipe2                  : usize = 59;
pub const __NR_quotactl               : usize = 60;
pub const __NR_getdents64             : usize = 61;
pub const __NR_lseek                  : usize = 62;
pub const __NR_read                   : usize = 63;
pub const __NR_write                  : usize = 64;
pub const __NR_readv                  : usize = 65;
pub const __NR_writev                 : usize = 66;
pub const __NR_pread64                : usize = 67;
pub const __NR_pwrite64               : usize = 68;
pub const __NR_preadv                 : usize = 69;
pub const __NR_pwritev                : usize = 70;
pub const __NR_sendfile64             : usize = 71;
pub const __NR_pselect6               : usize = 72;
pub const __NR_ppoll                  : usize = 73;
pub const __NR_signalfd4              : usize = 74;
pub const __NR_vmsplice               : usize = 75;
pub const __NR_splice                 : usize = 76;
pub const __NR_tee                    : usize = 77;
pub const __NR_readlinkat             : usize = 78;
pub const __NR_newfstatat             : usize = 79;
pub const __NR_newfstat               : usize = 80;
pub const __NR_sync                   : usize = 81;
pub const __NR_fsync                  : usize = 82;
pub const __NR_fdatasync              : usize = 83;
pub const __NR_sync_file_range        : usize = 84;
pub const __NR_timerfd_create         : usize = 85;
pub const __NR_timerfd_settime        : usize = 86;
pub const __NR_timerfd_gettime        : usize = 87;
pub const __NR_utimensat              : usize = 88;
pub const __NR_acct                   : usize = 89;
pub const __NR_capget                 : usize = 90;
pub const __NR_capset                 : usize = 91;
pub const __NR_personality            : usize = 92;
pub const __NR_exit                   : usize = 93;
pub const __NR_exit_group             : usize = 94;
pub const __NR_waitid                 : usize = 95;
pub const __NR_set_tid_address        : usize = 96;
pub const __NR_unshare                : usize = 97;
pub const __NR_futex                  : usize = 98;
pub const __NR_set_robust_list        : usize = 99;
pub const __NR_get_robust_list        : usize = 100;
pub const __NR_nanosleep              : usize = 101;
pub const __NR_getitimer              : usize = 102;
pub const __NR_setitimer              : usize = 103;
pub const __NR_kexec_load             : usize = 104;
pub const __NR_init_module            : usize = 105;
pub const __NR_delete_module          : usize = 106;
pub const __NR_timer_create           : usize = 107;
pub const __NR_timer_gettime          : usize = 108;
pub const __NR_timer_getoverrun       : usize = 109;
pub const __NR_timer_settime          : usize = 110;
pub const __NR_timer_delete           : usize = 111;
pub const __NR_clock_settime          : usize = 112;
pub const __NR_clock_gettime          : usize = 113;
pub const __NR_clock_getres           : usize = 114;
pub const __NR_clock_nanosleep        : usize = 115;
pub const __NR_syslog                 : usize = 116;
pub const __NR_ptrace                 : usize = 117;
pub const __NR_sched_setparam         : usize = 118;
pub const __NR_sched_setscheduler     : usize = 119;
pub const __NR_sched_getscheduler     : usize = 120;
pub const __NR_sched_getparam         : usize = 121;
pub const __NR_sched_setaffinity      : usize = 122;
pub const __NR_sched_getaffinity      : usize = 123;
pub const __NR_sched_yield            : usize = 124;
pub const __NR_sched_get_priority_max : usize = 125;
pub const __NR_sched_get_priority_min : usize = 126;
pub const __NR_sched_rr_get_interval  : usize = 127;
pub const __NR_restart_syscall        : usize = 128;
pub const __NR_kill                   : usize = 129;
pub const __NR_tkill                  : usize = 130;
pub const __NR_tgkill                 : usize = 131;
pub const __NR_sigaltstack            : usize = 132;
pub const __NR_rt_sigsuspend          : usize = 133;
pub const __NR_rt_sigaction           : usize = 134;
pub const __NR_rt_sigprocmask         : usize = 135;
pub const __NR_rt_sigpending          : usize = 136;
pub const __NR_rt_sigtimedwait        : usize = 137;
pub const __NR_rt_sigqueueinfo        : usize = 138;
pub const __NR_rt_sigreturn           : usize = 139;
pub const __NR_setpriority            : usize = 140;
pub const __NR_getpriority            : usize = 141;
pub const __NR_reboot                 : usize = 142;
pub const __NR_setregid               : usize = 143;
pub const __NR_setgid                 : usize = 144;
pub const __NR_setreuid               : usize = 145;
pub const __NR_setuid                 : usize = 146;
pub const __NR_setresuid              : usize = 147;
pub const __NR_getresuid              : usize = 148;
pub const __NR_setresgid              : usize = 149;
pub const __NR_getresgid              : usize = 150;
pub const __NR_setfsuid               : usize = 151;
pub const __NR_setfsgid               : usize = 152;
pub const __NR_times                  : usize = 153;
pub const __NR_setpgid                : usize = 154;
pub const __NR_getpgid                : usize = 155;
pub const __NR_getsid                 : usize = 156;
pub const __NR_setsid                 : usize = 157;
pub const __NR_getgroups              : usize = 158;
pub const __NR_setgroups              : usize = 159;
pub const __NR_newuname               : usize = 160;
pub const __NR_sethostname            : usize = 161;
pub const __NR_setdomainname          : usize = 162;
pub const __NR_getrlimit              : usize = 163;
pub const __NR_setrlimit              : usize = 164;
pub const __NR_getrusage              : usize = 165;
pub const __NR_umask                  : usize = 166;
pub const __NR_prctl                  : usize = 167;
pub const __NR_getcpu                 : usize = 168;
pub const __NR_gettimeofday           : usize = 169;
pub const __NR_settimeofday           : usize = 170;
pub const __NR_adjtimex               : usize = 171;
pub const __NR_getpid                 : usize = 172;
pub const __NR_getppid                : usize = 173;
pub const __NR_getuid                 : usize = 174;
pub const __NR_geteuid                : usize = 175;
pub const __NR_getgid                 : usize = 176;
pub const __NR_getegid                : usize = 177;
pub const __NR_gettid                 : usize = 178;
pub const __NR_sysinfo                : usize = 179;
pub const __NR_mq_open                : usize = 180;
pub const __NR_mq_unlink              : usize = 181;
pub const __NR_mq_timedsend           : usize = 182;
pub const __NR_mq_timedreceive        : usize = 183;
pub const __NR_mq_notify              : usize = 184;
pub const __NR_mq_getsetattr          : usize = 185;
pub const __NR_msgget                 : usize = 186;
pub const __NR_msgctl                 : usize = 187;
pub const __NR_msgrcv                 : usize = 188;
pub const __NR_msgsnd                 : usize = 189;
pub const __NR_semget                 : usize = 190;
pub const __NR_semctl                 : usize = 191;
pub const __NR_semtimedop             : usize = 192;
pub const __NR_semop                  : usize = 193;
pub const __NR_shmget                 : usize = 194;
pub const __NR_shmctl                 : usize = 195;
pub const __NR_shmat                  : usize = 196;
pub const __NR_shmdt                  : usize = 197;
pub const __NR_socket                 : usize = 198;
pub const __NR_socketpair             : usize = 199;
pub const __NR_bind                   : usize = 200;
pub const __NR_listen                 : usize = 201;
pub const __NR_accept                 : usize = 202;
pub const __NR_connect                : usize = 203;
pub const __NR_getsockname            : usize = 204;
pub const __NR_getpeername            : usize = 205;
pub const __NR_sendto                 : usize = 206;
pub const __NR_recvfrom               : usize = 207;
pub const __NR_setsockopt             : usize = 208;
pub const __NR_getsockopt             : usize = 209;
pub const __NR_shutdown               : usize = 210;
pub const __NR_sendmsg                : usize = 211;
pub const __NR_recvmsg                : usize = 212;
pub const __NR_readahead              : usize = 213;
pub const __NR_brk                    : usize = 214;
pub const __NR_munmap                 : usize = 215;
pub const __NR_mremap                 : usize = 216;
pub const __NR_add_key                : usize = 217;
pub const __NR_request_key            : usize = 218;
pub const __NR_keyctl                 : usize = 219;
pub const __NR_clone                  : usize = 220;
pub const __NR_execve                 : usize = 221;
pub const __NR_mmap                   : usize = 222;
pub const __NR_fadvise64_64           : usize = 223;
pub const __NR_swapon                 : usize = 224;
pub const __NR_swapoff                : usize = 225;
pub const __NR_mprotect               : usize = 226;
pub const __NR_msync                  : usize = 227;
pub const __NR_mlock                  : usize = 228;
pub const __NR_munlock                : usize = 229;
pub const __NR_mlockall               : usize = 230;
pub const __NR_munlockall             : usize = 231;
pub const __NR_mincore                : usize = 232;
pub const __NR_madvise                : usize = 233;
pub const __NR_remap_file_pages       : usize = 234;
pub const __NR_mbind                  : usize = 235;
pub const __NR_get_mempolicy          : usize = 236;
pub const __NR_set_mempolicy          : usize = 237;
pub const __NR_migrate_pages          : usize = 238;
pub const __NR_move_pages             : usize = 239;
pub const __NR_rt_tgsigqueueinfo      : usize = 240;
pub const __NR_perf_event_open        : usize = 241;
pub const __NR_accept4                : usize = 242;
pub const __NR_recvmmsg               : usize = 243;
pub const __NR_wait4                  : usize = 260;
pub const __NR_prlimit64              : usize = 261;
pub const __NR_fanotify_init          : usize = 262;
pub const __NR_fanotify_mark          : usize = 263;
pub const __NR_name_to_handle_at      : usize = 264;
pub const __NR_open_by_handle_at      : usize = 265;
pub const __NR_clock_adjtime          : usize = 266;
pub const __NR_syncfs                 : usize = 267;
pub const __NR_setns                  : usize = 268;
pub const __NR_sendmmsg               : usize = 269;
pub const __NR_process_vm_readv       : usize = 270;
pub const __NR_process_vm_writev      : usize = 271;
pub const __NR_kcmp                   : usize = 272;
pub const __NR_finit_module           : usize = 273;
pub const __NR_sched_setattr          : usize = 274;
pub const __NR_sched_getattr          : usize = 275;
pub const __NR_renameat2              : usize = 276;
pub const __NR_seccomp                : usize = 277;
pub const __NR_getrandom              : usize = 278;
pub const __NR_memfd_create           : usize = 279;
pub const __NR_bpf                    : usize = 280;
pub const __NR_execveat               : usize = 281;

///////////////////////////////////////
// arch/arm64/include/uapi/asm/auxvec.h
///////////////////////////////////////

pub const AT_SYSINFO_EHDR : usize = 33;

/// The largest AT_* value plus one.
pub const AUX_CNT : usize = 34;

///////////////////////////
// include/uapi/linux/elf.h
///////////////////////////

pub type ElfDyn  = ::Elf64_Dyn;
pub type ElfRel  = ::Elf64_Rel;
pub type ElfRela = ::Elf64_Rela;
pub type ElfSym  = ::Elf64_Sym;
pub type ElfEhdr = ::Elf64_Ehdr;
pub type ElfPhdr = ::Elf64_Phdr;
pub type ElfShdr = ::Elf64_Shdr;
pub type ElfNhdr = ::Elf64_Nhdr;
