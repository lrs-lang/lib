// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(non_camel_case_types)]

// XXX: Only use fully qualified types here!!! The types in here might be overridden by
// the types in the non-generic files, therefore we can't use them directly.

pub type __s8  = ::c_schar;
pub type __u8  = ::c_uchar;
pub type __s16 = ::c_short;
pub type __u16 = ::c_ushort;
pub type __s32 = ::c_int;
pub type __u32 = ::c_uint;
pub type __s64 = ::c_longlong;
pub type __u64 = ::c_ulonglong;

pub const __BITS_PER_LONG : usize = 32;
pub const BYTES_PER_KERNEL_MODE_T : usize = ::BYTES_PER_INT;

pub type __kernel_long_t      = ::c_long;
pub type __kernel_ulong_t     = ::c_ulong;
pub type __kernel_ino_t       = ::__kernel_ulong_t;
pub type __kernel_mode_t      = ::c_uint;
pub type __kernel_pid_t       = ::c_int;
pub type __kernel_ipc_pid_t   = ::c_int;
pub type __kernel_uid_t       = ::c_uint;
pub type __kernel_gid_t       = ::c_uint;
pub type __kernel_suseconds_t = ::__kernel_long_t;
pub type __kernel_daddr_t     = ::c_int;
pub type __kernel_uid32_t     = ::c_uint;
pub type __kernel_gid32_t     = ::c_uint;
pub type __kernel_old_uid_t   = ::__kernel_uid_t;
pub type __kernel_old_gid_t   = ::__kernel_gid_t;
pub type __kernel_old_dev_t   = ::c_uint;
pub type __kernel_off_t       = ::__kernel_long_t;
pub type __kernel_loff_t      = ::c_longlong;
pub type __kernel_time_t      = ::__kernel_long_t;
pub type __kernel_clock_t     = ::__kernel_long_t;
pub type __kernel_timer_t     = ::c_int;
pub type __kernel_clockid_t   = ::c_int;
pub type __kernel_caddr_t     = *mut ::c_char;
pub type __kernel_uid16_t     = ::c_ushort;
pub type __kernel_gid16_t     = ::c_ushort;

#[repr(C)]
#[derive(Copy, Eq)]
pub struct __kernel_fsid_t {
    pub val: [::c_int; 2],
}

// errno-base.h & errno.h

// XXX: lives in linux_error

// stat.h

pub const STAT_HAVE_NSEC: ::c_int = 1;

#[repr(C)]
#[derive(Copy, Eq)]
pub struct stat {
	pub st_dev:        ::c_ulong,
	pub st_ino:        ::c_ulong,
	pub st_mode:       ::c_uint,
	pub st_nlink:      ::c_uint,
	pub st_uid:        ::c_uint,
	pub st_gid:        ::c_uint,
	pub st_rdev:       ::c_ulong,
	pub __pad1:        ::c_ulong,
	pub st_size:       ::c_long,
	pub st_blksize:    ::c_int,
	pub __pad2:        ::c_int,
	pub st_blocks:     ::c_long,
	pub st_atime:      ::c_long,
	pub st_atime_nsec: ::c_ulong,
	pub st_mtime:      ::c_long,
	pub st_mtime_nsec: ::c_ulong,
	pub st_ctime:      ::c_long,
	pub st_ctime_nsec: ::c_ulong,
	pub __unused4:     ::c_uint,
	pub __unused5:     ::c_uint,
}

#[repr(C)]
#[derive(Copy, Eq)]
pub struct stat64 {
	pub st_dev:        ::c_ulonglong,
	pub st_ino:        ::c_ulonglong,
	pub st_mode:       ::c_uint,
	pub st_nlink:      ::c_uint,
	pub st_uid:        ::c_uint,
	pub st_gid:        ::c_uint,
	pub st_rdev:       ::c_ulonglong,
	pub __pad1:        ::c_ulonglong,
	pub st_size:       ::c_longlong,
	pub st_blksize:    ::c_int,
	pub __pad2:        ::c_int,
	pub st_blocks:     ::c_longlong,
	pub st_atime:      ::c_int,
	pub st_atime_nsec: ::c_uint,
	pub st_mtime:      ::c_int,
	pub st_mtime_nsec: ::c_uint,
	pub st_ctime:      ::c_int,
	pub st_ctime_nsec: ::c_uint,
	pub __unused4:     ::c_uint,
	pub __unused5:     ::c_uint,
}

#[repr(C)]
#[derive(Copy, Eq)]
pub struct statfs {
	pub f_type:    ::__statfs_word,
	pub f_bsize:   ::__statfs_word,
	pub f_blocks:  ::__statfs_word,
	pub f_bfree:   ::__statfs_word,
	pub f_bavail:  ::__statfs_word,
	pub f_files:   ::__statfs_word,
	pub f_ffree:   ::__statfs_word,
	pub f_fsid:    ::__kernel_fsid_t,
	pub f_namelen: ::__statfs_word,
	pub f_frsize:  ::__statfs_word,
	pub f_flags:   ::__statfs_word,
	pub f_spare:   [::__statfs_word; 4],
}

// signal-defs.h

pub const SIG_BLOCK   : ::c_int = 0;
pub const SIG_UNBLOCK : ::c_int = 1;
pub const SIG_SETMASK : ::c_int = 2;

// can't write this in Rust
// type __signalfn_t = *(extern fn(c_int));
pub type __sighandler_t = extern fn(::c_int);

// can't write this in Rust
// type __restorefn_t = *(extern fn());
pub type __sigrestore_t = extern fn();

pub const SIG_DFL : usize = 0;
pub const SIG_IGN : usize = 1;
pub const SIG_ERR : usize = !0;

