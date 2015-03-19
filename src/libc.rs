#![allow(non_camel_case_types, raw_pointer_derive)]

pub use self::width::*;
pub use self::arch::*;

#[repr(u8)]
pub enum c_void {
    __variant1,
    __variant2,
}

pub type c_bool       = i8;
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

pub type __S16_TYPE       = c_short;
pub type __U16_TYPE       = c_ushort;
pub type __S32_TYPE       = c_int;
pub type __U32_TYPE       = c_uint;
pub type __SLONGWORD_TYPE = c_long;
pub type __ULONGWORD_TYPE = c_ulong;

pub type __SYSCALL_SLONG_TYPE = __SLONGWORD_TYPE;
pub type __SYSCALL_ULONG_TYPE = __ULONGWORD_TYPE;

pub type __DEV_T_TYPE	 = __UQUAD_TYPE;
pub type __UID_T_TYPE	 = __U32_TYPE;
pub type __GID_T_TYPE	 = __U32_TYPE;
pub type __INO_T_TYPE	 = __SYSCALL_ULONG_TYPE;
pub type __INO64_T_TYPE	 = __UQUAD_TYPE;
pub type __MODE_T_TYPE	 = __U32_TYPE;

pub type __OFF_T_TYPE        = __SYSCALL_SLONG_TYPE;
pub type __OFF64_T_TYPE      = __SQUAD_TYPE;
pub type __PID_T_TYPE        = __S32_TYPE;
pub type __RLIM_T_TYPE       = __SYSCALL_ULONG_TYPE;
pub type __RLIM64_T_TYPE     = __UQUAD_TYPE;
pub type __BLKCNT_T_TYPE     = __SYSCALL_SLONG_TYPE;
pub type __BLKCNT64_T_TYPE   = __SQUAD_TYPE;
pub type __FSBLKCNT_T_TYPE   = __SYSCALL_ULONG_TYPE;
pub type __FSBLKCNT64_T_TYPE = __UQUAD_TYPE;
pub type __FSFILCNT_T_TYPE   = __SYSCALL_ULONG_TYPE;
pub type __FSFILCNT64_T_TYPE = __UQUAD_TYPE;
pub type __ID_T_TYPE         = __U32_TYPE;
pub type __CLOCK_T_TYPE      = __SYSCALL_SLONG_TYPE;
pub type __TIME_T_TYPE       = __SYSCALL_SLONG_TYPE;
pub type __USECONDS_T_TYPE   = __U32_TYPE;
pub type __SUSECONDS_T_TYPE  = __SYSCALL_SLONG_TYPE;
pub type __DADDR_T_TYPE      = __S32_TYPE;
pub type __KEY_T_TYPE        = __S32_TYPE;
pub type __CLOCKID_T_TYPE    = __S32_TYPE;
pub type __TIMER_T_TYPE      = *mut c_void;
pub type __BLKSIZE_T_TYPE    = __SYSCALL_SLONG_TYPE;
pub type __SSIZE_T_TYPE      = __SWORD_TYPE;

pub type __dev_t           = __DEV_T_TYPE;
pub type __uid_t           = __UID_T_TYPE;
pub type __gid_t           = __GID_T_TYPE;
pub type __ino_t           = __INO_T_TYPE;
pub type __ino64_t         = __INO64_T_TYPE;
pub type __mode_t          = __MODE_T_TYPE;
pub type __nlink_t         = __NLINK_T_TYPE;
pub type __off_t           = __OFF_T_TYPE;
pub type __off64_t         = __OFF64_T_TYPE;
pub type __pid_t           = __PID_T_TYPE;
pub type __fsid_t          = __FSID_T_TYPE;
pub type __clock_t         = __CLOCK_T_TYPE;
pub type __rlim_t          = __RLIM_T_TYPE;
pub type __rlim64_t        = __RLIM64_T_TYPE;
pub type __id_t            = __ID_T_TYPE;
pub type __time_t          = __TIME_T_TYPE;
pub type __useconds_t      = __USECONDS_T_TYPE;
pub type __suseconds_t     = __SUSECONDS_T_TYPE;
pub type __daddr_t         = __DADDR_T_TYPE;
pub type __key_t           = __KEY_T_TYPE;
pub type __clockid_t       = __CLOCKID_T_TYPE;
pub type __timer_t         = __TIMER_T_TYPE;
pub type __blksize_t       = __BLKSIZE_T_TYPE;
pub type __blkcnt_t        = __BLKCNT_T_TYPE;
pub type __blkcnt64_t      = __BLKCNT64_T_TYPE;
pub type __fsblkcnt_t      = __FSBLKCNT_T_TYPE;
pub type __fsblkcnt64_t    = __FSBLKCNT64_T_TYPE;
pub type __fsfilcnt_t      = __FSFILCNT_T_TYPE;
pub type __fsfilcnt64_t    = __FSFILCNT64_T_TYPE;
pub type __fsword_t        = __FSWORD_T_TYPE;
pub type __ssize_t         = __SSIZE_T_TYPE;
pub type __syscall_slong_t = __SYSCALL_SLONG_TYPE;
pub type __syscall_ulong_t = __SYSCALL_ULONG_TYPE;

