// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(non_camel_case_types, raw_pointer_derive)]

#[cfg(target_pointer_width = "32")]
#[path = "x32.rs"]
mod abi;

#[cfg(target_pointer_width = "64")]
#[path = "x64.rs"]
mod abi;

#[repr(u8)]
#[derive(Copy, Eq)]
pub enum c_void {
    __variant1,
    __variant2,
}

pub type c_char       = i8;
pub type c_schar      = i8;
pub type c_uchar      = u8;
pub type c_short      = i16;
pub type c_ushort     = u16;
pub type c_int        = i32;
pub type c_uint       = u32;
pub type c_longlong   = i64;
pub type c_ulonglong  = u64;
pub type c_float      = f32;
pub type c_double     = f64;

pub use self::abi::{
    c_long, c_ulong,
};

// XXX: This is actually configurable and must not be correct. But musl just uses a
// constant too. The real page size is given to programs via the auxiliary vector.
pub const PAGE_SIZE: usize = 4096;

pub const INT_MAX: c_int = i32::max();

// Bits per k_long, not c_long.
pub const __BITS_PER_LONG : usize = 64;
pub const BYTES_PER_LONG  : usize = 8;
pub const BYTES_PER_SHORT : usize = 2;
pub const BYTES_PER_INT   : usize = 4;

// We have to define this type because x32 doesn't use a compat layer.
pub type timespec_tv_nsec_type = __kernel_long_t;

// bitfield manipulation

pub fn bf32_get(f: u32, start: usize, width: usize) -> u32 {
    (f >> start) & ((1 << width) - 1)
}

pub fn bf32_set(f: u32, start: usize, width: usize, val: u32) -> u32 {
    let mask = (1 << width) - 1;
    (f & !(mask << start)) | ((val & mask) << start)
}

pub fn bf64_get(f: u64, start: usize, width: usize) -> u64 {
    (f >> start) & ((1 << width) - 1)
}

pub fn bf64_set(f: u64, start: usize, width: usize, val: u64) -> u64 {
    let mask = (1 << width) - 1;
    (f & !(mask << start)) | ((val & mask) << start)
}

pub use self::abi::{
    USER_POINTER_ALIGN, BITS_PER_C_ULONG,
};

pub use self::abi::{
    user_size_t,
};

/////////////////////////////////
// include/uapi/linux/eventpoll.h
/////////////////////////////////

#[repr(C, packed)]
#[derive(Pod, Eq)]
pub struct epoll_event {
    pub events: __u32,
    pub data:   __u64,
}

///////////////////////////////
// include/uapi/linux/fadvise.h
///////////////////////////////

pub use ::gen::{
    POSIX_FADV_DONTNEED, POSIX_FADV_NOREUSE,
};

///////////////////////////
// include/uapi/linux/net.h
///////////////////////////

pub use gen::{
    SOCK_STREAM, SOCK_DGRAM, SOCK_RAW, SOCK_RDM, SOCK_SEQPACKET, SOCK_DCCP, SOCK_PACKET,
    SOCK_MAX, SOCK_TYPE_MASK, SOCK_CLOEXEC, SOCK_NONBLOCK,
};

////////////////////////////////////
// arch/x86/include/uapi/asm/fcntl.h
////////////////////////////////////