// signal.h

pub const _NSIG       : usize = 64;
pub const _NSIG_BPW   : usize = ::__BITS_PER_LONG;
pub const _NSIG_WORDS : usize = ::_NSIG / ::_NSIG_BPW;

pub const SIGHUP    : ::c_int = 1;
pub const SIGINT    : ::c_int = 2;
pub const SIGQUIT   : ::c_int = 3;
pub const SIGILL    : ::c_int = 4;
pub const SIGTRAP   : ::c_int = 5;
pub const SIGABRT   : ::c_int = 6;
pub const SIGIOT    : ::c_int = 6;
pub const SIGBUS    : ::c_int = 7;
pub const SIGFPE    : ::c_int = 8;
pub const SIGKILL   : ::c_int = 9;
pub const SIGUSR1   : ::c_int = 10;
pub const SIGSEGV   : ::c_int = 11;
pub const SIGUSR2   : ::c_int = 12;
pub const SIGPIPE   : ::c_int = 13;
pub const SIGALRM   : ::c_int = 14;
pub const SIGTERM   : ::c_int = 15;
pub const SIGSTKFLT : ::c_int = 16;
pub const SIGCHLD   : ::c_int = 17;
pub const SIGCONT   : ::c_int = 18;
pub const SIGSTOP   : ::c_int = 19;
pub const SIGTSTP   : ::c_int = 20;
pub const SIGTTIN   : ::c_int = 21;
pub const SIGTTOU   : ::c_int = 22;
pub const SIGURG    : ::c_int = 23;
pub const SIGXCPU   : ::c_int = 24;
pub const SIGXFSZ   : ::c_int = 25;
pub const SIGVTALRM : ::c_int = 26;
pub const SIGPROF   : ::c_int = 27;
pub const SIGWINCH  : ::c_int = 28;
pub const SIGIO     : ::c_int = 29;
pub const SIGPOLL   : ::c_int = ::SIGIO;
pub const SIGPWR    : ::c_int = 30;
pub const SIGSYS    : ::c_int = 31;
pub const SIGUNUSED : ::c_int = 31;

pub const SA_NOCLDSTOP : ::c_int = 0x00000001;
pub const SA_NOCLDWAIT : ::c_int = 0x00000002;
pub const SA_SIGINFO   : ::c_int = 0x00000004;
pub const SA_ONSTACK   : ::c_int = 0x08000000;
pub const SA_RESTART   : ::c_int = 0x10000000;
pub const SA_NODEFER   : ::c_int = 0x40000000;
#[allow(overflowing_literals)]
pub const SA_RESETHAND : ::c_int = 0x80000000;
pub const SA_NOMASK    : ::c_int = ::SA_NODEFER;
pub const SA_ONESHOT   : ::c_int = ::SA_RESETHAND;

pub const MINSIGSTKSZ : usize = 2048;
pub const SIGSTKSZ    : usize = 8192;

#[repr(C)]
#[derive(Copy, Eq)]
pub struct sigset_t {
    pub sig: [::c_ulong; ::_NSIG_WORDS],
}

pub type old_sigset_t = ::c_ulong;

#[repr(C)]
#[derive(Copy, Eq)]
pub struct sigaltstack {
	pub ss_sp: *mut ::c_void,
	pub ss_flags: ::c_int,
	pub ss_size: ::size_t,
}

pub type stack_t = ::sigaltstack;

// sockios.h

pub const FIOSETOWN    : ::c_int = 0x8901;
pub const SIOCSPGRP    : ::c_int = 0x8902;
pub const FIOGETOWN    : ::c_int = 0x8903;
pub const SIOCGPGRP    : ::c_int = 0x8904;
pub const SIOCATMARK   : ::c_int = 0x8905;
pub const SIOCGSTAMP   : ::c_int = 0x8906;
pub const SIOCGSTAMPNS : ::c_int = 0x8907;

// socket.h