pub type __SIZE_TYPE__ = c_ulong;

pub type __loff_t = __off64_t;
pub type __qaddr_t = *mut __quad_t;
pub type __caddr_t = *mut c_char;

pub type __intptr_t =  __SWORD_TYPE;
pub type __socklen_t =  __U32_TYPE;

pub type mode_t = __mode_t;
pub type off_t = __off64_t;
pub type off64_t = __off64_t;
pub type pid_t = __pid_t;
pub type uid_t = __uid_t;
pub type gid_t = __gid_t;

pub type ssize_t = __ssize_t;
pub type size_t = __SIZE_TYPE__;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct __FSID_T_TYPE {
    __val: [c_int; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: size_t,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct passwd {
    pub pw_name:   *mut c_char,
    pub pw_passwd: *mut c_char,
    pub pw_uid:    uid_t,
    pub pw_gid:    gid_t,
    pub pw_gecos:  *mut c_char,
    pub pw_dir:    *mut c_char,
    pub pw_shell:  *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct group {
    pub gr_name: *mut c_char,
    pub gr_passwd: *mut c_char,
    pub gr_gid: gid_t,
    pub gr_mem: *mut *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct statfs64 {
    pub f_type:    __fsword_t,
    pub f_bsize:   __fsword_t,
    pub f_blocks:  __fsblkcnt64_t,
    pub f_bfree:   __fsblkcnt64_t,
    pub f_bavail:  __fsblkcnt64_t,
    pub f_files:   __fsfilcnt64_t,
    pub f_ffree:   __fsfilcnt64_t,
    pub f_fsid:    __fsid_t,
    pub f_namelen: __fsword_t,
    pub f_frsize:  __fsword_t,
    pub f_flags:   __fsword_t,
    pub f_spare:   [__fsword_t; 4],
}

#[cfg(target_pointer_width = "64")]
mod width {
    pub type c_long       = i64;
    pub type c_ulong      = u64;

    pub type __quad_t   = super::c_long;
    pub type __u_quad_t = super::c_ulong;

    pub type __SQUAD_TYPE   = super::c_long;
    pub type __UQUAD_TYPE   = super::c_ulong;
    pub type __SWORD_TYPE   = super::c_long;
    pub type __UWORD_TYPE   = super::c_ulong;
    pub type __SLONG32_TYPE = super::c_int;
    pub type __ULONG32_TYPE = super::c_uint;
    pub type __S64_TYPE     = super::c_long;
    pub type __U64_TYPE     = super::c_ulong;
}

#[cfg(target_pointer_width = "32")]
mod width {
    pub type c_long       = i32;
    pub type c_ulong      = u32;

    pub type __quad_t   = super::c_longlong;
    pub type __u_quad_t = super::c_ulonglong;

    pub type __SQUAD_TYPE   = super::__quad_t;
    pub type __UQUAD_TYPE   = super::__u_quad_t;
    pub type __SWORD_TYPE   = super::c_int;
    pub type __UWORD_TYPE   = super::c_uint;
    pub type __SLONG32_TYPE = super::c_long;
    pub type __ULONG32_TYPE = super::c_ulong;
    pub type __S64_TYPE     = super::__quad_t;
    pub type __U64_TYPE     = super::__u_quad_t;
}

#[cfg(target_arch = "x86_64")]
mod arch {
    pub type __NLINK_T_TYPE	 = super::__SYSCALL_ULONG_TYPE;
    pub type __FSWORD_T_TYPE = super::__SYSCALL_SLONG_TYPE;
}

#[cfg(not(target_arch = "x86_64"))]
mod arch {
    pub type __NLINK_T_TYPE	 = super::__UWORD_TYPE;
    pub type __FSWORD_T_TYPE = super::__SWORD_TYPE;
}

pub const O_RDONLY:    c_int = 0o0;
pub const O_WRONLY:    c_int = 0o1;
pub const O_RDWR:      c_int = 0o2;
pub const O_ACCMODE:   c_int = 0o3;
pub const O_CREAT:     c_int = 0o100;
pub const O_EXCL:      c_int = 0o200;
pub const O_NOCTTY:    c_int = 0o400;
pub const O_TRUNC:     c_int = 0o1000;
pub const O_APPEND:    c_int = 0o2000;
pub const O_NONBLOCK:  c_int = 0o4000;
pub const O_DSYNC:     c_int = 0o10000;
pub const O_ASYNC:     c_int = 0o20000;
pub const O_DIRECT:    c_int = 0o40000;
pub const O_LARGEFILE: c_int = 0o100000;
pub const O_DIRECTORY: c_int = 0o200000;
pub const O_NOFOLLOW:  c_int = 0o400000;
pub const O_NOATIME:   c_int = 0o1000000;
pub const O_CLOEXEC:   c_int = 0o2000000;
pub const O_SYNC:      c_int = 0o4010000;
pub const O_PATH:      c_int = 0o10000000;
pub const O_TMPFILE:   c_int = 0o20200000;

pub const S_ISUID: mode_t = 0o4000;
pub const S_ISGID: mode_t = 0o2000;
pub const S_ISVTX: mode_t = 0o1000;
pub const S_IRUSR: mode_t = 0o400;
pub const S_IWUSR: mode_t = 0o200;
pub const S_IXUSR: mode_t = 0o100;
pub const S_IRGRP: mode_t = 0o40;
pub const S_IWGRP: mode_t = 0o20;
pub const S_IXGRP: mode_t = 0o10;
pub const S_IROTH: mode_t = 0o4;
pub const S_IWOTH: mode_t = 0o2;
pub const S_IXOTH: mode_t = 0o1;

pub const SEEK_SET: c_int = 0;
pub const SEEK_CUR: c_int = 1;
pub const SEEK_END: c_int = 2;

pub const F_DUPFD: c_int = 0;
pub const F_GETFD: c_int = 1;
pub const F_SETFD: c_int = 2;
pub const F_GETFL: c_int = 3;
pub const F_SETFL: c_int = 4;
pub const F_DUPFD_CLOEXEC: c_int = 1030;

pub const IOV_MAX: usize = 1024;

pub const ST_RDONLY      : __fsword_t = 1;
pub const ST_NOSUID      : __fsword_t = 2;
pub const ST_NODEV       : __fsword_t = 4;
pub const ST_NOEXEC      : __fsword_t = 8;
pub const ST_SYNCHRONOUS : __fsword_t = 16;
pub const ST_MANDLOCK    : __fsword_t = 64;
pub const ST_WRITE       : __fsword_t = 128;
pub const ST_APPEND      : __fsword_t = 256;
pub const ST_IMMUTABLE   : __fsword_t = 512;
pub const ST_NOATIME     : __fsword_t = 1024;
pub const ST_NODIRATIME  : __fsword_t = 2048;
pub const ST_RELATIME    : __fsword_t = 4096;

extern {
    pub fn open64(fd: *const c_char, flags: c_int, ...) -> c_int;
    pub fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    pub fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    pub fn close(fd: c_int) -> c_int;
    pub fn lseek64(fd: c_int, offset: off64_t, whence: c_int) -> off64_t;
    pub fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    pub fn pread64(fd: c_int, buf: *mut c_void, count: size_t, offset: off64_t) -> ssize_t;
    pub fn pwrite64(fd: c_int, buf: *const c_void, count: size_t, offset: off64_t) -> ssize_t;
    pub fn readv(fd: c_int, iovec: *const iovec, count: c_int) -> ssize_t;
    pub fn writev(fd: c_int, iovec: *const iovec, count: c_int) -> ssize_t;
    pub fn preadv64(fd: c_int, iovec: *const iovec, count: c_int, offset: off64_t) -> ssize_t;
    pub fn pwritev64(fd: c_int, iovec: *const iovec, count: c_int, offset: off64_t) -> ssize_t;
    pub fn ftruncate64(fd: c_int, offset: off64_t) -> c_int;
    pub fn getpid() -> pid_t;
    pub fn getppid() -> pid_t;
    pub fn clearenv() -> c_int;
    pub fn sysconf(name: c_int) -> c_long;
    pub fn __errno_location() -> *mut c_int;
    pub fn getpwuid_r(uid: uid_t, pwd: *mut passwd, buf: *mut c_char, len: size_t,
                      result: *mut *mut passwd) -> c_int;
    pub fn getpwnam_r(name: *const c_char, pwd: *mut passwd, buf: *mut c_char,
                      len: size_t, result: *mut *mut passwd) -> c_int;
    pub fn getgrgid_r(gid: gid_t, grp: *mut group, buf: *mut c_char, len: size_t,
                      result: *mut *mut group) -> c_int;
    pub fn getgrnam_r(name: *const c_char, grp: *mut group, buf: *mut c_char,
                      len: size_t, result: *mut *mut group) -> c_int;
    pub fn getresuid(ruid: *const uid_t, euid: *const uid_t, suid: *const uid_t) -> c_int;
    pub fn getresgid(rgid: *const gid_t, egid: *const gid_t, sgid: *const gid_t) -> c_int;
    pub fn setresuid(ruid: uid_t, euid: uid_t, suid: uid_t) -> c_int;
    pub fn setresgid(rgid: gid_t, egid: gid_t, sgid: gid_t) -> c_int;
    pub fn getgroups(size: c_int, list: *mut gid_t) -> c_int;
    pub fn setgroups(size: size_t, list: *const gid_t) -> c_int;
    pub fn fsync(fd: c_int) -> c_int;
    pub fn fdatasync(fd: c_int) -> c_int;
    pub fn sync();
    pub fn syncfs(fd: c_int) -> c_int;
    pub fn posix_fadvise64(fd: c_int, offset: off64_t, len: off64_t,
                           advise: c_int) -> c_int;
    pub fn statfs64(file: *const c_char, buf: *mut statfs64) -> c_int;
    pub fn fstatfs64(fd: c_int, buf: *mut statfs64) -> c_int;
}
