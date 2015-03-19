#![allow(dead_code)]

pub use imp::arch::cty::*;

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

pub const ST_RDONLY:      c_ulong = 1;
pub const ST_NOSUID:      c_ulong = 2;
pub const ST_NODEV:       c_ulong = 4;
pub const ST_NOEXEC:      c_ulong = 8;
pub const ST_SYNCHRONOUS: c_ulong = 16;
pub const ST_MANDLOCK:    c_ulong = 64;
pub const ST_WRITE:       c_ulong = 128;
pub const ST_APPEND:      c_ulong = 256;
pub const ST_IMMUTABLE:   c_ulong = 512;
pub const ST_NOATIME:     c_ulong = 1024;
pub const ST_NODIRATIME:  c_ulong = 2048;
pub const ST_RELATIME:    c_ulong = 4096;

pub const RLIM_INFINITY: c_ulonglong = !0;

pub type rlim_t = c_ulonglong;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct fsid_t {
	pub val: [c_int; 2],
}