pub const SOL_SOCKET                       : ::c_int = 1;
pub const SO_DEBUG                         : ::c_int = 1;
pub const SO_REUSEADDR                     : ::c_int = 2;
pub const SO_TYPE                          : ::c_int = 3;
pub const SO_ERROR                         : ::c_int = 4;
pub const SO_DONTROUTE                     : ::c_int = 5;
pub const SO_BROADCAST                     : ::c_int = 6;
pub const SO_SNDBUF                        : ::c_int = 7;
pub const SO_RCVBUF                        : ::c_int = 8;
pub const SO_SNDBUFFORCE                   : ::c_int = 32;
pub const SO_RCVBUFFORCE                   : ::c_int = 33;
pub const SO_KEEPALIVE                     : ::c_int = 9;
pub const SO_OOBINLINE                     : ::c_int = 10;
pub const SO_NO_CHECK                      : ::c_int = 11;
pub const SO_PRIORITY                      : ::c_int = 12;
pub const SO_LINGER                        : ::c_int = 13;
pub const SO_BSDCOMPAT                     : ::c_int = 14;
pub const SO_REUSEPORT                     : ::c_int = 15;
pub const SO_PASSCRED                      : ::c_int = 16;
pub const SO_PEERCRED                      : ::c_int = 17;
pub const SO_RCVLOWAT                      : ::c_int = 18;
pub const SO_SNDLOWAT                      : ::c_int = 19;
pub const SO_RCVTIMEO                      : ::c_int = 20;
pub const SO_SNDTIMEO                      : ::c_int = 21;
pub const SO_SECURITY_AUTHENTICATION       : ::c_int = 22;
pub const SO_SECURITY_ENCRYPTION_TRANSPORT : ::c_int = 23;
pub const SO_SECURITY_ENCRYPTION_NETWORK   : ::c_int = 24;
pub const SO_BINDTODEVICE                  : ::c_int = 25;
pub const SO_ATTACH_FILTER                 : ::c_int = 26;
pub const SO_DETACH_FILTER                 : ::c_int = 27;
pub const SO_GET_FILTER                    : ::c_int = SO_ATTACH_FILTER;
pub const SO_PEERNAME                      : ::c_int = 28;
pub const SO_TIMESTAMP                     : ::c_int = 29;
pub const SCM_TIMESTAMP                    : ::c_int = SO_TIMESTAMP;
pub const SO_ACCEPTCONN                    : ::c_int = 30;
pub const SO_PEERSEC                       : ::c_int = 31;
pub const SO_PASSSEC                       : ::c_int = 34;
pub const SO_TIMESTAMPNS                   : ::c_int = 35;
pub const SCM_TIMESTAMPNS                  : ::c_int = SO_TIMESTAMPNS;
pub const SO_MARK                          : ::c_int = 36;
pub const SO_TIMESTAMPING                  : ::c_int = 37;
pub const SCM_TIMESTAMPING                 : ::c_int = SO_TIMESTAMPING;
pub const SO_PROTOCOL                      : ::c_int = 38;
pub const SO_DOMAIN                        : ::c_int = 39;
pub const SO_RXQ_OVFL                      : ::c_int = 40;
pub const SO_WIFI_STATUS                   : ::c_int = 41;
pub const SCM_WIFI_STATUS                  : ::c_int = SO_WIFI_STATUS;
pub const SO_PEEK_OFF                      : ::c_int = 42;
pub const SO_NOFCS                         : ::c_int = 43;
pub const SO_LOCK_FILTER                   : ::c_int = 44;
pub const SO_SELECT_ERR_QUEUE              : ::c_int = 45;
pub const SO_BUSY_POLL                     : ::c_int = 46;
pub const SO_MAX_PACING_RATE               : ::c_int = 47;
pub const SO_BPF_EXTENSIONS                : ::c_int = 48;

// fcntl.h

pub const O_ACCMODE       : ::c_int = 0o0000003;
pub const O_RDONLY        : ::c_int = 0o0000000;
pub const O_WRONLY        : ::c_int = 0o0000001;
pub const O_RDWR          : ::c_int = 0o0000002;
pub const O_CREAT         : ::c_int = 0o0000100;
pub const O_EXCL          : ::c_int = 0o0000200;
pub const O_NOCTTY        : ::c_int = 0o0000400;
pub const O_TRUNC         : ::c_int = 0o0001000;
pub const O_APPEND        : ::c_int = 0o0002000;
pub const O_NONBLOCK      : ::c_int = 0o0004000;
pub const O_DSYNC         : ::c_int = 0o0010000;
pub const FASYNC          : ::c_int = 0o0020000;
pub const O_DIRECT        : ::c_int = 0o0040000;
pub const O_LARGEFILE     : ::c_int = 0o0100000;
pub const O_DIRECTORY     : ::c_int = 0o0200000;
pub const O_NOFOLLOW      : ::c_int = 0o0400000;
pub const O_NOATIME       : ::c_int = 0o1000000;
pub const O_CLOEXEC       : ::c_int = 0o2000000;
pub const __O_SYNC        : ::c_int = 0o4000000;
pub const O_SYNC          : ::c_int = __O_SYNC|O_DSYNC;
pub const O_PATH          : ::c_int = 0o10000000;
pub const __O_TMPFILE     : ::c_int = 0o20000000;
pub const O_TMPFILE       : ::c_int = __O_TMPFILE|O_DIRECTORY;
pub const O_TMPFILE_MASK  : ::c_int = __O_TMPFILE|O_DIRECTORY|O_CREAT;
pub const O_NDELAY        : ::c_int = O_NONBLOCK;
pub const F_DUPFD         : ::c_uint = 0;
pub const F_GETFD         : ::c_uint = 1;
pub const F_SETFD         : ::c_uint = 2;
pub const F_GETFL         : ::c_uint = 3;
pub const F_SETFL         : ::c_uint = 4;
pub const F_GETLK         : ::c_uint = 5;
pub const F_SETLK         : ::c_uint = 6;
pub const F_SETLKW        : ::c_uint = 7;
pub const F_SETOWN        : ::c_uint = 8;
pub const F_GETOWN        : ::c_uint = 9;
pub const F_SETSIG        : ::c_uint = 10;
pub const F_GETSIG        : ::c_uint = 11;
pub const F_GETLK64       : ::c_uint = 12;
pub const F_SETLK64       : ::c_uint = 13;
pub const F_SETLKW64      : ::c_uint = 14;
pub const F_SETOWN_EX     : ::c_uint = 15;
pub const F_GETOWN_EX     : ::c_uint = 16;
pub const F_GETOWNER_UIDS : ::c_uint = 17;
pub const F_OFD_GETLK     : ::c_uint = 36;
pub const F_OFD_SETLK     : ::c_uint = 37;
pub const F_OFD_SETLKW    : ::c_uint = 38;
pub const F_OWNER_TID     : ::c_uint = 0;
pub const F_OWNER_PID     : ::c_uint = 1;
pub const F_OWNER_PGRP    : ::c_uint = 2;

#[repr(C)]
#[derive(Copy, Eq)]
pub struct f_owner_ex {
    pub type_: ::c_int,
    pub pid: ::__kernel_pid_t,
}

