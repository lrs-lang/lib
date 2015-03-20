// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(non_camel_case_types, raw_pointer_derive)]

pub use self::_statfs::{statfs};
pub use self::resource::{SYSCALL_RLIM_INFINITY};

mod _statfs;
mod resource;

// From musl/arch/x86_64/bits/alltypes.h

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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
pub type c_long       = i64;
pub type c_ulong      = u64;
pub type c_longlong   = i64;
pub type c_ulonglong  = u64;
pub type c_float      = f32;
pub type c_double     = f64;

pub type time_t             = i64;
pub type suseconds_t        = i64;
pub type size_t             = u64;
pub type uintptr_t          = u64;
pub type ptrdiff_t          = i64;
pub type ssize_t            = i64;
pub type intptr_t           = i64;
pub type regoff_t           = i64;
pub type register_t         = i64;
pub type intmax_t           = i64;
pub type uintmax_t          = u64;
pub type mode_t             = u32;
pub type nlink_t            = u64;
pub type off_t              = i64;
pub type ino_t              = u64;
pub type dev_t              = u64;
pub type blksize_t          = i64;
pub type blkcnt_t           = i64;
pub type fsblkcnt_t         = u64;
pub type fsfilcnt_t         = u64;
pub type clockid_t          = i32;
pub type clock_t            = i64;
pub type pid_t              = i32;
pub type id_t               = u32;
pub type uid_t              = u32;
pub type gid_t              = u32;
pub type key_t              = i32;
pub type useconds_t         = u32;
pub type pthread_once_t     = i32;
pub type pthread_key_t      = u32;
pub type pthread_spinlock_t = i32;
pub type socklen_t          = u32;
pub type sa_family_t        = u16;

pub type timer_t = *mut c_void;

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct pthread_mutexattr_t {
    __attr: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct pthread_condattr_t {
    __attr: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct pthread_barrierattr_t {
    __attr: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct pthread_rwlockattr_t {
    __attr: [c_uint; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct mbstate_t {
    __opaque1: c_uint,
    __opaque2: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct locale_t {
    s: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct sigset_t {
    __bits: [c_ulong; 16],
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: size_t,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct pthread_attr_t {
    s: [c_ulong; 7],
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct pthread_mutex_t {
    s: [*mut c_void; 5],
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct pthread_cond_t {
    s: [*mut c_void; 6],
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct pthread_rwlock_t {
    s: [*mut c_void; 7],
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct pthread_barrier_t {
    s: [*mut c_void; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct timeval {
    pub tv_sec: time_t,
    pub tv_usec: suseconds_t,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: c_long,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct pthread_t {
    s: *mut c_void,
}