pub use ::gen::{
    O_ACCMODE, O_RDONLY, O_WRONLY, O_RDWR, O_CREAT, O_EXCL, O_NOCTTY, O_TRUNC, O_APPEND,
    O_NONBLOCK, O_DSYNC, FASYNC, O_DIRECT, O_LARGEFILE, O_DIRECTORY, O_NOFOLLOW,
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
// arch/x86/include/uapi/asm/ioctl.h
////////////////////////////////////

pub use ::gen::{
    _IOC_NRBITS, _IOC_TYPEBITS, _IOC_SIZEBITS, _IOC_DIRBITS, _IOC_NRMASK, _IOC_TYPEMASK,
    _IOC_SIZEMASK, _IOC_DIRMASK, _IOC_NRSHIFT, _IOC_TYPESHIFT, _IOC_SIZESHIFT,
    _IOC_DIRSHIFT, _IOC_NONE, _IOC_WRITE, _IOC_READ,

    _IOC, _IOC_TYPECHECK, _IO, _IOR, _IOW, _IOWR, _IOR_BAD, _IOW_BAD, _IOWR_BAD, _IOC_DIR,
    _IOC_TYPE, _IOC_NR, _IOC_SIZE, IOC_IN, IOC_OUT, IOC_INOUT, IOCSIZE_MASK,
    IOCSIZE_SHIFT,
};

/////////////////////////////////////
// arch/x86/include/uapi/asm/ioctls.h
/////////////////////////////////////

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

/////////////////////////////////////
// arch/x86/include/uapi/asm/ipcbuf.h
/////////////////////////////////////

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

///////////////////////////////////
// arch/x86/include/uapi/asm/mman.h
///////////////////////////////////

pub const MAP_32BIT    : ::c_int = 0x40;
pub const MAP_HUGE_2MB : ::c_int = 21 << MAP_HUGE_SHIFT;
pub const MAP_HUGE_1GB : ::c_int = 30 << MAP_HUGE_SHIFT;

pub use gen::{
    MAP_GROWSDOWN, MAP_DENYWRITE, MAP_EXECUTABLE, MAP_LOCKED, MAP_NORESERVE, MAP_POPULATE,
    MAP_NONBLOCK, MAP_STACK, MAP_HUGETLB, MCL_CURRENT, MCL_FUTURE,
};

/////////////////////////////////////
// arch/x86/include/uapi/asm/msgbuf.h
/////////////////////////////////////

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

////////////////////////////////////
// arch/x86/include/uapi/asm/param.h
////////////////////////////////////

pub use gen::{
    HZ, EXEC_PAGESIZE, NOGROUP, MAXHOSTNAMELEN,
};

///////////////////////////////////
// arch/x86/include/uapi/asm/poll.h
///////////////////////////////////

pub use ::gen::{
    POLLIN, POLLPRI, POLLOUT, POLLERR, POLLHUP, POLLNVAL, POLLRDNORM, POLLRDBAND,
    POLLWRNORM, POLLWRBAND, POLLMSG, POLLREMOVE, POLLRDHUP, POLLFREE, POLL_BUSY_LOOP,
    pollfd,
};

//////////////////////////////////////////
// arch/x86/include/uapi/asm/posix_types.h
//////////////////////////////////////////

pub use self::abi::{
    __kernel_old_uid_t, __kernel_old_gid_t, __kernel_old_dev_t, __kernel_long_t,
    __kernel_ulong_t,
};

pub use ::gen::{
    __kernel_ino_t, __kernel_mode_t, BYTES_PER_KERNEL_MODE_T, __kernel_pid_t,
    __kernel_ipc_pid_t, __kernel_uid_t, __kernel_gid_t, __kernel_suseconds_t,
    __kernel_daddr_t, __kernel_uid32_t, __kernel_gid32_t,

    __kernel_fsid_t,

    __kernel_off_t, __kernel_loff_t, __kernel_time_t, __kernel_clock_t, __kernel_timer_t,
    __kernel_clockid_t, __kernel_caddr_t, __kernel_uid16_t, __kernel_gid16_t,
};

pub type __kernel_size_t    = __kernel_ulong_t;
pub type __kernel_ssize_t   = __kernel_long_t;
pub type __kernel_ptrdiff_t = __kernel_long_t;

///////////////////////////////////////
// arch/x86/include/uapi/asm/resource.h
///////////////////////////////////////

pub use ::gen::{
    RLIMIT_CPU, RLIMIT_FSIZE, RLIMIT_DATA, RLIMIT_STACK, RLIMIT_CORE, RLIMIT_RSS,
    RLIMIT_NPROC, RLIMIT_NOFILE, RLIMIT_MEMLOCK, RLIMIT_AS, RLIMIT_LOCKS,
    RLIMIT_SIGPENDING, RLIMIT_MSGQUEUE, RLIMIT_NICE, RLIMIT_RTPRIO, RLIMIT_RTTIME,
    RLIM_NLIMITS, RLIM_INFINITY,
};

/////////////////////////////////////
// arch/x86/include/uapi/asm/sembuf.h
/////////////////////////////////////

#[repr(C)]
#[derive(Pod, Eq)]
pub struct semid64_ds {
    pub sem_perm:  ipc64_perm,
    pub sem_otime: __kernel_time_t,
    pub __unused1: __kernel_ulong_t,
    pub sem_ctime: __kernel_time_t,
    pub __unused2: __kernel_ulong_t,
    pub sem_nsems: __kernel_ulong_t,
    pub __unused3: __kernel_ulong_t,
    pub __unused4: __kernel_ulong_t,
}

/////////////////////////////////////
// arch/x86/include/uapi/asm/shmbuf.h
/////////////////////////////////////

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

//////////////////////////////////////
// arch/x86/include/uapi/asm/siginfo.h
//////////////////////////////////////

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

pub use self::abi::{
    __ARCH_SI_CLOCK_T, __ARCH_SI_PREAMBLE_SIZE,
};

pub use ::gen::{
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

/////////////////////////////////////
// arch/x86/include/uapi/asm/signal.h
/////////////////////////////////////

pub use ::gen::{
    _NSIG, _NSIG_BPW, _NSIG_WORDS,
    old_sigset_t, SigsetVal, sigset_t, sigaction,
};

pub const SIGHUP    : c_int = 1;
pub const SIGINT    : c_int = 2;
pub const SIGQUIT   : c_int = 3;
pub const SIGILL    : c_int = 4;
pub const SIGTRAP   : c_int = 5;
pub const SIGABRT   : c_int = 6;
pub const SIGIOT    : c_int = 6;
pub const SIGBUS    : c_int = 7;
pub const SIGFPE    : c_int = 8;
pub const SIGKILL   : c_int = 9;
pub const SIGUSR1   : c_int = 10;
pub const SIGSEGV   : c_int = 11;
pub const SIGUSR2   : c_int = 12;
pub const SIGPIPE   : c_int = 13;
pub const SIGALRM   : c_int = 14;
pub const SIGTERM   : c_int = 15;
pub const SIGSTKFLT : c_int = 16;
pub const SIGCHLD   : c_int = 17;
pub const SIGCONT   : c_int = 18;
pub const SIGSTOP   : c_int = 19;
pub const SIGTSTP   : c_int = 20;
pub const SIGTTIN   : c_int = 21;
pub const SIGTTOU   : c_int = 22;
pub const SIGURG    : c_int = 23;
pub const SIGXCPU   : c_int = 24;
pub const SIGXFSZ   : c_int = 25;
pub const SIGVTALRM : c_int = 26;
pub const SIGPROF   : c_int = 27;
pub const SIGWINCH  : c_int = 28;
pub const SIGIO     : c_int = 29;
pub const SIGPOLL   : c_int = SIGIO;
pub const SIGPWR    : c_int = 30;
pub const SIGSYS    : c_int = 31;
pub const SIGUNUSED : c_int = 31;

pub const SA_NOCLDSTOP : c_int = 0x00000001;
pub const SA_NOCLDWAIT : c_int = 0x00000002;
pub const SA_SIGINFO   : c_int = 0x00000004;
pub const SA_ONSTACK   : c_int = 0x08000000;
pub const SA_RESTART   : c_int = 0x10000000;
pub const SA_NODEFER   : c_int = 0x40000000;
#[allow(overflowing_literals)]
pub const SA_RESETHAND : c_int = 0x80000000;
pub const SA_NOMASK    : c_int = SA_NODEFER;
pub const SA_ONESHOT   : c_int = SA_RESETHAND;
pub const SA_RESTORER  : c_int = 0x04000000;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct sigaltstack {
    pub ss_sp: *mut c_void,
    pub ss_flags: c_int,
    pub ss_size: ::user_size_t,
}

pub type stack_t = sigaltstack;

pub const MINSIGSTKSZ : usize = 2048;
pub const SIGSTKSZ    : usize = 8192;

/////////////////////////////////////
// arch/x86/include/uapi/asm/socket.h
/////////////////////////////////////

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

//////////////////////////////////////
// arch/x86/include/uapi/asm/sockios.h
//////////////////////////////////////

pub use ::gen::{
    FIOSETOWN, SIOCSPGRP, FIOGETOWN, SIOCGPGRP, SIOCATMARK, SIOCGSTAMP, SIOCGSTAMPNS,
};

/////////////////////////////////////
// arch/x86/include/uapi/asm/statfs.h
/////////////////////////////////////

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

///////////////////////////////////
// arch/x86/include/uapi/asm/stat.h
///////////////////////////////////

#[repr(C)]
#[derive(Pod, Eq)]
pub struct stat {
    pub st_dev:        __kernel_ulong_t,
    pub st_ino:        __kernel_ulong_t,
    pub st_nlink:      __kernel_ulong_t,
    pub st_mode:       c_uint,
    pub st_uid:        c_uint,
    pub st_gid:        c_uint,
    pub __pad0:        c_uint,
    pub st_rdev:       __kernel_ulong_t,
    pub st_size:       __kernel_long_t,
    pub st_blksize:    __kernel_long_t,
    pub st_blocks:     __kernel_long_t,
    pub st_atime:      __kernel_ulong_t,
    pub st_atime_nsec: __kernel_ulong_t,
    pub st_mtime:      __kernel_ulong_t,
    pub st_mtime_nsec: __kernel_ulong_t,
    pub st_ctime:      __kernel_ulong_t,
    pub st_ctime_nsec: __kernel_ulong_t,
    pub __unused:      [__kernel_long_t; 3],
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct __old_kernel_stat {
    pub st_dev:   c_ushort,
    pub st_ino:   c_ushort,
    pub st_mode:  c_ushort,
    pub st_nlink: c_ushort,
    pub st_uid:   c_ushort,
    pub st_gid:   c_ushort,
    pub st_rdev:  c_ushort,
    pub st_size:  c_uint,
    pub st_atime: c_uint,
    pub st_mtime: c_uint,
    pub st_ctime: c_uint,
}

///////////////////////////////////////
// arch/x86/include/uapi/asm/termbits.h
///////////////////////////////////////

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

//////////////////////////////////////
// arch/x86/include/uapi/asm/termios.h
//////////////////////////////////////

pub use ::gen::{
    winsize, NCC, termio, TIOCM_LE, TIOCM_DTR, TIOCM_RTS, TIOCM_ST, TIOCM_SR, TIOCM_CTS,
    TIOCM_CAR, TIOCM_RNG, TIOCM_DSR, TIOCM_CD, TIOCM_RI, TIOCM_OUT1, TIOCM_OUT2,
    TIOCM_LOOP,
};

pub use self::abi::{
    __NR_rt_sigaction, __NR_rt_sigreturn, __NR_ioctl, __NR_readv, __NR_writev,
    __NR_recvfrom, __NR_sendmsg, __NR_recvmsg, __NR_execve, __NR_ptrace,
    __NR_rt_sigpending, __NR_rt_sigtimedwait, __NR_rt_sigqueueinfo, __NR_sigaltstack,
    __NR_timer_create, __NR_mq_notify, __NR_kexec_load, __NR_waitid, __NR_set_robust_list,
    __NR_get_robust_list, __NR_vmsplice, __NR_move_pages, __NR_preadv, __NR_pwritev,
    __NR_rt_tgsigqueueinfo, __NR_recvmmsg, __NR_sendmmsg, __NR_process_vm_readv,
    __NR_process_vm_writev, __NR_setsockopt, __NR_getsockopt, __NR_io_setup,
    __NR_io_submit, __NR_execveat,
};

/////////////////////////////////////
// arch/x86/include/uapi/asm/unistd.h
/////////////////////////////////////

pub const __X32_SYSCALL_BIT: c_longlong = 0x40000000;

///////////////////////////////
// include/uapi/linux/sysinfo.h
///////////////////////////////

pub const SYSINFO_PADDING: usize = 0;

///////////////////////////////////
// arch/x86/syscalls/syscall_64.tbl
///////////////////////////////////

pub const __NR_read                   : usize = 0;
pub const __NR_write                  : usize = 1;
pub const __NR_open                   : usize = 2;
pub const __NR_close                  : usize = 3;
pub const __NR_newstat                : usize = 4;
pub const __NR_newfstat               : usize = 5;
pub const __NR_newlstat               : usize = 6;
pub const __NR_poll                   : usize = 7;
pub const __NR_lseek                  : usize = 8;
pub const __NR_mmap                   : usize = 9;
pub const __NR_mprotect               : usize = 10;
pub const __NR_munmap                 : usize = 11;
pub const __NR_brk                    : usize = 12;
pub const __NR_rt_sigprocmask         : usize = 14;
pub const __NR_pread64                : usize = 17;
pub const __NR_pwrite64               : usize = 18;
pub const __NR_access                 : usize = 21;
pub const __NR_pipe                   : usize = 22;
pub const __NR_select                 : usize = 23;
pub const __NR_sched_yield            : usize = 24;
pub const __NR_mremap                 : usize = 25;
pub const __NR_msync                  : usize = 26;
pub const __NR_mincore                : usize = 27;
pub const __NR_madvise                : usize = 28;
pub const __NR_shmget                 : usize = 29;
pub const __NR_shmat                  : usize = 30;
pub const __NR_shmctl                 : usize = 31;
pub const __NR_dup                    : usize = 32;
pub const __NR_dup2                   : usize = 33;
pub const __NR_pause                  : usize = 34;
pub const __NR_nanosleep              : usize = 35;
pub const __NR_getitimer              : usize = 36;
pub const __NR_alarm                  : usize = 37;
pub const __NR_setitimer              : usize = 38;
pub const __NR_getpid                 : usize = 39;
pub const __NR_sendfile64             : usize = 40;
pub const __NR_socket                 : usize = 41;
pub const __NR_connect                : usize = 42;
pub const __NR_accept                 : usize = 43;
pub const __NR_sendto                 : usize = 44;
pub const __NR_shutdown               : usize = 48;
pub const __NR_bind                   : usize = 49;
pub const __NR_listen                 : usize = 50;
pub const __NR_getsockname            : usize = 51;
pub const __NR_getpeername            : usize = 52;
pub const __NR_socketpair             : usize = 53;
pub const __NR_clone                  : usize = 56;
pub const __NR_fork                   : usize = 57;
pub const __NR_vfork                  : usize = 58;
pub const __NR_exit                   : usize = 60;
pub const __NR_wait4                  : usize = 61;
pub const __NR_kill                   : usize = 62;
pub const __NR_newuname               : usize = 63;
pub const __NR_semget                 : usize = 64;
pub const __NR_semop                  : usize = 65;
pub const __NR_semctl                 : usize = 66;
pub const __NR_shmdt                  : usize = 67;
pub const __NR_msgget                 : usize = 68;
pub const __NR_msgsnd                 : usize = 69;
pub const __NR_msgrcv                 : usize = 70;
pub const __NR_msgctl                 : usize = 71;
pub const __NR_fcntl                  : usize = 72;
pub const __NR_flock                  : usize = 73;
pub const __NR_fsync                  : usize = 74;
pub const __NR_fdatasync              : usize = 75;
pub const __NR_truncate               : usize = 76;
pub const __NR_ftruncate              : usize = 77;
pub const __NR_getdents               : usize = 78;
pub const __NR_getcwd                 : usize = 79;
pub const __NR_chdir                  : usize = 80;
pub const __NR_fchdir                 : usize = 81;
pub const __NR_rename                 : usize = 82;
pub const __NR_mkdir                  : usize = 83;
pub const __NR_rmdir                  : usize = 84;
pub const __NR_creat                  : usize = 85;
pub const __NR_link                   : usize = 86;
pub const __NR_unlink                 : usize = 87;
pub const __NR_symlink                : usize = 88;
pub const __NR_readlink               : usize = 89;
pub const __NR_chmod                  : usize = 90;
pub const __NR_fchmod                 : usize = 91;
pub const __NR_chown                  : usize = 92;
pub const __NR_fchown                 : usize = 93;
pub const __NR_lchown                 : usize = 94;
pub const __NR_umask                  : usize = 95;
pub const __NR_gettimeofday           : usize = 96;
pub const __NR_getrlimit              : usize = 97;
pub const __NR_getrusage              : usize = 98;
pub const __NR_sysinfo                : usize = 99;
pub const __NR_times                  : usize = 100;
pub const __NR_getuid                 : usize = 102;
pub const __NR_syslog                 : usize = 103;
pub const __NR_getgid                 : usize = 104;
pub const __NR_setuid                 : usize = 105;
pub const __NR_setgid                 : usize = 106;
pub const __NR_geteuid                : usize = 107;
pub const __NR_getegid                : usize = 108;
pub const __NR_setpgid                : usize = 109;
pub const __NR_getppid                : usize = 110;
pub const __NR_getpgrp                : usize = 111;
pub const __NR_setsid                 : usize = 112;
pub const __NR_setreuid               : usize = 113;
pub const __NR_setregid               : usize = 114;
pub const __NR_getgroups              : usize = 115;
pub const __NR_setgroups              : usize = 116;
pub const __NR_setresuid              : usize = 117;
pub const __NR_getresuid              : usize = 118;
pub const __NR_setresgid              : usize = 119;
pub const __NR_getresgid              : usize = 120;
pub const __NR_getpgid                : usize = 121;
pub const __NR_setfsuid               : usize = 122;
pub const __NR_setfsgid               : usize = 123;
pub const __NR_getsid                 : usize = 124;
pub const __NR_capget                 : usize = 125;
pub const __NR_capset                 : usize = 126;
pub const __NR_rt_sigsuspend          : usize = 130;
pub const __NR_utime                  : usize = 132;
pub const __NR_mknod                  : usize = 133;
pub const __NR_personality            : usize = 135;
pub const __NR_ustat                  : usize = 136;
pub const __NR_statfs                 : usize = 137;
pub const __NR_fstatfs                : usize = 138;
pub const __NR_sysfs                  : usize = 139;
pub const __NR_getpriority            : usize = 140;
pub const __NR_setpriority            : usize = 141;
pub const __NR_sched_setparam         : usize = 142;
pub const __NR_sched_getparam         : usize = 143;
pub const __NR_sched_setscheduler     : usize = 144;
pub const __NR_sched_getscheduler     : usize = 145;
pub const __NR_sched_get_priority_max : usize = 146;
pub const __NR_sched_get_priority_min : usize = 147;
pub const __NR_sched_rr_get_interval  : usize = 148;
pub const __NR_mlock                  : usize = 149;
pub const __NR_munlock                : usize = 150;
pub const __NR_mlockall               : usize = 151;
pub const __NR_munlockall             : usize = 152;
pub const __NR_vhangup                : usize = 153;
pub const __NR_modify_ldt             : usize = 154;
pub const __NR_pivot_root             : usize = 155;
pub const __NR_prctl                  : usize = 157;
pub const __NR_arch_prctl             : usize = 158;
pub const __NR_adjtimex               : usize = 159;
pub const __NR_setrlimit              : usize = 160;
pub const __NR_chroot                 : usize = 161;
pub const __NR_sync                   : usize = 162;
pub const __NR_acct                   : usize = 163;
pub const __NR_settimeofday           : usize = 164;
pub const __NR_mount                  : usize = 165;
pub const __NR_umount                 : usize = 166;
pub const __NR_swapon                 : usize = 167;
pub const __NR_swapoff                : usize = 168;
pub const __NR_reboot                 : usize = 169;
pub const __NR_sethostname            : usize = 170;
pub const __NR_setdomainname          : usize = 171;
pub const __NR_iopl                   : usize = 172;
pub const __NR_ioperm                 : usize = 173;
pub const __NR_init_module            : usize = 175;
pub const __NR_delete_module          : usize = 176;
pub const __NR_quotactl               : usize = 179;
pub const __NR_getpmsg                : usize = 181;
pub const __NR_putpmsg                : usize = 182;
pub const __NR_afs_syscall            : usize = 183;
pub const __NR_tuxcall                : usize = 184;
pub const __NR_security               : usize = 185;
pub const __NR_gettid                 : usize = 186;
pub const __NR_readahead              : usize = 187;
pub const __NR_setxattr               : usize = 188;
pub const __NR_lsetxattr              : usize = 189;
pub const __NR_fsetxattr              : usize = 190;
pub const __NR_getxattr               : usize = 191;
pub const __NR_lgetxattr              : usize = 192;
pub const __NR_fgetxattr              : usize = 193;
pub const __NR_listxattr              : usize = 194;
pub const __NR_llistxattr             : usize = 195;
pub const __NR_flistxattr             : usize = 196;
pub const __NR_removexattr            : usize = 197;
pub const __NR_lremovexattr           : usize = 198;
pub const __NR_fremovexattr           : usize = 199;
pub const __NR_tkill                  : usize = 200;
pub const __NR_time                   : usize = 201;
pub const __NR_futex                  : usize = 202;
pub const __NR_sched_setaffinity      : usize = 203;
pub const __NR_sched_getaffinity      : usize = 204;
pub const __NR_io_destroy             : usize = 207;
pub const __NR_io_getevents           : usize = 208;
pub const __NR_io_cancel              : usize = 210;
pub const __NR_lookup_dcookie         : usize = 212;
pub const __NR_epoll_create           : usize = 213;
pub const __NR_remap_file_pages       : usize = 216;
pub const __NR_getdents64             : usize = 217;
pub const __NR_set_tid_address        : usize = 218;
pub const __NR_restart_syscall        : usize = 219;
pub const __NR_semtimedop             : usize = 220;
pub const __NR_fadvise64              : usize = 221;
pub const __NR_timer_settime          : usize = 223;
pub const __NR_timer_gettime          : usize = 224;
pub const __NR_timer_getoverrun       : usize = 225;
pub const __NR_timer_delete           : usize = 226;
pub const __NR_clock_settime          : usize = 227;
pub const __NR_clock_gettime          : usize = 228;
pub const __NR_clock_getres           : usize = 229;
pub const __NR_clock_nanosleep        : usize = 230;
pub const __NR_exit_group             : usize = 231;
pub const __NR_epoll_wait             : usize = 232;
pub const __NR_epoll_ctl              : usize = 233;
pub const __NR_tgkill                 : usize = 234;
pub const __NR_utimes                 : usize = 235;
pub const __NR_mbind                  : usize = 237;
pub const __NR_set_mempolicy          : usize = 238;
pub const __NR_get_mempolicy          : usize = 239;
pub const __NR_mq_open                : usize = 240;
pub const __NR_mq_unlink              : usize = 241;
pub const __NR_mq_timedsend           : usize = 242;
pub const __NR_mq_timedreceive        : usize = 243;
pub const __NR_mq_getsetattr          : usize = 245;
pub const __NR_add_key                : usize = 248;
pub const __NR_request_key            : usize = 249;
pub const __NR_keyctl                 : usize = 250;
pub const __NR_ioprio_set             : usize = 251;
pub const __NR_ioprio_get             : usize = 252;
pub const __NR_inotify_init           : usize = 253;
pub const __NR_inotify_add_watch      : usize = 254;
pub const __NR_inotify_rm_watch       : usize = 255;
pub const __NR_migrate_pages          : usize = 256;
pub const __NR_openat                 : usize = 257;
pub const __NR_mkdirat                : usize = 258;
pub const __NR_mknodat                : usize = 259;
pub const __NR_fchownat               : usize = 260;
pub const __NR_futimesat              : usize = 261;
pub const __NR_newfstatat             : usize = 262;
pub const __NR_unlinkat               : usize = 263;
pub const __NR_renameat               : usize = 264;
pub const __NR_linkat                 : usize = 265;
pub const __NR_symlinkat              : usize = 266;
pub const __NR_readlinkat             : usize = 267;
pub const __NR_fchmodat               : usize = 268;
pub const __NR_faccessat              : usize = 269;
pub const __NR_pselect6               : usize = 270;
pub const __NR_ppoll                  : usize = 271;
pub const __NR_unshare                : usize = 272;
pub const __NR_splice                 : usize = 275;
pub const __NR_tee                    : usize = 276;
pub const __NR_sync_file_range        : usize = 277;
pub const __NR_utimensat              : usize = 280;
pub const __NR_epoll_pwait            : usize = 281;
pub const __NR_signalfd               : usize = 282;
pub const __NR_timerfd_create         : usize = 283;
pub const __NR_eventfd                : usize = 284;
pub const __NR_fallocate              : usize = 285;
pub const __NR_timerfd_settime        : usize = 286;
pub const __NR_timerfd_gettime        : usize = 287;
pub const __NR_accept4                : usize = 288;
pub const __NR_signalfd4              : usize = 289;
pub const __NR_eventfd2               : usize = 290;
pub const __NR_epoll_create1          : usize = 291;
pub const __NR_dup3                   : usize = 292;
pub const __NR_pipe2                  : usize = 293;
pub const __NR_inotify_init1          : usize = 294;
pub const __NR_perf_event_open        : usize = 298;
pub const __NR_fanotify_init          : usize = 300;
pub const __NR_fanotify_mark          : usize = 301;
pub const __NR_prlimit64              : usize = 302;
pub const __NR_name_to_handle_at      : usize = 303;
pub const __NR_open_by_handle_at      : usize = 304;
pub const __NR_clock_adjtime          : usize = 305;
pub const __NR_syncfs                 : usize = 306;
pub const __NR_setns                  : usize = 308;
pub const __NR_getcpu                 : usize = 309;
pub const __NR_kcmp                   : usize = 312;
pub const __NR_finit_module           : usize = 313;
pub const __NR_sched_setattr          : usize = 314;
pub const __NR_sched_getattr          : usize = 315;
pub const __NR_renameat2              : usize = 316;
pub const __NR_seccomp                : usize = 317;
pub const __NR_getrandom              : usize = 318;
pub const __NR_memfd_create           : usize = 319;
pub const __NR_kexec_file_load        : usize = 320;
pub const __NR_bpf                    : usize = 321;

// bpf.h

impl ::bpf_insn {
    pub fn dst_reg(self) -> u8 { self.reg & 0xF }
    pub fn set_dst_reg(&mut self, val: u8) { self.reg = (self.reg & 0xF0) | (val & 0x0F) }

    pub fn src_reg(self) -> u8 { self.reg >> 4 }
    pub fn set_src_reg(&mut self, val: u8) { self.reg = (self.reg & 0x0F) | val << 4 }
}

// x86_64 specific:

//////////////////////////////////
// arch/x86/include/uapi/asm/ldt.h
//////////////////////////////////

pub const LDT_ENTRIES    : c_int = 8192;
pub const LDT_ENTRY_SIZE : c_int = 8;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct user_desc {
    pub entry_number: c_uint,
    pub base_addr:    c_uint,
    pub limit:        c_uint,
    //unsigned int seg_32bit:1;
    //unsigned int contents:2;
    //unsigned int read_exec_only:1;
    //unsigned int limit_in_pages:1;
    //unsigned int seg_not_present:1;
    //unsigned int useable:1;
    //unsigned int lm:1;
    __bitfield_one: c_uint,
}

impl user_desc {
    pub fn seg_32bit       (&self) -> bool { bf32_get(self.__bitfield_one, 0, 1) != 0 }
    pub fn contents        (&self) -> c_uint { bf32_get(self.__bitfield_one, 1, 2) }
    pub fn read_exec_only  (&self) -> bool { bf32_get(self.__bitfield_one, 3, 1) != 0 }
    pub fn limit_in_pages  (&self) -> bool { bf32_get(self.__bitfield_one, 4, 1) != 0 }
    pub fn seg_not_present (&self) -> bool { bf32_get(self.__bitfield_one, 5, 1) != 0 }
    pub fn useable         (&self) -> bool { bf32_get(self.__bitfield_one, 6, 1) != 0 }
    pub fn lm              (&self) -> bool { bf32_get(self.__bitfield_one, 7, 1) != 0 }

    pub fn set_seg_32bit       (&mut self, val: bool) { self.__bitfield_one = bf32_set(self.__bitfield_one, 0, 1, val as c_uint) }
    pub fn set_contents        (&mut self, val: c_uint) { self.__bitfield_one = bf32_set(self.__bitfield_one, 1, 2, val) }
    pub fn set_read_exec_only  (&mut self, val: bool) { self.__bitfield_one = bf32_set(self.__bitfield_one, 3, 1, val as c_uint) }
    pub fn set_limit_in_pages  (&mut self, val: bool) { self.__bitfield_one = bf32_set(self.__bitfield_one, 4, 1, val as c_uint) }
    pub fn set_seg_not_present (&mut self, val: bool) { self.__bitfield_one = bf32_set(self.__bitfield_one, 5, 1, val as c_uint) }
    pub fn set_useable         (&mut self, val: bool) { self.__bitfield_one = bf32_set(self.__bitfield_one, 6, 1, val as c_uint) }
    pub fn set_lm              (&mut self, val: bool) { self.__bitfield_one = bf32_set(self.__bitfield_one, 7, 1, val as c_uint) }
}

////////////////////////////////////
// arch/x86/include/uapi/asm/prctl.h
////////////////////////////////////

pub const ARCH_SET_GS: c_int = 0x1001;
pub const ARCH_SET_FS: c_int = 0x1002;
pub const ARCH_GET_FS: c_int = 0x1003;
pub const ARCH_GET_GS: c_int = 0x1004;

/////////////////////////////////////
// arch/x86/include/uapi/asm/auxvec.h
/////////////////////////////////////

pub const AT_SYSINFO_EHDR : usize = 33;

/// The largest AT_* value plus one.
pub const AUX_CNT : usize = 34;

///////////////////////////
// include/uapi/linux/elf.h
///////////////////////////

pub use self::abi::{ElfDyn, ElfRel, ElfRela, ElfSym, ElfEhdr, ElfPhdr, ElfShdr, ElfNhdr};