pub const F_LINUX_SPECIFIC_BASE : ::c_uint = 1024;
pub const FD_CLOEXEC            : ::c_uint = 1;
pub const F_RDLCK               : ::c_uint = 0;
pub const F_WRLCK               : ::c_uint = 1;
pub const F_UNLCK               : ::c_uint = 2;
pub const F_EXLCK               : ::c_uint = 4;
pub const F_SHLCK               : ::c_uint = 8;
pub const LOCK_SH               : ::c_int = 1;
pub const LOCK_EX               : ::c_int = 2;
pub const LOCK_NB               : ::c_int = 4;
pub const LOCK_UN               : ::c_int = 8;
pub const LOCK_MAND             : ::c_int = 32;
pub const LOCK_READ             : ::c_int = 64;
pub const LOCK_WRITE            : ::c_int = 128;
pub const LOCK_RW               : ::c_int = 192;

// ipcbuf.h

#[repr(C)]
#[derive(Copy, Eq)]
pub struct ipc64_perm {
    pub key:  ::__kernel_key_t,
    pub uid:  ::__kernel_uid32_t,
    pub gid:  ::__kernel_gid32_t,
    pub cuid: ::__kernel_uid32_t,
    pub cgid: ::__kernel_gid32_t,
    pub mode: ::__kernel_mode_t,
    pub __pad1: [::c_uchar; 4 - ::BYTES_PER_KERNEL_MODE_T],
    pub seq:       ::c_ushort,
    pub __pad2:    ::c_ushort,
    pub __unused1: ::__kernel_ulong_t,
    pub __unused2: ::__kernel_ulong_t,
}

// ioctl.h

pub const _IOC_NRBITS   : ::c_uint = 8;
pub const _IOC_TYPEBITS : ::c_uint = 8;

pub const _IOC_SIZEBITS : ::c_uint = 14;
pub const _IOC_DIRBITS  : ::c_uint = 2;

pub const _IOC_NRMASK    : ::c_uint = (1 << ::_IOC_NRBITS) - 1;
pub const _IOC_TYPEMASK  : ::c_uint = (1 << ::_IOC_TYPEBITS) - 1;
pub const _IOC_SIZEMASK  : ::c_uint = (1 << ::_IOC_SIZEBITS) - 1;
pub const _IOC_DIRMASK   : ::c_uint = (1 << ::_IOC_DIRBITS) - 1;
pub const _IOC_NRSHIFT   : ::c_uint = 0;
pub const _IOC_TYPESHIFT : ::c_uint = ::_IOC_NRSHIFT + ::_IOC_NRBITS;
pub const _IOC_SIZESHIFT : ::c_uint = ::_IOC_TYPESHIFT + ::_IOC_TYPEBITS;
pub const _IOC_DIRSHIFT  : ::c_uint = ::_IOC_SIZESHIFT + ::_IOC_SIZEBITS;

pub const _IOC_NONE  : ::c_uint = 0;
pub const _IOC_WRITE : ::c_uint = 1;
pub const _IOC_READ  : ::c_uint = 2;

pub fn _IOC(dir: ::c_uint, ty: ::c_uint, nr: ::c_uint,
            size: ::c_uint) -> ::c_uint {
	(dir << ::_IOC_DIRSHIFT) | (ty   << ::_IOC_TYPESHIFT) |
	(nr  << ::_IOC_NRSHIFT)  | (size << ::_IOC_SIZESHIFT)
}

pub fn _IOC_TYPECHECK<T>(_: T) -> ::c_uint { ::core::mem::size_of::<T>() as ::c_uint }

pub fn _IO(ty: ::c_uint, nr: ::c_uint) -> ::c_uint {
    _IOC(::_IOC_NONE, ty, nr, 0)
}

pub fn _IOR<T>(ty: ::c_uint, nr: ::c_uint) -> ::c_uint {
    _IOC(::_IOC_READ, ty, nr, ::core::mem::size_of::<T>() as ::c_uint)
}

pub fn _IOW<T>(ty: ::c_uint, nr: ::c_uint) -> ::c_uint {
    _IOC(::_IOC_WRITE, ty, nr, ::core::mem::size_of::<T>() as ::c_uint)
}

pub fn _IOWR<T>(ty: ::c_uint, nr: ::c_uint) -> ::c_uint {
    _IOC(::_IOC_READ|::_IOC_WRITE, ty, nr, ::core::mem::size_of::<T>() as ::c_uint)
}

pub fn _IOR_BAD<T>(ty: ::c_uint, nr: ::c_uint) -> ::c_uint {
    _IOC(::_IOC_READ, ty, nr, ::core::mem::size_of::<T>() as ::c_uint)
}

pub fn _IOW_BAD<T>(ty: ::c_uint, nr: ::c_uint) -> ::c_uint {
    _IOC(::_IOC_WRITE, ty, nr, ::core::mem::size_of::<T>() as ::c_uint)
}

pub fn _IOWR_BAD<T>(ty: ::c_uint, nr: ::c_uint) -> ::c_uint {
    _IOC(::_IOC_READ|::_IOC_WRITE, ty, nr, ::core::mem::size_of::<T>() as ::c_uint)
}

pub fn _IOC_DIR(nr:  ::c_uint) -> ::c_uint { (nr >> ::_IOC_DIRSHIFT)  & ::_IOC_DIRMASK  }
pub fn _IOC_TYPE(nr: ::c_uint) -> ::c_uint { (nr >> ::_IOC_TYPESHIFT) & ::_IOC_TYPEMASK }
pub fn _IOC_NR(nr:   ::c_uint) -> ::c_uint { (nr >> ::_IOC_NRSHIFT)   & ::_IOC_NRMASK   }
pub fn _IOC_SIZE(nr: ::c_uint) -> ::c_uint { (nr >> ::_IOC_SIZESHIFT) & ::_IOC_SIZEMASK }

