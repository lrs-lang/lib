// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(non_camel_case_types)]

// XXX: Only use fully qualified types here!!! The types in here might be overridden by
// the types in the non-generic files, therefore we can't use them directly.

pub type __s8  = ::cty::c_schar;
pub type __u8  = ::cty::c_uchar;
pub type __s16 = ::cty::c_short;
pub type __u16 = ::cty::c_ushort;
pub type __s32 = ::cty::c_int;
pub type __u32 = ::cty::c_uint;
pub type __s64 = ::cty::c_longlong;
pub type __u64 = ::cty::c_ulonglong;

pub const __BITS_PER_LONG : usize = 32;
pub const BYTES_PER_KERNEL_MODE_T : usize = ::cty::BYTES_PER_INT;

pub type __kernel_long_t      = ::cty::c_long;
pub type __kernel_ulong_t     = ::cty::c_ulong;
pub type __kernel_ino_t       = ::cty::__kernel_ulong_t;
pub type __kernel_mode_t      = ::cty::c_uint;
pub type __kernel_pid_t       = ::cty::c_int;
pub type __kernel_ipc_pid_t   = ::cty::c_int;
pub type __kernel_uid_t       = ::cty::c_uint;
pub type __kernel_gid_t       = ::cty::c_uint;
pub type __kernel_suseconds_t = ::cty::__kernel_long_t;
pub type __kernel_daddr_t     = ::cty::c_int;
pub type __kernel_uid32_t     = ::cty::c_uint;
pub type __kernel_gid32_t     = ::cty::c_uint;
pub type __kernel_old_uid_t   = ::cty::__kernel_uid_t;
pub type __kernel_old_gid_t   = ::cty::__kernel_gid_t;
pub type __kernel_old_dev_t   = ::cty::c_uint;
pub type __kernel_off_t       = ::cty::__kernel_long_t;
pub type __kernel_loff_t      = ::cty::c_longlong;
pub type __kernel_time_t      = ::cty::__kernel_long_t;
pub type __kernel_clock_t     = ::cty::__kernel_long_t;
pub type __kernel_timer_t     = ::cty::c_int;
pub type __kernel_clockid_t   = ::cty::c_int;
pub type __kernel_caddr_t     = *mut ::cty::c_char;
pub type __kernel_uid16_t     = ::cty::c_ushort;
pub type __kernel_gid16_t     = ::cty::c_ushort;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct __kernel_fsid_t {
    pub val: [::cty::c_int; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct epoll_event {
	pub events: ::cty::__u32,
	pub data:   ::cty::__u64,
}

// errno-base.h & errno.h

pub const EPERM           : ::cty::c_int = 1;
pub const ENOENT          : ::cty::c_int = 2;
pub const ESRCH           : ::cty::c_int = 3;
pub const EINTR           : ::cty::c_int = 4;
pub const EIO             : ::cty::c_int = 5;
pub const ENXIO           : ::cty::c_int = 6;
pub const E2BIG           : ::cty::c_int = 7;
pub const ENOEXEC         : ::cty::c_int = 8;
pub const EBADF           : ::cty::c_int = 9;
pub const ECHILD          : ::cty::c_int = 10;
pub const EAGAIN          : ::cty::c_int = 11;
pub const ENOMEM          : ::cty::c_int = 12;
pub const EACCES          : ::cty::c_int = 13;
pub const EFAULT          : ::cty::c_int = 14;
pub const ENOTBLK         : ::cty::c_int = 15;
pub const EBUSY           : ::cty::c_int = 16;
pub const EEXIST          : ::cty::c_int = 17;
pub const EXDEV           : ::cty::c_int = 18;
pub const ENODEV          : ::cty::c_int = 19;
pub const ENOTDIR         : ::cty::c_int = 20;
pub const EISDIR          : ::cty::c_int = 21;
pub const EINVAL          : ::cty::c_int = 22;
pub const ENFILE          : ::cty::c_int = 23;
pub const EMFILE          : ::cty::c_int = 24;
pub const ENOTTY          : ::cty::c_int = 25;
pub const ETXTBSY         : ::cty::c_int = 26;
pub const EFBIG           : ::cty::c_int = 27;
pub const ENOSPC          : ::cty::c_int = 28;
pub const ESPIPE          : ::cty::c_int = 29;
pub const EROFS           : ::cty::c_int = 30;
pub const EMLINK          : ::cty::c_int = 31;
pub const EPIPE           : ::cty::c_int = 32;
pub const EDOM            : ::cty::c_int = 33;
pub const ERANGE          : ::cty::c_int = 34;
pub const EDEADLK         : ::cty::c_int = 35;
pub const ENAMETOOLONG    : ::cty::c_int = 36;
pub const ENOLCK          : ::cty::c_int = 37;
pub const ENOSYS          : ::cty::c_int = 38;
pub const ENOTEMPTY       : ::cty::c_int = 39;
pub const ELOOP           : ::cty::c_int = 40;
pub const EWOULDBLOCK     : ::cty::c_int = ::cty::EAGAIN;
pub const ENOMSG          : ::cty::c_int = 42;
pub const EIDRM           : ::cty::c_int = 43;
pub const ECHRNG          : ::cty::c_int = 44;
pub const EL2NSYNC        : ::cty::c_int = 45;
pub const EL3HLT          : ::cty::c_int = 46;
pub const EL3RST          : ::cty::c_int = 47;
pub const ELNRNG          : ::cty::c_int = 48;
pub const EUNATCH         : ::cty::c_int = 49;
pub const ENOCSI          : ::cty::c_int = 50;
pub const EL2HLT          : ::cty::c_int = 51;
pub const EBADE           : ::cty::c_int = 52;
pub const EBADR           : ::cty::c_int = 53;
pub const EXFULL          : ::cty::c_int = 54;
pub const ENOANO          : ::cty::c_int = 55;
pub const EBADRQC         : ::cty::c_int = 56;
pub const EBADSLT         : ::cty::c_int = 57;
pub const EDEADLOCK       : ::cty::c_int = ::cty::EDEADLK;
pub const EBFONT          : ::cty::c_int = 59;
pub const ENOSTR          : ::cty::c_int = 60;
pub const ENODATA         : ::cty::c_int = 61;
pub const ETIME           : ::cty::c_int = 62;
pub const ENOSR           : ::cty::c_int = 63;
pub const ENONET          : ::cty::c_int = 64;
pub const ENOPKG          : ::cty::c_int = 65;
pub const EREMOTE         : ::cty::c_int = 66;
pub const ENOLINK         : ::cty::c_int = 67;
pub const EADV            : ::cty::c_int = 68;
pub const ESRMNT          : ::cty::c_int = 69;
pub const ECOMM           : ::cty::c_int = 70;
pub const EPROTO          : ::cty::c_int = 71;
pub const EMULTIHOP       : ::cty::c_int = 72;
pub const EDOTDOT         : ::cty::c_int = 73;
pub const EBADMSG         : ::cty::c_int = 74;
pub const EOVERFLOW       : ::cty::c_int = 75;
pub const ENOTUNIQ        : ::cty::c_int = 76;
pub const EBADFD          : ::cty::c_int = 77;
pub const EREMCHG         : ::cty::c_int = 78;
pub const ELIBACC         : ::cty::c_int = 79;
pub const ELIBBAD         : ::cty::c_int = 80;
pub const ELIBSCN         : ::cty::c_int = 81;
pub const ELIBMAX         : ::cty::c_int = 82;
pub const ELIBEXEC        : ::cty::c_int = 83;
pub const EILSEQ          : ::cty::c_int = 84;
pub const ERESTART        : ::cty::c_int = 85;
pub const ESTRPIPE        : ::cty::c_int = 86;
pub const EUSERS          : ::cty::c_int = 87;
pub const ENOTSOCK        : ::cty::c_int = 88;
pub const EDESTADDRREQ    : ::cty::c_int = 89;
pub const EMSGSIZE        : ::cty::c_int = 90;
pub const EPROTOTYPE      : ::cty::c_int = 91;
pub const ENOPROTOOPT     : ::cty::c_int = 92;
pub const EPROTONOSUPPORT : ::cty::c_int = 93;
pub const ESOCKTNOSUPPORT : ::cty::c_int = 94;
pub const EOPNOTSUPP      : ::cty::c_int = 95;
pub const EPFNOSUPPORT    : ::cty::c_int = 96;
pub const EAFNOSUPPORT    : ::cty::c_int = 97;
pub const EADDRINUSE      : ::cty::c_int = 98;
pub const EADDRNOTAVAIL   : ::cty::c_int = 99;
pub const ENETDOWN        : ::cty::c_int = 100;
pub const ENETUNREACH     : ::cty::c_int = 101;
pub const ENETRESET       : ::cty::c_int = 102;
pub const ECONNABORTED    : ::cty::c_int = 103;
pub const ECONNRESET      : ::cty::c_int = 104;
pub const ENOBUFS         : ::cty::c_int = 105;
pub const EISCONN         : ::cty::c_int = 106;
pub const ENOTCONN        : ::cty::c_int = 107;
pub const ESHUTDOWN       : ::cty::c_int = 108;
pub const ETOOMANYREFS    : ::cty::c_int = 109;
pub const ETIMEDOUT       : ::cty::c_int = 110;
pub const ECONNREFUSED    : ::cty::c_int = 111;
pub const EHOSTDOWN       : ::cty::c_int = 112;
pub const EHOSTUNREACH    : ::cty::c_int = 113;
pub const EALREADY        : ::cty::c_int = 114;
pub const EINPROGRESS     : ::cty::c_int = 115;
pub const ESTALE          : ::cty::c_int = 116;
pub const EUCLEAN         : ::cty::c_int = 117;
pub const ENOTNAM         : ::cty::c_int = 118;
pub const ENAVAIL         : ::cty::c_int = 119;
pub const EISNAM          : ::cty::c_int = 120;
pub const EREMOTEIO       : ::cty::c_int = 121;
pub const EDQUOT          : ::cty::c_int = 122;
pub const ENOMEDIUM       : ::cty::c_int = 123;
pub const EMEDIUMTYPE     : ::cty::c_int = 124;
pub const ECANCELED       : ::cty::c_int = 125;
pub const ENOKEY          : ::cty::c_int = 126;
pub const EKEYEXPIRED     : ::cty::c_int = 127;
pub const EKEYREVOKED     : ::cty::c_int = 128;
pub const EKEYREJECTED    : ::cty::c_int = 129;
pub const EOWNERDEAD      : ::cty::c_int = 130;
pub const ENOTRECOVERABLE : ::cty::c_int = 131;
pub const ERFKILL         : ::cty::c_int = 132;
pub const EHWPOISON       : ::cty::c_int = 133;

// stat.h

pub const STAT_HAVE_NSEC: ::cty::c_int = 1;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct stat {
	pub st_dev:        ::cty::c_ulong,
	pub st_ino:        ::cty::c_ulong,
	pub st_mode:       ::cty::c_uint,
	pub st_nlink:      ::cty::c_uint,
	pub st_uid:        ::cty::c_uint,
	pub st_gid:        ::cty::c_uint,
	pub st_rdev:       ::cty::c_ulong,
	pub __pad1:        ::cty::c_ulong,
	pub st_size:       ::cty::c_long,
	pub st_blksize:    ::cty::c_int,
	pub __pad2:        ::cty::c_int,
	pub st_blocks:     ::cty::c_long,
	pub st_atime:      ::cty::c_long,
	pub st_atime_nsec: ::cty::c_ulong,
	pub st_mtime:      ::cty::c_long,
	pub st_mtime_nsec: ::cty::c_ulong,
	pub st_ctime:      ::cty::c_long,
	pub st_ctime_nsec: ::cty::c_ulong,
	pub __unused4:     ::cty::c_uint,
	pub __unused5:     ::cty::c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct stat64 {
	pub st_dev:        ::cty::c_ulonglong,
	pub st_ino:        ::cty::c_ulonglong,
	pub st_mode:       ::cty::c_uint,
	pub st_nlink:      ::cty::c_uint,
	pub st_uid:        ::cty::c_uint,
	pub st_gid:        ::cty::c_uint,
	pub st_rdev:       ::cty::c_ulonglong,
	pub __pad1:        ::cty::c_ulonglong,
	pub st_size:       ::cty::c_longlong,
	pub st_blksize:    ::cty::c_int,
	pub __pad2:        ::cty::c_int,
	pub st_blocks:     ::cty::c_longlong,
	pub st_atime:      ::cty::c_int,
	pub st_atime_nsec: ::cty::c_uint,
	pub st_mtime:      ::cty::c_int,
	pub st_mtime_nsec: ::cty::c_uint,
	pub st_ctime:      ::cty::c_int,
	pub st_ctime_nsec: ::cty::c_uint,
	pub __unused4:     ::cty::c_uint,
	pub __unused5:     ::cty::c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct statfs {
	pub f_type:    ::cty::__statfs_word,
	pub f_bsize:   ::cty::__statfs_word,
	pub f_blocks:  ::cty::__statfs_word,
	pub f_bfree:   ::cty::__statfs_word,
	pub f_bavail:  ::cty::__statfs_word,
	pub f_files:   ::cty::__statfs_word,
	pub f_ffree:   ::cty::__statfs_word,
	pub f_fsid:    ::cty::__kernel_fsid_t,
	pub f_namelen: ::cty::__statfs_word,
	pub f_frsize:  ::cty::__statfs_word,
	pub f_flags:   ::cty::__statfs_word,
	pub f_spare:   [::cty::__statfs_word; 4],
}

// signal-defs.h

pub const SIG_BLOCK   : ::cty::c_int = 0;
pub const SIG_UNBLOCK : ::cty::c_int = 1;
pub const SIG_SETMASK : ::cty::c_int = 2;

// can't write this in Rust
// type __signalfn_t = *(extern fn(c_int));
pub type __sighandler_t = extern fn(::cty::c_int);

// can't write this in Rust
// type __restorefn_t = *(extern fn());
pub type __sigrestore_t = extern fn();

pub const SIG_DFL : usize = 0;
pub const SIG_IGN : usize = 1;
pub const SIG_ERR : usize = !0;

// signal.h

pub const _NSIG       : usize = 64;
pub const _NSIG_BPW   : usize = ::cty::__BITS_PER_LONG;
pub const _NSIG_WORDS : usize = ::cty::_NSIG / ::cty::_NSIG_BPW;

pub const SIGHUP    : ::cty::c_int = 1;
pub const SIGINT    : ::cty::c_int = 2;
pub const SIGQUIT   : ::cty::c_int = 3;
pub const SIGILL    : ::cty::c_int = 4;
pub const SIGTRAP   : ::cty::c_int = 5;
pub const SIGABRT   : ::cty::c_int = 6;
pub const SIGIOT    : ::cty::c_int = 6;
pub const SIGBUS    : ::cty::c_int = 7;
pub const SIGFPE    : ::cty::c_int = 8;
pub const SIGKILL   : ::cty::c_int = 9;
pub const SIGUSR1   : ::cty::c_int = 10;
pub const SIGSEGV   : ::cty::c_int = 11;
pub const SIGUSR2   : ::cty::c_int = 12;
pub const SIGPIPE   : ::cty::c_int = 13;
pub const SIGALRM   : ::cty::c_int = 14;
pub const SIGTERM   : ::cty::c_int = 15;
pub const SIGSTKFLT : ::cty::c_int = 16;
pub const SIGCHLD   : ::cty::c_int = 17;
pub const SIGCONT   : ::cty::c_int = 18;
pub const SIGSTOP   : ::cty::c_int = 19;
pub const SIGTSTP   : ::cty::c_int = 20;
pub const SIGTTIN   : ::cty::c_int = 21;
pub const SIGTTOU   : ::cty::c_int = 22;
pub const SIGURG    : ::cty::c_int = 23;
pub const SIGXCPU   : ::cty::c_int = 24;
pub const SIGXFSZ   : ::cty::c_int = 25;
pub const SIGVTALRM : ::cty::c_int = 26;
pub const SIGPROF   : ::cty::c_int = 27;
pub const SIGWINCH  : ::cty::c_int = 28;
pub const SIGIO     : ::cty::c_int = 29;
pub const SIGPOLL   : ::cty::c_int = ::cty::SIGIO;
pub const SIGPWR    : ::cty::c_int = 30;
pub const SIGSYS    : ::cty::c_int = 31;
pub const SIGUNUSED : ::cty::c_int = 31;

pub const SA_NOCLDSTOP : ::cty::c_int = 0x00000001;
pub const SA_NOCLDWAIT : ::cty::c_int = 0x00000002;
pub const SA_SIGINFO   : ::cty::c_int = 0x00000004;
pub const SA_ONSTACK   : ::cty::c_int = 0x08000000;
pub const SA_RESTART   : ::cty::c_int = 0x10000000;
pub const SA_NODEFER   : ::cty::c_int = 0x40000000;
#[allow(overflowing_literals)]
pub const SA_RESETHAND : ::cty::c_int = 0x80000000;
pub const SA_NOMASK    : ::cty::c_int = ::cty::SA_NODEFER;
pub const SA_ONESHOT   : ::cty::c_int = ::cty::SA_RESETHAND;

pub const MINSIGSTKSZ : usize = 2048;
pub const SIGSTKSZ    : usize = 8192;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct sigset_t {
    pub sig: [::cty::c_ulong; ::cty::_NSIG_WORDS],
}

pub type old_sigset_t = ::cty::c_ulong;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct sigaltstack {
	pub ss_sp: *mut ::cty::c_void,
	pub ss_flags: ::cty::c_int,
	pub ss_size: ::cty::size_t,
}

pub type stack_t = ::cty::sigaltstack;

// sockios.h

pub const FIOSETOWN    : ::cty::c_int = 0x8901;
pub const SIOCSPGRP    : ::cty::c_int = 0x8902;
pub const FIOGETOWN    : ::cty::c_int = 0x8903;
pub const SIOCGPGRP    : ::cty::c_int = 0x8904;
pub const SIOCATMARK   : ::cty::c_int = 0x8905;
pub const SIOCGSTAMP   : ::cty::c_int = 0x8906;
pub const SIOCGSTAMPNS : ::cty::c_int = 0x8907;

// socket.h

pub const SOL_SOCKET                       : ::cty::c_int = 1;
pub const SO_DEBUG                         : ::cty::c_int = 1;
pub const SO_REUSEADDR                     : ::cty::c_int = 2;
pub const SO_TYPE                          : ::cty::c_int = 3;
pub const SO_ERROR                         : ::cty::c_int = 4;
pub const SO_DONTROUTE                     : ::cty::c_int = 5;
pub const SO_BROADCAST                     : ::cty::c_int = 6;
pub const SO_SNDBUF                        : ::cty::c_int = 7;
pub const SO_RCVBUF                        : ::cty::c_int = 8;
pub const SO_SNDBUFFORCE                   : ::cty::c_int = 32;
pub const SO_RCVBUFFORCE                   : ::cty::c_int = 33;
pub const SO_KEEPALIVE                     : ::cty::c_int = 9;
pub const SO_OOBINLINE                     : ::cty::c_int = 10;
pub const SO_NO_CHECK                      : ::cty::c_int = 11;
pub const SO_PRIORITY                      : ::cty::c_int = 12;
pub const SO_LINGER                        : ::cty::c_int = 13;
pub const SO_BSDCOMPAT                     : ::cty::c_int = 14;
pub const SO_REUSEPORT                     : ::cty::c_int = 15;
pub const SO_PASSCRED                      : ::cty::c_int = 16;
pub const SO_PEERCRED                      : ::cty::c_int = 17;
pub const SO_RCVLOWAT                      : ::cty::c_int = 18;
pub const SO_SNDLOWAT                      : ::cty::c_int = 19;
pub const SO_RCVTIMEO                      : ::cty::c_int = 20;
pub const SO_SNDTIMEO                      : ::cty::c_int = 21;
pub const SO_SECURITY_AUTHENTICATION       : ::cty::c_int = 22;
pub const SO_SECURITY_ENCRYPTION_TRANSPORT : ::cty::c_int = 23;
pub const SO_SECURITY_ENCRYPTION_NETWORK   : ::cty::c_int = 24;
pub const SO_BINDTODEVICE                  : ::cty::c_int = 25;
pub const SO_ATTACH_FILTER                 : ::cty::c_int = 26;
pub const SO_DETACH_FILTER                 : ::cty::c_int = 27;
pub const SO_GET_FILTER                    : ::cty::c_int = SO_ATTACH_FILTER;
pub const SO_PEERNAME                      : ::cty::c_int = 28;
pub const SO_TIMESTAMP                     : ::cty::c_int = 29;
pub const SCM_TIMESTAMP                    : ::cty::c_int = SO_TIMESTAMP;
pub const SO_ACCEPTCONN                    : ::cty::c_int = 30;
pub const SO_PEERSEC                       : ::cty::c_int = 31;
pub const SO_PASSSEC                       : ::cty::c_int = 34;
pub const SO_TIMESTAMPNS                   : ::cty::c_int = 35;
pub const SCM_TIMESTAMPNS                  : ::cty::c_int = SO_TIMESTAMPNS;
pub const SO_MARK                          : ::cty::c_int = 36;
pub const SO_TIMESTAMPING                  : ::cty::c_int = 37;
pub const SCM_TIMESTAMPING                 : ::cty::c_int = SO_TIMESTAMPING;
pub const SO_PROTOCOL                      : ::cty::c_int = 38;
pub const SO_DOMAIN                        : ::cty::c_int = 39;
pub const SO_RXQ_OVFL                      : ::cty::c_int = 40;
pub const SO_WIFI_STATUS                   : ::cty::c_int = 41;
pub const SCM_WIFI_STATUS                  : ::cty::c_int = SO_WIFI_STATUS;
pub const SO_PEEK_OFF                      : ::cty::c_int = 42;
pub const SO_NOFCS                         : ::cty::c_int = 43;
pub const SO_LOCK_FILTER                   : ::cty::c_int = 44;
pub const SO_SELECT_ERR_QUEUE              : ::cty::c_int = 45;
pub const SO_BUSY_POLL                     : ::cty::c_int = 46;
pub const SO_MAX_PACING_RATE               : ::cty::c_int = 47;
pub const SO_BPF_EXTENSIONS                : ::cty::c_int = 48;

// fcntl.h

pub const O_ACCMODE       : ::cty::c_int = 0o0000003;
pub const O_RDONLY        : ::cty::c_int = 0o0000000;
pub const O_WRONLY        : ::cty::c_int = 0o0000001;
pub const O_RDWR          : ::cty::c_int = 0o0000002;
pub const O_CREAT         : ::cty::c_int = 0o0000100;
pub const O_EXCL          : ::cty::c_int = 0o0000200;
pub const O_NOCTTY        : ::cty::c_int = 0o0000400;
pub const O_TRUNC         : ::cty::c_int = 0o0001000;
pub const O_APPEND        : ::cty::c_int = 0o0002000;
pub const O_NONBLOCK      : ::cty::c_int = 0o0004000;
pub const O_DSYNC         : ::cty::c_int = 0o0010000;
pub const FASYNC          : ::cty::c_int = 0o0020000;
pub const O_DIRECT        : ::cty::c_int = 0o0040000;
pub const O_LARGEFILE     : ::cty::c_int = 0o0100000;
pub const O_DIRECTORY     : ::cty::c_int = 0o0200000;
pub const O_NOFOLLOW      : ::cty::c_int = 0o0400000;
pub const O_NOATIME       : ::cty::c_int = 0o1000000;
pub const O_CLOEXEC       : ::cty::c_int = 0o2000000;
pub const __O_SYNC        : ::cty::c_int = 0o4000000;
pub const O_SYNC          : ::cty::c_int = __O_SYNC|O_DSYNC;
pub const O_PATH          : ::cty::c_int = 0o10000000;
pub const __O_TMPFILE     : ::cty::c_int = 0o20000000;
pub const O_TMPFILE       : ::cty::c_int = __O_TMPFILE|O_DIRECTORY;
pub const O_TMPFILE_MASK  : ::cty::c_int = __O_TMPFILE|O_DIRECTORY|O_CREAT;
pub const O_NDELAY        : ::cty::c_int = O_NONBLOCK;
pub const F_DUPFD         : ::cty::c_int = 0;
pub const F_GETFD         : ::cty::c_int = 1;
pub const F_SETFD         : ::cty::c_int = 2;
pub const F_GETFL         : ::cty::c_int = 3;
pub const F_SETFL         : ::cty::c_int = 4;
pub const F_GETLK         : ::cty::c_int = 5;
pub const F_SETLK         : ::cty::c_int = 6;
pub const F_SETLKW        : ::cty::c_int = 7;
pub const F_SETOWN        : ::cty::c_int = 8;
pub const F_GETOWN        : ::cty::c_int = 9;
pub const F_SETSIG        : ::cty::c_int = 10;
pub const F_GETSIG        : ::cty::c_int = 11;
pub const F_GETLK64       : ::cty::c_int = 12;
pub const F_SETLK64       : ::cty::c_int = 13;
pub const F_SETLKW64      : ::cty::c_int = 14;
pub const F_SETOWN_EX     : ::cty::c_int = 15;
pub const F_GETOWN_EX     : ::cty::c_int = 16;
pub const F_GETOWNER_UIDS : ::cty::c_int = 17;
pub const F_OFD_GETLK     : ::cty::c_int = 36;
pub const F_OFD_SETLK     : ::cty::c_int = 37;
pub const F_OFD_SETLKW    : ::cty::c_int = 38;
pub const F_OWNER_TID     : ::cty::c_int = 0;
pub const F_OWNER_PID     : ::cty::c_int = 1;
pub const F_OWNER_PGRP    : ::cty::c_int = 2;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct f_owner_ex {
    pub type_: ::cty::c_int,
    pub pid: ::cty::__kernel_pid_t,
}

pub const FD_CLOEXEC            : ::cty::c_int = 1;
pub const F_RDLCK               : ::cty::c_int = 0;
pub const F_WRLCK               : ::cty::c_int = 1;
pub const F_UNLCK               : ::cty::c_int = 2;
pub const F_EXLCK               : ::cty::c_int = 4;
pub const F_SHLCK               : ::cty::c_int = 8;
pub const LOCK_SH               : ::cty::c_int = 1;
pub const LOCK_EX               : ::cty::c_int = 2;
pub const LOCK_NB               : ::cty::c_int = 4;
pub const LOCK_UN               : ::cty::c_int = 8;
pub const LOCK_MAND             : ::cty::c_int = 32;
pub const LOCK_READ             : ::cty::c_int = 64;
pub const LOCK_WRITE            : ::cty::c_int = 128;
pub const LOCK_RW               : ::cty::c_int = 192;
pub const F_LINUX_SPECIFIC_BASE : ::cty::c_int = 1024;

// ipcbuf.h

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ipc64_perm {
    pub key:  ::cty::__kernel_key_t,
    pub uid:  ::cty::__kernel_uid32_t,
    pub gid:  ::cty::__kernel_gid32_t,
    pub cuid: ::cty::__kernel_uid32_t,
    pub cgid: ::cty::__kernel_gid32_t,
    pub mode: ::cty::__kernel_mode_t,
    pub __pad1: [::cty::c_uchar; 4 - ::cty::BYTES_PER_KERNEL_MODE_T],
    pub seq:       ::cty::c_ushort,
    pub __pad2:    ::cty::c_ushort,
    pub __unused1: ::cty::__kernel_ulong_t,
    pub __unused2: ::cty::__kernel_ulong_t,
}

// ioctl.h

pub const _IOC_NRBITS   : ::cty::c_uint = 8;
pub const _IOC_TYPEBITS : ::cty::c_uint = 8;

pub const _IOC_SIZEBITS : ::cty::c_uint = 14;
pub const _IOC_DIRBITS  : ::cty::c_uint = 2;

pub const _IOC_NRMASK    : ::cty::c_uint = (1 << ::cty::_IOC_NRBITS) - 1;
pub const _IOC_TYPEMASK  : ::cty::c_uint = (1 << ::cty::_IOC_TYPEBITS) - 1;
pub const _IOC_SIZEMASK  : ::cty::c_uint = (1 << ::cty::_IOC_SIZEBITS) - 1;
pub const _IOC_DIRMASK   : ::cty::c_uint = (1 << ::cty::_IOC_DIRBITS) - 1;
pub const _IOC_NRSHIFT   : ::cty::c_uint = 0;
pub const _IOC_TYPESHIFT : ::cty::c_uint = ::cty::_IOC_NRSHIFT + ::cty::_IOC_NRBITS;
pub const _IOC_SIZESHIFT : ::cty::c_uint = ::cty::_IOC_TYPESHIFT + ::cty::_IOC_TYPEBITS;
pub const _IOC_DIRSHIFT  : ::cty::c_uint = ::cty::_IOC_SIZESHIFT + ::cty::_IOC_SIZEBITS;

pub const _IOC_NONE  : ::cty::c_uint = 0;
pub const _IOC_WRITE : ::cty::c_uint = 1;
pub const _IOC_READ  : ::cty::c_uint = 2;

pub fn _IOC(dir: ::cty::c_uint, ty: ::cty::c_uint, nr: ::cty::c_uint,
            size: ::cty::c_uint) -> ::cty::c_uint {
	(dir << ::cty::_IOC_DIRSHIFT) | (ty   << ::cty::_IOC_TYPESHIFT) |
	(nr  << ::cty::_IOC_NRSHIFT)  | (size << ::cty::_IOC_SIZESHIFT)
}

pub fn _IOC_TYPECHECK<T>(_: T) -> ::cty::c_uint { ::std::mem::size_of::<T>() as ::cty::c_uint }

pub fn _IO(ty: ::cty::c_uint, nr: ::cty::c_uint) -> ::cty::c_uint {
    _IOC(::cty::_IOC_NONE, ty, nr, 0)
}

pub fn _IOR<T>(ty: ::cty::c_uint, nr: ::cty::c_uint) -> ::cty::c_uint {
    _IOC(::cty::_IOC_READ, ty, nr, ::std::mem::size_of::<T>() as ::cty::c_uint)
}

pub fn _IOW<T>(ty: ::cty::c_uint, nr: ::cty::c_uint) -> ::cty::c_uint {
    _IOC(::cty::_IOC_WRITE, ty, nr, ::std::mem::size_of::<T>() as ::cty::c_uint)
}

pub fn _IOWR<T>(ty: ::cty::c_uint, nr: ::cty::c_uint) -> ::cty::c_uint {
    _IOC(::cty::_IOC_READ|::cty::_IOC_WRITE, ty, nr, ::std::mem::size_of::<T>() as ::cty::c_uint)
}

pub fn _IOR_BAD<T>(ty: ::cty::c_uint, nr: ::cty::c_uint) -> ::cty::c_uint {
    _IOC(::cty::_IOC_READ, ty, nr, ::std::mem::size_of::<T>() as ::cty::c_uint)
}

pub fn _IOW_BAD<T>(ty: ::cty::c_uint, nr: ::cty::c_uint) -> ::cty::c_uint {
    _IOC(::cty::_IOC_WRITE, ty, nr, ::std::mem::size_of::<T>() as ::cty::c_uint)
}

pub fn _IOWR_BAD<T>(ty: ::cty::c_uint, nr: ::cty::c_uint) -> ::cty::c_uint {
    _IOC(::cty::_IOC_READ|::cty::_IOC_WRITE, ty, nr, ::std::mem::size_of::<T>() as ::cty::c_uint)
}

pub fn _IOC_DIR(nr:  ::cty::c_uint) -> ::cty::c_uint { (nr >> ::cty::_IOC_DIRSHIFT)  & ::cty::_IOC_DIRMASK  }
pub fn _IOC_TYPE(nr: ::cty::c_uint) -> ::cty::c_uint { (nr >> ::cty::_IOC_TYPESHIFT) & ::cty::_IOC_TYPEMASK }
pub fn _IOC_NR(nr:   ::cty::c_uint) -> ::cty::c_uint { (nr >> ::cty::_IOC_NRSHIFT)   & ::cty::_IOC_NRMASK   }
pub fn _IOC_SIZE(nr: ::cty::c_uint) -> ::cty::c_uint { (nr >> ::cty::_IOC_SIZESHIFT) & ::cty::_IOC_SIZEMASK }

pub const IOC_IN        : ::cty::c_uint = _IOC_WRITE             << _IOC_DIRSHIFT;
pub const IOC_OUT       : ::cty::c_uint = _IOC_READ              << _IOC_DIRSHIFT;
pub const IOC_INOUT     : ::cty::c_uint = (_IOC_WRITE|_IOC_READ) << _IOC_DIRSHIFT;
pub const IOCSIZE_MASK  : ::cty::c_uint = _IOC_SIZEMASK          << _IOC_SIZESHIFT;
pub const IOCSIZE_SHIFT : ::cty::c_uint = _IOC_SIZESHIFT;

// ioctls.h

pub const TCGETS             : ::cty::c_uint = 0x5401;
pub const TCSETS             : ::cty::c_uint = 0x5402;
pub const TCSETSW            : ::cty::c_uint = 0x5403;
pub const TCSETSF            : ::cty::c_uint = 0x5404;
pub const TCGETA             : ::cty::c_uint = 0x5405;
pub const TCSETA             : ::cty::c_uint = 0x5406;
pub const TCSETAW            : ::cty::c_uint = 0x5407;
pub const TCSETAF            : ::cty::c_uint = 0x5408;
pub const TCSBRK             : ::cty::c_uint = 0x5409;
pub const TCXONC             : ::cty::c_uint = 0x540A;
pub const TCFLSH             : ::cty::c_uint = 0x540B;
pub const TIOCEXCL           : ::cty::c_uint = 0x540C;
pub const TIOCNXCL           : ::cty::c_uint = 0x540D;
pub const TIOCSCTTY          : ::cty::c_uint = 0x540E;
pub const TIOCGPGRP          : ::cty::c_uint = 0x540F;
pub const TIOCSPGRP          : ::cty::c_uint = 0x5410;
pub const TIOCOUTQ           : ::cty::c_uint = 0x5411;
pub const TIOCSTI            : ::cty::c_uint = 0x5412;
pub const TIOCGWINSZ         : ::cty::c_uint = 0x5413;
pub const TIOCSWINSZ         : ::cty::c_uint = 0x5414;
pub const TIOCMGET           : ::cty::c_uint = 0x5415;
pub const TIOCMBIS           : ::cty::c_uint = 0x5416;
pub const TIOCMBIC           : ::cty::c_uint = 0x5417;
pub const TIOCMSET           : ::cty::c_uint = 0x5418;
pub const TIOCGSOFTCAR       : ::cty::c_uint = 0x5419;
pub const TIOCSSOFTCAR       : ::cty::c_uint = 0x541A;
pub const FIONREAD           : ::cty::c_uint = 0x541B;
pub const TIOCINQ            : ::cty::c_uint = FIONREAD;
pub const TIOCLINUX          : ::cty::c_uint = 0x541C;
pub const TIOCCONS           : ::cty::c_uint = 0x541D;
pub const TIOCGSERIAL        : ::cty::c_uint = 0x541E;
pub const TIOCSSERIAL        : ::cty::c_uint = 0x541F;
pub const TIOCPKT            : ::cty::c_uint = 0x5420;
pub const FIONBIO            : ::cty::c_uint = 0x5421;
pub const TIOCNOTTY          : ::cty::c_uint = 0x5422;
pub const TIOCSETD           : ::cty::c_uint = 0x5423;
pub const TIOCGETD           : ::cty::c_uint = 0x5424;
pub const TCSBRKP            : ::cty::c_uint = 0x5425;
pub const TIOCSBRK           : ::cty::c_uint = 0x5427;
pub const TIOCCBRK           : ::cty::c_uint = 0x5428;
pub const TIOCGSID           : ::cty::c_uint = 0x5429;
pub const TIOCGRS485         : ::cty::c_uint = 0x542E;
pub const TIOCSRS485         : ::cty::c_uint = 0x542F;
pub const TCGETX             : ::cty::c_uint = 0x5432;
pub const TCSETX             : ::cty::c_uint = 0x5433;
pub const TCSETXF            : ::cty::c_uint = 0x5434;
pub const TCSETXW            : ::cty::c_uint = 0x5435;
pub const TIOCVHANGUP        : ::cty::c_uint = 0x5437;
pub const FIONCLEX           : ::cty::c_uint = 0x5450;
pub const FIOCLEX            : ::cty::c_uint = 0x5451;
pub const FIOASYNC           : ::cty::c_uint = 0x5452;
pub const TIOCSERCONFIG      : ::cty::c_uint = 0x5453;
pub const TIOCSERGWILD       : ::cty::c_uint = 0x5454;
pub const TIOCSERSWILD       : ::cty::c_uint = 0x5455;
pub const TIOCGLCKTRMIOS     : ::cty::c_uint = 0x5456;
pub const TIOCSLCKTRMIOS     : ::cty::c_uint = 0x5457;
pub const TIOCSERGSTRUCT     : ::cty::c_uint = 0x5458;
pub const TIOCSERGETLSR      : ::cty::c_uint = 0x5459;
pub const TIOCSERGETMULTI    : ::cty::c_uint = 0x545A;
pub const TIOCSERSETMULTI    : ::cty::c_uint = 0x545B;
pub const TIOCMIWAIT         : ::cty::c_uint = 0x545C;
pub const TIOCGICOUNT        : ::cty::c_uint = 0x545D;
pub const FIOQSIZE           : ::cty::c_uint = 0x5460;
pub const TIOCPKT_DATA       : ::cty::c_uint = 0;
pub const TIOCPKT_FLUSHREAD  : ::cty::c_uint = 1;
pub const TIOCPKT_FLUSHWRITE : ::cty::c_uint = 2;
pub const TIOCPKT_STOP       : ::cty::c_uint = 4;
pub const TIOCPKT_START      : ::cty::c_uint = 8;
pub const TIOCPKT_NOSTOP     : ::cty::c_uint = 16;
pub const TIOCPKT_DOSTOP     : ::cty::c_uint = 32;
pub const TIOCPKT_IOCTL      : ::cty::c_uint = 64;
pub const TIOCSER_TEMT       : ::cty::c_uint = 0x01;

pub fn TCGETS2()    -> ::cty::c_uint { _IOR::<::cty::termios2>(b'T' as ::cty::c_uint, 0x2A) }
pub fn TCSETS2()    -> ::cty::c_uint { _IOW::<::cty::termios2>(b'T' as ::cty::c_uint, 0x2B) }
pub fn TCSETSW2()   -> ::cty::c_uint { _IOW::<::cty::termios2>(b'T' as ::cty::c_uint, 0x2C) }
pub fn TCSETSF2()   -> ::cty::c_uint { _IOW::<::cty::termios2>(b'T' as ::cty::c_uint, 0x2D) }
pub fn TIOCGPTN()   -> ::cty::c_uint { _IOR::<::cty::c_uint>(b'T'   as ::cty::c_uint, 0x30) }
pub fn TIOCSPTLCK() -> ::cty::c_uint { _IOW::<::cty::c_int>(b'T'    as ::cty::c_uint, 0x31) }
pub fn TIOCGDEV()   -> ::cty::c_uint { _IOR::<::cty::c_uint>(b'T'   as ::cty::c_uint, 0x32) }
pub fn TIOCSIG()    -> ::cty::c_uint { _IOW::<::cty::c_int>(b'T'    as ::cty::c_uint, 0x36) }
pub fn TIOCGPKT()   -> ::cty::c_uint { _IOR::<::cty::c_int>(b'T'    as ::cty::c_uint, 0x38) }
pub fn TIOCGPTLCK() -> ::cty::c_uint { _IOR::<::cty::c_int>(b'T'    as ::cty::c_uint, 0x39) }
pub fn TIOCGEXCL()  -> ::cty::c_uint { _IOR::<::cty::c_int>(b'T'    as ::cty::c_uint, 0x40) }

// termbits.h

pub type cc_t     = ::cty::c_uchar;
pub type speed_t  = ::cty::c_uint;
pub type tcflag_t = ::cty::c_uint;

pub const NCCS : usize = 19;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct termios {
	pub c_iflag:    ::cty::tcflag_t,
	pub c_oflag:    ::cty::tcflag_t,
	pub c_cflag:    ::cty::tcflag_t,
	pub c_lflag:    ::cty::tcflag_t,
	pub c_line:     ::cty::cc_t,
	pub c_cc: [::cty::cc_t; ::cty::NCCS],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct termios2 {
    pub c_iflag:    ::cty::tcflag_t,
    pub c_oflag:    ::cty::tcflag_t,
    pub c_cflag:    ::cty::tcflag_t,
    pub c_lflag:    ::cty::tcflag_t,
    pub c_line:     ::cty::cc_t,
    pub c_cc: [::cty::cc_t; ::cty::NCCS],
    pub c_ispeed:   ::cty::speed_t,
    pub c_ospeed:   ::cty::speed_t,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ktermios {
    pub c_iflag:    ::cty::tcflag_t,
    pub c_oflag:    ::cty::tcflag_t,
    pub c_cflag:    ::cty::tcflag_t,
    pub c_lflag:    ::cty::tcflag_t,
    pub c_line:     ::cty::cc_t,
    pub c_cc: [::cty::cc_t; ::cty::NCCS],
    pub c_ispeed:   ::cty::speed_t,
    pub c_ospeed:   ::cty::speed_t,
}

pub const VINTR    : ::cty::cc_t = 0;
pub const VQUIT    : ::cty::cc_t = 1;
pub const VERASE   : ::cty::cc_t = 2;
pub const VKILL    : ::cty::cc_t = 3;
pub const VEOF     : ::cty::cc_t = 4;
pub const VTIME    : ::cty::cc_t = 5;
pub const VMIN     : ::cty::cc_t = 6;
pub const VSWTC    : ::cty::cc_t = 7;
pub const VSTART   : ::cty::cc_t = 8;
pub const VSTOP    : ::cty::cc_t = 9;
pub const VSUSP    : ::cty::cc_t = 10;
pub const VEOL     : ::cty::cc_t = 11;
pub const VREPRINT : ::cty::cc_t = 12;
pub const VDISCARD : ::cty::cc_t = 13;
pub const VWERASE  : ::cty::cc_t = 14;
pub const VLNEXT   : ::cty::cc_t = 15;
pub const VEOL2    : ::cty::cc_t = 16;

pub const IGNBRK  : ::cty::tcflag_t = 0o000001;
pub const BRKINT  : ::cty::tcflag_t = 0o000002;
pub const IGNPAR  : ::cty::tcflag_t = 0o000004;
pub const PARMRK  : ::cty::tcflag_t = 0o000010;
pub const INPCK   : ::cty::tcflag_t = 0o000020;
pub const ISTRIP  : ::cty::tcflag_t = 0o000040;
pub const INLCR   : ::cty::tcflag_t = 0o000100;
pub const IGNCR   : ::cty::tcflag_t = 0o000200;
pub const ICRNL   : ::cty::tcflag_t = 0o000400;
pub const IUCLC   : ::cty::tcflag_t = 0o001000;
pub const IXON    : ::cty::tcflag_t = 0o002000;
pub const IXANY   : ::cty::tcflag_t = 0o004000;
pub const IXOFF   : ::cty::tcflag_t = 0o010000;
pub const IMAXBEL : ::cty::tcflag_t = 0o020000;
pub const IUTF8   : ::cty::tcflag_t = 0o040000;

pub const OPOST  : ::cty::tcflag_t = 0o000001;
pub const OLCUC  : ::cty::tcflag_t = 0o000002;
pub const ONLCR  : ::cty::tcflag_t = 0o000004;
pub const OCRNL  : ::cty::tcflag_t = 0o000010;
pub const ONOCR  : ::cty::tcflag_t = 0o000020;
pub const ONLRET : ::cty::tcflag_t = 0o000040;
pub const OFILL  : ::cty::tcflag_t = 0o000100;
pub const OFDEL  : ::cty::tcflag_t = 0o000200;
pub const NLDLY  : ::cty::tcflag_t = 0o000400;
pub const NL0    : ::cty::tcflag_t = 0o000000;
pub const NL1    : ::cty::tcflag_t = 0o000400;
pub const CRDLY  : ::cty::tcflag_t = 0o003000;
pub const CR0    : ::cty::tcflag_t = 0o000000;
pub const CR1    : ::cty::tcflag_t = 0o001000;
pub const CR2    : ::cty::tcflag_t = 0o002000;
pub const CR3    : ::cty::tcflag_t = 0o003000;
pub const TABDLY : ::cty::tcflag_t = 0o014000;
pub const TAB0   : ::cty::tcflag_t = 0o000000;
pub const TAB1   : ::cty::tcflag_t = 0o004000;
pub const TAB2   : ::cty::tcflag_t = 0o010000;
pub const TAB3   : ::cty::tcflag_t = 0o014000;
pub const XTABS  : ::cty::tcflag_t = 0o014000;
pub const BSDLY  : ::cty::tcflag_t = 0o020000;
pub const BS0    : ::cty::tcflag_t = 0o000000;
pub const BS1    : ::cty::tcflag_t = 0o020000;
pub const VTDLY  : ::cty::tcflag_t = 0o040000;
pub const VT0    : ::cty::tcflag_t = 0o000000;
pub const VT1    : ::cty::tcflag_t = 0o040000;
pub const FFDLY  : ::cty::tcflag_t = 0o100000;
pub const FF0    : ::cty::tcflag_t = 0o000000;
pub const FF1    : ::cty::tcflag_t = 0o100000;

pub const CBAUD    : ::cty::tcflag_t = 0o010017;
pub const B0       : ::cty::tcflag_t = 0o000000;
pub const B50      : ::cty::tcflag_t = 0o000001;
pub const B75      : ::cty::tcflag_t = 0o000002;
pub const B110     : ::cty::tcflag_t = 0o000003;
pub const B134     : ::cty::tcflag_t = 0o000004;
pub const B150     : ::cty::tcflag_t = 0o000005;
pub const B200     : ::cty::tcflag_t = 0o000006;
pub const B300     : ::cty::tcflag_t = 0o000007;
pub const B600     : ::cty::tcflag_t = 0o000010;
pub const B1200    : ::cty::tcflag_t = 0o000011;
pub const B1800    : ::cty::tcflag_t = 0o000012;
pub const B2400    : ::cty::tcflag_t = 0o000013;
pub const B4800    : ::cty::tcflag_t = 0o000014;
pub const B9600    : ::cty::tcflag_t = 0o000015;
pub const B19200   : ::cty::tcflag_t = 0o000016;
pub const B38400   : ::cty::tcflag_t = 0o000017;
pub const EXTA     : ::cty::tcflag_t = ::cty::B19200;
pub const EXTB     : ::cty::tcflag_t = ::cty::B38400;
pub const CSIZE    : ::cty::tcflag_t = 0o000060;
pub const CS5      : ::cty::tcflag_t = 0o000000;
pub const CS6      : ::cty::tcflag_t = 0o000020;
pub const CS7      : ::cty::tcflag_t = 0o000040;
pub const CS8      : ::cty::tcflag_t = 0o000060;
pub const CSTOPB   : ::cty::tcflag_t = 0o000100;
pub const CREAD    : ::cty::tcflag_t = 0o000200;
pub const PARENB   : ::cty::tcflag_t = 0o000400;
pub const PARODD   : ::cty::tcflag_t = 0o001000;
pub const HUPCL    : ::cty::tcflag_t = 0o002000;
pub const CLOCAL   : ::cty::tcflag_t = 0o004000;
pub const CBAUDEX  : ::cty::tcflag_t = 0o010000;
pub const BOTHER   : ::cty::tcflag_t = 0o010000;
pub const B57600   : ::cty::tcflag_t = 0o010001;
pub const B115200  : ::cty::tcflag_t = 0o010002;
pub const B230400  : ::cty::tcflag_t = 0o010003;
pub const B460800  : ::cty::tcflag_t = 0o010004;
pub const B500000  : ::cty::tcflag_t = 0o010005;
pub const B576000  : ::cty::tcflag_t = 0o010006;
pub const B921600  : ::cty::tcflag_t = 0o010007;
pub const B1000000 : ::cty::tcflag_t = 0o010010;
pub const B1152000 : ::cty::tcflag_t = 0o010011;
pub const B1500000 : ::cty::tcflag_t = 0o010012;
pub const B2000000 : ::cty::tcflag_t = 0o010013;
pub const B2500000 : ::cty::tcflag_t = 0o010014;
pub const B3000000 : ::cty::tcflag_t = 0o010015;
pub const B3500000 : ::cty::tcflag_t = 0o010016;
pub const B4000000 : ::cty::tcflag_t = 0o010017;
pub const CIBAUD   : ::cty::tcflag_t = 0o02003600000;
pub const CMSPAR   : ::cty::tcflag_t = 0o10000000000;
pub const CRTSCTS  : ::cty::tcflag_t = 0o20000000000;

pub const IBSHIFT : ::cty::tcflag_t = 16;

pub const ISIG    : ::cty::tcflag_t = 0o000001;
pub const ICANON  : ::cty::tcflag_t = 0o000002;
pub const XCASE   : ::cty::tcflag_t = 0o000004;
pub const ECHO    : ::cty::tcflag_t = 0o000010;
pub const ECHOE   : ::cty::tcflag_t = 0o000020;
pub const ECHOK   : ::cty::tcflag_t = 0o000040;
pub const ECHONL  : ::cty::tcflag_t = 0o000100;
pub const NOFLSH  : ::cty::tcflag_t = 0o000200;
pub const TOSTOP  : ::cty::tcflag_t = 0o000400;
pub const ECHOCTL : ::cty::tcflag_t = 0o001000;
pub const ECHOPRT : ::cty::tcflag_t = 0o002000;
pub const ECHOKE  : ::cty::tcflag_t = 0o004000;
pub const FLUSHO  : ::cty::tcflag_t = 0o010000;
pub const PENDIN  : ::cty::tcflag_t = 0o040000;
pub const IEXTEN  : ::cty::tcflag_t = 0o100000;
pub const EXTPROC : ::cty::tcflag_t = 0o200000;

pub const TCOOFF : ::cty::c_uint = 0;
pub const TCOON  : ::cty::c_uint = 1;
pub const TCIOFF : ::cty::c_uint = 2;
pub const TCION  : ::cty::c_uint = 3;

pub const TCIFLUSH  : ::cty::c_uint = 0;
pub const TCOFLUSH  : ::cty::c_uint = 1;
pub const TCIOFLUSH : ::cty::c_uint = 2;

pub const TCSANOW   : ::cty::c_uint = 0;
pub const TCSADRAIN : ::cty::c_uint = 1;
pub const TCSAFLUSH : ::cty::c_uint = 2;

// termios.h

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct winsize {
    pub ws_row:    ::cty::c_ushort,
    pub ws_col:    ::cty::c_ushort,
    pub ws_xpixel: ::cty::c_ushort,
    pub ws_ypixel: ::cty::c_ushort,
}

pub const NCC : usize = 8;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct termio {
    pub c_iflag:   ::cty::c_ushort,
    pub c_oflag:   ::cty::c_ushort,
    pub c_cflag:   ::cty::c_ushort,
    pub c_lflag:   ::cty::c_ushort,
    pub c_line:    ::cty::c_uchar,
    pub c_cc: [::cty::c_uchar; ::cty::NCC],
}

pub const TIOCM_LE   : ::cty::c_uint = 0x001;
pub const TIOCM_DTR  : ::cty::c_uint = 0x002;
pub const TIOCM_RTS  : ::cty::c_uint = 0x004;
pub const TIOCM_ST   : ::cty::c_uint = 0x008;
pub const TIOCM_SR   : ::cty::c_uint = 0x010;
pub const TIOCM_CTS  : ::cty::c_uint = 0x020;
pub const TIOCM_CAR  : ::cty::c_uint = 0x040;
pub const TIOCM_RNG  : ::cty::c_uint = 0x080;
pub const TIOCM_DSR  : ::cty::c_uint = 0x100;
pub const TIOCM_CD   : ::cty::c_uint = ::cty::TIOCM_CAR;
pub const TIOCM_RI   : ::cty::c_uint = ::cty::TIOCM_RNG;
pub const TIOCM_OUT1 : ::cty::c_uint = 0x2000;
pub const TIOCM_OUT2 : ::cty::c_uint = 0x4000;
pub const TIOCM_LOOP : ::cty::c_uint = 0x8000;

// poll.h

pub const POLLIN         : ::cty::c_int = 0x0001;
pub const POLLPRI        : ::cty::c_int = 0x0002;
pub const POLLOUT        : ::cty::c_int = 0x0004;
pub const POLLERR        : ::cty::c_int = 0x0008;
pub const POLLHUP        : ::cty::c_int = 0x0010;
pub const POLLNVAL       : ::cty::c_int = 0x0020;
pub const POLLRDNORM     : ::cty::c_int = 0x0040;
pub const POLLRDBAND     : ::cty::c_int = 0x0080;
pub const POLLWRNORM     : ::cty::c_int = 0x0100;
pub const POLLWRBAND     : ::cty::c_int = 0x0200;
pub const POLLMSG        : ::cty::c_int = 0x0400;
pub const POLLREMOVE     : ::cty::c_int = 0x1000;
pub const POLLRDHUP      : ::cty::c_int = 0x2000;
pub const POLLFREE       : ::cty::c_int = 0x4000;
pub const POLL_BUSY_LOOP : ::cty::c_int = 0x8000;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct pollfd {
    pub fd:      ::cty::c_int,
    pub events:  ::cty::c_short,
    pub revents: ::cty::c_short,
}

// resource.h

pub const RLIMIT_CPU        : ::cty::c_ulong = 0;
pub const RLIMIT_FSIZE      : ::cty::c_ulong = 1;
pub const RLIMIT_DATA       : ::cty::c_ulong = 2;
pub const RLIMIT_STACK      : ::cty::c_ulong = 3;
pub const RLIMIT_CORE       : ::cty::c_ulong = 4;
pub const RLIMIT_RSS        : ::cty::c_ulong = 5;
pub const RLIMIT_NPROC      : ::cty::c_ulong = 6;
pub const RLIMIT_NOFILE     : ::cty::c_ulong = 7;
pub const RLIMIT_MEMLOCK    : ::cty::c_ulong = 8;
pub const RLIMIT_AS         : ::cty::c_ulong = 9;
pub const RLIMIT_LOCKS      : ::cty::c_ulong = 10;
pub const RLIMIT_SIGPENDING : ::cty::c_ulong = 11;
pub const RLIMIT_MSGQUEUE   : ::cty::c_ulong = 12;
pub const RLIMIT_NICE       : ::cty::c_ulong = 13;
pub const RLIMIT_RTPRIO     : ::cty::c_ulong = 14;
pub const RLIMIT_RTTIME     : ::cty::c_ulong = 15;
pub const RLIM_NLIMITS      : ::cty::c_ulong = 16;
pub const RLIM_INFINITY     : ::cty::c_ulong = !0;

// shmbuf.h

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct shminfo64 {
    pub shmmax:    ::cty::__kernel_ulong_t,
    pub shmmin:    ::cty::__kernel_ulong_t,
    pub shmmni:    ::cty::__kernel_ulong_t,
    pub shmseg:    ::cty::__kernel_ulong_t,
    pub shmall:    ::cty::__kernel_ulong_t,
    pub __unused1: ::cty::__kernel_ulong_t,
    pub __unused2: ::cty::__kernel_ulong_t,
    pub __unused3: ::cty::__kernel_ulong_t,
    pub __unused4: ::cty::__kernel_ulong_t,
}