pub const IOC_IN        : ::c_uint = _IOC_WRITE             << _IOC_DIRSHIFT;
pub const IOC_OUT       : ::c_uint = _IOC_READ              << _IOC_DIRSHIFT;
pub const IOC_INOUT     : ::c_uint = (_IOC_WRITE|_IOC_READ) << _IOC_DIRSHIFT;
pub const IOCSIZE_MASK  : ::c_uint = _IOC_SIZEMASK          << _IOC_SIZESHIFT;
pub const IOCSIZE_SHIFT : ::c_uint = _IOC_SIZESHIFT;

// ioctls.h

pub const TCGETS             : ::c_uint = 0x5401;
pub const TCSETS             : ::c_uint = 0x5402;
pub const TCSETSW            : ::c_uint = 0x5403;
pub const TCSETSF            : ::c_uint = 0x5404;
pub const TCGETA             : ::c_uint = 0x5405;
pub const TCSETA             : ::c_uint = 0x5406;
pub const TCSETAW            : ::c_uint = 0x5407;
pub const TCSETAF            : ::c_uint = 0x5408;
pub const TCSBRK             : ::c_uint = 0x5409;
pub const TCXONC             : ::c_uint = 0x540A;
pub const TCFLSH             : ::c_uint = 0x540B;
pub const TIOCEXCL           : ::c_uint = 0x540C;
pub const TIOCNXCL           : ::c_uint = 0x540D;
pub const TIOCSCTTY          : ::c_uint = 0x540E;
pub const TIOCGPGRP          : ::c_uint = 0x540F;
pub const TIOCSPGRP          : ::c_uint = 0x5410;
pub const TIOCOUTQ           : ::c_uint = 0x5411;
pub const TIOCSTI            : ::c_uint = 0x5412;
pub const TIOCGWINSZ         : ::c_uint = 0x5413;
pub const TIOCSWINSZ         : ::c_uint = 0x5414;
pub const TIOCMGET           : ::c_uint = 0x5415;
pub const TIOCMBIS           : ::c_uint = 0x5416;
pub const TIOCMBIC           : ::c_uint = 0x5417;
pub const TIOCMSET           : ::c_uint = 0x5418;
pub const TIOCGSOFTCAR       : ::c_uint = 0x5419;
pub const TIOCSSOFTCAR       : ::c_uint = 0x541A;
pub const FIONREAD           : ::c_uint = 0x541B;
pub const TIOCINQ            : ::c_uint = FIONREAD;
pub const TIOCLINUX          : ::c_uint = 0x541C;
pub const TIOCCONS           : ::c_uint = 0x541D;
pub const TIOCGSERIAL        : ::c_uint = 0x541E;
pub const TIOCSSERIAL        : ::c_uint = 0x541F;
pub const TIOCPKT            : ::c_uint = 0x5420;
pub const FIONBIO            : ::c_uint = 0x5421;
pub const TIOCNOTTY          : ::c_uint = 0x5422;
pub const TIOCSETD           : ::c_uint = 0x5423;
pub const TIOCGETD           : ::c_uint = 0x5424;
pub const TCSBRKP            : ::c_uint = 0x5425;
pub const TIOCSBRK           : ::c_uint = 0x5427;
pub const TIOCCBRK           : ::c_uint = 0x5428;
pub const TIOCGSID           : ::c_uint = 0x5429;
pub const TIOCGRS485         : ::c_uint = 0x542E;
pub const TIOCSRS485         : ::c_uint = 0x542F;
pub const TCGETX             : ::c_uint = 0x5432;
pub const TCSETX             : ::c_uint = 0x5433;
pub const TCSETXF            : ::c_uint = 0x5434;
pub const TCSETXW            : ::c_uint = 0x5435;
pub const TIOCVHANGUP        : ::c_uint = 0x5437;
pub const FIONCLEX           : ::c_uint = 0x5450;
pub const FIOCLEX            : ::c_uint = 0x5451;
pub const FIOASYNC           : ::c_uint = 0x5452;
pub const TIOCSERCONFIG      : ::c_uint = 0x5453;
pub const TIOCSERGWILD       : ::c_uint = 0x5454;
pub const TIOCSERSWILD       : ::c_uint = 0x5455;
pub const TIOCGLCKTRMIOS     : ::c_uint = 0x5456;
pub const TIOCSLCKTRMIOS     : ::c_uint = 0x5457;
pub const TIOCSERGSTRUCT     : ::c_uint = 0x5458;
pub const TIOCSERGETLSR      : ::c_uint = 0x5459;
pub const TIOCSERGETMULTI    : ::c_uint = 0x545A;
pub const TIOCSERSETMULTI    : ::c_uint = 0x545B;
pub const TIOCMIWAIT         : ::c_uint = 0x545C;
pub const TIOCGICOUNT        : ::c_uint = 0x545D;
pub const FIOQSIZE           : ::c_uint = 0x5460;
pub const TIOCPKT_DATA       : ::c_uint = 0;
pub const TIOCPKT_FLUSHREAD  : ::c_uint = 1;
pub const TIOCPKT_FLUSHWRITE : ::c_uint = 2;
pub const TIOCPKT_STOP       : ::c_uint = 4;
pub const TIOCPKT_START      : ::c_uint = 8;
pub const TIOCPKT_NOSTOP     : ::c_uint = 16;
pub const TIOCPKT_DOSTOP     : ::c_uint = 32;
pub const TIOCPKT_IOCTL      : ::c_uint = 64;
pub const TIOCSER_TEMT       : ::c_uint = 0x01;

pub fn TCGETS2()    -> ::c_uint { _IOR::<::termios2>(b'T' as ::c_uint, 0x2A) }
pub fn TCSETS2()    -> ::c_uint { _IOW::<::termios2>(b'T' as ::c_uint, 0x2B) }
pub fn TCSETSW2()   -> ::c_uint { _IOW::<::termios2>(b'T' as ::c_uint, 0x2C) }
pub fn TCSETSF2()   -> ::c_uint { _IOW::<::termios2>(b'T' as ::c_uint, 0x2D) }
pub fn TIOCGPTN()   -> ::c_uint { _IOR::<::c_uint>(b'T'   as ::c_uint, 0x30) }
pub fn TIOCSPTLCK() -> ::c_uint { _IOW::<::c_int>(b'T'    as ::c_uint, 0x31) }
pub fn TIOCGDEV()   -> ::c_uint { _IOR::<::c_uint>(b'T'   as ::c_uint, 0x32) }
pub fn TIOCSIG()    -> ::c_uint { _IOW::<::c_int>(b'T'    as ::c_uint, 0x36) }
pub fn TIOCGPKT()   -> ::c_uint { _IOR::<::c_int>(b'T'    as ::c_uint, 0x38) }
pub fn TIOCGPTLCK() -> ::c_uint { _IOR::<::c_int>(b'T'    as ::c_uint, 0x39) }
pub fn TIOCGEXCL()  -> ::c_uint { _IOR::<::c_int>(b'T'    as ::c_uint, 0x40) }

// termbits.h

pub type cc_t     = ::c_uchar;
pub type speed_t  = ::c_uint;
pub type tcflag_t = ::c_uint;

pub const NCCS : usize = 19;

#[repr(C)]
#[derive(Copy, Eq)]
pub struct termios {
	pub c_iflag:    ::tcflag_t,
	pub c_oflag:    ::tcflag_t,
	pub c_cflag:    ::tcflag_t,
	pub c_lflag:    ::tcflag_t,
	pub c_line:     ::cc_t,
	pub c_cc: [::cc_t; ::NCCS],
}

#[repr(C)]
#[derive(Copy, Eq)]
pub struct termios2 {
    pub c_iflag:    ::tcflag_t,
    pub c_oflag:    ::tcflag_t,
    pub c_cflag:    ::tcflag_t,
    pub c_lflag:    ::tcflag_t,
    pub c_line:     ::cc_t,
    pub c_cc: [::cc_t; ::NCCS],
    pub c_ispeed:   ::speed_t,
    pub c_ospeed:   ::speed_t,
}

#[repr(C)]
#[derive(Copy, Eq)]
pub struct ktermios {
    pub c_iflag:    ::tcflag_t,
    pub c_oflag:    ::tcflag_t,
    pub c_cflag:    ::tcflag_t,
    pub c_lflag:    ::tcflag_t,
    pub c_line:     ::cc_t,
    pub c_cc: [::cc_t; ::NCCS],
    pub c_ispeed:   ::speed_t,
    pub c_ospeed:   ::speed_t,
}

pub const VINTR    : ::cc_t = 0;
pub const VQUIT    : ::cc_t = 1;
pub const VERASE   : ::cc_t = 2;
pub const VKILL    : ::cc_t = 3;
pub const VEOF     : ::cc_t = 4;
pub const VTIME    : ::cc_t = 5;
pub const VMIN     : ::cc_t = 6;
pub const VSWTC    : ::cc_t = 7;
pub const VSTART   : ::cc_t = 8;
pub const VSTOP    : ::cc_t = 9;
pub const VSUSP    : ::cc_t = 10;
pub const VEOL     : ::cc_t = 11;
pub const VREPRINT : ::cc_t = 12;
pub const VDISCARD : ::cc_t = 13;
pub const VWERASE  : ::cc_t = 14;
pub const VLNEXT   : ::cc_t = 15;
pub const VEOL2    : ::cc_t = 16;

pub const IGNBRK  : ::tcflag_t = 0o000001;
pub const BRKINT  : ::tcflag_t = 0o000002;
pub const IGNPAR  : ::tcflag_t = 0o000004;
pub const PARMRK  : ::tcflag_t = 0o000010;
pub const INPCK   : ::tcflag_t = 0o000020;
pub const ISTRIP  : ::tcflag_t = 0o000040;
pub const INLCR   : ::tcflag_t = 0o000100;
pub const IGNCR   : ::tcflag_t = 0o000200;
pub const ICRNL   : ::tcflag_t = 0o000400;
pub const IUCLC   : ::tcflag_t = 0o001000;
pub const IXON    : ::tcflag_t = 0o002000;
pub const IXANY   : ::tcflag_t = 0o004000;
pub const IXOFF   : ::tcflag_t = 0o010000;
pub const IMAXBEL : ::tcflag_t = 0o020000;
pub const IUTF8   : ::tcflag_t = 0o040000;

pub const OPOST  : ::tcflag_t = 0o000001;
pub const OLCUC  : ::tcflag_t = 0o000002;
pub const ONLCR  : ::tcflag_t = 0o000004;
pub const OCRNL  : ::tcflag_t = 0o000010;
pub const ONOCR  : ::tcflag_t = 0o000020;
pub const ONLRET : ::tcflag_t = 0o000040;
pub const OFILL  : ::tcflag_t = 0o000100;
pub const OFDEL  : ::tcflag_t = 0o000200;
pub const NLDLY  : ::tcflag_t = 0o000400;
pub const NL0    : ::tcflag_t = 0o000000;
pub const NL1    : ::tcflag_t = 0o000400;
pub const CRDLY  : ::tcflag_t = 0o003000;
pub const CR0    : ::tcflag_t = 0o000000;
pub const CR1    : ::tcflag_t = 0o001000;
pub const CR2    : ::tcflag_t = 0o002000;
pub const CR3    : ::tcflag_t = 0o003000;
pub const TABDLY : ::tcflag_t = 0o014000;
pub const TAB0   : ::tcflag_t = 0o000000;
pub const TAB1   : ::tcflag_t = 0o004000;
pub const TAB2   : ::tcflag_t = 0o010000;
pub const TAB3   : ::tcflag_t = 0o014000;
pub const XTABS  : ::tcflag_t = 0o014000;
pub const BSDLY  : ::tcflag_t = 0o020000;
pub const BS0    : ::tcflag_t = 0o000000;
pub const BS1    : ::tcflag_t = 0o020000;
pub const VTDLY  : ::tcflag_t = 0o040000;
pub const VT0    : ::tcflag_t = 0o000000;
pub const VT1    : ::tcflag_t = 0o040000;
pub const FFDLY  : ::tcflag_t = 0o100000;
pub const FF0    : ::tcflag_t = 0o000000;
pub const FF1    : ::tcflag_t = 0o100000;

pub const CBAUD    : ::tcflag_t = 0o010017;
pub const B0       : ::tcflag_t = 0o000000;
pub const B50      : ::tcflag_t = 0o000001;
pub const B75      : ::tcflag_t = 0o000002;
pub const B110     : ::tcflag_t = 0o000003;
pub const B134     : ::tcflag_t = 0o000004;
pub const B150     : ::tcflag_t = 0o000005;
pub const B200     : ::tcflag_t = 0o000006;
pub const B300     : ::tcflag_t = 0o000007;
pub const B600     : ::tcflag_t = 0o000010;
pub const B1200    : ::tcflag_t = 0o000011;
pub const B1800    : ::tcflag_t = 0o000012;
pub const B2400    : ::tcflag_t = 0o000013;
pub const B4800    : ::tcflag_t = 0o000014;
pub const B9600    : ::tcflag_t = 0o000015;
pub const B19200   : ::tcflag_t = 0o000016;
pub const B38400   : ::tcflag_t = 0o000017;
pub const EXTA     : ::tcflag_t = ::B19200;
pub const EXTB     : ::tcflag_t = ::B38400;
pub const CSIZE    : ::tcflag_t = 0o000060;
pub const CS5      : ::tcflag_t = 0o000000;
pub const CS6      : ::tcflag_t = 0o000020;
pub const CS7      : ::tcflag_t = 0o000040;
pub const CS8      : ::tcflag_t = 0o000060;
pub const CSTOPB   : ::tcflag_t = 0o000100;
pub const CREAD    : ::tcflag_t = 0o000200;
pub const PARENB   : ::tcflag_t = 0o000400;
pub const PARODD   : ::tcflag_t = 0o001000;
pub const HUPCL    : ::tcflag_t = 0o002000;
pub const CLOCAL   : ::tcflag_t = 0o004000;
pub const CBAUDEX  : ::tcflag_t = 0o010000;
pub const BOTHER   : ::tcflag_t = 0o010000;
pub const B57600   : ::tcflag_t = 0o010001;
pub const B115200  : ::tcflag_t = 0o010002;
pub const B230400  : ::tcflag_t = 0o010003;
pub const B460800  : ::tcflag_t = 0o010004;
pub const B500000  : ::tcflag_t = 0o010005;
pub const B576000  : ::tcflag_t = 0o010006;
pub const B921600  : ::tcflag_t = 0o010007;
pub const B1000000 : ::tcflag_t = 0o010010;
pub const B1152000 : ::tcflag_t = 0o010011;
pub const B1500000 : ::tcflag_t = 0o010012;
pub const B2000000 : ::tcflag_t = 0o010013;
pub const B2500000 : ::tcflag_t = 0o010014;
pub const B3000000 : ::tcflag_t = 0o010015;
pub const B3500000 : ::tcflag_t = 0o010016;
pub const B4000000 : ::tcflag_t = 0o010017;
pub const CIBAUD   : ::tcflag_t = 0o02003600000;
pub const CMSPAR   : ::tcflag_t = 0o10000000000;
pub const CRTSCTS  : ::tcflag_t = 0o20000000000;

pub const IBSHIFT : ::tcflag_t = 16;

pub const ISIG    : ::tcflag_t = 0o000001;
pub const ICANON  : ::tcflag_t = 0o000002;
pub const XCASE   : ::tcflag_t = 0o000004;
pub const ECHO    : ::tcflag_t = 0o000010;
pub const ECHOE   : ::tcflag_t = 0o000020;
pub const ECHOK   : ::tcflag_t = 0o000040;
pub const ECHONL  : ::tcflag_t = 0o000100;
pub const NOFLSH  : ::tcflag_t = 0o000200;
pub const TOSTOP  : ::tcflag_t = 0o000400;
pub const ECHOCTL : ::tcflag_t = 0o001000;
pub const ECHOPRT : ::tcflag_t = 0o002000;
pub const ECHOKE  : ::tcflag_t = 0o004000;
pub const FLUSHO  : ::tcflag_t = 0o010000;
pub const PENDIN  : ::tcflag_t = 0o040000;
pub const IEXTEN  : ::tcflag_t = 0o100000;
pub const EXTPROC : ::tcflag_t = 0o200000;

pub const TCOOFF : ::c_uint = 0;
pub const TCOON  : ::c_uint = 1;
pub const TCIOFF : ::c_uint = 2;
pub const TCION  : ::c_uint = 3;

pub const TCIFLUSH  : ::c_uint = 0;
pub const TCOFLUSH  : ::c_uint = 1;
pub const TCIOFLUSH : ::c_uint = 2;

pub const TCSANOW   : ::c_uint = 0;
pub const TCSADRAIN : ::c_uint = 1;
pub const TCSAFLUSH : ::c_uint = 2;

// termios.h

#[repr(C)]
#[derive(Copy, Eq)]
pub struct winsize {
    pub ws_row:    ::c_ushort,
    pub ws_col:    ::c_ushort,
    pub ws_xpixel: ::c_ushort,
    pub ws_ypixel: ::c_ushort,
}

pub const NCC : usize = 8;

#[repr(C)]
#[derive(Copy, Eq)]
pub struct termio {
    pub c_iflag:   ::c_ushort,
    pub c_oflag:   ::c_ushort,
    pub c_cflag:   ::c_ushort,
    pub c_lflag:   ::c_ushort,
    pub c_line:    ::c_uchar,
    pub c_cc: [::c_uchar; ::NCC],
}

pub const TIOCM_LE   : ::c_uint = 0x001;
pub const TIOCM_DTR  : ::c_uint = 0x002;
pub const TIOCM_RTS  : ::c_uint = 0x004;
pub const TIOCM_ST   : ::c_uint = 0x008;
pub const TIOCM_SR   : ::c_uint = 0x010;
pub const TIOCM_CTS  : ::c_uint = 0x020;
pub const TIOCM_CAR  : ::c_uint = 0x040;
pub const TIOCM_RNG  : ::c_uint = 0x080;
pub const TIOCM_DSR  : ::c_uint = 0x100;
pub const TIOCM_CD   : ::c_uint = ::TIOCM_CAR;
pub const TIOCM_RI   : ::c_uint = ::TIOCM_RNG;
pub const TIOCM_OUT1 : ::c_uint = 0x2000;
pub const TIOCM_OUT2 : ::c_uint = 0x4000;
pub const TIOCM_LOOP : ::c_uint = 0x8000;

// poll.h

pub const POLLIN         : ::c_uint = 0x0001;
pub const POLLPRI        : ::c_uint = 0x0002;
pub const POLLOUT        : ::c_uint = 0x0004;
pub const POLLERR        : ::c_uint = 0x0008;
pub const POLLHUP        : ::c_uint = 0x0010;
pub const POLLNVAL       : ::c_uint = 0x0020;
pub const POLLRDNORM     : ::c_uint = 0x0040;
pub const POLLRDBAND     : ::c_uint = 0x0080;
pub const POLLWRNORM     : ::c_uint = 0x0100;
pub const POLLWRBAND     : ::c_uint = 0x0200;
pub const POLLMSG        : ::c_uint = 0x0400;
pub const POLLREMOVE     : ::c_uint = 0x1000;
pub const POLLRDHUP      : ::c_uint = 0x2000;
pub const POLLFREE       : ::c_uint = 0x4000;
pub const POLL_BUSY_LOOP : ::c_uint = 0x8000;

#[repr(C)]
#[derive(Copy, Eq)]
pub struct pollfd {
    pub fd:      ::c_int,
    pub events:  ::c_short,
    pub revents: ::c_short,
}

// resource.h

pub const RLIMIT_CPU        : ::c_ulong = 0;
pub const RLIMIT_FSIZE      : ::c_ulong = 1;
pub const RLIMIT_DATA       : ::c_ulong = 2;
pub const RLIMIT_STACK      : ::c_ulong = 3;
pub const RLIMIT_CORE       : ::c_ulong = 4;
pub const RLIMIT_RSS        : ::c_ulong = 5;
pub const RLIMIT_NPROC      : ::c_ulong = 6;
pub const RLIMIT_NOFILE     : ::c_ulong = 7;
pub const RLIMIT_MEMLOCK    : ::c_ulong = 8;
pub const RLIMIT_AS         : ::c_ulong = 9;
pub const RLIMIT_LOCKS      : ::c_ulong = 10;
pub const RLIMIT_SIGPENDING : ::c_ulong = 11;
pub const RLIMIT_MSGQUEUE   : ::c_ulong = 12;
pub const RLIMIT_NICE       : ::c_ulong = 13;
pub const RLIMIT_RTPRIO     : ::c_ulong = 14;
pub const RLIMIT_RTTIME     : ::c_ulong = 15;
pub const RLIM_NLIMITS      : ::c_ulong = 16;
pub const RLIM_INFINITY     : ::c_ulong = !0;

// shmbuf.h

#[repr(C)]
#[derive(Copy, Eq)]
pub struct shminfo64 {
    pub shmmax:    ::__kernel_ulong_t,
    pub shmmin:    ::__kernel_ulong_t,
    pub shmmni:    ::__kernel_ulong_t,
    pub shmseg:    ::__kernel_ulong_t,
    pub shmall:    ::__kernel_ulong_t,
    pub __unused1: ::__kernel_ulong_t,
    pub __unused2: ::__kernel_ulong_t,
    pub __unused3: ::__kernel_ulong_t,
    pub __unused4: ::__kernel_ulong_t,
}

// eventpoll.h

#[repr(C)]
#[derive(Copy, Eq)]
pub struct epoll_event {
    pub events: ::__u32,
    pub data:   ::__u64,
}

// fadvise.h

pub const POSIX_FADV_DONTNEED : ::c_int = 4;
pub const POSIX_FADV_NOREUSE  : ::c_int = 5;
