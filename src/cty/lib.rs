// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_cty"]
#![crate_type = "lib"]
#![feature(negate_unsigned, plugin, no_std, custom_derive)]
#![plugin(lrs_core_plugin)]
#![no_std]
#![allow(non_camel_case_types, raw_pointer_derive, overflowing_literals, non_snake_case,
         non_upper_case_globals, dead_code)]

extern crate lrs_core as core;
extern crate lrs_cty_base as cty_base;
extern crate lrs_base as base;

use core::{mem};

pub use self::arch::*;
pub use cty_base::errno::*;

pub mod alias {
    pub type UserId    = ::uid_t;
    pub type GroupId   = ::gid_t;
    pub type DeviceId  = ::dev_t;
    pub type InodeId   = ::ino_t;
    pub type ProcessId = ::pid_t;
}

mod lrs { pub use ::base::lrs::*; }

mod gen;

#[cfg(target_arch = "x86_64")]
#[path = "x86_64/mod.rs"]
mod arch;

// Userspace aliases. NB: These are only correct for the kernel ABI. E.g. size_t is 32 bit
// on x32 but __kernel_size_t is 64 bit on x32.

pub type fd_set           = __kernel_fd_set;
pub type dev_t            = __kernel_dev_t;
pub type ino_t            = __kernel_ino_t;
pub type mode_t           = __kernel_mode_t;
pub type umode_t          = k_ushort;
pub type nlink_t          = __u32;
pub type off_t            = __kernel_off_t;
pub type pid_t            = __kernel_pid_t;
pub type daddr_t          = __kernel_daddr_t;
pub type key_t            = __kernel_key_t;
pub type suseconds_t      = __kernel_suseconds_t;
pub type timer_t          = __kernel_timer_t;
pub type clockid_t        = __kernel_clockid_t;
pub type mqd_t            = __kernel_mqd_t;
pub type uid_t            = __kernel_uid32_t;
pub type gid_t            = __kernel_gid32_t;
pub type uid16_t          = __kernel_uid16_t;
pub type gid16_t          = __kernel_gid16_t;
pub type uintptr_t        = k_ulong;
pub type old_uid_t        = __kernel_old_uid_t;
pub type old_gid_t        = __kernel_old_gid_t;
pub type loff_t           = __kernel_loff_t;
pub type size_t           = __kernel_size_t;
pub type ssize_t          = __kernel_ssize_t;
pub type ptrdiff_t        = __kernel_ptrdiff_t;
pub type time_t           = __kernel_time_t;
pub type clock_t          = __kernel_clock_t;
pub type caddr_t          = __kernel_caddr_t;
pub type sighandler_t     = __kernel_sighandler_t;
pub type sa_family_t      = __kernel_sa_family_t;
pub type sockaddr_storage = __kernel_sockaddr_storage;

// XXX: These two can also be u64 depending on a kernel option.
pub type sector_t         = k_ulong;
pub type blkcnt_t         = k_ulong;

// C type aliases. Only c_long and __kernel_long_t might disagree (x32), but we define
// all to avoid confusion.

pub type k_char      = c_char;
pub type k_schar     = c_schar;
pub type k_uchar     = c_uchar;
pub type k_short     = c_short;
pub type k_ushort    = c_ushort;
pub type k_int       = c_int;
pub type k_uint      = c_uint;
pub type k_long      = __kernel_long_t;
pub type k_ulong     = __kernel_ulong_t;
pub type k_longlong  = c_longlong;
pub type k_ulonglong = c_ulonglong;
pub type k_float     = c_float;
pub type k_double    = c_double;

pub const __FD_SETSIZE : usize = 1024;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct __kernel_fd_set {
    pub fds_bits: [c_ulong; __FD_SETSIZE / (8 * BYTES_PER_LONG)],
}

pub type __kernel_dev_t = __u32;
pub type __kernel_sighandler_t = extern fn(c_int);
pub type __kernel_key_t = k_int;
pub type __kernel_mqd_t = k_int;

pub type __le16  = __u16;
pub type __be16  = __u16;
pub type __le32  = __u32;
pub type __be32  = __u32;
pub type __le64  = __u64;
pub type __be64  = __u64;
pub type __sum16 = __u16;
pub type __wsum  = __u32;

// time.h

// TODO: Explain why the second type is not c_long.
#[repr(C)]
#[derive(Pod, Eq)]
pub struct timespec {
    pub tv_sec:  __kernel_time_t,
    pub tv_nsec: __kernel_long_t,
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct timeval {
    pub tv_sec: __kernel_time_t,
    pub tv_usec: __kernel_suseconds_t,
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct timezone {
    pub tz_minuteswest: c_int,
    pub tz_dsttime:     c_int,
}

pub const ITIMER_REAL    : c_int = 0;
pub const ITIMER_VIRTUAL : c_int = 1;
pub const ITIMER_PROF    : c_int = 2;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value:    timespec,
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct itimerval {
    pub it_interval: timeval,
    pub it_value:    timeval,
}

pub const CLOCK_REALTIME           : clockid_t = 0;
pub const CLOCK_MONOTONIC          : clockid_t = 1;
pub const CLOCK_PROCESS_CPUTIME_ID : clockid_t = 2;
pub const CLOCK_THREAD_CPUTIME_ID  : clockid_t = 3;
pub const CLOCK_MONOTONIC_RAW      : clockid_t = 4;
pub const CLOCK_REALTIME_COARSE    : clockid_t = 5;
pub const CLOCK_MONOTONIC_COARSE   : clockid_t = 6;
pub const CLOCK_BOOTTIME           : clockid_t = 7;
pub const CLOCK_REALTIME_ALARM     : clockid_t = 8;
pub const CLOCK_BOOTTIME_ALARM     : clockid_t = 9;
pub const CLOCK_SGI_CYCLE          : clockid_t = 10;
pub const CLOCK_TAI                : clockid_t = 11;

pub const TIMER_ABSTIME : c_int = 0x01;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct linux_dirent64 {
    pub d_ino:    u64,
    pub d_off:    i64,
    pub d_reclen: c_ushort,
    pub d_type:   c_uchar,
    pub d_name:   [c_char; 0],
}

pub const SI_LOAD_SHIFT : __kernel_ulong_t = 16;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct sysinfo {
    pub uptime:    __kernel_long_t,
    pub loads:     [__kernel_ulong_t; 3],
    pub totalram:  __kernel_ulong_t,
    pub freeram:   __kernel_ulong_t,
    pub sharedram: __kernel_ulong_t,
    pub bufferram: __kernel_ulong_t,
    pub totalswap: __kernel_ulong_t,
    pub freeswap:  __kernel_ulong_t,
    pub procs:     __u16,
    pub pad:       __u16,
    pub totalhigh: __kernel_ulong_t,
    pub freehigh:  __kernel_ulong_t,
    pub mem_unit:  __u32,
    pub _f:        [c_char; SYSINFO_PADDING],
}

pub const F_SETLEASE          : c_uint = F_LINUX_SPECIFIC_BASE+0;
pub const F_GETLEASE          : c_uint = F_LINUX_SPECIFIC_BASE+1;
pub const F_CANCELLK          : c_uint = F_LINUX_SPECIFIC_BASE+5;
pub const F_DUPFD_CLOEXEC     : c_uint = F_LINUX_SPECIFIC_BASE+6;
pub const F_NOTIFY            : c_uint = F_LINUX_SPECIFIC_BASE+2;
pub const F_SETPIPE_SZ        : c_uint = F_LINUX_SPECIFIC_BASE+7;
pub const F_GETPIPE_SZ        : c_uint = F_LINUX_SPECIFIC_BASE+8;
pub const F_ADD_SEALS         : c_uint = F_LINUX_SPECIFIC_BASE+9;
pub const F_GET_SEALS         : c_uint = F_LINUX_SPECIFIC_BASE+10;
pub const F_SEAL_SEAL         : c_uint = 0x0001;
pub const F_SEAL_SHRINK       : c_uint = 0x0002;
pub const F_SEAL_GROW         : c_uint = 0x0004;
pub const F_SEAL_WRITE        : c_uint = 0x0008;
pub const DN_ACCESS           : c_int = 0x00000001;
pub const DN_MODIFY           : c_int = 0x00000002;
pub const DN_CREATE           : c_int = 0x00000004;
pub const DN_DELETE           : c_int = 0x00000008;
pub const DN_RENAME           : c_int = 0x00000010;
pub const DN_ATTRIB           : c_int = 0x00000020;
#[allow(overflowing_literals)]
pub const DN_MULTISHOT        : c_int = 0x80000000;
pub const AT_FDCWD            : c_int = -100;
pub const AT_SYMLINK_NOFOLLOW : c_int = 0x100;
pub const AT_REMOVEDIR        : c_int = 0x200;
pub const AT_SYMLINK_FOLLOW   : c_int = 0x400;
pub const AT_NO_AUTOMOUNT     : c_int = 0x800;
pub const AT_EMPTY_PATH       : c_int = 0x1000;

// bpf_common.h

pub fn BPF_CLASS (code: u8) -> u8 { code & 0x07 }
pub fn BPF_SIZE  (code: u8) -> u8 { code & 0x18 }
pub fn BPF_MODE  (code: u8) -> u8 { code & 0xe0 }
pub fn BPF_OP    (code: u8) -> u8 { code & 0xf0 }
pub fn BPF_SRC   (code: u8) -> u8 { code & 0x08 }

pub const BPF_LD   : u8 = 0x00;
pub const BPF_LDX  : u8 = 0x01;
pub const BPF_ST   : u8 = 0x02;
pub const BPF_STX  : u8 = 0x03;
pub const BPF_ALU  : u8 = 0x04;
pub const BPF_JMP  : u8 = 0x05;
pub const BPF_RET  : u8 = 0x06;
pub const BPF_MISC : u8 = 0x07;
pub const BPF_W    : u8 = 0x00;
pub const BPF_H    : u8 = 0x08;
pub const BPF_B    : u8 = 0x10;
pub const BPF_IMM  : u8 = 0x00;
pub const BPF_ABS  : u8 = 0x20;
pub const BPF_IND  : u8 = 0x40;
pub const BPF_MEM  : u8 = 0x60;
pub const BPF_LEN  : u8 = 0x80;
pub const BPF_MSH  : u8 = 0xa0;
pub const BPF_ADD  : u8 = 0x00;
pub const BPF_SUB  : u8 = 0x10;
pub const BPF_MUL  : u8 = 0x20;
pub const BPF_DIV  : u8 = 0x30;
pub const BPF_OR   : u8 = 0x40;
pub const BPF_AND  : u8 = 0x50;
pub const BPF_LSH  : u8 = 0x60;
pub const BPF_RSH  : u8 = 0x70;
pub const BPF_NEG  : u8 = 0x80;
pub const BPF_MOD  : u8 = 0x90;
pub const BPF_XOR  : u8 = 0xa0;
pub const BPF_JA   : u8 = 0x00;
pub const BPF_JEQ  : u8 = 0x10;
pub const BPF_JGT  : u8 = 0x20;
pub const BPF_JGE  : u8 = 0x30;
pub const BPF_JSET : u8 = 0x40;
pub const BPF_K    : u8 = 0x00;
pub const BPF_X    : u8 = 0x08;

pub const BPF_MAXINSNS : usize = 4096;

// bpf.h

pub const BPF_ALU64   : u8 = 0x07;
pub const BPF_DW      : u8 = 0x18;
pub const BPF_XADD    : u8 = 0xc0;
pub const BPF_MOV     : u8 = 0xb0;
pub const BPF_ARSH    : u8 = 0xc0;
pub const BPF_END     : u8 = 0xd0;
pub const BPF_TO_LE   : u8 = 0x00;
pub const BPF_TO_BE   : u8 = 0x08;
pub const BPF_FROM_LE : u8 = BPF_TO_LE;
pub const BPF_FROM_BE : u8 = BPF_TO_BE;
pub const BPF_JNE     : u8 = 0x50;
pub const BPF_JSGT    : u8 = 0x60;
pub const BPF_JSGE    : u8 = 0x70;
pub const BPF_CALL    : u8 = 0x80;
pub const BPF_EXIT    : u8 = 0x90;

pub const BPF_REG_0     : u8 = 0;
pub const BPF_REG_1     : u8 = 1;
pub const BPF_REG_2     : u8 = 2;
pub const BPF_REG_3     : u8 = 3;
pub const BPF_REG_4     : u8 = 4;
pub const BPF_REG_5     : u8 = 5;
pub const BPF_REG_6     : u8 = 6;
pub const BPF_REG_7     : u8 = 7;
pub const BPF_REG_8     : u8 = 8;
pub const BPF_REG_9     : u8 = 9;
pub const BPF_REG_10    : u8 = 10;
pub const __MAX_BPF_REG : u8 = 11;

pub const MAX_BPF_REG : u8 = __MAX_BPF_REG;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct bpf_insn {
    pub code: __u8,
    // __u8 dst_reg:4;  /* dest register */
    // __u8 src_reg:4;  /* source register */
    //
    // "systems language"
    reg:  __u8,
    pub off:  __s16,
    pub imm:  __s32,
}

pub const BPF_MAP_CREATE       : c_int = 0;
pub const BPF_MAP_LOOKUP_ELEM  : c_int = 1;
pub const BPF_MAP_UPDATE_ELEM  : c_int = 2;
pub const BPF_MAP_DELETE_ELEM  : c_int = 3;
pub const BPF_MAP_GET_NEXT_KEY : c_int = 4;
pub const BPF_PROG_LOAD        : c_int = 5;

pub const BPF_MAP_TYPE_UNSPEC : c_int = 0;
pub const BPF_MAP_TYPE_HASH   : c_int = 1;
pub const BPF_MAP_TYPE_ARRAY  : c_int = 2;

pub const BPF_ANY     : c_int = 0;
pub const BPF_NOEXIST : c_int = 1;
pub const BPF_EXIST   : c_int = 2;

// XXX(WRONG) this needs an ((aligned(8))) attribute
// "systems language"
#[repr(C)]
#[derive(Pod, Eq)]
pub struct bpf_attr {
    _dummy: [u64; 2],
}

pub const BPF_FUNC_unspec          : c_int = 0;
pub const BPF_FUNC_map_lookup_elem : c_int = 1;
pub const BPF_FUNC_map_update_elem : c_int = 2;
pub const BPF_FUNC_map_delete_elem : c_int = 3;
pub const __BPF_FUNC_MAX_ID        : c_int = 4;

// capability.h

pub const _LINUX_CAPABILITY_VERSION_1 : c_int = 0x19980330;
pub const _LINUX_CAPABILITY_U32S_1    : c_int = 1;
pub const _LINUX_CAPABILITY_VERSION_2 : c_int = 0x20071026;
pub const _LINUX_CAPABILITY_U32S_2    : c_int = 2;
pub const _LINUX_CAPABILITY_VERSION_3 : c_int = 0x20080522;
pub const _LINUX_CAPABILITY_U32S_3    : c_int = 2;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct __user_cap_header_struct {
    pub version: __u32,
    pub pid: c_int,
}

pub type cap_user_header_t = *mut __user_cap_header_struct;

#[repr(C)]
#[derive(Pod, Eq)]
struct __user_cap_data_struct {
    pub effective:   __u32,
    pub permitted:   __u32,
    pub inheritable: __u32,
}

pub type cap_user_data_t = *mut __user_cap_data_struct;

pub const SIZEOF__le32 : usize = 4;

pub const VFS_CAP_REVISION_MASK   : c_int = 0xFF000000;
pub const VFS_CAP_REVISION_SHIFT  : c_int = 24;
pub const VFS_CAP_FLAGS_MASK      : c_int = !VFS_CAP_REVISION_MASK;
pub const VFS_CAP_FLAGS_EFFECTIVE : c_int = 0x000001;
pub const VFS_CAP_REVISION_1      : c_int = 0x01000000;
pub const VFS_CAP_U32_1           : c_int = 1;
pub const XATTR_CAPS_SZ_1         : c_int = (SIZEOF__le32 as c_int)*(1+2*VFS_CAP_U32_1);
pub const VFS_CAP_REVISION_2      : c_int = 0x02000000;
pub const VFS_CAP_U32_2           : c_int = 2;
pub const XATTR_CAPS_SZ_2         : c_int = (SIZEOF__le32 as c_int)*(1+2*VFS_CAP_U32_2);
pub const XATTR_CAPS_SZ           : c_int = XATTR_CAPS_SZ_2;
pub const VFS_CAP_U32             : c_int = VFS_CAP_U32_2;
pub const VFS_CAP_REVISION        : c_int = VFS_CAP_REVISION_2;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct vfs_cap_data_array {
    pub permitted:   __le32,
    pub inheritable: __le32,
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct vfs_cap_data {
    pub magic_etc: __le32,
    pub data: [vfs_cap_data_array; VFS_CAP_U32 as usize],
}

pub const _LINUX_CAPABILITY_VERSION : c_int = _LINUX_CAPABILITY_VERSION_1;
pub const _LINUX_CAPABILITY_U32S    : c_int = _LINUX_CAPABILITY_U32S_1;

pub const CAP_CHOWN            : c_int = 0;
pub const CAP_DAC_OVERRIDE     : c_int = 1;
pub const CAP_DAC_READ_SEARCH  : c_int = 2;
pub const CAP_FOWNER           : c_int = 3;
pub const CAP_FSETID           : c_int = 4;
pub const CAP_KILL             : c_int = 5;
pub const CAP_SETGID           : c_int = 6;
pub const CAP_SETUID           : c_int = 7;
pub const CAP_SETPCAP          : c_int = 8;
pub const CAP_LINUX_IMMUTABLE  : c_int = 9;
pub const CAP_NET_BIND_SERVICE : c_int = 10;
pub const CAP_NET_BROADCAST    : c_int = 11;
pub const CAP_NET_ADMIN        : c_int = 12;
pub const CAP_NET_RAW          : c_int = 13;
pub const CAP_IPC_LOCK         : c_int = 14;
pub const CAP_IPC_OWNER        : c_int = 15;
pub const CAP_SYS_MODULE       : c_int = 16;
pub const CAP_SYS_RAWIO        : c_int = 17;
pub const CAP_SYS_CHROOT       : c_int = 18;
pub const CAP_SYS_PTRACE       : c_int = 19;
pub const CAP_SYS_PACCT        : c_int = 20;
pub const CAP_SYS_ADMIN        : c_int = 21;
pub const CAP_SYS_BOOT         : c_int = 22;
pub const CAP_SYS_NICE         : c_int = 23;
pub const CAP_SYS_RESOURCE     : c_int = 24;
pub const CAP_SYS_TIME         : c_int = 25;
pub const CAP_SYS_TTY_CONFIG   : c_int = 26;
pub const CAP_MKNOD            : c_int = 27;
pub const CAP_LEASE            : c_int = 28;
pub const CAP_AUDIT_WRITE      : c_int = 29;
pub const CAP_AUDIT_CONTROL    : c_int = 30;
pub const CAP_SETFCAP          : c_int = 31;
pub const CAP_MAC_OVERRIDE     : c_int = 32;
pub const CAP_MAC_ADMIN        : c_int = 33;
pub const CAP_SYSLOG           : c_int = 34;
pub const CAP_WAKE_ALARM       : c_int = 35;
pub const CAP_BLOCK_SUSPEND    : c_int = 36;
pub const CAP_AUDIT_READ       : c_int = 37;
pub const CAP_LAST_CAP         : c_int = CAP_AUDIT_READ;

pub fn cap_valid(x: c_int) -> bool { x >= 0 && x <= CAP_LAST_CAP }

pub fn CAP_TO_INDEX(x: c_int) -> c_int { x >> 5 }
pub fn CAP_TO_MASK(x: c_int) -> c_int { 1 << (x & 31) }

// key.h

pub type key_serial_t = i32;
pub type key_perm_t   = u32;

// uio.h

#[repr(C)]
#[derive(Pod, Eq)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: user_size_t,
}

pub const UIO_FASTIOV : usize = 8;
pub const UIO_MAXIOV  : usize = 1024;

//////////////////////////////
// include/uapi/linux/socket.h
//////////////////////////////

pub type __kernel_sa_family_t = c_ushort;

pub const _K_SS_MAXSIZE   : usize = 128;
pub const _K_SS_ALIGNSIZE : usize = USER_POINTER_ALIGN;

// NOTE: The C structure looks different but has an `aligned` attribute with
// _K_SS_ALIGNSIZE alignment. We just use u64 to make sure that we get (at least) the
// correct alignment.  This is actually a bit too much on x32 which aligns u64 at 8 bytes
// instead of 4 but what can you do? (The alternative would be usize which is a bit too
// unwieldy for my taste.)
#[repr(C)]
#[derive(Pod)]
pub struct __kernel_sockaddr_storage {
    pub _data: [u64; _K_SS_MAXSIZE / 8],
}

/////////////////////////
// include/linux/socket.h
/////////////////////////

#[repr(C)]
#[derive(Pod)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [k_char; 14],
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct linger {
    pub l_onoff: k_int,
    pub l_linger: k_int,
}

// NOTE: This is written so that it agrees with user_msghdr in non-compat context and
// compat_msghdr in compat context.
#[repr(C)]
#[derive(Pod, Eq)]
pub struct msghdr {
    pub msg_name:       *mut c_void,
    pub msg_namelen:    c_int,
    pub msg_iov:        *mut iovec,
    pub msg_iovlen:     user_size_t,
    pub msg_control:    *mut c_void,
    pub msg_controllen: user_size_t,
    pub msg_flags:      c_uint,
}

// NOTE: As above.
#[repr(C)]
#[derive(Pod, Eq)]
pub struct mmsghdr {
    pub msg_hdr: msghdr,
    pub msg_len: c_uint,
}

// NOTE: As above.
#[repr(C)]
#[derive(Pod, Eq)]
pub struct cmsghdr {
    pub cmsg_len:   user_size_t,
    pub cmsg_level: c_int,
    pub cmsg_type:  c_int,
}

pub const SCM_RIGHTS      : c_int = 0x01;
pub const SCM_CREDENTIALS : c_int = 0x02;
pub const SCM_SECURITY    : c_int = 0x03;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct ucred {
    pub pid: __u32,
    pub uid: __u32,
    pub gid: __u32,
}

pub const AF_UNSPEC      : c_int = 0;
pub const AF_UNIX        : c_int = 1;
pub const AF_LOCAL       : c_int = 1;
pub const AF_INET        : c_int = 2;
pub const AF_AX25        : c_int = 3;
pub const AF_IPX         : c_int = 4;
pub const AF_APPLETALK   : c_int = 5;
pub const AF_NETROM      : c_int = 6;
pub const AF_BRIDGE      : c_int = 7;
pub const AF_ATMPVC      : c_int = 8;
pub const AF_X25         : c_int = 9;
pub const AF_INET6       : c_int = 10;
pub const AF_ROSE        : c_int = 11;
pub const AF_DECnet      : c_int = 12;
pub const AF_NETBEUI     : c_int = 13;
pub const AF_SECURITY    : c_int = 14;
pub const AF_KEY         : c_int = 15;
pub const AF_NETLINK     : c_int = 16;
pub const AF_ROUTE       : c_int = AF_NETLINK;
pub const AF_PACKET      : c_int = 17;
pub const AF_ASH         : c_int = 18;
pub const AF_ECONET      : c_int = 19;
pub const AF_ATMSVC      : c_int = 20;
pub const AF_RDS         : c_int = 21;
pub const AF_SNA         : c_int = 22;
pub const AF_IRDA        : c_int = 23;
pub const AF_PPPOX       : c_int = 24;
pub const AF_WANPIPE     : c_int = 25;
pub const AF_LLC         : c_int = 26;
pub const AF_IB          : c_int = 27;
pub const AF_CAN         : c_int = 29;
pub const AF_TIPC        : c_int = 30;
pub const AF_BLUETOOTH   : c_int = 31;
pub const AF_IUCV        : c_int = 32;
pub const AF_RXRPC       : c_int = 33;
pub const AF_ISDN        : c_int = 34;
pub const AF_PHONET      : c_int = 35;
pub const AF_IEEE802154  : c_int = 36;
pub const AF_CAIF        : c_int = 37;
pub const AF_ALG         : c_int = 38;
pub const AF_NFC         : c_int = 39;
pub const AF_VSOCK       : c_int = 40;
pub const AF_MAX         : c_int = 41;

pub const PF_UNSPEC      : c_int = AF_UNSPEC;
pub const PF_UNIX        : c_int = AF_UNIX;
pub const PF_LOCAL       : c_int = AF_LOCAL;
pub const PF_INET        : c_int = AF_INET;
pub const PF_AX25        : c_int = AF_AX25;
pub const PF_IPX         : c_int = AF_IPX;
pub const PF_APPLETALK   : c_int = AF_APPLETALK;
pub const PF_NETROM      : c_int = AF_NETROM;
pub const PF_BRIDGE      : c_int = AF_BRIDGE;
pub const PF_ATMPVC      : c_int = AF_ATMPVC;
pub const PF_X25         : c_int = AF_X25;
pub const PF_INET6       : c_int = AF_INET6;
pub const PF_ROSE        : c_int = AF_ROSE;
pub const PF_DECnet      : c_int = AF_DECnet;
pub const PF_NETBEUI     : c_int = AF_NETBEUI;
pub const PF_SECURITY    : c_int = AF_SECURITY;
pub const PF_KEY         : c_int = AF_KEY;
pub const PF_NETLINK     : c_int = AF_NETLINK;
pub const PF_ROUTE       : c_int = AF_ROUTE;
pub const PF_PACKET      : c_int = AF_PACKET;
pub const PF_ASH         : c_int = AF_ASH;
pub const PF_ECONET      : c_int = AF_ECONET;
pub const PF_ATMSVC      : c_int = AF_ATMSVC;
pub const PF_RDS         : c_int = AF_RDS;
pub const PF_SNA         : c_int = AF_SNA;
pub const PF_IRDA        : c_int = AF_IRDA;
pub const PF_PPPOX       : c_int = AF_PPPOX;
pub const PF_WANPIPE     : c_int = AF_WANPIPE;
pub const PF_LLC         : c_int = AF_LLC;
pub const PF_IB          : c_int = AF_IB;
pub const PF_CAN         : c_int = AF_CAN;
pub const PF_TIPC        : c_int = AF_TIPC;
pub const PF_BLUETOOTH   : c_int = AF_BLUETOOTH;
pub const PF_IUCV        : c_int = AF_IUCV;
pub const PF_RXRPC       : c_int = AF_RXRPC;
pub const PF_ISDN        : c_int = AF_ISDN;
pub const PF_PHONET      : c_int = AF_PHONET;
pub const PF_IEEE802154  : c_int = AF_IEEE802154;
pub const PF_CAIF        : c_int = AF_CAIF;
pub const PF_ALG         : c_int = AF_ALG;
pub const PF_NFC         : c_int = AF_NFC;
pub const PF_VSOCK       : c_int = AF_VSOCK;
pub const PF_MAX         : c_int = AF_MAX;

pub const SOMAXCONN : c_int = 128;

pub const MSG_OOB              : c_int = 1;
pub const MSG_PEEK             : c_int = 2;
pub const MSG_DONTROUTE        : c_int = 4;
pub const MSG_TRYHARD          : c_int = 4;
pub const MSG_CTRUNC           : c_int = 8;
pub const MSG_PROBE            : c_int = 0x10;
pub const MSG_TRUNC            : c_int = 0x20;
pub const MSG_DONTWAIT         : c_int = 0x40;
pub const MSG_EOR              : c_int = 0x80;
pub const MSG_WAITALL          : c_int = 0x100;
pub const MSG_FIN              : c_int = 0x200;
pub const MSG_SYN              : c_int = 0x400;
pub const MSG_CONFIRM          : c_int = 0x800;
pub const MSG_RST              : c_int = 0x1000;
pub const MSG_ERRQUEUE         : c_int = 0x2000;
pub const MSG_NOSIGNAL         : c_int = 0x4000;
pub const MSG_MORE             : c_int = 0x8000;
pub const MSG_WAITFORONE       : c_int = 0x10000;
pub const MSG_SENDPAGE_NOTLAST : c_int = 0x20000;
pub const MSG_EOF              : c_int = MSG_FIN;
pub const MSG_FASTOPEN         : c_int = 0x20000000;
pub const MSG_CMSG_CLOEXEC     : c_int = 0x40000000;

pub const SOL_IP        : c_int = 0;
pub const SOL_TCP       : c_int = 6;
pub const SOL_UDP       : c_int = 17;
pub const SOL_IPV6      : c_int = 41;
pub const SOL_ICMPV6    : c_int = 58;
pub const SOL_SCTP      : c_int = 132;
pub const SOL_UDPLITE   : c_int = 136;
pub const SOL_RAW       : c_int = 255;
pub const SOL_IPX       : c_int = 256;
pub const SOL_AX25      : c_int = 257;
pub const SOL_ATALK     : c_int = 258;
pub const SOL_NETROM    : c_int = 259;
pub const SOL_ROSE      : c_int = 260;
pub const SOL_DECNET    : c_int = 261;
pub const SOL_X25       : c_int = 262;
pub const SOL_PACKET    : c_int = 263;
pub const SOL_ATM       : c_int = 264;
pub const SOL_AAL       : c_int = 265;
pub const SOL_IRDA      : c_int = 266;
pub const SOL_NETBEUI   : c_int = 267;
pub const SOL_LLC       : c_int = 268;
pub const SOL_DCCP      : c_int = 269;
pub const SOL_NETLINK   : c_int = 270;
pub const SOL_TIPC      : c_int = 271;
pub const SOL_RXRPC     : c_int = 272;
pub const SOL_PPPOL2TP  : c_int = 273;
pub const SOL_BLUETOOTH : c_int = 274;
pub const SOL_PNPIPE    : c_int = 275;
pub const SOL_RDS       : c_int = 276;
pub const SOL_IUCV      : c_int = 277;
pub const SOL_CAIF      : c_int = 278;
pub const SOL_ALG       : c_int = 279;
pub const SOL_NFC       : c_int = 280;

pub const IPX_TYPE : c_int = 1;

// timex.h

#[repr(C)]
#[derive(Pod, Eq)]
pub struct timex {
    pub modes:     c_uint,
    pub offset:    __kernel_long_t,
    pub freq:      __kernel_long_t,
    pub maxerror:  __kernel_long_t,
    pub esterror:  __kernel_long_t,
    pub status:    c_int,
    pub constant:  __kernel_long_t,
    pub precision: __kernel_long_t,
    pub tolerance: __kernel_long_t,
    pub time:      timeval,
    pub tick:      __kernel_long_t,
    pub ppsfreq:   __kernel_long_t,
    pub jitter:    __kernel_long_t,
    pub shift:     c_int,
    pub stabil:    __kernel_long_t,
    pub jitcnt:    __kernel_long_t,
    pub calcnt:    __kernel_long_t,
    pub errcnt:    __kernel_long_t,
    pub stbcnt:    __kernel_long_t,

    pub tai: c_int,

    pub _unused: [u32; 11],
}

pub const ADJ_OFFSET            : c_int = 0x0001;
pub const ADJ_FREQUENCY         : c_int = 0x0002;
pub const ADJ_MAXERROR          : c_int = 0x0004;
pub const ADJ_ESTERROR          : c_int = 0x0008;
pub const ADJ_STATUS            : c_int = 0x0010;
pub const ADJ_TIMECONST         : c_int = 0x0020;
pub const ADJ_TAI               : c_int = 0x0080;
pub const ADJ_SETOFFSET         : c_int = 0x0100;
pub const ADJ_MICRO             : c_int = 0x1000;
pub const ADJ_NANO              : c_int = 0x2000;
pub const ADJ_TICK              : c_int = 0x4000;
pub const ADJ_OFFSET_SINGLESHOT : c_int = 0x8001;
pub const ADJ_OFFSET_SS_READ    : c_int = 0xa001;

pub const MOD_OFFSET    : c_int = ADJ_OFFSET;
pub const MOD_FREQUENCY : c_int = ADJ_FREQUENCY;
pub const MOD_MAXERROR  : c_int = ADJ_MAXERROR;
pub const MOD_ESTERROR  : c_int = ADJ_ESTERROR;
pub const MOD_STATUS    : c_int = ADJ_STATUS;
pub const MOD_TIMECONST : c_int = ADJ_TIMECONST;
pub const MOD_TAI       : c_int = ADJ_TAI;
pub const MOD_MICRO     : c_int = ADJ_MICRO;
pub const MOD_NANO      : c_int = ADJ_NANO;

pub const STA_PLL       : c_int = 0x0001;
pub const STA_PPSFREQ   : c_int = 0x0002;
pub const STA_PPSTIME   : c_int = 0x0004;
pub const STA_FLL       : c_int = 0x0008;
pub const STA_INS       : c_int = 0x0010;
pub const STA_DEL       : c_int = 0x0020;
pub const STA_UNSYNC    : c_int = 0x0040;
pub const STA_FREQHOLD  : c_int = 0x0080;
pub const STA_PPSSIGNAL : c_int = 0x0100;
pub const STA_PPSJITTER : c_int = 0x0200;
pub const STA_PPSWANDER : c_int = 0x0400;
pub const STA_PPSERROR  : c_int = 0x0800;
pub const STA_CLOCKERR  : c_int = 0x1000;
pub const STA_NANO      : c_int = 0x2000;
pub const STA_MODE      : c_int = 0x4000;
pub const STA_CLK       : c_int = 0x8000;

pub const STA_RONLY : c_int = STA_PPSSIGNAL | STA_PPSJITTER | STA_PPSWANDER |
                              STA_PPSERROR | STA_CLOCKERR | STA_NANO | STA_MODE | STA_CLK;

pub const TIME_OK    : c_int = 0;
pub const TIME_INS   : c_int = 1;
pub const TIME_DEL   : c_int = 2;
pub const TIME_OOP   : c_int = 3;
pub const TIME_WAIT  : c_int = 4;
pub const TIME_ERROR : c_int = 5;
pub const TIME_BAD   : c_int = TIME_ERROR;

// aio_abi.h

pub type aio_context_t = __kernel_ulong_t;

pub const IOCB_CMD_PREAD   : c_int = 0;
pub const IOCB_CMD_PWRITE  : c_int = 1;
pub const IOCB_CMD_FSYNC   : c_int = 2;
pub const IOCB_CMD_FDSYNC  : c_int = 3;
pub const IOCB_CMD_PREADX  : c_int = 4;
pub const IOCB_CMD_POLL    : c_int = 5;
pub const IOCB_CMD_NOOP    : c_int = 6;
pub const IOCB_CMD_PREADV  : c_int = 7;
pub const IOCB_CMD_PWRITEV : c_int = 8;

pub const IOCB_FLAG_RESFD : c_int = 1 << 0;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct io_event {
    pub data: __u64,
    pub obj:  __u64,
    pub res:  __s64,
    pub res2: __s64,
}

#[cfg(target_endian = "little")]
#[repr(C)]
#[derive(Pod, Eq)]
pub struct iocb {
    pub aio_data:       __u64,
    pub aio_key:        __u32,
    pub aio_reserved1:  __u32,
    pub aio_lio_opcode: __u16,
    pub aio_reqprio:    __s16,
    pub aio_fildes:     __u32,
    pub aio_buf:        __u64,
    pub aio_nbytes:     __u64,
    pub aio_offset:     __s64,
    pub aio_reserved2:  __u64,
    pub aio_flags:      __u32,
    pub aio_resfd:      __u32,
}

#[cfg(target_endian = "big")]
#[repr(C)]
#[derive(Pod, Eq)]
pub struct iocb {
    pub aio_data:       __u64,
    pub aio_reserved1:  __u32, // these two fields are
    pub aio_key:        __u32, // swapped
    pub aio_lio_opcode: __u16,
    pub aio_reqprio:    __s16,
    pub aio_fildes:     __u32,
    pub aio_buf:        __u64,
    pub aio_nbytes:     __u64,
    pub aio_offset:     __s64,
    pub aio_reserved2:  __u64,
    pub aio_flags:      __u32,
    pub aio_resfd:      __u32,
}

// fs.h

#[repr(C)]
#[derive(Pod, Eq)]
pub struct file_handle {
    pub handle_bytes: __u32,
    pub handle_type: k_int,
    pub f_handle: [c_uchar; 0],
}

// getcpu.h

#[repr(C)]
#[derive(Pod, Eq)]
pub struct getcpu_cache {
    pub blob: [k_long; 128 / BYTES_PER_LONG],
}

// kexec.h

pub const KEXEC_ON_CRASH         : c_int = 0x00000001;
pub const KEXEC_PRESERVE_CONTEXT : c_int = 0x00000002;
pub const KEXEC_ARCH_MASK        : c_int = 0xffff0000;

pub const KEXEC_FILE_UNLOAD       : c_int = 0x00000001;
pub const KEXEC_FILE_ON_CRASH     : c_int = 0x00000002;
pub const KEXEC_FILE_NO_INITRAMFS : c_int = 0x00000004;

pub const KEXEC_ARCH_DEFAULT : c_int =  0 << 16;
pub const KEXEC_ARCH_386     : c_int =  3 << 16;
pub const KEXEC_ARCH_68K     : c_int =  4 << 16;
pub const KEXEC_ARCH_X86_64  : c_int = 62 << 16;
pub const KEXEC_ARCH_PPC     : c_int = 20 << 16;
pub const KEXEC_ARCH_PPC64   : c_int = 21 << 16;
pub const KEXEC_ARCH_IA_64   : c_int = 50 << 16;
pub const KEXEC_ARCH_ARM     : c_int = 40 << 16;
pub const KEXEC_ARCH_S390    : c_int = 22 << 16;
pub const KEXEC_ARCH_SH      : c_int = 42 << 16;
pub const KEXEC_ARCH_MIPS_LE : c_int = 10 << 16;
pub const KEXEC_ARCH_MIPS    : c_int =  8 << 16;

pub const KEXEC_SEGMENT_MAX : c_int = 16;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct kexec_segment {
    pub buf: *const c_void,
    pub bufsz: user_size_t,
    pub mem: *const c_void,
    pub memsz: user_size_t,
}

// straight from fs/readdir.c

#[repr(C)]
#[derive(Pod, Eq)]
pub struct linux_dirent {
    pub d_ino:    k_ulong,
    pub d_off:    k_ulong,
    pub d_reclen: k_ushort,
    pub d_name: [k_char; 1],
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct old_linux_dirent {
    pub d_ino:    k_ulong,
    pub d_offset: k_ulong,
    pub d_namlen: k_ushort,
    pub d_name: [k_char; 1],
}

// straight from mm/mmap.c

#[repr(C)]
#[derive(Pod, Eq)]
pub struct mmap_arg_struct {
    pub addr:   k_ulong,
    pub len:    k_ulong,
    pub prot:   k_ulong,
    pub flags:  k_ulong,
    pub fd:     k_ulong,
    pub offset: k_ulong,
}

// mqueue.h

pub const MQ_PRIO_MAX  : c_int = 32768;
pub const MQ_BYTES_MAX : c_int = 819200;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct mq_attr {
    pub mq_flags:   __kernel_long_t,
    pub mq_maxmsg:  __kernel_long_t,
    pub mq_msgsize: __kernel_long_t,
    pub mq_curmsgs: __kernel_long_t,
    pub __reserved: [__kernel_long_t; 4],
}

pub const NOTIFY_NONE       : c_int = 0;
pub const NOTIFY_WOKENUP    : c_int = 1;
pub const NOTIFY_REMOVED    : c_int = 2;
pub const NOTIFY_COOKIE_LEN : c_int = 32;

// ipc.h

pub const IPC_PRIVATE : __kernel_key_t = 0;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct ipc_perm {
    pub key:  __kernel_key_t,
    pub uid:  __kernel_uid_t,
    pub gid:  __kernel_gid_t,
    pub cuid: __kernel_uid_t,
    pub cgid: __kernel_gid_t,
    pub mode: __kernel_mode_t,
    pub seq:  c_ushort,
}

pub const IPC_CREAT  : c_int = 0o0001000;
pub const IPC_EXCL   : c_int = 0o0002000;
pub const IPC_NOWAIT : c_int = 0o0004000;

pub const IPC_DIPC : c_int = 00010000;
pub const IPC_OWN  : c_int = 00020000;

pub const IPC_RMID : c_int = 0;
pub const IPC_SET  : c_int = 1;
pub const IPC_STAT : c_int = 2;
pub const IPC_INFO : c_int = 3;

pub const IPC_OLD : c_int = 0;
pub const IPC_64  : c_int = 0x0100;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct ipc_kludge {
    pub msgp: *mut msgbuf,
    pub msgtyp: c_long, // XXX: Maybe use k_long here?
}

pub const SEMOP      : c_int = 1;
pub const SEMGET     : c_int = 2;
pub const SEMCTL     : c_int = 3;
pub const SEMTIMEDOP : c_int = 4;
pub const MSGSND     : c_int = 11;
pub const MSGRCV     : c_int = 12;
pub const MSGGET     : c_int = 13;
pub const MSGCTL     : c_int = 14;
pub const SHMAT      : c_int = 21;
pub const SHMDT      : c_int = 22;
pub const SHMGET     : c_int = 23;
pub const SHMCTL     : c_int = 24;
pub const DIPC       : c_int = 25;

pub fn IPCCALL(version: c_int, op: c_int) -> c_int { version << 16 | op }

// msg.h

pub const MSG_STAT : c_int = 11;
pub const MSG_INFO : c_int = 12;

pub const MSG_NOERROR : c_int = 0o10000;
pub const MSG_EXCEPT  : c_int = 0o20000;
pub const MSG_COPY    : c_int = 0o40000;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct msgbuf {
    pub mtype: __kernel_long_t,
    pub mtext: [c_char; 1],
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct msginfo {
    pub msgpool: c_int,
    pub msgmap:  c_int,
    pub msgmax:  c_int,
    pub msgmnb:  c_int,
    pub msgmni:  c_int,
    pub msgssz:  c_int,
    pub msgtql:  c_int,
    pub msgseg:  c_ushort,
}

pub const MSGMNI : c_int = 32000;
pub const MSGMAX : c_int = 8192;
pub const MSGMNB : c_int = 16384;

pub const MSGPOOL : c_int = MSGMNI * MSGMNB / 1024;
pub const MSGTQL : c_int = MSGMNB;
pub const MSGMAP : c_int =  MSGMNB;
pub const MSGSSZ : c_int =  16;
pub const __MSGSEG : c_int = (MSGPOOL * 1024) / MSGSSZ;

// utsname.h

pub const __OLD_UTS_LEN : usize = 8;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct oldold_utsname {
    pub sysname:  [c_char; 9],
    pub nodename: [c_char; 9],
    pub release:  [c_char; 9],
    pub version:  [c_char; 9],
    pub machine:  [c_char; 9],
}

pub const __NEW_UTS_LEN : usize = 64;

#[repr(C)]
#[derive(Pod)]
pub struct old_utsname {
    pub sysname:  [c_char; 65],
    pub nodename: [c_char; 65],
    pub release:  [c_char; 65],
    pub version:  [c_char; 65],
    pub machine:  [c_char; 65],
}

#[repr(C)]
#[derive(Pod)]
pub struct new_utsname {
    pub sysname:    [c_char; __NEW_UTS_LEN + 1],
    pub nodename:   [c_char; __NEW_UTS_LEN + 1],
    pub release:    [c_char; __NEW_UTS_LEN + 1],
    pub version:    [c_char; __NEW_UTS_LEN + 1],
    pub machine:    [c_char; __NEW_UTS_LEN + 1],
    pub domainname: [c_char; __NEW_UTS_LEN + 1],
}

// signal.h

#[repr(C)]
#[derive(Pod)]
pub struct old_sigaction {
    pub sa_handler:  __sighandler_t,
    pub sa_mask:     old_sigset_t,
    pub sa_flags:    k_ulong,
    pub sa_restorer: __sigrestore_t,
}

// perf_event.h

pub const PERF_TYPE_HARDWARE   : c_int = 0;
pub const PERF_TYPE_SOFTWARE   : c_int = 1;
pub const PERF_TYPE_TRACEPOINT : c_int = 2;
pub const PERF_TYPE_HW_CACHE   : c_int = 3;
pub const PERF_TYPE_RAW        : c_int = 4;
pub const PERF_TYPE_BREAKPOINT : c_int = 5;
pub const PERF_TYPE_MAX        : c_int = 6;

pub const PERF_COUNT_HW_CPU_CYCLES              : c_int = 0;
pub const PERF_COUNT_HW_INSTRUCTIONS            : c_int = 1;
pub const PERF_COUNT_HW_CACHE_REFERENCES        : c_int = 2;
pub const PERF_COUNT_HW_CACHE_MISSES            : c_int = 3;
pub const PERF_COUNT_HW_BRANCH_INSTRUCTIONS     : c_int = 4;
pub const PERF_COUNT_HW_BRANCH_MISSES           : c_int = 5;
pub const PERF_COUNT_HW_BUS_CYCLES              : c_int = 6;
pub const PERF_COUNT_HW_STALLED_CYCLES_FRONTEND : c_int = 7;
pub const PERF_COUNT_HW_STALLED_CYCLES_BACKEND  : c_int = 8;
pub const PERF_COUNT_HW_REF_CPU_CYCLES          : c_int = 9;
pub const PERF_COUNT_HW_MAX                     : c_int = 10;

pub const PERF_COUNT_HW_CACHE_L1D  : c_int = 0;
pub const PERF_COUNT_HW_CACHE_L1I  : c_int = 1;
pub const PERF_COUNT_HW_CACHE_LL   : c_int = 2;
pub const PERF_COUNT_HW_CACHE_DTLB : c_int = 3;
pub const PERF_COUNT_HW_CACHE_ITLB : c_int = 4;
pub const PERF_COUNT_HW_CACHE_BPU  : c_int = 5;
pub const PERF_COUNT_HW_CACHE_NODE : c_int = 6;
pub const PERF_COUNT_HW_CACHE_MAX  : c_int = 8;

pub const PERF_COUNT_HW_CACHE_OP_READ     : c_int = 0;
pub const PERF_COUNT_HW_CACHE_OP_WRITE    : c_int = 1;
pub const PERF_COUNT_HW_CACHE_OP_PREFETCH : c_int = 2;
pub const PERF_COUNT_HW_CACHE_OP_MAX      : c_int = 3;

pub const PERF_COUNT_HW_CACHE_RESULT_ACCESS : c_int = 0;
pub const PERF_COUNT_HW_CACHE_RESULT_MISS   : c_int = 1;
pub const PERF_COUNT_HW_CACHE_RESULT_MAX    : c_int = 2;

pub const PERF_COUNT_SW_CPU_CLOCK        : c_int = 0;
pub const PERF_COUNT_SW_TASK_CLOCK       : c_int = 1;
pub const PERF_COUNT_SW_PAGE_FAULTS      : c_int = 2;
pub const PERF_COUNT_SW_CONTEXT_SWITCHES : c_int = 3;
pub const PERF_COUNT_SW_CPU_MIGRATIONS   : c_int = 4;
pub const PERF_COUNT_SW_PAGE_FAULTS_MIN  : c_int = 5;
pub const PERF_COUNT_SW_PAGE_FAULTS_MAJ  : c_int = 6;
pub const PERF_COUNT_SW_ALIGNMENT_FAULTS : c_int = 7;
pub const PERF_COUNT_SW_EMULATION_FAULTS : c_int = 8;
pub const PERF_COUNT_SW_DUMMY            : c_int = 9;
pub const PERF_COUNT_SW_MAX              : c_int = 10;

pub const PERF_SAMPLE_IP           : c_uint = 1 << 0;
pub const PERF_SAMPLE_TID          : c_uint = 1 << 1;
pub const PERF_SAMPLE_TIME         : c_uint = 1 << 2;
pub const PERF_SAMPLE_ADDR         : c_uint = 1 << 3;
pub const PERF_SAMPLE_READ         : c_uint = 1 << 4;
pub const PERF_SAMPLE_CALLCHAIN    : c_uint = 1 << 5;
pub const PERF_SAMPLE_ID           : c_uint = 1 << 6;
pub const PERF_SAMPLE_CPU          : c_uint = 1 << 7;
pub const PERF_SAMPLE_PERIOD       : c_uint = 1 << 8;
pub const PERF_SAMPLE_STREAM_ID    : c_uint = 1 << 9;
pub const PERF_SAMPLE_RAW          : c_uint = 1 << 10;
pub const PERF_SAMPLE_BRANCH_STACK : c_uint = 1 << 11;
pub const PERF_SAMPLE_REGS_USER    : c_uint = 1 << 12;
pub const PERF_SAMPLE_STACK_USER   : c_uint = 1 << 13;
pub const PERF_SAMPLE_WEIGHT       : c_uint = 1 << 14;
pub const PERF_SAMPLE_DATA_SRC     : c_uint = 1 << 15;
pub const PERF_SAMPLE_IDENTIFIER   : c_uint = 1 << 16;
pub const PERF_SAMPLE_TRANSACTION  : c_uint = 1 << 17;
pub const PERF_SAMPLE_REGS_INTR    : c_uint = 1 << 18;
pub const PERF_SAMPLE_MAX          : c_uint = 1 << 19;

pub const PERF_SAMPLE_BRANCH_USER       : c_uint = 1 << 0;
pub const PERF_SAMPLE_BRANCH_KERNEL     : c_uint = 1 << 1;
pub const PERF_SAMPLE_BRANCH_HV         : c_uint = 1 << 2;
pub const PERF_SAMPLE_BRANCH_ANY        : c_uint = 1 << 3;
pub const PERF_SAMPLE_BRANCH_ANY_CALL   : c_uint = 1 << 4;
pub const PERF_SAMPLE_BRANCH_ANY_RETURN : c_uint = 1 << 5;
pub const PERF_SAMPLE_BRANCH_IND_CALL   : c_uint = 1 << 6;
pub const PERF_SAMPLE_BRANCH_ABORT_TX   : c_uint = 1 << 7;
pub const PERF_SAMPLE_BRANCH_IN_TX      : c_uint = 1 << 8;
pub const PERF_SAMPLE_BRANCH_NO_TX      : c_uint = 1 << 9;
pub const PERF_SAMPLE_BRANCH_COND       : c_uint = 1 << 10;
pub const PERF_SAMPLE_BRANCH_MAX        : c_uint = 1 << 11;

pub const PERF_SAMPLE_BRANCH_PLM_ALL : c_uint = PERF_SAMPLE_BRANCH_USER |
                                                PERF_SAMPLE_BRANCH_KERNEL |
                                                PERF_SAMPLE_BRANCH_HV;

pub const PERF_SAMPLE_REGS_ABI_NONE : c_int = 0;
pub const PERF_SAMPLE_REGS_ABI_32   : c_int = 1;
pub const PERF_SAMPLE_REGS_ABI_64   : c_int = 2;

pub const PERF_TXN_ELISION        : c_ulonglong = (1          << 0);
pub const PERF_TXN_TRANSACTION    : c_ulonglong = (1          << 1);
pub const PERF_TXN_SYNC           : c_ulonglong = (1          << 2);
pub const PERF_TXN_ASYNC          : c_ulonglong = (1          << 3);
pub const PERF_TXN_RETRY          : c_ulonglong = (1          << 4);
pub const PERF_TXN_CONFLICT       : c_ulonglong = (1          << 5);
pub const PERF_TXN_CAPACITY_WRITE : c_ulonglong = (1          << 6);
pub const PERF_TXN_CAPACITY_READ  : c_ulonglong = (1          << 7);
pub const PERF_TXN_MAX            : c_ulonglong = (1          << 8);
pub const PERF_TXN_ABORT_MASK     : c_ulonglong = (0xffffffff << 32);
pub const PERF_TXN_ABORT_SHIFT    : c_ulonglong = 32;

pub const PERF_FORMAT_TOTAL_TIME_ENABLED : c_uint = 1 << 0;
pub const PERF_FORMAT_TOTAL_TIME_RUNNING : c_uint = 1 << 1;
pub const PERF_FORMAT_ID                 : c_uint = 1 << 2;
pub const PERF_FORMAT_GROUP              : c_uint = 1 << 3;
pub const PERF_FORMAT_MAX                : c_uint = 1 << 4;

pub const PERF_ATTR_SIZE_VER0 : c_int = 64;
pub const PERF_ATTR_SIZE_VER1 : c_int = 72;
pub const PERF_ATTR_SIZE_VER2 : c_int = 80;
pub const PERF_ATTR_SIZE_VER3 : c_int = 96;
pub const PERF_ATTR_SIZE_VER4 : c_int = 104;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct perf_event_attr {
    pub ty: __u32,
    pub size: __u32,
    pub config: __u64,
    //union {
    //	__u64		sample_period;
    //	__u64		sample_freq;
    //};
    __union_one: __u64,
    pub sample_type: __u64,
    pub read_format: __u64,
    //__u64		disabled       :  1,
    //			inherit	       :  1,
    //			pinned	       :  1,
    //			exclusive      :  1,
    //			exclude_user   :  1,
    //			exclude_kernel :  1,
    //			exclude_hv     :  1,
    //			exclude_idle   :  1,
    //			mmap           :  1,
    //			comm	       :  1,
    //			freq           :  1,
    //			inherit_stat   :  1,
    //			enable_on_exec :  1,
    //			task           :  1,
    //			watermark      :  1,
    //			precise_ip     :  2,
    //			mmap_data      :  1,
    //			sample_id_all  :  1,
    //			exclude_host   :  1,
    //			exclude_guest  :  1,
    //			exclude_callchain_kernel : 1,
    //			exclude_callchain_user   : 1,
    //			mmap2          :  1,
    //			comm_exec      :  1,
    //			__reserved_1   : 39;
    pub perf_flags: __u64,
    //union {
    //	__u32		wakeup_events;
    //	__u32		wakeup_watermark;
    //};
    __union_two: __u32,
    pub bp_type: __u32,
    //union {
    //	__u64		bp_addr;
    //	__u64		config1;
    //};
    __union_three: __u64,
    //union {
    //	__u64		bp_len;
    //	__u64		config2;
    //};
    __union_four: __u64,
    pub branch_sample_type: __u64,
    pub sample_regs_user: __u64,
    pub sample_stack_user: __u32,
    pub __reserved_2: __u32,
    pub sample_regs_intr: __u64,
}

impl perf_event_attr {
    pub fn sample_period(&self) -> __u64 { self.__union_one }
    pub fn set_sample_period(&mut self, val: __u64) { self.__union_one = val }

    pub fn sample_freq(&self) -> __u64 { self.__union_one }
    pub fn set_sample_freq(&mut self, val: __u64) { self.__union_one = val }

    pub fn disabled                 (&self) -> bool { bf64_get(self.perf_flags, 0, 1) != 0 }
    pub fn inherit                  (&self) -> bool { bf64_get(self.perf_flags, 1, 1) != 0 }
    pub fn pinned                   (&self) -> bool { bf64_get(self.perf_flags, 2, 1) != 0 }
    pub fn exclusive                (&self) -> bool { bf64_get(self.perf_flags, 3, 1) != 0 }
    pub fn exclude_user             (&self) -> bool { bf64_get(self.perf_flags, 4, 1) != 0 }
    pub fn exclude_kernel           (&self) -> bool { bf64_get(self.perf_flags, 5, 1) != 0 }
    pub fn exclude_hv               (&self) -> bool { bf64_get(self.perf_flags, 6, 1) != 0 }
    pub fn exclude_idle             (&self) -> bool { bf64_get(self.perf_flags, 7, 1) != 0 }
    pub fn mmap                     (&self) -> bool { bf64_get(self.perf_flags, 8, 1) != 0 }
    pub fn comm                     (&self) -> bool { bf64_get(self.perf_flags, 9, 1) != 0 }
    pub fn freq                     (&self) -> bool { bf64_get(self.perf_flags, 10, 1) != 0 }
    pub fn inherit_stat             (&self) -> bool { bf64_get(self.perf_flags, 11, 1) != 0 }
    pub fn enable_on_exec           (&self) -> bool { bf64_get(self.perf_flags, 12, 1) != 0 }
    pub fn task                     (&self) -> bool { bf64_get(self.perf_flags, 13, 1) != 0 }
    pub fn watermark                (&self) -> bool { bf64_get(self.perf_flags, 14, 1) != 0 }
    pub fn precise_ip               (&self) -> u64  { bf64_get(self.perf_flags, 15, 2) }
    pub fn mmap_data                (&self) -> bool { bf64_get(self.perf_flags, 17, 1) != 0 }
    pub fn sample_id_all            (&self) -> bool { bf64_get(self.perf_flags, 18, 1) != 0 }
    pub fn exclude_host             (&self) -> bool { bf64_get(self.perf_flags, 19, 1) != 0 }
    pub fn exclude_guest            (&self) -> bool { bf64_get(self.perf_flags, 20, 1) != 0 }
    pub fn exclude_callchain_kernel (&self) -> bool { bf64_get(self.perf_flags, 21, 1) != 0 }
    pub fn exclude_callchain_user   (&self) -> bool { bf64_get(self.perf_flags, 22, 1) != 0 }
    pub fn mmap2                    (&self) -> bool { bf64_get(self.perf_flags, 23, 1) != 0 }
    pub fn comm_exec                (&self) -> bool { bf64_get(self.perf_flags, 24, 1) != 0 }

    pub fn set_disabled                 (&mut self, val: bool) { self.perf_flags = bf64_set(self.perf_flags, 0, 1, val as u64) }
    pub fn set_inherit                  (&mut self, val: bool) { self.perf_flags = bf64_set(self.perf_flags, 1, 1, val as u64) }
    pub fn set_pinned                   (&mut self, val: bool) { self.perf_flags = bf64_set(self.perf_flags, 2, 1, val as u64) }
    pub fn set_exclusive                (&mut self, val: bool) { self.perf_flags = bf64_set(self.perf_flags, 3, 1, val as u64) }
    pub fn set_exclude_user             (&mut self, val: bool) { self.perf_flags = bf64_set(self.perf_flags, 4, 1, val as u64) }
    pub fn set_exclude_kernel           (&mut self, val: bool) { self.perf_flags = bf64_set(self.perf_flags, 5, 1, val as u64) }
    pub fn set_exclude_hv               (&mut self, val: bool) { self.perf_flags = bf64_set(self.perf_flags, 6, 1, val as u64) }
    pub fn set_exclude_idle             (&mut self, val: bool) { self.perf_flags = bf64_set(self.perf_flags, 7, 1, val as u64) }
    pub fn set_mmap                     (&mut self, val: bool) { self.perf_flags = bf64_set(self.perf_flags, 8, 1, val as u64) }
    pub fn set_comm                     (&mut self, val: bool) { self.perf_flags = bf64_set(self.perf_flags, 9, 1, val as u64) }
    pub fn set_freq                     (&mut self, val: bool) { self.perf_flags = bf64_set(self.perf_flags, 10, 1, val as u64) }
    pub fn set_inherit_stat             (&mut self, val: bool) { self.perf_flags = bf64_set(self.perf_flags, 11, 1, val as u64) }
    pub fn set_enable_on_exec           (&mut self, val: bool) { self.perf_flags = bf64_set(self.perf_flags, 12, 1, val as u64) }
    pub fn set_task                     (&mut self, val: bool) { self.perf_flags = bf64_set(self.perf_flags, 13, 1, val as u64) }
    pub fn set_watermark                (&mut self, val: bool) { self.perf_flags = bf64_set(self.perf_flags, 14, 1, val as u64) }
    pub fn set_precise_ip               (&mut self, val: u64 ) { self.perf_flags = bf64_set(self.perf_flags, 15, 2, val) }
    pub fn set_mmap_data                (&mut self, val: bool) { self.perf_flags = bf64_set(self.perf_flags, 17, 1, val as u64) }
    pub fn set_sample_id_all            (&mut self, val: bool) { self.perf_flags = bf64_set(self.perf_flags, 18, 1, val as u64) }
    pub fn set_exclude_host             (&mut self, val: bool) { self.perf_flags = bf64_set(self.perf_flags, 19, 1, val as u64) }
    pub fn set_exclude_guest            (&mut self, val: bool) { self.perf_flags = bf64_set(self.perf_flags, 20, 1, val as u64) }
    pub fn set_exclude_callchain_kernel (&mut self, val: bool) { self.perf_flags = bf64_set(self.perf_flags, 21, 1, val as u64) }
    pub fn set_exclude_callchain_user   (&mut self, val: bool) { self.perf_flags = bf64_set(self.perf_flags, 22, 1, val as u64) }
    pub fn set_mmap2                    (&mut self, val: bool) { self.perf_flags = bf64_set(self.perf_flags, 23, 1, val as u64) }
    pub fn set_comm_exec                (&mut self, val: bool) { self.perf_flags = bf64_set(self.perf_flags, 24, 1, val as u64) }

    pub fn wakeup_events(&self) -> __u32 { self.__union_two }
    pub fn set_wakeup_events(&mut self, val: __u32) { self.__union_two = val }

    pub fn wakeup_watermark(&self) -> __u32 { self.__union_two }
    pub fn set_wakeup_watermark(&mut self, val: __u32) { self.__union_two = val }

    pub fn bp_addr(&self) -> __u64 { self.__union_three }
    pub fn set_bp_addr(&mut self, val: __u64) { self.__union_three = val }

    pub fn config1(&self) -> __u64 { self.__union_three }
    pub fn set_config1(&mut self, val: __u64) { self.__union_three = val }

    pub fn bp_len(&self) -> __u64 { self.__union_four }
    pub fn set_bp_len(&mut self, val: __u64) { self.__union_four = val }

    pub fn config2(&self) -> __u64 { self.__union_four }
    pub fn set_config2(&mut self, val: __u64) { self.__union_four = val }
}

pub fn PERF_EVENT_IOC_ENABLE()     -> c_uint { _IO(b'$' as c_uint, 0) }
pub fn PERF_EVENT_IOC_DISABLE()    -> c_uint { _IO(b'$' as c_uint, 1) }
pub fn PERF_EVENT_IOC_REFRESH()    -> c_uint { _IO(b'$' as c_uint, 2) }
pub fn PERF_EVENT_IOC_RESET()      -> c_uint { _IO(b'$' as c_uint, 3) }
pub fn PERF_EVENT_IOC_PERIOD()     -> c_uint { _IOW::<__u64>(b'$' as c_uint, 4) }
pub fn PERF_EVENT_IOC_SET_OUTPUT() -> c_uint { _IO(b'$' as c_uint, 5) }
pub fn PERF_EVENT_IOC_SET_FILTER() -> c_uint { _IOW::<*mut c_char>(b'$' as c_uint, 6) }
pub fn PERF_EVENT_IOC_ID()         -> c_uint { _IOR::<*mut __u64>(b'$' as c_uint, 7 ) }

pub const PERF_IOC_FLAG_GROUP : c_uint = 1;

#[repr(C)]
#[derive(Pod)]
pub struct perf_event_mmap_page {
    pub version:        __u32,
    pub compat_version: __u32,
    pub lock:           __u32,
    pub index:          __u32,
    pub offset:         __s64,
    pub time_enabled:   __u64,
    pub time_running:   __u64,
    //union {
    //	__u64	capabilities;
    //	struct {
    //		__u64	cap_bit0		: 1,
    //			cap_bit0_is_deprecated	: 1,

    //			cap_user_rdpmc		: 1,
    //			cap_user_time		: 1,
    //			cap_user_time_zero	: 1,
    //			cap_____res		: 59;
    //	};
    //};
    __union_one: __u64,
    pub pmc_width:   __u16,
    pub time_shift:  __u16,
    pub time_mult:   __u32,
    pub time_offset: __u64,
    pub time_zero:   __u64,
    pub size:        __u32,
    pub __reserved:  [__u8; 118*8+4],
    pub data_head:   __u64,
    pub data_tail:   __u64,
}

impl perf_event_mmap_page {
    pub fn capabilities(&self) -> __u64 { self.__union_one }
    pub fn set_capabilities(&mut self, val: __u64) { self.__union_one = val }

    pub fn cap_bit0               (&self) -> bool { bf64_get(self.__union_one, 0, 1) != 0 }
    pub fn cap_bit0_is_deprecated (&self) -> bool { bf64_get(self.__union_one, 1, 1) != 0 }
    pub fn cap_user_rdpmc         (&self) -> bool { bf64_get(self.__union_one, 2, 1) != 0 }
    pub fn cap_user_time          (&self) -> bool { bf64_get(self.__union_one, 3, 1) != 0 }
    pub fn cap_user_time_zero     (&self) -> bool { bf64_get(self.__union_one, 4, 1) != 0 }

    pub fn set_cap_bit0               (&mut self, val: bool) { self.__union_one = bf64_set(self.__union_one, 0, 1, val as u64) }
    pub fn set_cap_bit0_is_deprecated (&mut self, val: bool) { self.__union_one = bf64_set(self.__union_one, 1, 1, val as u64) }
    pub fn set_cap_user_rdpmc         (&mut self, val: bool) { self.__union_one = bf64_set(self.__union_one, 2, 1, val as u64) }
    pub fn set_cap_user_time          (&mut self, val: bool) { self.__union_one = bf64_set(self.__union_one, 3, 1, val as u64) }
    pub fn set_cap_user_time_zero     (&mut self, val: bool) { self.__union_one = bf64_set(self.__union_one, 4, 1, val as u64) }
}


pub const PERF_RECORD_MISC_CPUMODE_MASK    : c_int = 7 << 0;
pub const PERF_RECORD_MISC_CPUMODE_UNKNOWN : c_int = 0 << 0;
pub const PERF_RECORD_MISC_KERNEL          : c_int = 1 << 0;
pub const PERF_RECORD_MISC_USER            : c_int = 2 << 0;
pub const PERF_RECORD_MISC_HYPERVISOR      : c_int = 3 << 0;
pub const PERF_RECORD_MISC_GUEST_KERNEL    : c_int = 4 << 0;
pub const PERF_RECORD_MISC_GUEST_USER      : c_int = 5 << 0;
pub const PERF_RECORD_MISC_MMAP_DATA       : c_int = 1 << 13;
pub const PERF_RECORD_MISC_COMM_EXEC       : c_int = 1 << 13;
pub const PERF_RECORD_MISC_EXACT_IP        : c_int = 1 << 14;
pub const PERF_RECORD_MISC_EXT_RESERVED    : c_int = 1 << 15;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct perf_event_header {
    pub ty: __u32,
    pub mi: __u16,
    pub si: __u16,
}

pub const PERF_RECORD_MMAP       : c_int = 1;
pub const PERF_RECORD_LOST       : c_int = 2;
pub const PERF_RECORD_COMM       : c_int = 3;
pub const PERF_RECORD_EXIT       : c_int = 4;
pub const PERF_RECORD_THROTTLE   : c_int = 5;
pub const PERF_RECORD_UNTHROTTLE : c_int = 6;
pub const PERF_RECORD_FORK       : c_int = 7;
pub const PERF_RECORD_READ       : c_int = 8;
pub const PERF_RECORD_SAMPLE     : c_int = 9;
pub const PERF_RECORD_MMAP2      : c_int = 10;
pub const PERF_RECORD_MAX        : c_int = 11;

pub const PERF_MAX_STACK_DEPTH : c_int = 127;

pub const PERF_CONTEXT_HV           : __u64 = -32;
pub const PERF_CONTEXT_KERNEL       : __u64 = -128;
pub const PERF_CONTEXT_USER         : __u64 = -512;
pub const PERF_CONTEXT_GUEST        : __u64 = -2048;
pub const PERF_CONTEXT_GUEST_KERNEL : __u64 = -2176;
pub const PERF_CONTEXT_GUEST_USER   : __u64 = -2560;
pub const PERF_CONTEXT_MAX          : __u64 = -4095;

pub const PERF_FLAG_FD_NO_GROUP : c_ulong = 1 << 0;
pub const PERF_FLAG_FD_OUTPUT   : c_ulong = 1 << 1;
pub const PERF_FLAG_PID_CGROUP  : c_ulong = 1 << 2;
pub const PERF_FLAG_FD_CLOEXEC  : c_ulong = 1 << 3;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct perf_mem_data_src {
    pub val: __u64,
}

impl perf_mem_data_src {
    pub fn mem_op(&self)    -> __u64 { bf64_get(self.val, 0,              5) }
    pub fn mem_lvl(&self)   -> __u64 { bf64_get(self.val, 5,              14) }
    pub fn mem_snoop(&self) -> __u64 { bf64_get(self.val, 5 + 14,         5) }
    pub fn mem_lock(&self)  -> __u64 { bf64_get(self.val, 5 + 14 + 5,     2) }
    pub fn mem_dtlb(&self)  -> __u64 { bf64_get(self.val, 5 + 14 + 5 + 2, 7) }

    pub fn set_mem_op(&mut    self, val: __u64) { self.val = bf64_set(self.val, 0,              5, val) }
    pub fn set_mem_lvl(&mut   self, val: __u64) { self.val = bf64_set(self.val, 5,              14, val) }
    pub fn set_mem_snoop(&mut self, val: __u64) { self.val = bf64_set(self.val, 5 + 14,         5, val) }
    pub fn set_mem_lock(&mut  self, val: __u64) { self.val = bf64_set(self.val, 5 + 14 + 5,     2, val) }
    pub fn set_mem_dtlb(&mut  self, val: __u64) { self.val = bf64_set(self.val, 5 + 14 + 5 + 2, 7, val) }
}

pub const PERF_MEM_OP_NA        : c_int = 0x01;
pub const PERF_MEM_OP_LOAD      : c_int = 0x02;
pub const PERF_MEM_OP_STORE     : c_int = 0x04;
pub const PERF_MEM_OP_PFETCH    : c_int = 0x08;
pub const PERF_MEM_OP_EXEC      : c_int = 0x10;
pub const PERF_MEM_OP_SHIFT     : c_int = 0;
pub const PERF_MEM_LVL_NA       : c_int = 0x01;
pub const PERF_MEM_LVL_HIT      : c_int = 0x02;
pub const PERF_MEM_LVL_MISS     : c_int = 0x04;
pub const PERF_MEM_LVL_L1       : c_int = 0x08;
pub const PERF_MEM_LVL_LFB      : c_int = 0x10;
pub const PERF_MEM_LVL_L2       : c_int = 0x20;
pub const PERF_MEM_LVL_L3       : c_int = 0x40;
pub const PERF_MEM_LVL_LOC_RAM  : c_int = 0x80;
pub const PERF_MEM_LVL_REM_RAM1 : c_int = 0x100;
pub const PERF_MEM_LVL_REM_RAM2 : c_int = 0x200;
pub const PERF_MEM_LVL_REM_CCE1 : c_int = 0x400;
pub const PERF_MEM_LVL_REM_CCE2 : c_int = 0x800;
pub const PERF_MEM_LVL_IO       : c_int = 0x1000;
pub const PERF_MEM_LVL_UNC      : c_int = 0x2000;
pub const PERF_MEM_LVL_SHIFT    : c_int = 5;
pub const PERF_MEM_SNOOP_NA     : c_int = 0x01;
pub const PERF_MEM_SNOOP_NONE   : c_int = 0x02;
pub const PERF_MEM_SNOOP_HIT    : c_int = 0x04;
pub const PERF_MEM_SNOOP_MISS   : c_int = 0x08;
pub const PERF_MEM_SNOOP_HITM   : c_int = 0x10;
pub const PERF_MEM_SNOOP_SHIFT  : c_int = 19;
pub const PERF_MEM_LOCK_NA      : c_int = 0x01;
pub const PERF_MEM_LOCK_LOCKED  : c_int = 0x02;
pub const PERF_MEM_LOCK_SHIFT   : c_int = 24;
pub const PERF_MEM_TLB_NA       : c_int = 0x01;
pub const PERF_MEM_TLB_HIT      : c_int = 0x02;
pub const PERF_MEM_TLB_MISS     : c_int = 0x04;
pub const PERF_MEM_TLB_L1       : c_int = 0x08;
pub const PERF_MEM_TLB_L2       : c_int = 0x10;
pub const PERF_MEM_TLB_WK       : c_int = 0x20;
pub const PERF_MEM_TLB_OS       : c_int = 0x40;
pub const PERF_MEM_TLB_SHIFT    : c_int = 26;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct perf_branch_entry {
    pub from: __u64,
    pub to: __u64,
    //__u64	mispred:1,
    //	predicted:1,
    //	in_tx:1,
    //	abort:1,
    //	reserved:60;
    __bitfield_one: __u64,
}

impl perf_branch_entry {
    pub fn mispred   (&self) -> bool { bf64_get(self.__bitfield_one, 0, 1) != 0 }
    pub fn predicted (&self) -> bool { bf64_get(self.__bitfield_one, 1, 1) != 0 }
    pub fn in_tx     (&self) -> bool { bf64_get(self.__bitfield_one, 2, 1) != 0 }
    pub fn abort     (&self) -> bool { bf64_get(self.__bitfield_one, 3, 1) != 0 }

    pub fn set_mispred   (&mut self, val: bool) { self.__bitfield_one = bf64_set(self.__bitfield_one, 0, 1, val as u64) }
    pub fn set_predicted (&mut self, val: bool) { self.__bitfield_one = bf64_set(self.__bitfield_one, 1, 1, val as u64) }
    pub fn set_in_tx     (&mut self, val: bool) { self.__bitfield_one = bf64_set(self.__bitfield_one, 2, 1, val as u64) }
    pub fn set_abort     (&mut self, val: bool) { self.__bitfield_one = bf64_set(self.__bitfield_one, 3, 1, val as u64) }
}

// quota.h

pub type qid_t = __kernel_uid32_t;

// resource.h

pub const RUSAGE_SELF     : c_int = 0;
pub const RUSAGE_CHILDREN : c_int = -1;
pub const RUSAGE_BOTH     : c_int = -2;
pub const RUSAGE_THREAD   : c_int = 1;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct rusage {
    pub ru_utime:    timeval,
    pub ru_stime:    timeval,
    pub ru_maxrss:   __kernel_long_t,
    pub ru_ixrss:    __kernel_long_t,
    pub ru_idrss:    __kernel_long_t,
    pub ru_isrss:    __kernel_long_t,
    pub ru_minflt:   __kernel_long_t,
    pub ru_majflt:   __kernel_long_t,
    pub ru_nswap:    __kernel_long_t,
    pub ru_inblock:  __kernel_long_t,
    pub ru_oublock:  __kernel_long_t,
    pub ru_msgsnd:   __kernel_long_t,
    pub ru_msgrcv:   __kernel_long_t,
    pub ru_nsignals: __kernel_long_t,
    pub ru_nvcsw:    __kernel_long_t,
    pub ru_nivcsw:   __kernel_long_t,
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct rlimit {
    pub rlim_cur: __kernel_ulong_t,
    pub rlim_max: __kernel_ulong_t,
}

pub const RLIM64_INFINITY: c_ulonglong = !0;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct rlimit64 {
    pub rlim_cur: __u64,
    pub rlim_max: __u64,
}

pub const PRIO_MIN     : c_int = -20;
pub const PRIO_MAX     : c_int = 20;
pub const PRIO_PROCESS : c_int = 0;
pub const PRIO_PGRP    : c_int = 1;
pub const PRIO_USER    : c_int = 2;

pub const _STK_LIM : c_int = 8*1024*1024;

// futex.h

pub const FUTEX_WAIT                    : c_int = 0;
pub const FUTEX_WAKE                    : c_int = 1;
pub const FUTEX_FD                      : c_int = 2;
pub const FUTEX_REQUEUE                 : c_int = 3;
pub const FUTEX_CMP_REQUEUE             : c_int = 4;
pub const FUTEX_WAKE_OP                 : c_int = 5;
pub const FUTEX_LOCK_PI                 : c_int = 6;
pub const FUTEX_UNLOCK_PI               : c_int = 7;
pub const FUTEX_TRYLOCK_PI              : c_int = 8;
pub const FUTEX_WAIT_BITSET             : c_int = 9;
pub const FUTEX_WAKE_BITSET             : c_int = 10;
pub const FUTEX_WAIT_REQUEUE_PI         : c_int = 11;
pub const FUTEX_CMP_REQUEUE_PI          : c_int = 12;
pub const FUTEX_PRIVATE_FLAG            : c_int = 128;
pub const FUTEX_CLOCK_REALTIME          : c_int = 256;
pub const FUTEX_CMD_MASK                : c_int = !(FUTEX_PRIVATE_FLAG | FUTEX_CLOCK_REALTIME);
pub const FUTEX_WAIT_PRIVATE            : c_int = FUTEX_WAIT            | FUTEX_PRIVATE_FLAG;
pub const FUTEX_WAKE_PRIVATE            : c_int = FUTEX_WAKE            | FUTEX_PRIVATE_FLAG;
pub const FUTEX_REQUEUE_PRIVATE         : c_int = FUTEX_REQUEUE         | FUTEX_PRIVATE_FLAG;
pub const FUTEX_CMP_REQUEUE_PRIVATE     : c_int = FUTEX_CMP_REQUEUE     | FUTEX_PRIVATE_FLAG;
pub const FUTEX_WAKE_OP_PRIVATE         : c_int = FUTEX_WAKE_OP         | FUTEX_PRIVATE_FLAG;
pub const FUTEX_LOCK_PI_PRIVATE         : c_int = FUTEX_LOCK_PI         | FUTEX_PRIVATE_FLAG;
pub const FUTEX_UNLOCK_PI_PRIVATE       : c_int = FUTEX_UNLOCK_PI       | FUTEX_PRIVATE_FLAG;
pub const FUTEX_TRYLOCK_PI_PRIVATE      : c_int = FUTEX_TRYLOCK_PI      | FUTEX_PRIVATE_FLAG;
pub const FUTEX_WAIT_BITSET_PRIVATE     : c_int = FUTEX_WAIT_BITSET     | FUTEX_PRIVATE_FLAG;
pub const FUTEX_WAKE_BITSET_PRIVATE     : c_int = FUTEX_WAKE_BITSET     | FUTEX_PRIVATE_FLAG;
pub const FUTEX_WAIT_REQUEUE_PI_PRIVATE : c_int = FUTEX_WAIT_REQUEUE_PI | FUTEX_PRIVATE_FLAG;
pub const FUTEX_CMP_REQUEUE_PI_PRIVATE  : c_int = FUTEX_CMP_REQUEUE_PI  | FUTEX_PRIVATE_FLAG;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct robust_list {
    pub next: *mut robust_list,
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct robust_list_head {
    pub list: robust_list,
    pub futex_offset: c_long,
    pub list_op_pending: *mut robust_list,
}

pub const FUTEX_WAITERS          : c_uint = 0x80000000;
pub const FUTEX_OWNER_DIED       : c_uint = 0x40000000;
pub const FUTEX_TID_MASK         : c_uint = 0x3fffffff;
pub const ROBUST_LIST_LIMIT      : c_uint = 2048;
pub const FUTEX_BITSET_MATCH_ANY : c_uint = 0xffffffff;

pub const FUTEX_OP_SET         : c_int = 0;
pub const FUTEX_OP_ADD         : c_int = 1;
pub const FUTEX_OP_OR          : c_int = 2;
pub const FUTEX_OP_ANDN        : c_int = 3;
pub const FUTEX_OP_XOR         : c_int = 4;
pub const FUTEX_OP_OPARG_SHIFT : c_int = 8;
pub const FUTEX_OP_CMP_EQ      : c_int = 0;
pub const FUTEX_OP_CMP_NE      : c_int = 1;
pub const FUTEX_OP_CMP_LT      : c_int = 2;
pub const FUTEX_OP_CMP_LE      : c_int = 3;
pub const FUTEX_OP_CMP_GT      : c_int = 4;
pub const FUTEX_OP_CMP_GE      : c_int = 5;

pub fn FUTEX_OP(op: c_int, oparg: c_int, cmp: c_int, cmparg: c_int) -> c_int {
    ((op & 0xf) << 28) | ((cmp & 0xf) << 24) | ((oparg & 0xfff) << 12) | (cmparg & 0xfff)
}

// sched.h

#[repr(C)]
#[derive(Pod, Eq)]
pub struct sched_param {
    pub sched_priority: k_int,
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct sched_attr {
    pub size:           u32,
    pub sched_policy:   u32,
    pub sched_flags:    u64,
    pub sched_nice:     __s32,
    pub sched_priority: u32,
    pub sched_runtime:  u64,
    pub sched_deadline: u64,
    pub sched_period:   u64,
}

// straight from select.c

#[repr(C)]
#[derive(Pod, Eq)]
pub struct sel_arg_struct {
    pub n: k_ulong,
    pub inp: *mut fd_set,
    pub outp: *mut fd_set,
    pub exp: *mut fd_set,
    pub tvp: *mut timeval,
}

// sem.h

pub const SEM_UNDO : c_int = 0x1000;
pub const GETPID   : c_int = 11;
pub const GETVAL   : c_int = 12;
pub const GETALL   : c_int = 13;
pub const GETNCNT  : c_int = 14;
pub const GETZCNT  : c_int = 15;
pub const SETVAL   : c_int = 16;
pub const SETALL   : c_int = 17;
pub const SEM_STAT : c_int = 18;
pub const SEM_INFO : c_int = 19;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct sembuf {
    pub sem_num: c_ushort,
    pub sem_op:  c_short,
    pub sem_flg: c_short,
}

pub const SEMMNI : c_int = 32000;
pub const SEMMSL : c_int = 32000;
pub const SEMMNS : c_int = SEMMNI*SEMMSL;
pub const SEMOPM : c_int = 500;
pub const SEMVMX : c_int = 32767;
pub const SEMAEM : c_int = SEMVMX;
pub const SEMUME : c_int = SEMOPM;
pub const SEMMNU : c_int = SEMMNS;
pub const SEMMAP : c_int = SEMMNS;
pub const SEMUSZ : c_int = 20;

// shm.h

pub const SHMMIN : c_ulong = 1;
pub const SHMMNI : c_ulong = 4096;
pub const SHMMAX : c_ulong = !0 - (1 << 24);
pub const SHMALL : c_ulong = !0 - (1 << 24);
pub const SHMSEG : c_ulong = SHMMNI;

pub const SHM_R      : c_int = 0400;
pub const SHM_W      : c_int = 0200;
pub const SHM_RDONLY : c_int = 010000;
pub const SHM_RND    : c_int = 020000;
pub const SHM_REMAP  : c_int = 040000;
pub const SHM_EXEC   : c_int = 0100000;
pub const SHM_LOCK   : c_int = 11;
pub const SHM_UNLOCK : c_int = 12;
pub const SHM_STAT   : c_int = 13;
pub const SHM_INFO   : c_int = 14;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct shm_info {
    pub used_ids:       c_int,
    pub shm_tot:        __kernel_ulong_t,
    pub shm_rss:        __kernel_ulong_t,
    pub shm_swp:        __kernel_ulong_t,
    pub swap_attempts:  __kernel_ulong_t,
    pub swap_successes: __kernel_ulong_t,
}

// sysctl.h

pub const CTL_MAXNAME : c_int = 10;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct __sysctl_args {
    pub name:        *mut c_int,
    pub nlen:        c_int,
    pub oldval:      *mut c_void,
    pub oldlenp:     *mut size_t,
    pub newval:      *mut c_void,
    pub newlen:      size_t,
    pub __unused: [c_ulong; 4],
}

pub const CTL_KERN                                             : c_int = 1;
pub const CTL_VM                                               : c_int = 2;
pub const CTL_NET                                              : c_int = 3;
pub const CTL_PROC                                             : c_int = 4;
pub const CTL_FS                                               : c_int = 5;
pub const CTL_DEBUG                                            : c_int = 6;
pub const CTL_DEV                                              : c_int = 7;
pub const CTL_BUS                                              : c_int = 8;
pub const CTL_ABI                                              : c_int = 9;
pub const CTL_CPU                                              : c_int = 10;
pub const CTL_ARLAN                                            : c_int = 254;
pub const CTL_S390DBF                                          : c_int = 5677;
pub const CTL_SUNRPC                                           : c_int = 7249;
pub const CTL_PM                                               : c_int = 9899;
pub const CTL_FRV                                              : c_int = 9898;

pub const CTL_BUS_ISA                                          : c_int = 1;

pub const INOTIFY_MAX_USER_INSTANCES                           : c_int = 1;
pub const INOTIFY_MAX_USER_WATCHES                             : c_int = 2;
pub const INOTIFY_MAX_QUEUED_EVENTS                            : c_int = 3;

pub const KERN_OSTYPE                                          : c_int = 1;
pub const KERN_OSRELEASE                                       : c_int = 2;
pub const KERN_OSREV                                           : c_int = 3;
pub const KERN_VERSION                                         : c_int = 4;
pub const KERN_SECUREMASK                                      : c_int = 5;
pub const KERN_PROF                                            : c_int = 6;
pub const KERN_NODENAME                                        : c_int = 7;
pub const KERN_DOMAINNAME                                      : c_int = 8;

pub const KERN_PANIC                                           : c_int = 15;
pub const KERN_REALROOTDEV                                     : c_int = 16;

pub const KERN_SPARC_REBOOT                                    : c_int = 21;
pub const KERN_CTLALTDEL                                       : c_int = 22;
pub const KERN_PRINTK                                          : c_int = 23;
pub const KERN_NAMETRANS                                       : c_int = 24;
pub const KERN_PPC_HTABRECLAIM                                 : c_int = 25;
pub const KERN_PPC_ZEROPAGED                                   : c_int = 26;
pub const KERN_PPC_POWERSAVE_NAP                               : c_int = 27;
pub const KERN_MODPROBE                                        : c_int = 28;
pub const KERN_SG_BIG_BUFF                                     : c_int = 29;
pub const KERN_ACCT                                            : c_int = 30;
pub const KERN_PPC_L2CR                                        : c_int = 31;

pub const KERN_RTSIGNR                                         : c_int = 32;
pub const KERN_RTSIGMAX                                        : c_int = 33;

pub const KERN_SHMMAX                                          : c_int = 34;
pub const KERN_MSGMAX                                          : c_int = 35;
pub const KERN_MSGMNB                                          : c_int = 36;
pub const KERN_MSGPOOL                                         : c_int = 37;
pub const KERN_SYSRQ                                           : c_int = 38;
pub const KERN_MAX_THREADS                                     : c_int = 39;
pub const KERN_RANDOM                                          : c_int = 40;
pub const KERN_SHMALL                                          : c_int = 41;
pub const KERN_MSGMNI                                          : c_int = 42;
pub const KERN_SEM                                             : c_int = 43;
pub const KERN_SPARC_STOP_A                                    : c_int = 44;
pub const KERN_SHMMNI                                          : c_int = 45;
pub const KERN_OVERFLOWUID                                     : c_int = 46;
pub const KERN_OVERFLOWGID                                     : c_int = 47;
pub const KERN_SHMPATH                                         : c_int = 48;
pub const KERN_HOTPLUG                                         : c_int = 49;
pub const KERN_IEEE_EMULATION_WARNINGS                         : c_int = 50;
pub const KERN_S390_USER_DEBUG_LOGGING                         : c_int = 51;
pub const KERN_CORE_USES_PID                                   : c_int = 52;
pub const KERN_TAINTED                                         : c_int = 53;
pub const KERN_CADPID                                          : c_int = 54;
pub const KERN_PIDMAX                                          : c_int = 55;
pub const KERN_CORE_PATTERN                                    : c_int = 56;
pub const KERN_PANIC_ON_OOPS                                   : c_int = 57;
pub const KERN_HPPA_PWRSW                                      : c_int = 58;
pub const KERN_HPPA_UNALIGNED                                  : c_int = 59;
pub const KERN_PRINTK_RATELIMIT                                : c_int = 60;
pub const KERN_PRINTK_RATELIMIT_BURST                          : c_int = 61;
pub const KERN_PTY                                             : c_int = 62;
pub const KERN_NGROUPS_MAX                                     : c_int = 63;
pub const KERN_SPARC_SCONS_PWROFF                              : c_int = 64;
pub const KERN_HZ_TIMER                                        : c_int = 65;
pub const KERN_UNKNOWN_NMI_PANIC                               : c_int = 66;
pub const KERN_BOOTLOADER_TYPE                                 : c_int = 67;
pub const KERN_RANDOMIZE                                       : c_int = 68;
pub const KERN_SETUID_DUMPABLE                                 : c_int = 69;
pub const KERN_SPIN_RETRY                                      : c_int = 70;
pub const KERN_ACPI_VIDEO_FLAGS                                : c_int = 71;
pub const KERN_IA64_UNALIGNED                                  : c_int = 72;
pub const KERN_COMPAT_LOG                                      : c_int = 73;
pub const KERN_MAX_LOCK_DEPTH                                  : c_int = 74;
pub const KERN_NMI_WATCHDOG                                    : c_int = 75;
pub const KERN_PANIC_ON_NMI                                    : c_int = 76;
pub const KERN_PANIC_ON_WARN                                   : c_int = 77;

pub const VM_UNUSED1                                           : c_int = 1;
pub const VM_UNUSED2                                           : c_int = 2;
pub const VM_UNUSED3                                           : c_int = 3;
pub const VM_UNUSED4                                           : c_int = 4;
pub const VM_OVERCOMMIT_MEMORY                                 : c_int = 5;
pub const VM_UNUSED5                                           : c_int = 6;
pub const VM_UNUSED7                                           : c_int = 7;
pub const VM_UNUSED8                                           : c_int = 8;
pub const VM_UNUSED9                                           : c_int = 9;
pub const VM_PAGE_CLUSTER                                      : c_int = 10;
pub const VM_DIRTY_BACKGROUND                                  : c_int = 11;
pub const VM_DIRTY_RATIO                                       : c_int = 12;
pub const VM_DIRTY_WB_CS                                       : c_int = 13;
pub const VM_DIRTY_EXPIRE_CS                                   : c_int = 14;
pub const VM_NR_PDFLUSH_THREADS                                : c_int = 15;
pub const VM_OVERCOMMIT_RATIO                                  : c_int = 16;
pub const VM_PAGEBUF                                           : c_int = 17;
pub const VM_HUGETLB_PAGES                                     : c_int = 18;
pub const VM_SWAPPINESS                                        : c_int = 19;
pub const VM_LOWMEM_RESERVE_RATIO                              : c_int = 20;
pub const VM_MIN_FREE_KBYTES                                   : c_int = 21;
pub const VM_MAX_MAP_COUNT                                     : c_int = 22;
pub const VM_LAPTOP_MODE                                       : c_int = 23;
pub const VM_BLOCK_DUMP                                        : c_int = 24;
pub const VM_HUGETLB_GROUP                                     : c_int = 25;
pub const VM_VFS_CACHE_PRESSURE                                : c_int = 26;
pub const VM_LEGACY_VA_LAYOUT                                  : c_int = 27;
pub const VM_SWAP_TOKEN_TIMEOUT                                : c_int = 28;
pub const VM_DROP_PAGECACHE                                    : c_int = 29;
pub const VM_PERCPU_PAGELIST_FRACTION                          : c_int = 30;
pub const VM_ZONE_RECLAIM_MODE                                 : c_int = 31;
pub const VM_MIN_UNMAPPED                                      : c_int = 32;
pub const VM_PANIC_ON_OOM                                      : c_int = 33;
pub const VM_VDSO_ENABLED                                      : c_int = 34;
pub const VM_MIN_SLAB                                          : c_int = 35;

pub const NET_CORE                                             : c_int = 1;
pub const NET_ETHER                                            : c_int = 2;
pub const NET_802                                              : c_int = 3;
pub const NET_UNIX                                             : c_int = 4;
pub const NET_IPV4                                             : c_int = 5;
pub const NET_IPX                                              : c_int = 6;
pub const NET_ATALK                                            : c_int = 7;
pub const NET_NETROM                                           : c_int = 8;
pub const NET_AX25                                             : c_int = 9;
pub const NET_BRIDGE                                           : c_int = 10;
pub const NET_ROSE                                             : c_int = 11;
pub const NET_IPV6                                             : c_int = 12;
pub const NET_X25                                              : c_int = 13;
pub const NET_TR                                               : c_int = 14;
pub const NET_DECNET                                           : c_int = 15;
pub const NET_ECONET                                           : c_int = 16;
pub const NET_SCTP                                             : c_int = 17;
pub const NET_LLC                                              : c_int = 18;
pub const NET_NETFILTER                                        : c_int = 19;
pub const NET_DCCP                                             : c_int = 20;
pub const NET_IRDA                                             : c_int = 412;

pub const RANDOM_POOLSIZE                                      : c_int = 1;
pub const RANDOM_ENTROPY_COUNT                                 : c_int = 2;
pub const RANDOM_READ_THRESH                                   : c_int = 3;
pub const RANDOM_WRITE_THRESH                                  : c_int = 4;
pub const RANDOM_BOOT_ID                                       : c_int = 5;
pub const RANDOM_UUID                                          : c_int = 6;

pub const PTY_MAX                                              : c_int = 1;
pub const PTY_NR                                               : c_int = 2;

pub const BUS_ISA_MEM_BASE                                     : c_int = 1;
pub const BUS_ISA_PORT_BASE                                    : c_int = 2;
pub const BUS_ISA_PORT_SHIFT                                   : c_int = 3;

pub const NET_CORE_WMEM_MAX                                    : c_int = 1;
pub const NET_CORE_RMEM_MAX                                    : c_int = 2;
pub const NET_CORE_WMEM_DEFAULT                                : c_int = 3;
pub const NET_CORE_RMEM_DEFAULT                                : c_int = 4;
pub const NET_CORE_MAX_BACKLOG                                 : c_int = 6;
pub const NET_CORE_FASTROUTE                                   : c_int = 7;
pub const NET_CORE_MSG_COST                                    : c_int = 8;
pub const NET_CORE_MSG_BURST                                   : c_int = 9;
pub const NET_CORE_OPTMEM_MAX                                  : c_int = 10;
pub const NET_CORE_HOT_LIST_LENGTH                             : c_int = 11;
pub const NET_CORE_DIVERT_VERSION                              : c_int = 12;
pub const NET_CORE_NO_CONG_THRESH                              : c_int = 13;
pub const NET_CORE_NO_CONG                                     : c_int = 14;
pub const NET_CORE_LO_CONG                                     : c_int = 15;
pub const NET_CORE_MOD_CONG                                    : c_int = 16;
pub const NET_CORE_DEV_WEIGHT                                  : c_int = 17;
pub const NET_CORE_SOMAXCONN                                   : c_int = 18;
pub const NET_CORE_BUDGET                                      : c_int = 19;
pub const NET_CORE_AEVENT_ETIME                                : c_int = 20;
pub const NET_CORE_AEVENT_RSEQTH                               : c_int = 21;
pub const NET_CORE_WARNINGS                                    : c_int = 22;

pub const NET_UNIX_DESTROY_DELAY                               : c_int = 1;
pub const NET_UNIX_DELETE_DELAY                                : c_int = 2;
pub const NET_UNIX_MAX_DGRAM_QLEN                              : c_int = 3;

pub const NET_NF_CONNTRACK_MAX                                 : c_int = 1;
pub const NET_NF_CONNTRACK_TCP_TIMEOUT_SYN_SENT                : c_int = 2;
pub const NET_NF_CONNTRACK_TCP_TIMEOUT_SYN_RECV                : c_int = 3;
pub const NET_NF_CONNTRACK_TCP_TIMEOUT_ESTABLISHED             : c_int = 4;
pub const NET_NF_CONNTRACK_TCP_TIMEOUT_FIN_WAIT                : c_int = 5;
pub const NET_NF_CONNTRACK_TCP_TIMEOUT_CLOSE_WAIT              : c_int = 6;
pub const NET_NF_CONNTRACK_TCP_TIMEOUT_LAST_ACK                : c_int = 7;
pub const NET_NF_CONNTRACK_TCP_TIMEOUT_TIME_WAIT               : c_int = 8;
pub const NET_NF_CONNTRACK_TCP_TIMEOUT_CLOSE                   : c_int = 9;
pub const NET_NF_CONNTRACK_UDP_TIMEOUT                         : c_int = 10;
pub const NET_NF_CONNTRACK_UDP_TIMEOUT_STREAM                  : c_int = 11;
pub const NET_NF_CONNTRACK_ICMP_TIMEOUT                        : c_int = 12;
pub const NET_NF_CONNTRACK_GENERIC_TIMEOUT                     : c_int = 13;
pub const NET_NF_CONNTRACK_BUCKETS                             : c_int = 14;
pub const NET_NF_CONNTRACK_LOG_INVALID                         : c_int = 15;
pub const NET_NF_CONNTRACK_TCP_TIMEOUT_MAX_RETRANS             : c_int = 16;
pub const NET_NF_CONNTRACK_TCP_LOOSE                           : c_int = 17;
pub const NET_NF_CONNTRACK_TCP_BE_LIBERAL                      : c_int = 18;
pub const NET_NF_CONNTRACK_TCP_MAX_RETRANS                     : c_int = 19;
pub const NET_NF_CONNTRACK_SCTP_TIMEOUT_CLOSED                 : c_int = 20;
pub const NET_NF_CONNTRACK_SCTP_TIMEOUT_COOKIE_WAIT            : c_int = 21;
pub const NET_NF_CONNTRACK_SCTP_TIMEOUT_COOKIE_ECHOED          : c_int = 22;
pub const NET_NF_CONNTRACK_SCTP_TIMEOUT_ESTABLISHED            : c_int = 23;
pub const NET_NF_CONNTRACK_SCTP_TIMEOUT_SHUTDOWN_SENT          : c_int = 24;
pub const NET_NF_CONNTRACK_SCTP_TIMEOUT_SHUTDOWN_RECD          : c_int = 25;
pub const NET_NF_CONNTRACK_SCTP_TIMEOUT_SHUTDOWN_ACK_SENT      : c_int = 26;
pub const NET_NF_CONNTRACK_COUNT                               : c_int = 27;
pub const NET_NF_CONNTRACK_ICMPV6_TIMEOUT                      : c_int = 28;
pub const NET_NF_CONNTRACK_FRAG6_TIMEOUT                       : c_int = 29;
pub const NET_NF_CONNTRACK_FRAG6_LOW_THRESH                    : c_int = 30;
pub const NET_NF_CONNTRACK_FRAG6_HIGH_THRESH                   : c_int = 31;
pub const NET_NF_CONNTRACK_CHECKSUM                            : c_int = 32;

pub const NET_IPV4_FORWARD                                     : c_int = 8;
pub const NET_IPV4_DYNADDR                                     : c_int = 9;

pub const NET_IPV4_CONF                                        : c_int = 16;
pub const NET_IPV4_NEIGH                                       : c_int = 17;
pub const NET_IPV4_ROUTE                                       : c_int = 18;
pub const NET_IPV4_FIB_HASH                                    : c_int = 19;
pub const NET_IPV4_NETFILTER                                   : c_int = 20;

pub const NET_IPV4_TCP_TIMESTAMPS                              : c_int = 33;
pub const NET_IPV4_TCP_WINDOW_SCALING                          : c_int = 34;
pub const NET_IPV4_TCP_SACK                                    : c_int = 35;
pub const NET_IPV4_TCP_RETRANS_COLLAPSE                        : c_int = 36;
pub const NET_IPV4_DEFAULT_TTL                                 : c_int = 37;
pub const NET_IPV4_AUTOCONFIG                                  : c_int = 38;
pub const NET_IPV4_NO_PMTU_DISC                                : c_int = 39;
pub const NET_IPV4_TCP_SYN_RETRIES                             : c_int = 40;
pub const NET_IPV4_IPFRAG_HIGH_THRESH                          : c_int = 41;
pub const NET_IPV4_IPFRAG_LOW_THRESH                           : c_int = 42;
pub const NET_IPV4_IPFRAG_TIME                                 : c_int = 43;
pub const NET_IPV4_TCP_MAX_KA_PROBES                           : c_int = 44;
pub const NET_IPV4_TCP_KEEPALIVE_TIME                          : c_int = 45;
pub const NET_IPV4_TCP_KEEPALIVE_PROBES                        : c_int = 46;
pub const NET_IPV4_TCP_RETRIES1                                : c_int = 47;
pub const NET_IPV4_TCP_RETRIES2                                : c_int = 48;
pub const NET_IPV4_TCP_FIN_TIMEOUT                             : c_int = 49;
pub const NET_IPV4_IP_MASQ_DEBUG                               : c_int = 50;
pub const NET_TCP_SYNCOOKIES                                   : c_int = 51;
pub const NET_TCP_STDURG                                       : c_int = 52;
pub const NET_TCP_RFC1337                                      : c_int = 53;
pub const NET_TCP_SYN_TAILDROP                                 : c_int = 54;
pub const NET_TCP_MAX_SYN_BACKLOG                              : c_int = 55;
pub const NET_IPV4_LOCAL_PORT_RANGE                            : c_int = 56;
pub const NET_IPV4_ICMP_ECHO_IGNORE_ALL                        : c_int = 57;
pub const NET_IPV4_ICMP_ECHO_IGNORE_BROADCASTS                 : c_int = 58;
pub const NET_IPV4_ICMP_SOURCEQUENCH_RATE                      : c_int = 59;
pub const NET_IPV4_ICMP_DESTUNREACH_RATE                       : c_int = 60;
pub const NET_IPV4_ICMP_TIMEEXCEED_RATE                        : c_int = 61;
pub const NET_IPV4_ICMP_PARAMPROB_RATE                         : c_int = 62;
pub const NET_IPV4_ICMP_ECHOREPLY_RATE                         : c_int = 63;
pub const NET_IPV4_ICMP_IGNORE_BOGUS_ERROR_RESPONSES           : c_int = 64;
pub const NET_IPV4_IGMP_MAX_MEMBERSHIPS                        : c_int = 65;
pub const NET_TCP_TW_RECYCLE                                   : c_int = 66;
pub const NET_IPV4_ALWAYS_DEFRAG                               : c_int = 67;
pub const NET_IPV4_TCP_KEEPALIVE_INTVL                         : c_int = 68;
pub const NET_IPV4_INET_PEER_THRESHOLD                         : c_int = 69;
pub const NET_IPV4_INET_PEER_MINTTL                            : c_int = 70;
pub const NET_IPV4_INET_PEER_MAXTTL                            : c_int = 71;
pub const NET_IPV4_INET_PEER_GC_MINTIME                        : c_int = 72;
pub const NET_IPV4_INET_PEER_GC_MAXTIME                        : c_int = 73;
pub const NET_TCP_ORPHAN_RETRIES                               : c_int = 74;
pub const NET_TCP_ABORT_ON_OVERFLOW                            : c_int = 75;
pub const NET_TCP_SYNACK_RETRIES                               : c_int = 76;
pub const NET_TCP_MAX_ORPHANS                                  : c_int = 77;
pub const NET_TCP_MAX_TW_BUCKETS                               : c_int = 78;
pub const NET_TCP_FACK                                         : c_int = 79;
pub const NET_TCP_REORDERING                                   : c_int = 80;
pub const NET_TCP_ECN                                          : c_int = 81;
pub const NET_TCP_DSACK                                        : c_int = 82;
pub const NET_TCP_MEM                                          : c_int = 83;
pub const NET_TCP_WMEM                                         : c_int = 84;
pub const NET_TCP_RMEM                                         : c_int = 85;
pub const NET_TCP_APP_WIN                                      : c_int = 86;
pub const NET_TCP_ADV_WIN_SCALE                                : c_int = 87;
pub const NET_IPV4_NONLOCAL_BIND                               : c_int = 88;
pub const NET_IPV4_ICMP_RATELIMIT                              : c_int = 89;
pub const NET_IPV4_ICMP_RATEMASK                               : c_int = 90;
pub const NET_TCP_TW_REUSE                                     : c_int = 91;
pub const NET_TCP_FRTO                                         : c_int = 92;
pub const NET_TCP_LOW_LATENCY                                  : c_int = 93;
pub const NET_IPV4_IPFRAG_SECRET_INTERVAL                      : c_int = 94;
pub const NET_IPV4_IGMP_MAX_MSF                                : c_int = 96;
pub const NET_TCP_NO_METRICS_SAVE                              : c_int = 97;
pub const NET_TCP_DEFAULT_WIN_SCALE                            : c_int = 105;
pub const NET_TCP_MODERATE_RCVBUF                              : c_int = 106;
pub const NET_TCP_TSO_WIN_DIVISOR                              : c_int = 107;
pub const NET_TCP_BIC_BETA                                     : c_int = 108;
pub const NET_IPV4_ICMP_ERRORS_USE_INBOUND_IFADDR              : c_int = 109;
pub const NET_TCP_CONG_CONTROL                                 : c_int = 110;
pub const NET_TCP_ABC                                          : c_int = 111;
pub const NET_IPV4_IPFRAG_MAX_DIST                             : c_int = 112;
pub const NET_TCP_MTU_PROBING                                  : c_int = 113;
pub const NET_TCP_BASE_MSS                                     : c_int = 114;
pub const NET_IPV4_TCP_WORKAROUND_SIGNED_WINDOWS               : c_int = 115;
pub const NET_TCP_DMA_COPYBREAK                                : c_int = 116;
pub const NET_TCP_SLOW_START_AFTER_IDLE                        : c_int = 117;
pub const NET_CIPSOV4_CACHE_ENABLE                             : c_int = 118;
pub const NET_CIPSOV4_CACHE_BUCKET_SIZE                        : c_int = 119;
pub const NET_CIPSOV4_RBM_OPTFMT                               : c_int = 120;
pub const NET_CIPSOV4_RBM_STRICTVALID                          : c_int = 121;
pub const NET_TCP_AVAIL_CONG_CONTROL                           : c_int = 122;
pub const NET_TCP_ALLOWED_CONG_CONTROL                         : c_int = 123;
pub const NET_TCP_MAX_SSTHRESH                                 : c_int = 124;
pub const NET_TCP_FRTO_RESPONSE                                : c_int = 125;

pub const NET_IPV4_ROUTE_FLUSH                                 : c_int = 1;
pub const NET_IPV4_ROUTE_MIN_DELAY                             : c_int = 2;
pub const NET_IPV4_ROUTE_MAX_DELAY                             : c_int = 3;
pub const NET_IPV4_ROUTE_GC_THRESH                             : c_int = 4;
pub const NET_IPV4_ROUTE_MAX_SIZE                              : c_int = 5;
pub const NET_IPV4_ROUTE_GC_MIN_INTERVAL                       : c_int = 6;
pub const NET_IPV4_ROUTE_GC_TIMEOUT                            : c_int = 7;
pub const NET_IPV4_ROUTE_GC_INTERVAL                           : c_int = 8;
pub const NET_IPV4_ROUTE_REDIRECT_LOAD                         : c_int = 9;
pub const NET_IPV4_ROUTE_REDIRECT_NUMBER                       : c_int = 10;
pub const NET_IPV4_ROUTE_REDIRECT_SILENCE                      : c_int = 11;
pub const NET_IPV4_ROUTE_ERROR_COST                            : c_int = 12;
pub const NET_IPV4_ROUTE_ERROR_BURST                           : c_int = 13;
pub const NET_IPV4_ROUTE_GC_ELASTICITY                         : c_int = 14;
pub const NET_IPV4_ROUTE_MTU_EXPIRES                           : c_int = 15;
pub const NET_IPV4_ROUTE_MIN_PMTU                              : c_int = 16;
pub const NET_IPV4_ROUTE_MIN_ADVMSS                            : c_int = 17;
pub const NET_IPV4_ROUTE_SECRET_INTERVAL                       : c_int = 18;
pub const NET_IPV4_ROUTE_GC_MIN_INTERVAL_MS                    : c_int = 19;

pub const NET_PROTO_CONF_ALL                                   : c_int = -2;
pub const NET_PROTO_CONF_DEFAULT                               : c_int = -3;

pub const NET_IPV4_CONF_FORWARDING                             : c_int = 1;
pub const NET_IPV4_CONF_MC_FORWARDING                          : c_int = 2;
pub const NET_IPV4_CONF_PROXY_ARP                              : c_int = 3;
pub const NET_IPV4_CONF_ACCEPT_REDIRECTS                       : c_int = 4;
pub const NET_IPV4_CONF_SECURE_REDIRECTS                       : c_int = 5;
pub const NET_IPV4_CONF_SEND_REDIRECTS                         : c_int = 6;
pub const NET_IPV4_CONF_SHARED_MEDIA                           : c_int = 7;
pub const NET_IPV4_CONF_RP_FILTER                              : c_int = 8;
pub const NET_IPV4_CONF_ACCEPT_SOURCE_ROUTE                    : c_int = 9;
pub const NET_IPV4_CONF_BOOTP_RELAY                            : c_int = 10;
pub const NET_IPV4_CONF_LOG_MARTIANS                           : c_int = 11;
pub const NET_IPV4_CONF_TAG                                    : c_int = 12;
pub const NET_IPV4_CONF_ARPFILTER                              : c_int = 13;
pub const NET_IPV4_CONF_MEDIUM_ID                              : c_int = 14;
pub const NET_IPV4_CONF_NOXFRM                                 : c_int = 15;
pub const NET_IPV4_CONF_NOPOLICY                               : c_int = 16;
pub const NET_IPV4_CONF_FORCE_IGMP_VERSION                     : c_int = 17;
pub const NET_IPV4_CONF_ARP_ANNOUNCE                           : c_int = 18;
pub const NET_IPV4_CONF_ARP_IGNORE                             : c_int = 19;
pub const NET_IPV4_CONF_PROMOTE_SECONDARIES                    : c_int = 20;
pub const NET_IPV4_CONF_ARP_ACCEPT                             : c_int = 21;
pub const NET_IPV4_CONF_ARP_NOTIFY                             : c_int = 22;

pub const NET_IPV4_NF_CONNTRACK_MAX                            : c_int = 1;
pub const NET_IPV4_NF_CONNTRACK_TCP_TIMEOUT_SYN_SENT           : c_int = 2;
pub const NET_IPV4_NF_CONNTRACK_TCP_TIMEOUT_SYN_RECV           : c_int = 3;
pub const NET_IPV4_NF_CONNTRACK_TCP_TIMEOUT_ESTABLISHED        : c_int = 4;
pub const NET_IPV4_NF_CONNTRACK_TCP_TIMEOUT_FIN_WAIT           : c_int = 5;
pub const NET_IPV4_NF_CONNTRACK_TCP_TIMEOUT_CLOSE_WAIT         : c_int = 6;
pub const NET_IPV4_NF_CONNTRACK_TCP_TIMEOUT_LAST_ACK           : c_int = 7;
pub const NET_IPV4_NF_CONNTRACK_TCP_TIMEOUT_TIME_WAIT          : c_int = 8;
pub const NET_IPV4_NF_CONNTRACK_TCP_TIMEOUT_CLOSE              : c_int = 9;
pub const NET_IPV4_NF_CONNTRACK_UDP_TIMEOUT                    : c_int = 10;
pub const NET_IPV4_NF_CONNTRACK_UDP_TIMEOUT_STREAM             : c_int = 11;
pub const NET_IPV4_NF_CONNTRACK_ICMP_TIMEOUT                   : c_int = 12;
pub const NET_IPV4_NF_CONNTRACK_GENERIC_TIMEOUT                : c_int = 13;
pub const NET_IPV4_NF_CONNTRACK_BUCKETS                        : c_int = 14;
pub const NET_IPV4_NF_CONNTRACK_LOG_INVALID                    : c_int = 15;
pub const NET_IPV4_NF_CONNTRACK_TCP_TIMEOUT_MAX_RETRANS        : c_int = 16;
pub const NET_IPV4_NF_CONNTRACK_TCP_LOOSE                      : c_int = 17;
pub const NET_IPV4_NF_CONNTRACK_TCP_BE_LIBERAL                 : c_int = 18;
pub const NET_IPV4_NF_CONNTRACK_TCP_MAX_RETRANS                : c_int = 19;
pub const NET_IPV4_NF_CONNTRACK_SCTP_TIMEOUT_CLOSED            : c_int = 20;
pub const NET_IPV4_NF_CONNTRACK_SCTP_TIMEOUT_COOKIE_WAIT       : c_int = 21;
pub const NET_IPV4_NF_CONNTRACK_SCTP_TIMEOUT_COOKIE_ECHOED     : c_int = 22;
pub const NET_IPV4_NF_CONNTRACK_SCTP_TIMEOUT_ESTABLISHED       : c_int = 23;
pub const NET_IPV4_NF_CONNTRACK_SCTP_TIMEOUT_SHUTDOWN_SENT     : c_int = 24;
pub const NET_IPV4_NF_CONNTRACK_SCTP_TIMEOUT_SHUTDOWN_RECD     : c_int = 25;
pub const NET_IPV4_NF_CONNTRACK_SCTP_TIMEOUT_SHUTDOWN_ACK_SENT : c_int = 26;
pub const NET_IPV4_NF_CONNTRACK_COUNT                          : c_int = 27;
pub const NET_IPV4_NF_CONNTRACK_CHECKSUM                       : c_int = 28;

pub const NET_IPV6_CONF                                        : c_int = 16;
pub const NET_IPV6_NEIGH                                       : c_int = 17;
pub const NET_IPV6_ROUTE                                       : c_int = 18;
pub const NET_IPV6_ICMP                                        : c_int = 19;
pub const NET_IPV6_BINDV6ONLY                                  : c_int = 20;
pub const NET_IPV6_IP6FRAG_HIGH_THRESH                         : c_int = 21;
pub const NET_IPV6_IP6FRAG_LOW_THRESH                          : c_int = 22;
pub const NET_IPV6_IP6FRAG_TIME                                : c_int = 23;
pub const NET_IPV6_IP6FRAG_SECRET_INTERVAL                     : c_int = 24;
pub const NET_IPV6_MLD_MAX_MSF                                 : c_int = 25;

pub const NET_IPV6_ROUTE_FLUSH                                 : c_int = 1;
pub const NET_IPV6_ROUTE_GC_THRESH                             : c_int = 2;
pub const NET_IPV6_ROUTE_MAX_SIZE                              : c_int = 3;
pub const NET_IPV6_ROUTE_GC_MIN_INTERVAL                       : c_int = 4;
pub const NET_IPV6_ROUTE_GC_TIMEOUT                            : c_int = 5;
pub const NET_IPV6_ROUTE_GC_INTERVAL                           : c_int = 6;
pub const NET_IPV6_ROUTE_GC_ELASTICITY                         : c_int = 7;
pub const NET_IPV6_ROUTE_MTU_EXPIRES                           : c_int = 8;
pub const NET_IPV6_ROUTE_MIN_ADVMSS                            : c_int = 9;
pub const NET_IPV6_ROUTE_GC_MIN_INTERVAL_MS                    : c_int = 10;

pub const NET_IPV6_FORWARDING                                  : c_int = 1;
pub const NET_IPV6_HOP_LIMIT                                   : c_int = 2;
pub const NET_IPV6_MTU                                         : c_int = 3;
pub const NET_IPV6_ACCEPT_RA                                   : c_int = 4;
pub const NET_IPV6_ACCEPT_REDIRECTS                            : c_int = 5;
pub const NET_IPV6_AUTOCONF                                    : c_int = 6;
pub const NET_IPV6_DAD_TRANSMITS                               : c_int = 7;
pub const NET_IPV6_RTR_SOLICITS                                : c_int = 8;
pub const NET_IPV6_RTR_SOLICIT_INTERVAL                        : c_int = 9;
pub const NET_IPV6_RTR_SOLICIT_DELAY                           : c_int = 10;
pub const NET_IPV6_USE_TEMPADDR                                : c_int = 11;
pub const NET_IPV6_TEMP_VALID_LFT                              : c_int = 12;
pub const NET_IPV6_TEMP_PREFERED_LFT                           : c_int = 13;
pub const NET_IPV6_REGEN_MAX_RETRY                             : c_int = 14;
pub const NET_IPV6_MAX_DESYNC_FACTOR                           : c_int = 15;
pub const NET_IPV6_MAX_ADDRESSES                               : c_int = 16;
pub const NET_IPV6_FORCE_MLD_VERSION                           : c_int = 17;
pub const NET_IPV6_ACCEPT_RA_DEFRTR                            : c_int = 18;
pub const NET_IPV6_ACCEPT_RA_PINFO                             : c_int = 19;
pub const NET_IPV6_ACCEPT_RA_RTR_PREF                          : c_int = 20;
pub const NET_IPV6_RTR_PROBE_INTERVAL                          : c_int = 21;
pub const NET_IPV6_ACCEPT_RA_RT_INFO_MAX_PLEN                  : c_int = 22;
pub const NET_IPV6_PROXY_NDP                                   : c_int = 23;
pub const NET_IPV6_ACCEPT_SOURCE_ROUTE                         : c_int = 25;
pub const NET_IPV6_ACCEPT_RA_FROM_LOCAL                        : c_int = 26;
pub const __NET_IPV6_MAX                                       : c_int = 27;

pub const NET_IPV6_ICMP_RATELIMIT                              : c_int = 1;

pub const NET_NEIGH_MCAST_SOLICIT                              : c_int = 1;
pub const NET_NEIGH_UCAST_SOLICIT                              : c_int = 2;
pub const NET_NEIGH_APP_SOLICIT                                : c_int = 3;
pub const NET_NEIGH_RETRANS_TIME                               : c_int = 4;
pub const NET_NEIGH_REACHABLE_TIME                             : c_int = 5;
pub const NET_NEIGH_DELAY_PROBE_TIME                           : c_int = 6;
pub const NET_NEIGH_GC_STALE_TIME                              : c_int = 7;
pub const NET_NEIGH_UNRES_QLEN                                 : c_int = 8;
pub const NET_NEIGH_PROXY_QLEN                                 : c_int = 9;
pub const NET_NEIGH_ANYCAST_DELAY                              : c_int = 10;
pub const NET_NEIGH_PROXY_DELAY                                : c_int = 11;
pub const NET_NEIGH_LOCKTIME                                   : c_int = 12;
pub const NET_NEIGH_GC_INTERVAL                                : c_int = 13;
pub const NET_NEIGH_GC_THRESH1                                 : c_int = 14;
pub const NET_NEIGH_GC_THRESH2                                 : c_int = 15;
pub const NET_NEIGH_GC_THRESH3                                 : c_int = 16;
pub const NET_NEIGH_RETRANS_TIME_MS                            : c_int = 17;
pub const NET_NEIGH_REACHABLE_TIME_MS                          : c_int = 18;

pub const NET_DCCP_DEFAULT                                     : c_int = 1;

pub const NET_IPX_PPROP_BROADCASTING                           : c_int = 1;
pub const NET_IPX_FORWARDING                                   : c_int = 2;

pub const NET_LLC2                                             : c_int = 1;
pub const NET_LLC_STATION                                      : c_int = 2;

pub const NET_LLC2_TIMEOUT                                     : c_int = 1;

pub const NET_LLC_STATION_ACK_TIMEOUT                          : c_int = 1;

pub const NET_LLC2_ACK_TIMEOUT                                 : c_int = 1;
pub const NET_LLC2_P_TIMEOUT                                   : c_int = 2;
pub const NET_LLC2_REJ_TIMEOUT                                 : c_int = 3;
pub const NET_LLC2_BUSY_TIMEOUT                                : c_int = 4;

pub const NET_ATALK_AARP_EXPIRY_TIME                           : c_int = 1;
pub const NET_ATALK_AARP_TICK_TIME                             : c_int = 2;
pub const NET_ATALK_AARP_RETRANSMIT_LIMIT                      : c_int = 3;
pub const NET_ATALK_AARP_RESOLVE_TIME                          : c_int = 4;

pub const NET_NETROM_DEFAULT_PATH_QUALITY                      : c_int = 1;
pub const NET_NETROM_OBSOLESCENCE_COUNT_INITIALISER            : c_int = 2;
pub const NET_NETROM_NETWORK_TTL_INITIALISER                   : c_int = 3;
pub const NET_NETROM_TRANSPORT_TIMEOUT                         : c_int = 4;
pub const NET_NETROM_TRANSPORT_MAXIMUM_TRIES                   : c_int = 5;
pub const NET_NETROM_TRANSPORT_ACKNOWLEDGE_DELAY               : c_int = 6;
pub const NET_NETROM_TRANSPORT_BUSY_DELAY                      : c_int = 7;
pub const NET_NETROM_TRANSPORT_REQUESTED_WINDOW_SIZE           : c_int = 8;
pub const NET_NETROM_TRANSPORT_NO_ACTIVITY_TIMEOUT             : c_int = 9;
pub const NET_NETROM_ROUTING_CONTROL                           : c_int = 10;
pub const NET_NETROM_LINK_FAILS_COUNT                          : c_int = 11;
pub const NET_NETROM_RESET                                     : c_int = 12;

pub const NET_AX25_IP_DEFAULT_MODE                             : c_int = 1;
pub const NET_AX25_DEFAULT_MODE                                : c_int = 2;
pub const NET_AX25_BACKOFF_TYPE                                : c_int = 3;
pub const NET_AX25_CONNECT_MODE                                : c_int = 4;
pub const NET_AX25_STANDARD_WINDOW                             : c_int = 5;
pub const NET_AX25_EXTENDED_WINDOW                             : c_int = 6;
pub const NET_AX25_T1_TIMEOUT                                  : c_int = 7;
pub const NET_AX25_T2_TIMEOUT                                  : c_int = 8;
pub const NET_AX25_T3_TIMEOUT                                  : c_int = 9;
pub const NET_AX25_IDLE_TIMEOUT                                : c_int = 10;
pub const NET_AX25_N2                                          : c_int = 11;
pub const NET_AX25_PACLEN                                      : c_int = 12;
pub const NET_AX25_PROTOCOL                                    : c_int = 13;
pub const NET_AX25_DAMA_SLAVE_TIMEOUT                          : c_int = 14;

pub const NET_ROSE_RESTART_REQUEST_TIMEOUT                     : c_int = 1;
pub const NET_ROSE_CALL_REQUEST_TIMEOUT                        : c_int = 2;
pub const NET_ROSE_RESET_REQUEST_TIMEOUT                       : c_int = 3;
pub const NET_ROSE_CLEAR_REQUEST_TIMEOUT                       : c_int = 4;
pub const NET_ROSE_ACK_HOLD_BACK_TIMEOUT                       : c_int = 5;
pub const NET_ROSE_ROUTING_CONTROL                             : c_int = 6;
pub const NET_ROSE_LINK_FAIL_TIMEOUT                           : c_int = 7;
pub const NET_ROSE_MAX_VCS                                     : c_int = 8;
pub const NET_ROSE_WINDOW_SIZE                                 : c_int = 9;
pub const NET_ROSE_NO_ACTIVITY_TIMEOUT                         : c_int = 10;

pub const NET_X25_RESTART_REQUEST_TIMEOUT                      : c_int = 1;
pub const NET_X25_CALL_REQUEST_TIMEOUT                         : c_int = 2;
pub const NET_X25_RESET_REQUEST_TIMEOUT                        : c_int = 3;
pub const NET_X25_CLEAR_REQUEST_TIMEOUT                        : c_int = 4;
pub const NET_X25_ACK_HOLD_BACK_TIMEOUT                        : c_int = 5;
pub const NET_X25_FORWARD                                      : c_int = 6;

pub const NET_TR_RIF_TIMEOUT                                   : c_int = 1;

pub const NET_DECNET_NODE_TYPE                                 : c_int = 1;
pub const NET_DECNET_NODE_ADDRESS                              : c_int = 2;
pub const NET_DECNET_NODE_NAME                                 : c_int = 3;
pub const NET_DECNET_DEFAULT_DEVICE                            : c_int = 4;
pub const NET_DECNET_TIME_WAIT                                 : c_int = 5;
pub const NET_DECNET_DN_COUNT                                  : c_int = 6;
pub const NET_DECNET_DI_COUNT                                  : c_int = 7;
pub const NET_DECNET_DR_COUNT                                  : c_int = 8;
pub const NET_DECNET_DST_GC_INTERVAL                           : c_int = 9;
pub const NET_DECNET_CONF                                      : c_int = 10;
pub const NET_DECNET_NO_FC_MAX_CWND                            : c_int = 11;
pub const NET_DECNET_MEM                                       : c_int = 12;
pub const NET_DECNET_RMEM                                      : c_int = 13;
pub const NET_DECNET_WMEM                                      : c_int = 14;
pub const NET_DECNET_DEBUG_LEVEL                               : c_int = 255;

pub const NET_DECNET_CONF_LOOPBACK                             : c_int = -2;
pub const NET_DECNET_CONF_DDCMP                                : c_int = -3;
pub const NET_DECNET_CONF_PPP                                  : c_int = -4;
pub const NET_DECNET_CONF_X25                                  : c_int = -5;
pub const NET_DECNET_CONF_GRE                                  : c_int = -6;
pub const NET_DECNET_CONF_ETHER                                : c_int = -7;

pub const NET_DECNET_CONF_DEV_PRIORITY                         : c_int = 1;
pub const NET_DECNET_CONF_DEV_T1                               : c_int = 2;
pub const NET_DECNET_CONF_DEV_T2                               : c_int = 3;
pub const NET_DECNET_CONF_DEV_T3                               : c_int = 4;
pub const NET_DECNET_CONF_DEV_FORWARDING                       : c_int = 5;
pub const NET_DECNET_CONF_DEV_BLKSIZE                          : c_int = 6;
pub const NET_DECNET_CONF_DEV_STATE                            : c_int = 7;

pub const NET_SCTP_RTO_INITIAL                                 : c_int = 1;
pub const NET_SCTP_RTO_MIN                                     : c_int = 2;
pub const NET_SCTP_RTO_MAX                                     : c_int = 3;
pub const NET_SCTP_RTO_ALPHA                                   : c_int = 4;
pub const NET_SCTP_RTO_BETA                                    : c_int = 5;
pub const NET_SCTP_VALID_COOKIE_LIFE                           : c_int = 6;
pub const NET_SCTP_ASSOCIATION_MAX_RETRANS                     : c_int = 7;
pub const NET_SCTP_PATH_MAX_RETRANS                            : c_int = 8;
pub const NET_SCTP_MAX_INIT_RETRANSMITS                        : c_int = 9;
pub const NET_SCTP_HB_INTERVAL                                 : c_int = 10;
pub const NET_SCTP_PRESERVE_ENABLE                             : c_int = 11;
pub const NET_SCTP_MAX_BURST                                   : c_int = 12;
pub const NET_SCTP_ADDIP_ENABLE                                : c_int = 13;
pub const NET_SCTP_PRSCTP_ENABLE                               : c_int = 14;
pub const NET_SCTP_SNDBUF_POLICY                               : c_int = 15;
pub const NET_SCTP_SACK_TIMEOUT                                : c_int = 16;
pub const NET_SCTP_RCVBUF_POLICY                               : c_int = 17;

pub const NET_BRIDGE_NF_CALL_ARPTABLES                         : c_int = 1;
pub const NET_BRIDGE_NF_CALL_IPTABLES                          : c_int = 2;
pub const NET_BRIDGE_NF_CALL_IP6TABLES                         : c_int = 3;
pub const NET_BRIDGE_NF_FILTER_VLAN_TAGGED                     : c_int = 4;
pub const NET_BRIDGE_NF_FILTER_PPPOE_TAGGED                    : c_int = 5;

pub const NET_IRDA_DISCOVERY                                   : c_int = 1;
pub const NET_IRDA_DEVNAME                                     : c_int = 2;
pub const NET_IRDA_DEBUG                                       : c_int = 3;
pub const NET_IRDA_FAST_POLL                                   : c_int = 4;
pub const NET_IRDA_DISCOVERY_SLOTS                             : c_int = 5;
pub const NET_IRDA_DISCOVERY_TIMEOUT                           : c_int = 6;
pub const NET_IRDA_SLOT_TIMEOUT                                : c_int = 7;
pub const NET_IRDA_MAX_BAUD_RATE                               : c_int = 8;
pub const NET_IRDA_MIN_TX_TURN_TIME                            : c_int = 9;
pub const NET_IRDA_MAX_TX_DATA_SIZE                            : c_int = 10;
pub const NET_IRDA_MAX_TX_WINDOW                               : c_int = 11;
pub const NET_IRDA_MAX_NOREPLY_TIME                            : c_int = 12;
pub const NET_IRDA_WARN_NOREPLY_TIME                           : c_int = 13;
pub const NET_IRDA_LAP_KEEPALIVE_TIME                          : c_int = 14;

pub const FS_NRINODE                                           : c_int = 1;
pub const FS_STATINODE                                         : c_int = 2;
pub const FS_MAXINODE                                          : c_int = 3;
pub const FS_NRDQUOT                                           : c_int = 4;
pub const FS_MAXDQUOT                                          : c_int = 5;
pub const FS_NRFILE                                            : c_int = 6;
pub const FS_MAXFILE                                           : c_int = 7;
pub const FS_DENTRY                                            : c_int = 8;
pub const FS_NRSUPER                                           : c_int = 9;
pub const FS_MAXSUPER                                          : c_int = 10;
pub const FS_OVERFLOWUID                                       : c_int = 11;
pub const FS_OVERFLOWGID                                       : c_int = 12;
pub const FS_LEASES                                            : c_int = 13;
pub const FS_DIR_NOTIFY                                        : c_int = 14;
pub const FS_LEASE_TIME                                        : c_int = 15;
pub const FS_DQSTATS                                           : c_int = 16;
pub const FS_XFS                                               : c_int = 17;
pub const FS_AIO_NR                                            : c_int = 18;
pub const FS_AIO_MAX_NR                                        : c_int = 19;
pub const FS_INOTIFY                                           : c_int = 20;
pub const FS_OCFS2                                             : c_int = 988;

pub const FS_DQ_LOOKUPS                                        : c_int = 1;
pub const FS_DQ_DROPS                                          : c_int = 2;
pub const FS_DQ_READS                                          : c_int = 3;
pub const FS_DQ_WRITES                                         : c_int = 4;
pub const FS_DQ_CACHE_HITS                                     : c_int = 5;
pub const FS_DQ_ALLOCATED                                      : c_int = 6;
pub const FS_DQ_FREE                                           : c_int = 7;
pub const FS_DQ_SYNCS                                          : c_int = 8;
pub const FS_DQ_WARNINGS                                       : c_int = 9;

pub const DEV_CDROM                                            : c_int = 1;
pub const DEV_HWMON                                            : c_int = 2;
pub const DEV_PARPORT                                          : c_int = 3;
pub const DEV_RAID                                             : c_int = 4;
pub const DEV_MAC_HID                                          : c_int = 5;
pub const DEV_SCSI                                             : c_int = 6;
pub const DEV_IPMI                                             : c_int = 7;

pub const DEV_CDROM_INFO                                       : c_int = 1;
pub const DEV_CDROM_AUTOCLOSE                                  : c_int = 2;
pub const DEV_CDROM_AUTOEJECT                                  : c_int = 3;
pub const DEV_CDROM_DEBUG                                      : c_int = 4;
pub const DEV_CDROM_LOCK                                       : c_int = 5;
pub const DEV_CDROM_CHECK_MEDIA                                : c_int = 6;

pub const DEV_PARPORT_DEFAULT                                  : c_int = -3;

pub const DEV_RAID_SPEED_LIMIT_MIN                             : c_int = 1;
pub const DEV_RAID_SPEED_LIMIT_MAX                             : c_int = 2;

pub const DEV_PARPORT_DEFAULT_TIMESLICE                        : c_int = 1;
pub const DEV_PARPORT_DEFAULT_SPINTIME                         : c_int = 2;

pub const DEV_PARPORT_SPINTIME                                 : c_int = 1;
pub const DEV_PARPORT_BASE_ADDR                                : c_int = 2;
pub const DEV_PARPORT_IRQ                                      : c_int = 3;
pub const DEV_PARPORT_DMA                                      : c_int = 4;
pub const DEV_PARPORT_MODES                                    : c_int = 5;
pub const DEV_PARPORT_DEVICES                                  : c_int = 6;
pub const DEV_PARPORT_AUTOPROBE                                : c_int = 16;

pub const DEV_PARPORT_DEVICES_ACTIVE                           : c_int = -3;

pub const DEV_PARPORT_DEVICE_TIMESLICE                         : c_int = 1;

pub const DEV_MAC_HID_KEYBOARD_SENDS_LINUX_KEYCODES            : c_int = 1;
pub const DEV_MAC_HID_KEYBOARD_LOCK_KEYCODES                   : c_int = 2;
pub const DEV_MAC_HID_MOUSE_BUTTON_EMULATION                   : c_int = 3;
pub const DEV_MAC_HID_MOUSE_BUTTON2_KEYCODE                    : c_int = 4;
pub const DEV_MAC_HID_MOUSE_BUTTON3_KEYCODE                    : c_int = 5;
pub const DEV_MAC_HID_ADB_MOUSE_SENDS_KEYCODES                 : c_int = 6;

pub const DEV_SCSI_LOGGING_LEVEL                               : c_int = 1;

pub const DEV_IPMI_POWEROFF_POWERCYCLE                         : c_int = 1;

pub const ABI_DEFHANDLER_COFF                                  : c_int = 1;
pub const ABI_DEFHANDLER_ELF                                   : c_int = 2;
pub const ABI_DEFHANDLER_LCALL7                                : c_int = 3;
pub const ABI_DEFHANDLER_LIBCSO                                : c_int = 4;
pub const ABI_TRACE                                            : c_int = 5;
pub const ABI_FAKE_UTSNAME                                     : c_int = 6;

// tms.h

#[repr(C)]
#[derive(Pod, Eq)]
pub struct tms {
    pub tms_utime:  __kernel_clock_t,
    pub tms_stime:  __kernel_clock_t,
    pub tms_cutime: __kernel_clock_t,
    pub tms_cstime: __kernel_clock_t,
}

// types.h

#[repr(C)]
#[derive(Pod, Eq)]
pub struct ustat {
    pub f_tfree:  __kernel_daddr_t,
    pub f_tinode: __kernel_ino_t,
    pub f_fname: [c_char; 6],
    pub f_fpack: [c_char; 6],
}

// utime.h

#[repr(C)]
#[derive(Pod, Eq)]
pub struct utimbuf {
    pub actime:  __kernel_time_t,
    pub modtime: __kernel_time_t,
}

// limits.h

pub const NR_OPEN        : usize = 1024;
pub const NGROUPS_MAX    : usize = 65536;
pub const ARG_MAX        : usize = 131072;
pub const LINK_MAX       : usize = 127;
pub const MAX_CANON      : usize = 255;
pub const MAX_INPUT      : usize = 255;
pub const NAME_MAX       : usize = 255;
pub const PATH_MAX       : usize = 4096;
pub const PIPE_BUF       : usize = 4096;
pub const XATTR_NAME_MAX : usize = 255;
pub const XATTR_SIZE_MAX : usize = 65536;
pub const XATTR_LIST_MAX : usize = 65536;
pub const RTSIG_MAX      : usize = 32;

// fs.h

pub const INR_OPEN_CUR     : c_int = 1024;
pub const INR_OPEN_MAX     : c_int = 4096;
pub const BLOCK_SIZE_BITS  : c_int = 10;
pub const BLOCK_SIZE       : c_int = 1 << BLOCK_SIZE_BITS;
pub const SEEK_SET         : c_uint = 0;
pub const SEEK_CUR         : c_uint = 1;
pub const SEEK_END         : c_uint = 2;
pub const SEEK_DATA        : c_uint = 3;
pub const SEEK_HOLE        : c_uint = 4;
pub const SEEK_MAX         : c_uint = SEEK_HOLE;
pub const RENAME_NOREPLACE : c_int = 1 << 0;
pub const RENAME_EXCHANGE  : c_int = 1 << 1;
pub const RENAME_WHITEOUT  : c_int = 1 << 2;

pub const NR_FILE        : c_ulong = 8192;
pub const MS_RDONLY      : c_ulong = 1;
pub const MS_NOSUID      : c_ulong = 2;
pub const MS_NODEV       : c_ulong = 4;
pub const MS_NOEXEC      : c_ulong = 8;
pub const MS_SYNCHRONOUS : c_ulong = 16;
pub const MS_REMOUNT     : c_ulong = 32;
pub const MS_MANDLOCK    : c_ulong = 64;
pub const MS_DIRSYNC     : c_ulong = 128;
pub const MS_NOATIME     : c_ulong = 1024;
pub const MS_NODIRATIME  : c_ulong = 2048;
pub const MS_BIND        : c_ulong = 4096;
pub const MS_MOVE        : c_ulong = 8192;
pub const MS_REC         : c_ulong = 16384;
pub const MS_VERBOSE     : c_ulong = 32768;
pub const MS_SILENT      : c_ulong = 32768;
pub const MS_POSIXACL    : c_ulong = 1 << 16;
pub const MS_UNBINDABLE  : c_ulong = 1 << 17;
pub const MS_PRIVATE     : c_ulong = 1 << 18;
pub const MS_SLAVE       : c_ulong = 1 << 19;
pub const MS_SHARED      : c_ulong = 1 << 20;
pub const MS_RELATIME    : c_ulong = 1 << 21;
pub const MS_KERNMOUNT   : c_ulong = 1 << 22;
pub const MS_I_VERSION   : c_ulong = 1 << 23;
pub const MS_STRICTATIME : c_ulong = 1 << 24;
pub const MS_LAZYTIME    : c_ulong = 1 << 25;
pub const MS_NOSEC       : c_ulong = 1 << 28;
pub const MS_BORN        : c_ulong = 1 << 29;
pub const MS_ACTIVE      : c_ulong = 1 << 30;
pub const MS_NOUSER      : c_ulong = 1 << 31;

pub const MS_RMT_MASK : c_ulong = MS_RDONLY|MS_SYNCHRONOUS|MS_MANDLOCK|MS_I_VERSION|
                                    MS_LAZYTIME;

pub const MS_MGC_VAL : c_ulong = 0xC0ED0000;
pub const MS_MGC_MSK : c_ulong = 0xffff0000;

pub const MNT_FORCE       : c_int = 0x00000001;
pub const MNT_DETACH      : c_int = 0x00000002;
pub const MNT_EXPIRE      : c_int = 0x00000004;
pub const UMOUNT_NOFOLLOW : c_int = 0x00000008;
pub const UMOUNT_UNUSED   : c_int = 0x80000000;

// statfs.h

pub const ST_RDONLY      : c_ulong = 0x0001;
pub const ST_NOSUID      : c_ulong = 0x0002;
pub const ST_NODEV       : c_ulong = 0x0004;
pub const ST_NOEXEC      : c_ulong = 0x0008;
pub const ST_SYNCHRONOUS : c_ulong = 0x0010;
pub const ST_VALID       : c_ulong = 0x0020;
pub const ST_MANDLOCK    : c_ulong = 0x0040;
pub const ST_WRITE       : c_ulong = 0x0080;
pub const ST_APPEND      : c_ulong = 0x0100;
pub const ST_IMMUTABLE   : c_ulong = 0x0200;
pub const ST_NOATIME     : c_ulong = 0x0400;
pub const ST_NODIRATIME  : c_ulong = 0x0800;
pub const ST_RELATIME    : c_ulong = 0x1000;

// timerfd.h

pub const TFD_TIMER_ABSTIME       : c_int = 1 << 0;
pub const TFD_TIMER_CANCEL_ON_SET : c_int = 1 << 1;
pub const TFD_CLOEXEC             : c_int = O_CLOEXEC;
pub const TFD_NONBLOCK            : c_int = O_NONBLOCK;
pub const TFD_SHARED_FCNTL_FLAGS  : c_int = TFD_CLOEXEC | TFD_NONBLOCK;
pub const TFD_CREATE_FLAGS        : c_int = TFD_SHARED_FCNTL_FLAGS;
pub const TFD_SETTIME_FLAGS       : c_int = TFD_TIMER_ABSTIME | TFD_TIMER_CANCEL_ON_SET;
pub fn TFD_IOC_SET_TICKS() -> c_uint { _IOW::<u64>(b'T' as c_uint, 0) }

// random.h

pub fn RNDGETENTCNT   () -> c_uint { _IOR::<c_int>(b'R' as c_uint, 0x00) }
pub fn RNDADDTOENTCNT () -> c_uint { _IOW::<c_int>(b'R' as c_uint, 0x01) }
pub fn RNDGETPOOL     () -> c_uint { _IOR::<[c_int; 2]>(b'R' as c_uint, 0x02) }
pub fn RNDADDENTROPY  () -> c_uint { _IOW::<[c_int; 2]>(b'R' as c_uint, 0x03) }
pub fn RNDZAPENTCNT   () -> c_uint { _IO(b'R' as c_uint, 0x04) }
pub fn RNDCLEARPOOL   () -> c_uint { _IO(b'R' as c_uint, 0x06) }

#[repr(C)]
#[derive(Pod, Eq)]
pub struct rand_pool_info {
    pub entropy_count: c_int,
    pub buf_size:      c_int,
    pub buf:        [__u32; 0],
}

pub const GRND_NONBLOCK : c_uint = 0x0001;
pub const GRND_RANDOM   : c_uint = 0x0002;

// eventpoll.h

pub const EPOLL_CLOEXEC : c_int = O_CLOEXEC;
pub const EPOLL_CTL_ADD : c_int = 1;
pub const EPOLL_CTL_DEL : c_int = 2;
pub const EPOLL_CTL_MOD : c_int = 3;
pub const EPOLLWAKEUP   : c_uint = 1 << 29;
pub const EPOLLONESHOT  : c_uint = 1 << 30;
pub const EPOLLET       : c_uint = 1 << 31;

// stat.h

pub const S_IFMT   : umode_t = 0o170000;

// bit-start of the filetypes
pub const MODE_TYPE_SHIFT: usize = 12;

pub const S_IFSOCK : umode_t = 0o140000;
pub const S_IFLNK  : umode_t = 0o120000;
pub const S_IFREG  : umode_t = 0o100000;
pub const S_IFBLK  : umode_t = 0o060000;
pub const S_IFDIR  : umode_t = 0o040000;
pub const S_IFCHR  : umode_t = 0o020000;
pub const S_IFIFO  : umode_t = 0o010000;

pub const S_ISUID  : umode_t = 0o004000;
pub const S_ISGID  : umode_t = 0o002000;
pub const S_ISVTX  : umode_t = 0o001000;
pub const S_IRWXU  : umode_t = 0o000700;
pub const S_IRUSR  : umode_t = 0o000400;
pub const S_IWUSR  : umode_t = 0o000200;
pub const S_IXUSR  : umode_t = 0o000100;
pub const S_IRWXG  : umode_t = 0o000070;
pub const S_IRGRP  : umode_t = 0o000040;
pub const S_IWGRP  : umode_t = 0o000020;
pub const S_IXGRP  : umode_t = 0o000010;
pub const S_IRWXO  : umode_t = 0o000007;
pub const S_IROTH  : umode_t = 0o000004;
pub const S_IWOTH  : umode_t = 0o000002;
pub const S_IXOTH  : umode_t = 0o000001;

pub fn S_ISLNK(m: umode_t)  -> bool { m & S_IFMT == S_IFLNK }
pub fn S_ISREG(m: umode_t)  -> bool { m & S_IFMT == S_IFREG }
pub fn S_ISDIR(m: umode_t)  -> bool { m & S_IFMT == S_IFDIR }
pub fn S_ISCHR(m: umode_t)  -> bool { m & S_IFMT == S_IFCHR }
pub fn S_ISBLK(m: umode_t)  -> bool { m & S_IFMT == S_IFBLK }
pub fn S_ISFIFO(m: umode_t) -> bool { m & S_IFMT == S_IFIFO }
pub fn S_ISSOCK(m: umode_t) -> bool { m & S_IFMT == S_IFSOCK }

pub const S_IRWXUGO  : umode_t = S_IRWXU|S_IRWXG|S_IRWXO;
pub const S_IALLUGO  : umode_t = S_ISUID|S_ISGID|S_ISVTX|S_IRWXUGO;
pub const S_IRUGO    : umode_t = S_IRUSR|S_IRGRP|S_IROTH;
pub const S_IWUGO    : umode_t = S_IWUSR|S_IWGRP|S_IWOTH;
pub const S_IXUGO    : umode_t = S_IXUSR|S_IXGRP|S_IXOTH;

pub const UTIME_NOW  : k_long = (1 << 30) - 1;
pub const UTIME_OMIT : k_long = (1 << 30) - 2;

// falloc.h

pub const FALLOC_FL_KEEP_SIZE      : c_int = 0x01;
pub const FALLOC_FL_PUNCH_HOLE     : c_int = 0x02;
pub const FALLOC_FL_NO_HIDE_STALE  : c_int = 0x04;
pub const FALLOC_FL_COLLAPSE_RANGE : c_int = 0x08;
pub const FALLOC_FL_ZERO_RANGE     : c_int = 0x10;

// fcntl.h

pub const O_ASYNC : c_int = FASYNC;

// fadvise.h

pub const POSIX_FADV_NORMAL     : c_int = 0;
pub const POSIX_FADV_RANDOM     : c_int = 1;
pub const POSIX_FADV_SEQUENTIAL : c_int = 2;
pub const POSIX_FADV_WILLNEED   : c_int = 3;

// mman.h

pub const MREMAP_MAYMOVE    : c_int = 1;
pub const MREMAP_FIXED      : c_int = 2;
pub const OVERCOMMIT_GUESS  : c_int = 0;
pub const OVERCOMMIT_ALWAYS : c_int = 1;
pub const OVERCOMMIT_NEVER  : c_int = 2;

// wait.h

pub const WNOHANG     : c_int = 0x00000001;
pub const WUNTRACED   : c_int = 0x00000002;
pub const WSTOPPED    : c_int = WUNTRACED;
pub const WEXITED     : c_int = 0x00000004;
pub const WCONTINUED  : c_int = 0x00000008;
pub const WNOWAIT     : c_int = 0x01000000;
pub const __WNOTHREAD : c_int = 0x20000000;
pub const __WALL      : c_int = 0x40000000;
pub const __WCLONE    : c_int = 0x80000000;
pub const P_ALL       : c_int = 0;
pub const P_PID       : c_int = 1;
pub const P_PGID      : c_int = 2;

// un.h

pub const UNIX_PATH_MAX : usize = 108;

#[repr(C)]
#[derive(Pod)]
pub struct sockaddr_un {
    pub sun_family: __kernel_sa_family_t,
    pub sun_path: [c_char; UNIX_PATH_MAX],
}


// uapi/linux/net.h

pub const NPROTO          : c_int = AF_MAX;
pub const SYS_SOCKET      : c_int = 1;
pub const SYS_BIND        : c_int = 2;
pub const SYS_CONNECT     : c_int = 3;
pub const SYS_LISTEN      : c_int = 4;
pub const SYS_ACCEPT      : c_int = 5;
pub const SYS_GETSOCKNAME : c_int = 6;
pub const SYS_GETPEERNAME : c_int = 7;
pub const SYS_SOCKETPAIR  : c_int = 8;
pub const SYS_SEND        : c_int = 9;
pub const SYS_RECV        : c_int = 10;
pub const SYS_SENDTO      : c_int = 11;
pub const SYS_RECVFROM    : c_int = 12;
pub const SYS_SHUTDOWN    : c_int = 13;
pub const SYS_SETSOCKOPT  : c_int = 14;
pub const SYS_GETSOCKOPT  : c_int = 15;
pub const SYS_SENDMSG     : c_int = 16;
pub const SYS_RECVMSG     : c_int = 17;
pub const SYS_ACCEPT4     : c_int = 18;
pub const SYS_RECVMMSG    : c_int = 19;
pub const SYS_SENDMMSG    : c_int = 20;

pub const SS_FREE          : c_int = 0;
pub const SS_UNCONNECTED   : c_int = 1;
pub const SS_CONNECTING    : c_int = 2;
pub const SS_CONNECTED     : c_int = 3;
pub const SS_DISCONNECTING : c_int = 4;

pub const __SO_ACCEPTCON : c_int = 1 << 16;

// net.h

pub const SOCK_ASYNC_NOSPACE        : c_int = 0;
pub const SOCK_ASYNC_WAITDATA       : c_int = 1;
pub const SOCK_NOSPACE              : c_int = 2;
pub const SOCK_PASSCRED             : c_int = 3;
pub const SOCK_PASSSEC              : c_int = 4;
pub const SOCK_EXTERNALLY_ALLOCATED : c_int = 5;

pub const SHUT_RD   : c_int = 0;
pub const SHUT_WR   : c_int = 1;
pub const SHUT_RDWR : c_int = 2;

// XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX
// XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX
// XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX
// XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX XXX

//////////////////////////
// include/uapi/linux/in.h
//////////////////////////

pub const IPPROTO_IP      : c_int = 0;
pub const IPPROTO_ICMP    : c_int = 1;
pub const IPPROTO_IGMP    : c_int = 2;
pub const IPPROTO_IPIP    : c_int = 4;
pub const IPPROTO_TCP     : c_int = 6;
pub const IPPROTO_EGP     : c_int = 8;
pub const IPPROTO_PUP     : c_int = 12;
pub const IPPROTO_UDP     : c_int = 17;
pub const IPPROTO_IDP     : c_int = 22;
pub const IPPROTO_TP      : c_int = 29;
pub const IPPROTO_DCCP    : c_int = 33;
pub const IPPROTO_IPV6    : c_int = 41;
pub const IPPROTO_RSVP    : c_int = 46;
pub const IPPROTO_GRE     : c_int = 47;
pub const IPPROTO_ESP     : c_int = 50;
pub const IPPROTO_AH      : c_int = 51;
pub const IPPROTO_MTP     : c_int = 92;
pub const IPPROTO_BEETPH  : c_int = 94;
pub const IPPROTO_ENCAP   : c_int = 98;
pub const IPPROTO_PIM     : c_int = 103;
pub const IPPROTO_COMP    : c_int = 108;
pub const IPPROTO_SCTP    : c_int = 132;
pub const IPPROTO_UDPLITE : c_int = 136;
pub const IPPROTO_RAW     : c_int = 255;
pub const IPPROTO_MAX     : c_int = 256;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct in_addr {
    pub s_addr: __be32,
}

pub const BYTES_PER_IN_ADDR: usize = 4;

pub const IP_TOS                    : c_int = 1;
pub const IP_TTL                    : c_int = 2;
pub const IP_HDRINCL                : c_int = 3;
pub const IP_OPTIONS                : c_int = 4;
pub const IP_ROUTER_ALERT           : c_int = 5;
pub const IP_RECVOPTS               : c_int = 6;
pub const IP_RETOPTS                : c_int = 7;
pub const IP_PKTINFO                : c_int = 8;
pub const IP_PKTOPTIONS             : c_int = 9;
pub const IP_MTU_DISCOVER           : c_int = 10;
pub const IP_RECVERR                : c_int = 11;
pub const IP_RECVTTL                : c_int = 12;
pub const IP_RECVTOS                : c_int = 13;
pub const IP_MTU                    : c_int = 14;
pub const IP_FREEBIND               : c_int = 15;
pub const IP_IPSEC_POLICY           : c_int = 16;
pub const IP_XFRM_POLICY            : c_int = 17;
pub const IP_PASSSEC                : c_int = 18;
pub const IP_TRANSPARENT            : c_int = 19;
pub const IP_RECVRETOPTS            : c_int = IP_RETOPTS;
pub const IP_ORIGDSTADDR            : c_int = 20;
pub const IP_RECVORIGDSTADDR        : c_int = IP_ORIGDSTADDR;
pub const IP_MINTTL                 : c_int = 21;
pub const IP_NODEFRAG               : c_int = 22;
pub const IP_CHECKSUM               : c_int = 23;
pub const IP_PMTUDISC_DONT          : c_int = 0;
pub const IP_PMTUDISC_WANT          : c_int = 1;
pub const IP_PMTUDISC_DO            : c_int = 2;
pub const IP_PMTUDISC_PROBE         : c_int = 3;
pub const IP_PMTUDISC_INTERFACE     : c_int = 4;
pub const IP_PMTUDISC_OMIT          : c_int = 5;
pub const IP_MULTICAST_IF           : c_int = 32;
pub const IP_MULTICAST_TTL          : c_int = 33;
pub const IP_MULTICAST_LOOP         : c_int = 34;
pub const IP_ADD_MEMBERSHIP         : c_int = 35;
pub const IP_DROP_MEMBERSHIP        : c_int = 36;
pub const IP_UNBLOCK_SOURCE         : c_int = 37;
pub const IP_BLOCK_SOURCE           : c_int = 38;
pub const IP_ADD_SOURCE_MEMBERSHIP  : c_int = 39;
pub const IP_DROP_SOURCE_MEMBERSHIP : c_int = 40;
pub const IP_MSFILTER               : c_int = 41;
pub const MCAST_JOIN_GROUP          : c_int = 42;
pub const MCAST_BLOCK_SOURCE        : c_int = 43;
pub const MCAST_UNBLOCK_SOURCE      : c_int = 44;
pub const MCAST_LEAVE_GROUP         : c_int = 45;
pub const MCAST_JOIN_SOURCE_GROUP   : c_int = 46;
pub const MCAST_LEAVE_SOURCE_GROUP  : c_int = 47;
pub const MCAST_MSFILTER            : c_int = 48;
pub const IP_MULTICAST_ALL          : c_int = 49;
pub const IP_UNICAST_IF             : c_int = 50;
pub const MCAST_EXCLUDE             : c_int = 0;
pub const MCAST_INCLUDE             : c_int = 1;
pub const IP_DEFAULT_MULTICAST_TTL  : c_int = 1;
pub const IP_DEFAULT_MULTICAST_LOOP : c_int = 1;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct ip_mreq {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct ip_mreqn {
    pub imr_multiaddr: in_addr,
    pub imr_address:   in_addr,
    pub imr_ifindex:   c_int,
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct ip_mreq_source {
    pub imr_multiaddr:  __be32,
    pub imr_interface:  __be32,
    pub imr_sourceaddr: __be32,
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct ip_msfilter {
    pub imsf_multiaddr: __be32,
    pub imsf_interface: __be32,
    pub imsf_fmode:     __u32,
    pub imsf_numsrc:    __u32,
    pub imsf_slist: [__be32; 1],
}

pub fn IP_MSFILTER_SIZE(numsrc: usize) -> usize {
    mem::size_of::<ip_msfilter>() - mem::size_of::<__u32>()
                + numsrc * mem::size_of::<__u32>()
}

#[repr(C)]
#[derive(Pod)]
pub struct group_req {
    pub gr_interface: __u32,
    pub gr_group:     __kernel_sockaddr_storage,
}

#[repr(C)]
#[derive(Pod)]
pub struct group_source_req {
    pub gsr_interface: __u32,
    pub gsr_group:     __kernel_sockaddr_storage,
    pub gsr_source:    __kernel_sockaddr_storage,
}

#[repr(C)]
#[derive(Pod)]
pub struct group_filter {
    pub gf_interface: __u32,
    pub gf_group:     __kernel_sockaddr_storage,
    pub gf_fmode:     __u32,
    pub gf_numsrc:    __u32,
    pub gf_slist:     [__kernel_sockaddr_storage; 1],
}

pub fn GROUP_FILTER_SIZE(numsrc: usize ) -> usize {
    mem::size_of::<group_filter>() - mem::size_of::<__kernel_sockaddr_storage>()
            + numsrc * mem::size_of::<__kernel_sockaddr_storage>()
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct in_pktinfo {
    pub ipi_ifindex:  c_int,
    pub ipi_spec_dst: in_addr,
    pub ipi_addr:     in_addr,
}

pub const __SOCK_SIZE__: usize = 16;

#[repr(C)]
#[derive(Pod)]
pub struct sockaddr_in {
    pub sin_family: __kernel_sa_family_t,
    pub sin_port:   __be16,
    pub sin_addr:   in_addr,
    pub __pad: [c_uchar; __SOCK_SIZE__ - BYTES_PER_SHORT - BYTES_PER_SHORT - BYTES_PER_IN_ADDR],
}

pub fn IN_CLASSA(a: u32) -> bool { (a & 0x80000000) == 0 }
pub const IN_CLASSA_NET    : u32 = 0xff000000;
pub const IN_CLASSA_NSHIFT : u32 = 24;
pub const IN_CLASSA_HOST   : u32 = 0xffffffff & !IN_CLASSA_NET;
pub const IN_CLASSA_MAX    : u32 = 128;

pub fn IN_CLASSB(a: u32) -> bool { (a & 0xc0000000) == 0x80000000 }
pub const IN_CLASSB_NET    : u32 = 0xffff0000;
pub const IN_CLASSB_NSHIFT : u32 = 16;
pub const IN_CLASSB_HOST   : u32 = 0xffffffff & !IN_CLASSB_NET;
pub const IN_CLASSB_MAX    : u32 = 65536;

pub fn IN_CLASSC(a: u32) -> bool { (a & 0xe0000000) == 0xc0000000 }
pub const IN_CLASSC_NET    : u32 = 0xffffff00;
pub const IN_CLASSC_NSHIFT : u32 = 8;
pub const IN_CLASSC_HOST   : u32 = 0xffffffff & !IN_CLASSC_NET;

pub fn IN_CLASSD(a: u32) -> bool { (a & 0xf0000000) == 0xe0000000 }
pub fn IN_MULTICAST(a: u32) -> bool { IN_CLASSD(a) }
pub const IN_MULTICAST_NET : u32 = 0xF0000000;

pub fn IN_EXPERIMENTAL(a: u32) -> bool { (a & 0xf0000000) == 0xf0000000 }
pub fn IN_BADCLASS(a: u32) -> bool { IN_EXPERIMENTAL(a) }

pub const INADDR_ANY       : u32 = 0x00000000;
pub const INADDR_BROADCAST : u32 = 0xffffffff;
pub const INADDR_NONE      : u32 = 0xffffffff;
pub const IN_LOOPBACKNET   : u32 = 127;
pub const INADDR_LOOPBACK  : u32 = 0x7f000001;

pub fn IN_LOOPBACK(a: u32) -> bool { (a & 0xff000000) == 0x7f000000 }

pub const INADDR_UNSPEC_GROUP    : u32 = 0xe0000000;
pub const INADDR_ALLHOSTS_GROUP  : u32 = 0xe0000001;
pub const INADDR_ALLRTRS_GROUP   : u32 = 0xe0000002;
pub const INADDR_MAX_LOCAL_GROUP : u32 = 0xe00000ff;

///////////////////////////
// include/uapi/linux/in6.h
///////////////////////////

#[repr(C)]
#[derive(Pod, Eq)]
pub struct in6_addr {
    pub u6_addr16: [__be16; 8],
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct sockaddr_in6 {
    pub sin6_family:   c_ushort,
    pub sin6_port:     __be16,
    pub sin6_flowinfo: __be32,
    pub sin6_addr:     in6_addr,
    pub sin6_scope_id: __u32,
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct ipv6_mreq {
    pub ipv6mr_multiaddr: in6_addr,
    pub ipv6mr_ifindex:   c_int,
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct in6_flowlabel_req {
    pub flr_dst:     in6_addr,
    pub flr_label:   __be32,
    pub flr_action:  __u8,
    pub flr_share:   __u8,
    pub flr_flags:   __u16,
    pub flr_expires: __u16,
    pub flr_linger:  __u16,
    pub __flr_pad:   __u32,
}

pub const IPV6_FL_A_GET     : __u8 = 0;
pub const IPV6_FL_A_PUT     : __u8 = 1;
pub const IPV6_FL_A_RENEW   : __u8 = 2;
pub const IPV6_FL_F_CREATE  : __u8 = 1;
pub const IPV6_FL_F_EXCL    : __u8 = 2;
pub const IPV6_FL_F_REFLECT : __u8 = 4;
pub const IPV6_FL_F_REMOTE  : __u8 = 8;
pub const IPV6_FL_S_NONE    : __u8 = 0;
pub const IPV6_FL_S_EXCL    : __u8 = 1;
pub const IPV6_FL_S_PROCESS : __u8 = 2;
pub const IPV6_FL_S_USER    : __u8 = 3;
pub const IPV6_FL_S_ANY     : __u8 = 255;

pub const IPV6_FLOWINFO_FLOWLABEL : __le32 = 0x000fffff;
pub const IPV6_FLOWINFO_PRIORITY  : __le32 = 0x0ff00000;

pub const IPV6_PRIORITY_UNCHARACTERIZED : c_int = 0x0000;
pub const IPV6_PRIORITY_FILLER          : c_int = 0x0100;
pub const IPV6_PRIORITY_UNATTENDED      : c_int = 0x0200;
pub const IPV6_PRIORITY_RESERVED1       : c_int = 0x0300;
pub const IPV6_PRIORITY_BULK            : c_int = 0x0400;
pub const IPV6_PRIORITY_RESERVED2       : c_int = 0x0500;
pub const IPV6_PRIORITY_INTERACTIVE     : c_int = 0x0600;
pub const IPV6_PRIORITY_CONTROL         : c_int = 0x0700;
pub const IPV6_PRIORITY_8               : c_int = 0x0800;
pub const IPV6_PRIORITY_9               : c_int = 0x0900;
pub const IPV6_PRIORITY_10              : c_int = 0x0a00;
pub const IPV6_PRIORITY_11              : c_int = 0x0b00;
pub const IPV6_PRIORITY_12              : c_int = 0x0c00;
pub const IPV6_PRIORITY_13              : c_int = 0x0d00;
pub const IPV6_PRIORITY_14              : c_int = 0x0e00;
pub const IPV6_PRIORITY_15              : c_int = 0x0f00;

pub const IPPROTO_HOPOPTS  : c_int = 0;
pub const IPPROTO_ROUTING  : c_int = 43;
pub const IPPROTO_FRAGMENT : c_int = 44;
pub const IPPROTO_ICMPV6   : c_int = 58;
pub const IPPROTO_NONE     : c_int = 59;
pub const IPPROTO_DSTOPTS  : c_int = 60;
pub const IPPROTO_MH       : c_int = 135;

pub const IPV6_TLV_PAD1        : c_int = 0;
pub const IPV6_TLV_PADN        : c_int = 1;
pub const IPV6_TLV_ROUTERALERT : c_int = 5;
pub const IPV6_TLV_JUMBO       : c_int = 194;
pub const IPV6_TLV_HAO         : c_int = 201;

pub const IPV6_ADDRFORM           : c_int = 1;
pub const IPV6_2292PKTINFO        : c_int = 2;
pub const IPV6_2292HOPOPTS        : c_int = 3;
pub const IPV6_2292DSTOPTS        : c_int = 4;
pub const IPV6_2292RTHDR          : c_int = 5;
pub const IPV6_2292PKTOPTIONS     : c_int = 6;
pub const IPV6_CHECKSUM           : c_int = 7;
pub const IPV6_2292HOPLIMIT       : c_int = 8;
pub const IPV6_NEXTHOP            : c_int = 9;
pub const IPV6_AUTHHDR            : c_int = 10;
pub const IPV6_FLOWINFO           : c_int = 11;
pub const IPV6_UNICAST_HOPS       : c_int = 16;
pub const IPV6_MULTICAST_IF       : c_int = 17;
pub const IPV6_MULTICAST_HOPS     : c_int = 18;
pub const IPV6_MULTICAST_LOOP     : c_int = 19;
pub const IPV6_ADD_MEMBERSHIP     : c_int = 20;
pub const IPV6_DROP_MEMBERSHIP    : c_int = 21;
pub const IPV6_ROUTER_ALERT       : c_int = 22;
pub const IPV6_MTU_DISCOVER       : c_int = 23;
pub const IPV6_MTU                : c_int = 24;
pub const IPV6_RECVERR            : c_int = 25;
pub const IPV6_V6ONLY             : c_int = 26;
pub const IPV6_JOIN_ANYCAST       : c_int = 27;
pub const IPV6_LEAVE_ANYCAST      : c_int = 28;
pub const IPV6_PMTUDISC_DONT      : c_int = 0;
pub const IPV6_PMTUDISC_WANT      : c_int = 1;
pub const IPV6_PMTUDISC_DO        : c_int = 2;
pub const IPV6_PMTUDISC_PROBE     : c_int = 3;
pub const IPV6_PMTUDISC_INTERFACE : c_int = 4;
pub const IPV6_PMTUDISC_OMIT      : c_int = 5;
pub const IPV6_FLOWLABEL_MGR      : c_int = 32;
pub const IPV6_FLOWINFO_SEND      : c_int = 33;
pub const IPV6_IPSEC_POLICY       : c_int = 34;
pub const IPV6_XFRM_POLICY        : c_int = 35;

pub const IPV6_RECVPKTINFO  : c_int = 49;
pub const IPV6_PKTINFO      : c_int = 50;
pub const IPV6_RECVHOPLIMIT : c_int = 51;
pub const IPV6_HOPLIMIT     : c_int = 52;
pub const IPV6_RECVHOPOPTS  : c_int = 53;
pub const IPV6_HOPOPTS      : c_int = 54;
pub const IPV6_RTHDRDSTOPTS : c_int = 55;
pub const IPV6_RECVRTHDR    : c_int = 56;
pub const IPV6_RTHDR        : c_int = 57;
pub const IPV6_RECVDSTOPTS  : c_int = 58;
pub const IPV6_DSTOPTS      : c_int = 59;
pub const IPV6_RECVPATHMTU  : c_int = 60;
pub const IPV6_PATHMTU      : c_int = 61;
pub const IPV6_DONTFRAG     : c_int = 62;

pub const IPV6_RECVTCLASS                : c_int = 66;
pub const IPV6_TCLASS                    : c_int = 67;
pub const IPV6_AUTOFLOWLABEL             : c_int = 70;
pub const IPV6_ADDR_PREFERENCES          : c_int = 72;
pub const IPV6_PREFER_SRC_TMP            : c_int = 0x0001;
pub const IPV6_PREFER_SRC_PUBLIC         : c_int = 0x0002;
pub const IPV6_PREFER_SRC_PUBTMP_DEFAULT : c_int = 0x0100;
pub const IPV6_PREFER_SRC_COA            : c_int = 0x0004;
pub const IPV6_PREFER_SRC_HOME           : c_int = 0x0400;
pub const IPV6_PREFER_SRC_CGA            : c_int = 0x0008;
pub const IPV6_PREFER_SRC_NONCGA         : c_int = 0x0800;
pub const IPV6_MINHOPCOUNT               : c_int = 73;
pub const IPV6_ORIGDSTADDR               : c_int = 74;
pub const IPV6_RECVORIGDSTADDR           : c_int = IPV6_ORIGDSTADDR;
pub const IPV6_TRANSPARENT               : c_int = 75;
pub const IPV6_UNICAST_IF                : c_int = 76;

///////////////////////////////
// include/uapi/linux/sockios.h
///////////////////////////////

pub const SIOCINQ                : c_int = FIONREAD as c_int;
pub const SIOCOUTQ               : c_int = TIOCOUTQ as c_int;
pub const SIOCADDRT              : c_int = 0x890B;
pub const SIOCDELRT              : c_int = 0x890C;
pub const SIOCRTMSG              : c_int = 0x890D;
pub const SIOCGIFNAME            : c_int = 0x8910;
pub const SIOCSIFLINK            : c_int = 0x8911;
pub const SIOCGIFCONF            : c_int = 0x8912;
pub const SIOCGIFFLAGS           : c_int = 0x8913;
pub const SIOCSIFFLAGS           : c_int = 0x8914;
pub const SIOCGIFADDR            : c_int = 0x8915;
pub const SIOCSIFADDR            : c_int = 0x8916;
pub const SIOCGIFDSTADDR         : c_int = 0x8917;
pub const SIOCSIFDSTADDR         : c_int = 0x8918;
pub const SIOCGIFBRDADDR         : c_int = 0x8919;
pub const SIOCSIFBRDADDR         : c_int = 0x891a;
pub const SIOCGIFNETMASK         : c_int = 0x891b;
pub const SIOCSIFNETMASK         : c_int = 0x891c;
pub const SIOCGIFMETRIC          : c_int = 0x891d;
pub const SIOCSIFMETRIC          : c_int = 0x891e;
pub const SIOCGIFMEM             : c_int = 0x891f;
pub const SIOCSIFMEM             : c_int = 0x8920;
pub const SIOCGIFMTU             : c_int = 0x8921;
pub const SIOCSIFMTU             : c_int = 0x8922;
pub const SIOCSIFNAME            : c_int = 0x8923;
pub const SIOCSIFHWADDR          : c_int = 0x8924;
pub const SIOCGIFENCAP           : c_int = 0x8925;
pub const SIOCSIFENCAP           : c_int = 0x8926;
pub const SIOCGIFHWADDR          : c_int = 0x8927;
pub const SIOCGIFSLAVE           : c_int = 0x8929;
pub const SIOCSIFSLAVE           : c_int = 0x8930;
pub const SIOCADDMULTI           : c_int = 0x8931;
pub const SIOCDELMULTI           : c_int = 0x8932;
pub const SIOCGIFINDEX           : c_int = 0x8933;
pub const SIOGIFINDEX            : c_int = SIOCGIFINDEX;
pub const SIOCSIFPFLAGS          : c_int = 0x8934;
pub const SIOCGIFPFLAGS          : c_int = 0x8935;
pub const SIOCDIFADDR            : c_int = 0x8936;
pub const SIOCSIFHWBROADCAST     : c_int = 0x8937;
pub const SIOCGIFCOUNT           : c_int = 0x8938;
pub const SIOCGIFBR              : c_int = 0x8940;
pub const SIOCSIFBR              : c_int = 0x8941;
pub const SIOCGIFTXQLEN          : c_int = 0x8942;
pub const SIOCSIFTXQLEN          : c_int = 0x8943;
pub const SIOCETHTOOL            : c_int = 0x8946;
pub const SIOCGMIIPHY            : c_int = 0x8947;
pub const SIOCGMIIREG            : c_int = 0x8948;
pub const SIOCSMIIREG            : c_int = 0x8949;
pub const SIOCWANDEV             : c_int = 0x894A;
pub const SIOCOUTQNSD            : c_int = 0x894B;
pub const SIOCDARP               : c_int = 0x8953;
pub const SIOCGARP               : c_int = 0x8954;
pub const SIOCSARP               : c_int = 0x8955;
pub const SIOCDRARP              : c_int = 0x8960;
pub const SIOCGRARP              : c_int = 0x8961;
pub const SIOCSRARP              : c_int = 0x8962;
pub const SIOCGIFMAP             : c_int = 0x8970;
pub const SIOCSIFMAP             : c_int = 0x8971;
pub const SIOCADDDLCI            : c_int = 0x8980;
pub const SIOCDELDLCI            : c_int = 0x8981;
pub const SIOCGIFVLAN            : c_int = 0x8982;
pub const SIOCSIFVLAN            : c_int = 0x8983;
pub const SIOCBONDENSLAVE        : c_int = 0x8990;
pub const SIOCBONDRELEASE        : c_int = 0x8991;
pub const SIOCBONDSETHWADDR      : c_int = 0x8992;
pub const SIOCBONDSLAVEINFOQUERY : c_int = 0x8993;
pub const SIOCBONDINFOQUERY      : c_int = 0x8994;
pub const SIOCBONDCHANGEACTIVE   : c_int = 0x8995;
pub const SIOCBRADDBR            : c_int = 0x89a0;
pub const SIOCBRDELBR            : c_int = 0x89a1;
pub const SIOCBRADDIF            : c_int = 0x89a2;
pub const SIOCBRDELIF            : c_int = 0x89a3;
pub const SIOCSHWTSTAMP          : c_int = 0x89b0;
pub const SIOCGHWTSTAMP          : c_int = 0x89b1;
pub const SIOCDEVPRIVATE         : c_int = 0x89F0;
pub const SIOCPROTOPRIVATE       : c_int = 0x89E0;

///////////////////////////
// include/uapi/linux/tcp.h
///////////////////////////

pub const TCP_MSS_DEFAULT          : c_uint = 536;
pub const TCP_MSS_DESIRED          : c_uint = 1220;

pub const TCP_NODELAY              : c_int = 1;
pub const TCP_MAXSEG               : c_int = 2;
pub const TCP_CORK                 : c_int = 3;
pub const TCP_KEEPIDLE             : c_int = 4;
pub const TCP_KEEPINTVL            : c_int = 5;
pub const TCP_KEEPCNT              : c_int = 6;
pub const TCP_SYNCNT               : c_int = 7;
pub const TCP_LINGER2              : c_int = 8;
pub const TCP_DEFER_ACCEPT         : c_int = 9;
pub const TCP_WINDOW_CLAMP         : c_int = 10;
pub const TCP_INFO                 : c_int = 11;
pub const TCP_QUICKACK             : c_int = 12;
pub const TCP_CONGESTION           : c_int = 13;
pub const TCP_MD5SIG               : c_int = 14;
pub const TCP_THIN_LINEAR_TIMEOUTS : c_int = 16;
pub const TCP_THIN_DUPACK          : c_int = 17;
pub const TCP_USER_TIMEOUT         : c_int = 18;
pub const TCP_REPAIR               : c_int = 19;
pub const TCP_REPAIR_QUEUE         : c_int = 20;
pub const TCP_QUEUE_SEQ            : c_int = 21;
pub const TCP_REPAIR_OPTIONS       : c_int = 22;
pub const TCP_FASTOPEN             : c_int = 23;
pub const TCP_TIMESTAMP            : c_int = 24;
pub const TCP_NOTSENT_LOWAT        : c_int = 25;

///////////////////////////
// include/uapi/linux/udp.h
///////////////////////////

#[repr(C)]
#[derive(Pod, Eq)]
pub struct udphdr {
    pub source: __be16,
    pub dst:    __be16,
    pub len:    __be16,
    pub check:  __sum16,
}

pub const UDP_CORK                   : c_int = 1;
pub const UDP_ENCAP                  : c_int = 100;
pub const UDP_NO_CHECK6_TX           : c_int = 101;
pub const UDP_NO_CHECK6_RX           : c_int = 102;
pub const UDP_ENCAP_ESPINUDP_NON_IKE : c_int = 1;
pub const UDP_ENCAP_ESPINUDP         : c_int = 2;
pub const UDP_ENCAP_L2TPINUDP        : c_int = 3;

///////////////////////////////
// include/uapi/linux/netlink.h
///////////////////////////////

pub const NETLINK_ROUTE          : c_int = 0;
pub const NETLINK_UNUSED         : c_int = 1;
pub const NETLINK_USERSOCK       : c_int = 2;
pub const NETLINK_FIREWALL       : c_int = 3;
pub const NETLINK_SOCK_DIAG      : c_int = 4;
pub const NETLINK_NFLOG          : c_int = 5;
pub const NETLINK_XFRM           : c_int = 6;
pub const NETLINK_SELINUX        : c_int = 7;
pub const NETLINK_ISCSI          : c_int = 8;
pub const NETLINK_AUDIT          : c_int = 9;
pub const NETLINK_FIB_LOOKUP     : c_int = 10;
pub const NETLINK_CONNECTOR      : c_int = 11;
pub const NETLINK_NETFILTER      : c_int = 12;
pub const NETLINK_IP6_FW         : c_int = 13;
pub const NETLINK_DNRTMSG        : c_int = 14;
pub const NETLINK_KOBJECT_UEVENT : c_int = 15;
pub const NETLINK_GENERIC        : c_int = 16;
pub const NETLINK_SCSITRANSPORT  : c_int = 18;
pub const NETLINK_ECRYPTFS       : c_int = 19;
pub const NETLINK_RDMA           : c_int = 20;
pub const NETLINK_CRYPTO         : c_int = 21;
pub const NETLINK_INET_DIAG      : c_int = NETLINK_SOCK_DIAG;
pub const MAX_LINKS              : c_int = 32;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct sockaddr_nl {
    pub nl_family: __kernel_sa_family_t,
    pub nl_pad:    c_ushort,
    pub nl_pid:    __u32,
    pub nl_groups: __u32,
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct nlmsghdr {
    pub nlmsg_len:   __u32,
    pub nlmsg_type:  __u16,
    pub nlmsg_flags: __u16,
    pub nlmsg_seq:   __u32,
    pub nlmsg_pid:   __u32,
}

pub const NLM_F_REQUEST   : __u16 = 1;
pub const NLM_F_MULTI     : __u16 = 2;
pub const NLM_F_ACK       : __u16 = 4;
pub const NLM_F_ECHO      : __u16 = 8;
pub const NLM_F_DUMP_INTR : __u16 = 16;
pub const NLM_F_ROOT      : __u16 = 0x100;
pub const NLM_F_MATCH     : __u16 = 0x200;
pub const NLM_F_ATOMIC    : __u16 = 0x400;
pub const NLM_F_DUMP      : __u16 = NLM_F_ROOT|NLM_F_MATCH;
pub const NLM_F_REPLACE   : __u16 = 0x100;
pub const NLM_F_EXCL      : __u16 = 0x200;
pub const NLM_F_CREATE    : __u16 = 0x400;
pub const NLM_F_APPEND    : __u16 = 0x800;

pub const NLMSG_ALIGNTO	: c_uint = 4;

pub fn NLMSG_ALIGN(len: c_uint) -> c_uint {
    (len + NLMSG_ALIGNTO - 1) & !(NLMSG_ALIGNTO - 1)
}

pub fn NLMSG_LENGTH(len: c_uint) -> c_uint {
    len + NLMSG_ALIGN(mem::size_of::<nlmsghdr>() as c_uint)
}

pub fn NLMSG_SPACE(len: c_uint) -> c_uint {
    NLMSG_ALIGN(NLMSG_LENGTH(len))
}

pub unsafe fn NLMSG_DATA(nlh: *mut nlmsghdr) -> *mut c_void {
    (nlh as usize + NLMSG_LENGTH(0) as usize) as *mut c_void
}

pub unsafe fn NLMSG_NEXT(nlh: *mut nlmsghdr, len: &mut c_uint) -> *mut nlmsghdr {
    *len -= NLMSG_ALIGN((*nlh).nlmsg_len);
    (nlh as usize + NLMSG_ALIGN((*nlh).nlmsg_len as c_uint) as usize) as *mut nlmsghdr
}

pub unsafe fn NLMSG_OK(nlh: *mut nlmsghdr, len: c_uint) -> bool {
       len >= mem::size_of::<nlmsghdr>() as c_uint
    && (*nlh).nlmsg_len >= mem::size_of::<nlmsghdr>() as c_uint
    && (*nlh).nlmsg_len <= len
}

pub unsafe fn NLMSG_PAYLOAD(nlh: *mut nlmsghdr, len: c_uint) -> c_uint {
    (*nlh).nlmsg_len as c_uint - NLMSG_SPACE(len)
}

pub const NLMSG_NOOP     : __u16 = 0x1;
pub const NLMSG_ERROR    : __u16 = 0x2;
pub const NLMSG_DONE     : __u16 = 0x3;
pub const NLMSG_OVERRUN  : __u16 = 0x4;
pub const NLMSG_MIN_TYPE : __u16 = 0x10;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct nlmsgerr {
    pub error: c_int,
    pub msg:   nlmsghdr,
}

pub const NETLINK_ADD_MEMBERSHIP  : c_int = 1;
pub const NETLINK_DROP_MEMBERSHIP : c_int = 2;
pub const NETLINK_PKTINFO         : c_int = 3;
pub const NETLINK_BROADCAST_ERROR : c_int = 4;
pub const NETLINK_NO_ENOBUFS      : c_int = 5;
pub const NETLINK_RX_RING         : c_int = 6;
pub const NETLINK_TX_RING         : c_int = 7;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct nl_pktinfo {
    pub group: __u32,
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct nl_mmap_req {
    pub nm_block_size: c_uint,
    pub nm_block_nr:   c_uint,
    pub nm_frame_size: c_uint,
    pub nm_frame_nr:   c_uint,
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct nl_mmap_hdr {
    pub nm_status: c_uint,
    pub nm_len:    c_uint,
    pub nm_group:  __u32,
    pub nm_pid:    __u32,
    pub nm_uid:    __u32,
    pub nm_gid:    __u32,
}

pub type nl_mmap_status = c_int;

pub const NL_MMAP_STATUS_UNUSED:   nl_mmap_status = 0;
pub const NL_MMAP_STATUS_RESERVED: nl_mmap_status = 1;
pub const NL_MMAP_STATUS_VALID:    nl_mmap_status = 2;
pub const NL_MMAP_STATUS_COPY:     nl_mmap_status = 3;
pub const NL_MMAP_STATUS_SKIP:     nl_mmap_status = 4;

pub const NL_MMAP_MSG_ALIGNMENT : c_uint = NLMSG_ALIGNTO;
// #define NL_MMAP_MSG_ALIGN(sz)		__ALIGN_KERNEL(sz, NL_MMAP_MSG_ALIGNMENT)
// #define NL_MMAP_HDRLEN			NL_MMAP_MSG_ALIGN(sizeof(struct nl_mmap_hdr))

pub const NET_MAJOR : c_int = 36;

pub const NETLINK_UNCONNECTED : c_int = 0;
pub const NETLINK_CONNECTED   : c_int = 1;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct nlattr {
    pub nla_len:  __u16,
    pub nla_type: __u16,
}

pub const NLA_F_NESTED        : c_int = 1 << 15;
pub const NLA_F_NET_BYTEORDER : c_int = 1 << 14;
pub const NLA_TYPE_MASK       : c_int = !(NLA_F_NESTED | NLA_F_NET_BYTEORDER);

pub const NLA_ALIGNTO : c_uint = 4;
pub unsafe fn NLA_ALIGN(len: c_uint) -> c_uint {
    (len + NLA_ALIGNTO - 1) & !(NLA_ALIGNTO - 1)
}
// #define NLA_HDRLEN		((int) NLA_ALIGN(sizeof(struct nlattr)))

/////////////////////////////////
// include/uapi/linux/rtnetlink.h
/////////////////////////////////

pub const RTNL_FAMILY_IPMR  : c_int = 128;
pub const RTNL_FAMILY_IP6MR : c_int = 129;
pub const RTNL_FAMILY_MAX   : c_int = 129;

pub const RTM_BASE         : u16 = 16;
pub const RTM_NEWLINK      : u16 = 16;
pub const RTM_DELLINK      : u16 = 17;
pub const RTM_GETLINK      : u16 = 18;
pub const RTM_SETLINK      : u16 = 19;
pub const RTM_NEWADDR      : u16 = 20;
pub const RTM_DELADDR      : u16 = 21;
pub const RTM_GETADDR      : u16 = 22;
pub const RTM_NEWROUTE     : u16 = 24;
pub const RTM_DELROUTE     : u16 = 25;
pub const RTM_GETROUTE     : u16 = 26;
pub const RTM_NEWNEIGH     : u16 = 28;
pub const RTM_DELNEIGH     : u16 = 29;
pub const RTM_GETNEIGH     : u16 = 30;
pub const RTM_NEWRULE      : u16 = 32;
pub const RTM_DELRULE      : u16 = 33;
pub const RTM_GETRULE      : u16 = 34;
pub const RTM_NEWQDISC     : u16 = 36;
pub const RTM_DELQDISC     : u16 = 37;
pub const RTM_GETQDISC     : u16 = 38;
pub const RTM_NEWTCLASS    : u16 = 40;
pub const RTM_DELTCLASS    : u16 = 41;
pub const RTM_GETTCLASS    : u16 = 42;
pub const RTM_NEWTFILTER   : u16 = 44;
pub const RTM_DELTFILTER   : u16 = 45;
pub const RTM_GETTFILTER   : u16 = 46;
pub const RTM_NEWACTION    : u16 = 48;
pub const RTM_DELACTION    : u16 = 49;
pub const RTM_GETACTION    : u16 = 50;
pub const RTM_NEWPREFIX    : u16 = 52;
pub const RTM_GETMULTICAST : u16 = 58;
pub const RTM_GETANYCAST   : u16 = 62;
pub const RTM_NEWNEIGHTBL  : u16 = 64;
pub const RTM_GETNEIGHTBL  : u16 = 66;
pub const RTM_SETNEIGHTBL  : u16 = 67;
pub const RTM_NEWNDUSEROPT : u16 = 68;
pub const RTM_NEWADDRLABEL : u16 = 72;
pub const RTM_DELADDRLABEL : u16 = 73;
pub const RTM_GETADDRLABEL : u16 = 74;
pub const RTM_GETDCB       : u16 = 78;
pub const RTM_SETDCB       : u16 = 79;
pub const RTM_NEWNETCONF   : u16 = 80;
pub const RTM_GETNETCONF   : u16 = 82;
pub const RTM_NEWMDB       : u16 = 84;
pub const RTM_DELMDB       : u16 = 85;
pub const RTM_GETMDB       : u16 = 86;
pub const RTM_NEWNSID      : u16 = 88;
pub const RTM_GETNSID      : u16 = 90;
pub const __RTM_MAX        : u16 = 91;

pub const RTM_MAX : u16 = ((__RTM_MAX + 3) & !3) - 1;

pub const RTM_NR_MSGTYPES : u16 = RTM_MAX + 1 - RTM_BASE;
pub const RTM_NR_FAMILIES : u16 = RTM_NR_MSGTYPES >> 2;
pub fn RTM_FAM(cmd: u16) -> u16 { (((cmd) - RTM_BASE) >> 2) }

#[repr(C)]
#[derive(Pod, Eq)]
pub struct rtattr {
    pub rta_len:  c_ushort,
    pub rta_type: c_ushort,
}

pub const RTA_ALIGNTO : c_uint = 4;

pub fn RTA_ALIGN(len: c_uint) -> c_uint {
    (len + RTA_ALIGNTO - 1) & !(RTA_ALIGNTO - 1)
}

pub unsafe fn RTA_OK(rta: *const rtattr, len: c_uint) -> bool {
       len >= mem::size_of::<rtattr>() as c_uint
    && (*rta).rta_len >= mem::size_of::<rtattr>() as c_ushort
    && (*rta).rta_len as c_uint <= len
}

pub unsafe fn RTA_NEXT(rta: *mut rtattr, attrlen: &mut c_uint) -> *mut rtattr {
    *attrlen -= RTA_ALIGN((*rta).rta_len as c_uint);
    (rta as usize + RTA_ALIGN((*rta).rta_len as c_uint) as usize) as *mut rtattr
}

pub fn RTA_LENGTH(len: c_uint) -> c_uint {
    RTA_ALIGN(mem::size_of::<rtattr>() as c_uint) + len
}

pub fn RTA_SPACE(len: c_uint) -> c_uint {
    RTA_ALIGN(RTA_LENGTH(len))
}

pub unsafe fn RTA_DATA(rta: *mut rtattr) -> *mut c_void {
    (rta as usize + RTA_LENGTH(0) as usize) as *mut c_void
}

pub unsafe fn RTA_PAYLOAD(rta: *mut rtattr) -> c_uint {
    (*rta).rta_len as c_uint - RTA_LENGTH(0)
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct rtmsg {
    pub rtm_family:   c_uchar,
    pub rtm_dst_len:  c_uchar,
    pub rtm_src_len:  c_uchar,
    pub rtm_tos:      c_uchar,
    pub rtm_table:    c_uchar,
    pub rtm_protocol: c_uchar,
    pub rtm_scope:    c_uchar,
    pub rtm_type:     c_uchar,
    pub rtm_flags:    c_uint,
}

pub const RTN_UNSPEC:      c_ushort = 0;
pub const RTN_UNICAST:     c_ushort = 1;
pub const RTN_LOCAL:       c_ushort = 2;
pub const RTN_BROADCAST:   c_ushort = 3;
pub const RTN_ANYCAST:     c_ushort = 4;
pub const RTN_MULTICAST:   c_ushort = 5;
pub const RTN_BLACKHOLE:   c_ushort = 6;
pub const RTN_UNREACHABLE: c_ushort = 7;
pub const RTN_PROHIBIT:    c_ushort = 8;
pub const RTN_THROW:       c_ushort = 9;
pub const RTN_NAT:         c_ushort = 10;
pub const RTN_XRESOLVE:    c_ushort = 11;
pub const __RTN_MAX:       c_ushort = 12;

pub const RTN_MAX : c_ushort = __RTN_MAX - 1;

pub const RTPROT_UNSPEC   : c_uchar = 0;
pub const RTPROT_REDIRECT : c_uchar = 1;
pub const RTPROT_KERNEL   : c_uchar = 2;
pub const RTPROT_BOOT     : c_uchar = 3;
pub const RTPROT_STATIC   : c_uchar = 4;
pub const RTPROT_GATED    : c_uchar = 8;
pub const RTPROT_RA       : c_uchar = 9;
pub const RTPROT_MRT      : c_uchar = 10;
pub const RTPROT_ZEBRA    : c_uchar = 11;
pub const RTPROT_BIRD     : c_uchar = 12;
pub const RTPROT_DNROUTED : c_uchar = 13;
pub const RTPROT_XORP     : c_uchar = 14;
pub const RTPROT_NTK      : c_uchar = 15;
pub const RTPROT_DHCP     : c_uchar = 16;
pub const RTPROT_MROUTED  : c_uchar = 17;
pub const RTPROT_BABEL    : c_uchar = 42;

pub type rt_scope_t = c_uchar;
pub const RT_SCOPE_UNIVERSE : rt_scope_t = 0;
pub const RT_SCOPE_SITE     : rt_scope_t = 200;
pub const RT_SCOPE_LINK     : rt_scope_t = 253;
pub const RT_SCOPE_HOST     : rt_scope_t = 254;
pub const RT_SCOPE_NOWHERE  : rt_scope_t = 255;

pub const RTM_F_NOTIFY   : c_uint = 0x100;
pub const RTM_F_CLONED   : c_uint = 0x200;
pub const RTM_F_EQUALIZE : c_uint = 0x400;
pub const RTM_F_PREFIX   : c_uint = 0x800;

pub type rt_class_t = c_uchar;
pub const RT_TABLE_UNSPEC  : rt_class_t = 0;
pub const RT_TABLE_COMPAT  : rt_class_t = 252;
pub const RT_TABLE_DEFAULT : rt_class_t = 253;
pub const RT_TABLE_MAIN    : rt_class_t = 254;
pub const RT_TABLE_LOCAL   : rt_class_t = 255;
pub const RT_TABLE_MAX     : rt_class_t = 0xFFFFFFFF;

pub type rtattr_type_t = c_int;
pub const RTA_UNSPEC:    rtattr_type_t = 0;
pub const RTA_DST:       rtattr_type_t = 1;
pub const RTA_SRC:       rtattr_type_t = 2;
pub const RTA_IIF:       rtattr_type_t = 3;
pub const RTA_OIF:       rtattr_type_t = 4;
pub const RTA_GATEWAY:   rtattr_type_t = 5;
pub const RTA_PRIORITY:  rtattr_type_t = 6;
pub const RTA_PREFSRC:   rtattr_type_t = 7;
pub const RTA_METRICS:   rtattr_type_t = 8;
pub const RTA_MULTIPATH: rtattr_type_t = 9;
pub const RTA_PROTOINFO: rtattr_type_t = 10;
pub const RTA_FLOW:      rtattr_type_t = 11;
pub const RTA_CACHEINFO: rtattr_type_t = 12;
pub const RTA_SESSION:   rtattr_type_t = 13;
pub const RTA_MP_ALGO:   rtattr_type_t = 14;
pub const RTA_TABLE:     rtattr_type_t = 15;
pub const RTA_MARK:      rtattr_type_t = 16;
pub const RTA_MFC_STATS: rtattr_type_t = 17;
pub const __RTA_MAX:     rtattr_type_t = 18;

pub const RTA_MAX : rtattr_type_t = __RTA_MAX - 1;

// pub unsafe fn RTM_RTA(r)  ((struct rtattr*)(((char*)(r)) + NLMSG_ALIGN(sizeof(struct rtmsg))))
// pub unsafe fn RTM_PAYLOAD(n) NLMSG_PAYLOAD(n,sizeof(struct rtmsg))

#[repr(C)]
#[derive(Pod, Eq)]
pub struct rtnexthop {
    pub rtnh_len:     c_ushort,
    pub rtnh_flags:   c_uchar,
    pub rtnh_hops:    c_uchar,
    pub rtnh_ifindex: c_int,
}

pub const RTNH_F_DEAD      : c_uchar = 1;
pub const RTNH_F_PERVASIVE : c_uchar = 2;
pub const RTNH_F_ONLINK    : c_uchar = 4;

pub const RTNH_ALIGNTO : c_uint = 4;

pub fn RTNH_ALIGN(len: c_uint) -> c_uint {
    (len + RTNH_ALIGNTO - 1) & !(RTNH_ALIGNTO-1)
}

pub unsafe fn RTNH_OK(rtnh: *mut rtnexthop, len: c_uint) -> bool {
       (*rtnh).rtnh_len >= mem::size_of::<rtnexthop>() as c_ushort
    && (*rtnh).rtnh_len as c_uint <= len
}

pub unsafe fn RTNH_NEXT(rtnh: *mut rtnexthop) -> *mut rtnexthop {
    (rtnh as usize + RTNH_ALIGN((*rtnh).rtnh_len as c_uint) as usize) as *mut rtnexthop
}

pub fn RTNH_LENGTH(len: c_uint) -> c_uint {
    RTNH_ALIGN(mem::size_of::<rtnexthop>() as c_uint) + len
}

pub fn RTNH_SPACE(len: c_uint) -> c_uint {
    RTNH_ALIGN(RTNH_LENGTH(len))
}

pub unsafe fn RTNH_DATA(rtnh: *mut rtnexthop) -> *mut rtattr {
    (rtnh as usize + RTNH_LENGTH(0) as usize) as *mut rtattr
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct rta_cacheinfo {
    pub rta_clntref: __u32,
    pub rta_lastuse: __u32,
    pub rta_expires: __s32,
    pub rta_error:   __u32,
    pub rta_used:    __u32,
    pub rta_id:      __u32,
    pub rta_ts:      __u32,
    pub rta_tsage:   __u32,
}

pub const RTNETLINK_HAVE_PEERINFO: bool = true;

pub const RTAX_UNSPEC     : c_int = 0;
pub const RTAX_LOCK       : c_int = 1;
pub const RTAX_MTU        : c_int = 2;
pub const RTAX_WINDOW     : c_int = 3;
pub const RTAX_RTT        : c_int = 4;
pub const RTAX_RTTVAR     : c_int = 5;
pub const RTAX_SSTHRESH   : c_int = 6;
pub const RTAX_CWND       : c_int = 7;
pub const RTAX_ADVMSS     : c_int = 8;
pub const RTAX_REORDERING : c_int = 9;
pub const RTAX_HOPLIMIT   : c_int = 10;
pub const RTAX_INITCWND   : c_int = 11;
pub const RTAX_FEATURES   : c_int = 12;
pub const RTAX_RTO_MIN    : c_int = 13;
pub const RTAX_INITRWND   : c_int = 14;
pub const RTAX_QUICKACK   : c_int = 15;
pub const RTAX_CC_ALGO    : c_int = 16;
pub const __RTAX_MAX      : c_int = 17;

pub const RTAX_MAX : c_int = __RTAX_MAX - 1;

pub const RTAX_FEATURE_ECN       : c_int = 0x00000001;
pub const RTAX_FEATURE_SACK      : c_int = 0x00000002;
pub const RTAX_FEATURE_TIMESTAMP : c_int = 0x00000004;
pub const RTAX_FEATURE_ALLFRAG   : c_int = 0x00000008;

// #[repr(C)]
// #[derive(Pod, Eq)]
// pub struct rta_session {
// 	__u8	proto;
// 	__u8	pad1;
// 	__u16	pad2;
//
// 	union {
// 		struct {
// 			__u16	sport;
// 			__u16	dport;
// 		} ports;
//
// 		struct {
// 			__u8	type;
// 			__u8	code;
// 			__u16	ident;
// 		} icmpt;
//
// 		__u32		spi;
// 	} u;
// }

#[repr(C)]
#[derive(Pod, Eq)]
pub struct rta_mfc_stats {
    pub mfcs_packets:  __u64,
    pub mfcs_bytes:    __u64,
    pub mfcs_wrong_if: __u64,
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct rtgenmsg {
    pub rtgen_family: c_uchar,
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct ifinfomsg {
    pub ifi_family: c_uchar,
    pub __ifi_pad:  c_uchar,
    pub ifi_type:   c_ushort,
    pub ifi_index:  c_int,
    pub ifi_flags:  c_uint,
    pub ifi_change: c_uint,
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct prefixmsg {
    pub prefix_family:  c_uchar,
    pub prefix_pad1:    c_uchar,
    pub prefix_pad2:    c_ushort,
    pub prefix_ifindex: c_int,
    pub prefix_type:    c_uchar,
    pub prefix_len:     c_uchar,
    pub prefix_flags:   c_uchar,
    pub prefix_pad3:    c_uchar,
}

pub const PREFIX_UNSPEC    : c_int = 0;
pub const PREFIX_ADDRESS   : c_int = 1;
pub const PREFIX_CACHEINFO : c_int = 2;
pub const __PREFIX_MAX     : c_int = 3;

pub const PREFIX_MAX : c_int = __PREFIX_MAX - 1;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct prefix_cacheinfo {
    pub preferred_time: __u32,
    pub valid_time:     __u32,
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct tcmsg {
    pub tcm_family:  c_uchar,
    pub tcm__pad1:   c_uchar,
    pub tcm__pad2:   c_ushort,
    pub tcm_ifindex: c_int,
    pub tcm_handle:  __u32,
    pub tcm_parent:  __u32,
    pub tcm_info:    __u32,
}

pub const TCA_UNSPEC  : c_int = 0;
pub const TCA_KIND    : c_int = 1;
pub const TCA_OPTIONS : c_int = 2;
pub const TCA_STATS   : c_int = 3;
pub const TCA_XSTATS  : c_int = 4;
pub const TCA_RATE    : c_int = 5;
pub const TCA_FCNT    : c_int = 6;
pub const TCA_STATS2  : c_int = 7;
pub const TCA_STAB    : c_int = 8;
pub const __TCA_MAX   : c_int = 9;

pub const TCA_MAX : c_int = __TCA_MAX - 1;

pub unsafe fn TCA_RTA(r: *mut tcmsg) -> *mut rtattr {
    (r as usize + NLMSG_ALIGN(mem::size_of::<tcmsg>() as c_uint) as usize) as *mut rtattr
}

pub unsafe fn TCA_PAYLOAD(n: *mut nlmsghdr) -> c_uint {
    NLMSG_PAYLOAD(n, mem::size_of::<tcmsg>() as c_uint)
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct nduseroptmsg {
    pub nduseropt_family:    c_uchar,
    pub nduseropt_pad1:      c_uchar,
    pub nduseropt_opts_len:  c_ushort,
    pub nduseropt_ifindex:   c_int,
    pub nduseropt_icmp_type: __u8,
    pub nduseropt_icmp_code: __u8,
    pub nduseropt_pad2:      c_ushort,
    pub nduseropt_pad3:      c_uint,
}

pub const NDUSEROPT_UNSPEC  : c_int = 0;
pub const NDUSEROPT_SRCADDR : c_int = 1;
pub const __NDUSEROPT_MAX   : c_int = 2;

pub const NDUSEROPT_MAX	: c_int = __NDUSEROPT_MAX - 1;

pub const RTMGRP_LINK          : c_int = 1;
pub const RTMGRP_NOTIFY        : c_int = 2;
pub const RTMGRP_NEIGH         : c_int = 4;
pub const RTMGRP_TC            : c_int = 8;
pub const RTMGRP_IPV4_IFADDR   : c_int = 0x10;
pub const RTMGRP_IPV4_MROUTE   : c_int = 0x20;
pub const RTMGRP_IPV4_ROUTE    : c_int = 0x40;
pub const RTMGRP_IPV4_RULE     : c_int = 0x80;
pub const RTMGRP_IPV6_IFADDR   : c_int = 0x100;
pub const RTMGRP_IPV6_MROUTE   : c_int = 0x200;
pub const RTMGRP_IPV6_ROUTE    : c_int = 0x400;
pub const RTMGRP_IPV6_IFINFO   : c_int = 0x800;
pub const RTMGRP_DECnet_IFADDR : c_int = 0x1000;
pub const RTMGRP_DECnet_ROUTE  : c_int = 0x4000;
pub const RTMGRP_IPV6_PREFIX   : c_int = 0x20000;

pub type rtnetlink_groups = c_int;
pub const RTNLGRP_NONE          : rtnetlink_groups = 0;
pub const RTNLGRP_LINK          : rtnetlink_groups = 1;
pub const RTNLGRP_NOTIFY        : rtnetlink_groups = 2;
pub const RTNLGRP_NEIGH         : rtnetlink_groups = 3;
pub const RTNLGRP_TC            : rtnetlink_groups = 4;
pub const RTNLGRP_IPV4_IFADDR   : rtnetlink_groups = 5;
pub const RTNLGRP_IPV4_MROUTE   : rtnetlink_groups = 6;
pub const RTNLGRP_IPV4_ROUTE    : rtnetlink_groups = 7;
pub const RTNLGRP_IPV4_RULE     : rtnetlink_groups = 8;
pub const RTNLGRP_IPV6_IFADDR   : rtnetlink_groups = 9;
pub const RTNLGRP_IPV6_MROUTE   : rtnetlink_groups = 10;
pub const RTNLGRP_IPV6_ROUTE    : rtnetlink_groups = 11;
pub const RTNLGRP_IPV6_IFINFO   : rtnetlink_groups = 12;
pub const RTNLGRP_DECnet_IFADDR : rtnetlink_groups = 13;
pub const RTNLGRP_NOP2          : rtnetlink_groups = 14;
pub const RTNLGRP_DECnet_ROUTE  : rtnetlink_groups = 15;
pub const RTNLGRP_DECnet_RULE   : rtnetlink_groups = 16;
pub const RTNLGRP_NOP4          : rtnetlink_groups = 17;
pub const RTNLGRP_IPV6_PREFIX   : rtnetlink_groups = 18;
pub const RTNLGRP_IPV6_RULE     : rtnetlink_groups = 19;
pub const RTNLGRP_ND_USEROPT    : rtnetlink_groups = 20;
pub const RTNLGRP_PHONET_IFADDR : rtnetlink_groups = 21;
pub const RTNLGRP_PHONET_ROUTE  : rtnetlink_groups = 22;
pub const RTNLGRP_DCB           : rtnetlink_groups = 23;
pub const RTNLGRP_IPV4_NETCONF  : rtnetlink_groups = 24;
pub const RTNLGRP_IPV6_NETCONF  : rtnetlink_groups = 25;
pub const RTNLGRP_MDB           : rtnetlink_groups = 26;
pub const __RTNLGRP_MAX         : rtnetlink_groups = 27;

pub const RTNLGRP_MAX : rtnetlink_groups = __RTNLGRP_MAX - 1;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct tcamsg {
    pub tca_family: c_uchar,
    pub tca__pad1:  c_uchar,
    pub tca__pad2:  c_ushort,
}

pub unsafe fn TA_RTA(r: *mut tcamsg) -> *mut rtattr {
    (r as usize + NLMSG_ALIGN(mem::size_of::<tcamsg>() as c_uint) as usize) as *mut rtattr
}

pub unsafe fn TA_PAYLOAD(n: *mut nlmsghdr) -> c_uint {
    NLMSG_PAYLOAD(n, mem::size_of::<tcamsg>() as c_uint)
}
pub const TCA_ACT_TAB : c_int = 1;
pub const TCAA_MAX : c_int = 1;

pub const RTEXT_FILTER_VF                : c_int = 1 << 0;
pub const RTEXT_FILTER_BRVLAN            : c_int = 1 << 1;
pub const RTEXT_FILTER_BRVLAN_COMPRESSED : c_int = 1 << 2;

///////////////////////////////
// include/uapi/linux/if_addr.h
///////////////////////////////

#[repr(C)]
#[derive(Pod, Eq)]
pub struct ifaddrmsg {
    pub ifa_family:    __u8,
    pub ifa_prefixlen: __u8,
    pub ifa_flags:     __u8,
    pub ifa_scope:     __u8,
    pub ifa_index:     __u32,
}

pub const IFA_UNSPEC    : c_ushort = 0;
pub const IFA_ADDRESS   : c_ushort = 1;
pub const IFA_LOCAL     : c_ushort = 2;
pub const IFA_LABEL     : c_ushort = 3;
pub const IFA_BROADCAST : c_ushort = 4;
pub const IFA_ANYCAST   : c_ushort = 5;
pub const IFA_CACHEINFO : c_ushort = 6;
pub const IFA_MULTICAST : c_ushort = 7;
pub const IFA_FLAGS     : c_ushort = 8;
pub const __IFA_MAX     : c_ushort = 9;

pub const IFA_MAX : c_ushort = __IFA_MAX - 1;

pub const IFA_F_SECONDARY      : c_int = 0x01;
pub const IFA_F_TEMPORARY      : c_int = IFA_F_SECONDARY;
pub const IFA_F_NODAD          : c_int = 0x02;
pub const IFA_F_OPTIMISTIC     : c_int = 0x04;
pub const IFA_F_DADFAILED      : c_int = 0x08;
pub const IFA_F_HOMEADDRESS    : c_int = 0x10;
pub const IFA_F_DEPRECATED     : c_int = 0x20;
pub const IFA_F_TENTATIVE      : c_int = 0x40;
pub const IFA_F_PERMANENT      : c_int = 0x80;
pub const IFA_F_MANAGETEMPADDR : c_int = 0x100;
pub const IFA_F_NOPREFIXROUTE  : c_int = 0x200;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct ifa_cacheinfo {
    pub ifa_prefered: __u32,
    pub ifa_valid:    __u32,
    pub cstamp:       __u32,
    pub tstamp:       __u32,
}

///////////////////////////////
// include/uapi/linux/if_link.h
///////////////////////////////

#[repr(C)]
#[derive(Pod, Eq)]
pub struct rtnl_link_stats {
    pub rx_packets:          __u32,
    pub tx_packets:          __u32,
    pub rx_bytes:            __u32,
    pub tx_bytes:            __u32,
    pub rx_errors:           __u32,
    pub tx_errors:           __u32,
    pub rx_dropped:          __u32,
    pub tx_dropped:          __u32,
    pub multicast:           __u32,
    pub collisions:          __u32,
    pub rx_length_errors:    __u32,
    pub rx_over_errors:      __u32,
    pub rx_crc_errors:       __u32,
    pub rx_frame_errors:     __u32,
    pub rx_fifo_errors:      __u32,
    pub rx_missed_errors:    __u32,
    pub tx_aborted_errors:   __u32,
    pub tx_carrier_errors:   __u32,
    pub tx_fifo_errors:      __u32,
    pub tx_heartbeat_errors: __u32,
    pub tx_window_errors:    __u32,
    pub rx_compressed:       __u32,
    pub tx_compressed:       __u32,
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct rtnl_link_stats64 {
    pub rx_packets:          __u64,
    pub tx_packets:          __u64,
    pub rx_bytes:            __u64,
    pub tx_bytes:            __u64,
    pub rx_errors:           __u64,
    pub tx_errors:           __u64,
    pub rx_dropped:          __u64,
    pub tx_dropped:          __u64,
    pub multicast:           __u64,
    pub collisions:          __u64,
    pub rx_length_errors:    __u64,
    pub rx_over_errors:      __u64,
    pub rx_crc_errors:       __u64,
    pub rx_frame_errors:     __u64,
    pub rx_fifo_errors:      __u64,
    pub rx_missed_errors:    __u64,
    pub tx_aborted_errors:   __u64,
    pub tx_carrier_errors:   __u64,
    pub tx_fifo_errors:      __u64,
    pub tx_heartbeat_errors: __u64,
    pub tx_window_errors:    __u64,
    pub rx_compressed:       __u64,
    pub tx_compressed:       __u64,
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct rtnl_link_ifmap {
    pub mem_start: __u64,
    pub mem_end:   __u64,
    pub base_addr: __u64,
    pub irq:       __u16,
    pub dma:       __u8,
    pub port:      __u8,
}

pub const IFLA_UNSPEC          : u16 = 0;
pub const IFLA_ADDRESS         : u16 = 1;
pub const IFLA_BROADCAST       : u16 = 2;
pub const IFLA_IFNAME          : u16 = 3;
pub const IFLA_MTU             : u16 = 4;
pub const IFLA_LINK            : u16 = 5;
pub const IFLA_QDISC           : u16 = 6;
pub const IFLA_STATS           : u16 = 7;
pub const IFLA_COST            : u16 = 8;
pub const IFLA_PRIORITY        : u16 = 9;
pub const IFLA_MASTER          : u16 = 10;
pub const IFLA_WIRELESS        : u16 = 11;
pub const IFLA_PROTINFO        : u16 = 12;
pub const IFLA_TXQLEN          : u16 = 13;
pub const IFLA_MAP             : u16 = 14;
pub const IFLA_WEIGHT          : u16 = 15;
pub const IFLA_OPERSTATE       : u16 = 16;
pub const IFLA_LINKMODE        : u16 = 17;
pub const IFLA_LINKINFO        : u16 = 18;
pub const IFLA_NET_NS_PID      : u16 = 19;
pub const IFLA_IFALIAS         : u16 = 20;
pub const IFLA_NUM_VF          : u16 = 21;
pub const IFLA_VFINFO_LIST     : u16 = 22;
pub const IFLA_STATS64         : u16 = 23;
pub const IFLA_VF_PORTS        : u16 = 24;
pub const IFLA_PORT_SELF       : u16 = 25;
pub const IFLA_AF_SPEC         : u16 = 26;
pub const IFLA_GROUP           : u16 = 27;
pub const IFLA_NET_NS_FD       : u16 = 28;
pub const IFLA_EXT_MASK        : u16 = 29;
pub const IFLA_PROMISCUITY     : u16 = 30;
pub const IFLA_NUM_TX_QUEUES   : u16 = 31;
pub const IFLA_NUM_RX_QUEUES   : u16 = 32;
pub const IFLA_CARRIER         : u16 = 33;
pub const IFLA_PHYS_PORT_ID    : u16 = 34;
pub const IFLA_CARRIER_CHANGES : u16 = 35;
pub const IFLA_PHYS_SWITCH_ID  : u16 = 36;
pub const IFLA_LINK_NETNSID    : u16 = 37;
pub const __IFLA_MAX           : u16 = 38;

pub const IFLA_MAX : u16 = __IFLA_MAX - 1;

pub const IFLA_INET_UNSPEC : u16 = 0;
pub const IFLA_INET_CONF   : u16 = 1;
pub const __IFLA_INET_MAX  : u16 = 2;

pub const IFLA_INET_MAX : u16 = __IFLA_INET_MAX - 1;

pub const IFLA_INET6_UNSPEC        : u16 = 0;
pub const IFLA_INET6_FLAGS         : u16 = 1;
pub const IFLA_INET6_CONF          : u16 = 2;
pub const IFLA_INET6_STATS         : u16 = 3;
pub const IFLA_INET6_MCAST         : u16 = 4;
pub const IFLA_INET6_CACHEINFO     : u16 = 5;
pub const IFLA_INET6_ICMP6STATS    : u16 = 6;
pub const IFLA_INET6_TOKEN         : u16 = 7;
pub const IFLA_INET6_ADDR_GEN_MODE : u16 = 8;
pub const __IFLA_INET6_MAX         : u16 = 9;

pub const IFLA_INET6_MAX : u16 = __IFLA_INET6_MAX - 1;

pub type in6_addr_gen_mode = c_int;
pub const IN6_ADDR_GEN_MODE_EUI64 : in6_addr_gen_mode = 0;
pub const IN6_ADDR_GEN_MODE_NONE  : in6_addr_gen_mode = 1;

pub const IFLA_BR_UNSPEC        : c_int = 0;
pub const IFLA_BR_FORWARD_DELAY : c_int = 1;
pub const IFLA_BR_HELLO_TIME    : c_int = 2;
pub const IFLA_BR_MAX_AGE       : c_int = 3;
pub const __IFLA_BR_MAX         : c_int = 4;

pub const IFLA_BR_MAX : c_int = __IFLA_BR_MAX - 1;

pub const BRIDGE_MODE_UNSPEC  : c_int = 0;
pub const BRIDGE_MODE_HAIRPIN : c_int = 1;

pub const IFLA_BRPORT_UNSPEC        : c_int = 0;
pub const IFLA_BRPORT_STATE         : c_int = 1;
pub const IFLA_BRPORT_PRIORITY      : c_int = 2;
pub const IFLA_BRPORT_COST          : c_int = 3;
pub const IFLA_BRPORT_MODE          : c_int = 4;
pub const IFLA_BRPORT_GUARD         : c_int = 5;
pub const IFLA_BRPORT_PROTECT       : c_int = 6;
pub const IFLA_BRPORT_FAST_LEAVE    : c_int = 7;
pub const IFLA_BRPORT_LEARNING      : c_int = 8;
pub const IFLA_BRPORT_UNICAST_FLOOD : c_int = 9;
pub const IFLA_BRPORT_PROXYARP      : c_int = 10;
pub const IFLA_BRPORT_LEARNING_SYNC : c_int = 11;
pub const __IFLA_BRPORT_MAX         : c_int = 12;

pub const IFLA_BRPORT_MAX : c_int = __IFLA_BRPORT_MAX - 1;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct ifla_cacheinfo {
    pub max_reasm_len:  __u32,
    pub tstamp:         __u32,
    pub reachable_time: __u32,
    pub retrans_time:   __u32,
}

pub const IFLA_INFO_UNSPEC     : u16 = 0;
pub const IFLA_INFO_KIND       : u16 = 1;
pub const IFLA_INFO_DATA       : u16 = 2;
pub const IFLA_INFO_XSTATS     : u16 = 3;
pub const IFLA_INFO_SLAVE_KIND : u16 = 4;
pub const IFLA_INFO_SLAVE_DATA : u16 = 5;
pub const __IFLA_INFO_MAX      : u16 = 6;

pub const IFLA_INFO_MAX : u16 = __IFLA_INFO_MAX - 1;

pub const IFLA_VLAN_UNSPEC      : u16 = 0;
pub const IFLA_VLAN_ID          : u16 = 1;
pub const IFLA_VLAN_FLAGS       : u16 = 2;
pub const IFLA_VLAN_EGRESS_QOS  : u16 = 3;
pub const IFLA_VLAN_INGRESS_QOS : u16 = 4;
pub const IFLA_VLAN_PROTOCOL    : u16 = 5;
pub const __IFLA_VLAN_MAX       : u16 = 6;

pub const IFLA_VLAN_MAX : u16 = __IFLA_VLAN_MAX - 1;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct ifla_vlan_flags {
    pub flags: __u32,
    pub mask:  __u32,
}

pub const IFLA_VLAN_QOS_UNSPEC  : c_int = 0;
pub const IFLA_VLAN_QOS_MAPPING : c_int = 1;
pub const __IFLA_VLAN_QOS_MAX   : c_int = 2;

pub const IFLA_VLAN_QOS_MAX : c_int = __IFLA_VLAN_QOS_MAX - 1;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct ifla_vlan_qos_mapping {
    pub from: __u32,
    pub to:   __u32,
}

pub const IFLA_MACVLAN_UNSPEC        : c_int = 0;
pub const IFLA_MACVLAN_MODE          : c_int = 1;
pub const IFLA_MACVLAN_FLAGS         : c_int = 2;
pub const IFLA_MACVLAN_MACADDR_MODE  : c_int = 3;
pub const IFLA_MACVLAN_MACADDR       : c_int = 4;
pub const IFLA_MACVLAN_MACADDR_DATA  : c_int = 5;
pub const IFLA_MACVLAN_MACADDR_COUNT : c_int = 6;
pub const __IFLA_MACVLAN_MAX         : c_int = 7;

pub const IFLA_MACVLAN_MAX : c_int = __IFLA_MACVLAN_MAX - 1;

pub type macvlan_mode = c_int;
pub const MACVLAN_MODE_PRIVATE  : macvlan_mode = 1;
pub const MACVLAN_MODE_VEPA     : macvlan_mode = 2;
pub const MACVLAN_MODE_BRIDGE   : macvlan_mode = 4;
pub const MACVLAN_MODE_PASSTHRU : macvlan_mode = 8;
pub const MACVLAN_MODE_SOURCE   : macvlan_mode = 16;

pub type macvlan_macaddr_mode = c_int;
pub const MACVLAN_MACADDR_ADD   : macvlan_macaddr_mode = 0;
pub const MACVLAN_MACADDR_DEL   : macvlan_macaddr_mode = 1;
pub const MACVLAN_MACADDR_FLUSH : macvlan_macaddr_mode = 2;
pub const MACVLAN_MACADDR_SET   : macvlan_macaddr_mode = 3;

pub const MACVLAN_FLAG_NOPROMISC : macvlan_macaddr_mode = 1;

pub const IFLA_IPVLAN_UNSPEC : c_int = 0;
pub const IFLA_IPVLAN_MODE   : c_int = 1;
pub const __IFLA_IPVLAN_MAX  : c_int = 2;

pub const IFLA_IPVLAN_MAX : c_int = __IFLA_IPVLAN_MAX - 1;

pub type ipvlan_mode = c_int;
pub const IPVLAN_MODE_L2  : ipvlan_mode = 0;
pub const IPVLAN_MODE_L3  : ipvlan_mode = 1;
pub const IPVLAN_MODE_MAX : ipvlan_mode = 2;

pub const IFLA_VXLAN_UNSPEC            : c_int = 0;
pub const IFLA_VXLAN_ID                : c_int = 1;
pub const IFLA_VXLAN_GROUP             : c_int = 2;
pub const IFLA_VXLAN_LINK              : c_int = 3;
pub const IFLA_VXLAN_LOCAL             : c_int = 4;
pub const IFLA_VXLAN_TTL               : c_int = 5;
pub const IFLA_VXLAN_TOS               : c_int = 6;
pub const IFLA_VXLAN_LEARNING          : c_int = 7;
pub const IFLA_VXLAN_AGEING            : c_int = 8;
pub const IFLA_VXLAN_LIMIT             : c_int = 9;
pub const IFLA_VXLAN_PORT_RANGE        : c_int = 10;
pub const IFLA_VXLAN_PROXY             : c_int = 11;
pub const IFLA_VXLAN_RSC               : c_int = 12;
pub const IFLA_VXLAN_L2MISS            : c_int = 13;
pub const IFLA_VXLAN_L3MISS            : c_int = 14;
pub const IFLA_VXLAN_PORT              : c_int = 15;
pub const IFLA_VXLAN_GROUP6            : c_int = 16;
pub const IFLA_VXLAN_LOCAL6            : c_int = 17;
pub const IFLA_VXLAN_UDP_CSUM          : c_int = 18;
pub const IFLA_VXLAN_UDP_ZERO_CSUM6_TX : c_int = 19;
pub const IFLA_VXLAN_UDP_ZERO_CSUM6_RX : c_int = 20;
pub const IFLA_VXLAN_REMCSUM_TX        : c_int = 21;
pub const IFLA_VXLAN_REMCSUM_RX        : c_int = 22;
pub const IFLA_VXLAN_GBP               : c_int = 23;
pub const IFLA_VXLAN_REMCSUM_NOPARTIAL : c_int = 24;
pub const __IFLA_VXLAN_MAX             : c_int = 25;

pub const IFLA_VXLAN_MAX : c_int = __IFLA_VXLAN_MAX - 1;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct ifla_vxlan_port_range {
    pub low:  __be16,
    pub high: __be16,
}

pub const IFLA_BOND_UNSPEC            : c_int = 0;
pub const IFLA_BOND_MODE              : c_int = 1;
pub const IFLA_BOND_ACTIVE_SLAVE      : c_int = 2;
pub const IFLA_BOND_MIIMON            : c_int = 3;
pub const IFLA_BOND_UPDELAY           : c_int = 4;
pub const IFLA_BOND_DOWNDELAY         : c_int = 5;
pub const IFLA_BOND_USE_CARRIER       : c_int = 6;
pub const IFLA_BOND_ARP_INTERVAL      : c_int = 7;
pub const IFLA_BOND_ARP_IP_TARGET     : c_int = 8;
pub const IFLA_BOND_ARP_VALIDATE      : c_int = 9;
pub const IFLA_BOND_ARP_ALL_TARGETS   : c_int = 10;
pub const IFLA_BOND_PRIMARY           : c_int = 11;
pub const IFLA_BOND_PRIMARY_RESELECT  : c_int = 12;
pub const IFLA_BOND_FAIL_OVER_MAC     : c_int = 13;
pub const IFLA_BOND_XMIT_HASH_POLICY  : c_int = 14;
pub const IFLA_BOND_RESEND_IGMP       : c_int = 15;
pub const IFLA_BOND_NUM_PEER_NOTIF    : c_int = 16;
pub const IFLA_BOND_ALL_SLAVES_ACTIVE : c_int = 17;
pub const IFLA_BOND_MIN_LINKS         : c_int = 18;
pub const IFLA_BOND_LP_INTERVAL       : c_int = 19;
pub const IFLA_BOND_PACKETS_PER_SLAVE : c_int = 20;
pub const IFLA_BOND_AD_LACP_RATE      : c_int = 21;
pub const IFLA_BOND_AD_SELECT         : c_int = 22;
pub const IFLA_BOND_AD_INFO           : c_int = 23;
pub const __IFLA_BOND_MAX             : c_int = 24;

pub const IFLA_BOND_MAX : c_int = __IFLA_BOND_MAX - 1;

pub const IFLA_BOND_AD_INFO_UNSPEC      : c_int = 0;
pub const IFLA_BOND_AD_INFO_AGGREGATOR  : c_int = 1;
pub const IFLA_BOND_AD_INFO_NUM_PORTS   : c_int = 2;
pub const IFLA_BOND_AD_INFO_ACTOR_KEY   : c_int = 3;
pub const IFLA_BOND_AD_INFO_PARTNER_KEY : c_int = 4;
pub const IFLA_BOND_AD_INFO_PARTNER_MAC : c_int = 5;
pub const __IFLA_BOND_AD_INFO_MAX       : c_int = 6;

pub const IFLA_BOND_AD_INFO_MAX : c_int = __IFLA_BOND_AD_INFO_MAX - 1;

pub const IFLA_BOND_SLAVE_UNSPEC             : c_int = 0;
pub const IFLA_BOND_SLAVE_STATE              : c_int = 1;
pub const IFLA_BOND_SLAVE_MII_STATUS         : c_int = 2;
pub const IFLA_BOND_SLAVE_LINK_FAILURE_COUNT : c_int = 3;
pub const IFLA_BOND_SLAVE_PERM_HWADDR        : c_int = 4;
pub const IFLA_BOND_SLAVE_QUEUE_ID           : c_int = 5;
pub const IFLA_BOND_SLAVE_AD_AGGREGATOR_ID   : c_int = 6;
pub const __IFLA_BOND_SLAVE_MAX              : c_int = 7;

pub const IFLA_BOND_SLAVE_MAX : c_int = __IFLA_BOND_SLAVE_MAX - 1;

pub const IFLA_VF_INFO_UNSPEC : c_int = 0;
pub const IFLA_VF_INFO        : c_int = 1;
pub const __IFLA_VF_INFO_MAX  : c_int = 2;

pub const IFLA_VF_INFO_MAX : c_int = __IFLA_VF_INFO_MAX - 1;

pub const IFLA_VF_UNSPEC     : c_int = 0;
pub const IFLA_VF_MAC        : c_int = 1;
pub const IFLA_VF_VLAN       : c_int = 2;
pub const IFLA_VF_TX_RATE    : c_int = 3;
pub const IFLA_VF_SPOOFCHK   : c_int = 4;
pub const IFLA_VF_LINK_STATE : c_int = 5;
pub const IFLA_VF_RATE       : c_int = 6;
pub const __IFLA_VF_MAX      : c_int = 7;

pub const IFLA_VF_MAX : c_int = __IFLA_VF_MAX - 1;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct ifla_vf_mac {
    pub vf: __u32,
    pub mac: [__u8; 32],
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct ifla_vf_vlan {
    pub vf:   __u32,
    pub vlan: __u32,
    pub qos:  __u32,
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct ifla_vf_tx_rate {
    pub vf:   __u32,
    pub rate: __u32,
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct ifla_vf_rate {
    pub vf:          __u32,
    pub min_tx_rate: __u32,
    pub max_tx_rate: __u32,
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct ifla_vf_spoofchk {
    pub vf:      __u32,
    pub setting: __u32,
}

pub const IFLA_VF_LINK_STATE_AUTO    : c_int = 0;
pub const IFLA_VF_LINK_STATE_ENABLE  : c_int = 1;
pub const IFLA_VF_LINK_STATE_DISABLE : c_int = 2;
pub const __IFLA_VF_LINK_STATE_MAX   : c_int = 3;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct ifla_vf_link_state {
    pub vf:         __u32,
    pub link_state: __u32,
}

pub const IFLA_VF_PORT_UNSPEC : c_int = 0;
pub const IFLA_VF_PORT        : c_int = 1;
pub const __IFLA_VF_PORT_MAX  : c_int = 2;

pub const IFLA_VF_PORT_MAX : c_int = __IFLA_VF_PORT_MAX - 1;

pub const IFLA_PORT_UNSPEC        : c_int = 0;
pub const IFLA_PORT_VF            : c_int = 1;
pub const IFLA_PORT_PROFILE       : c_int = 2;
pub const IFLA_PORT_VSI_TYPE      : c_int = 3;
pub const IFLA_PORT_INSTANCE_UUID : c_int = 4;
pub const IFLA_PORT_HOST_UUID     : c_int = 5;
pub const IFLA_PORT_REQUEST       : c_int = 6;
pub const IFLA_PORT_RESPONSE      : c_int = 7;
pub const __IFLA_PORT_MAX         : c_int = 8;

pub const IFLA_PORT_MAX : c_int = __IFLA_PORT_MAX - 1;

pub const PORT_PROFILE_MAX : c_int = 40;
pub const PORT_UUID_MAX    : c_int = 16;
pub const PORT_SELF_VF     : c_int = -1;

pub const PORT_REQUEST_PREASSOCIATE    : c_int = 0;
pub const PORT_REQUEST_PREASSOCIATE_RR : c_int = 1;
pub const PORT_REQUEST_ASSOCIATE       : c_int = 2;
pub const PORT_REQUEST_DISASSOCIATE    : c_int = 3;

pub const PORT_VDP_RESPONSE_SUCCESS                : c_int = 0;
pub const PORT_VDP_RESPONSE_INVALID_FORMAT         : c_int = 1;
pub const PORT_VDP_RESPONSE_INSUFFICIENT_RESOURCES : c_int = 2;
pub const PORT_VDP_RESPONSE_UNUSED_VTID            : c_int = 3;
pub const PORT_VDP_RESPONSE_VTID_VIOLATION         : c_int = 4;
pub const PORT_VDP_RESPONSE_VTID_VERSION_VIOALTION : c_int = 5;
pub const PORT_VDP_RESPONSE_OUT_OF_SYNC            : c_int = 6;

pub const PORT_PROFILE_RESPONSE_SUCCESS                : c_int = 256;
pub const PORT_PROFILE_RESPONSE_INPROGRESS             : c_int = 257;
pub const PORT_PROFILE_RESPONSE_INVALID                : c_int = 258;
pub const PORT_PROFILE_RESPONSE_BADSTATE               : c_int = 259;
pub const PORT_PROFILE_RESPONSE_INSUFFICIENT_RESOURCES : c_int = 260;
pub const PORT_PROFILE_RESPONSE_ERROR                  : c_int = 261;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct ifla_port_vsi {
    pub vsi_mgr_id: __u8,
    pub vsi_type_id: [__u8; 3],
    pub vsi_type_version: __u8,
    pub pad: [__u8; 3],
}

pub const IFLA_IPOIB_UNSPEC : c_int = 0;
pub const IFLA_IPOIB_PKEY   : c_int = 1;
pub const IFLA_IPOIB_MODE   : c_int = 2;
pub const IFLA_IPOIB_UMCAST : c_int = 3;
pub const __IFLA_IPOIB_MAX  : c_int = 4;

pub const IPOIB_MODE_DATAGRAM  : c_int = 0;
pub const IPOIB_MODE_CONNECTED : c_int = 1;

pub const IFLA_IPOIB_MAX : c_int = __IFLA_IPOIB_MAX - 1;

pub const IFLA_HSR_UNSPEC           : c_int = 0;
pub const IFLA_HSR_SLAVE1           : c_int = 1;
pub const IFLA_HSR_SLAVE2           : c_int = 2;
pub const IFLA_HSR_MULTICAST_SPEC   : c_int = 3;
pub const IFLA_HSR_SUPERVISION_ADDR : c_int = 4;
pub const IFLA_HSR_SEQ_NR           : c_int = 5;
pub const __IFLA_HSR_MAX            : c_int = 6;

pub const IFLA_HSR_MAX : c_int = __IFLA_HSR_MAX - 1;

/////////////////////////////////
// include/uapi/linux/neighbour.h
/////////////////////////////////

#[repr(C)]
#[derive(Pod, Eq)]
pub struct ndmsg {
    pub ndm_family:  __u8,
    pub ndm_pad1:    __u8,
    pub ndm_pad2:    __u16,
    pub ndm_ifindex: __s32,
    pub ndm_state:   __u16,
    pub ndm_flags:   __u8,
    pub ndm_type:    __u8,
}

pub const NDA_UNSPEC       : c_int = 0;
pub const NDA_DST          : c_int = 1;
pub const NDA_LLADDR       : c_int = 2;
pub const NDA_CACHEINFO    : c_int = 3;
pub const NDA_PROBES       : c_int = 4;
pub const NDA_VLAN         : c_int = 5;
pub const NDA_PORT         : c_int = 6;
pub const NDA_VNI          : c_int = 7;
pub const NDA_IFINDEX      : c_int = 8;
pub const NDA_MASTER       : c_int = 9;
pub const NDA_LINK_NETNSID : c_int = 10;
pub const __NDA_MAX        : c_int = 11;

pub const NDA_MAX : c_int = __NDA_MAX - 1;

pub const NTF_USE         : u8 = 0x01;
pub const NTF_SELF        : u8 = 0x02;
pub const NTF_MASTER      : u8 = 0x04;
pub const NTF_PROXY       : u8 = 0x08;
pub const NTF_EXT_LEARNED : u8 = 0x10;
pub const NTF_ROUTER      : u8 = 0x80;

pub const NUD_INCOMPLETE : u8 = 0x01;
pub const NUD_REACHABLE  : u8 = 0x02;
pub const NUD_STALE      : u8 = 0x04;
pub const NUD_DELAY      : u8 = 0x08;
pub const NUD_PROBE      : u8 = 0x10;
pub const NUD_FAILED     : u8 = 0x20;
pub const NUD_NOARP      : u8 = 0x40;
pub const NUD_PERMANENT  : u8 = 0x80;
pub const NUD_NONE       : u8 = 0x00;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct nda_cacheinfo {
    pub ndm_confirmed: __u32,
    pub ndm_used:      __u32,
    pub ndm_updated:   __u32,
    pub ndm_refcnt:    __u32,
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct ndt_stats {
    pub ndts_allocs:           __u64,
    pub ndts_destroys:         __u64,
    pub ndts_hash_grows:       __u64,
    pub ndts_res_failed:       __u64,
    pub ndts_lookups:          __u64,
    pub ndts_hits:             __u64,
    pub ndts_rcv_probes_mcast: __u64,
    pub ndts_rcv_probes_ucast: __u64,
    pub ndts_periodic_gc_runs: __u64,
    pub ndts_forced_gc_runs:   __u64,
}

pub const NDTPA_UNSPEC              : c_int = 0;
pub const NDTPA_IFINDEX             : c_int = 1;
pub const NDTPA_REFCNT              : c_int = 2;
pub const NDTPA_REACHABLE_TIME      : c_int = 3;
pub const NDTPA_BASE_REACHABLE_TIME : c_int = 4;
pub const NDTPA_RETRANS_TIME        : c_int = 5;
pub const NDTPA_GC_STALETIME        : c_int = 6;
pub const NDTPA_DELAY_PROBE_TIME    : c_int = 7;
pub const NDTPA_QUEUE_LEN           : c_int = 8;
pub const NDTPA_APP_PROBES          : c_int = 9;
pub const NDTPA_UCAST_PROBES        : c_int = 10;
pub const NDTPA_MCAST_PROBES        : c_int = 11;
pub const NDTPA_ANYCAST_DELAY       : c_int = 12;
pub const NDTPA_PROXY_DELAY         : c_int = 13;
pub const NDTPA_PROXY_QLEN          : c_int = 14;
pub const NDTPA_LOCKTIME            : c_int = 15;
pub const NDTPA_QUEUE_LENBYTES      : c_int = 16;
pub const __NDTPA_MAX               : c_int = 17;

pub const NDTPA_MAX : c_int = __NDTPA_MAX - 1;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct ndtmsg {
    pub ndtm_family: __u8,
    pub ndtm_pad1:   __u8,
    pub ndtm_pad2:   __u16,
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct ndt_config {
    pub ndtc_key_len:       __u16,
    pub ndtc_entry_size:    __u16,
    pub ndtc_entries:       __u32,
    pub ndtc_last_flush:    __u32,
    pub ndtc_last_rand:     __u32,
    pub ndtc_hash_rnd:      __u32,
    pub ndtc_hash_mask:     __u32,
    pub ndtc_hash_chain_gc: __u32,
    pub ndtc_proxy_qlen:    __u32,
}

pub const NDTA_UNSPEC      : c_int = 0;
pub const NDTA_NAME        : c_int = 1;
pub const NDTA_THRESH1     : c_int = 2;
pub const NDTA_THRESH2     : c_int = 3;
pub const NDTA_THRESH3     : c_int = 4;
pub const NDTA_CONFIG      : c_int = 5;
pub const NDTA_PARMS       : c_int = 6;
pub const NDTA_STATS       : c_int = 7;
pub const NDTA_GC_INTERVAL : c_int = 8;
pub const __NDTA_MAX       : c_int = 9;

pub const NDTA_MAX : c_int = __NDTA_MAX - 1;

////////////////////////////
// include/uapi/hdlc/ioctl.h
////////////////////////////

pub const GENERIC_HDLC_VERSION   : c_int = 4;
pub const CLOCK_DEFAULT          : c_int = 0;
pub const CLOCK_EXT              : c_int = 1;
pub const CLOCK_INT              : c_int = 2;
pub const CLOCK_TXINT            : c_int = 3;
pub const CLOCK_TXFROMRX         : c_int = 4;
pub const ENCODING_DEFAULT       : c_int = 0;
pub const ENCODING_NRZ           : c_int = 1;
pub const ENCODING_NRZI          : c_int = 2;
pub const ENCODING_FM_MARK       : c_int = 3;
pub const ENCODING_FM_SPACE      : c_int = 4;
pub const ENCODING_MANCHESTER    : c_int = 5;
pub const PARITY_DEFAULT         : c_int = 0;
pub const PARITY_NONE            : c_int = 1;
pub const PARITY_CRC16_PR0       : c_int = 2;
pub const PARITY_CRC16_PR1       : c_int = 3;
pub const PARITY_CRC16_PR0_CCITT : c_int = 4;
pub const PARITY_CRC16_PR1_CCITT : c_int = 5;
pub const PARITY_CRC32_PR0_CCITT : c_int = 6;
pub const PARITY_CRC32_PR1_CCITT : c_int = 7;
pub const LMI_DEFAULT            : c_int = 0;
pub const LMI_NONE               : c_int = 1;
pub const LMI_ANSI               : c_int = 2;
pub const LMI_CCITT              : c_int = 3;
pub const LMI_CISCO              : c_int = 4;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct sync_serial_settings {
    pub clock_rate: c_uint,
    pub clock_type: c_uint,
    pub loopback:   c_ushort,
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct te1_settings {
    pub clock_rate: c_uint,
    pub clock_type: c_uint,
    pub loopback:   c_ushort,
    pub slot_map:   c_uint,
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct raw_hdlc_proto {
    pub encoding: c_ushort,
    pub parity:   c_ushort,
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct fr_proto {
    pub t391: c_uint,
    pub t392: c_uint,
    pub n391: c_uint,
    pub n392: c_uint,
    pub n393: c_uint,
    pub lmi:  c_ushort,
    pub dce:  c_ushort,
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct fr_proto_pvc {
    pub dlci: c_uint,
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct fr_proto_pvc_info {
    pub dlci: c_uint,
    pub master: [c_char; IFNAMSIZ],
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct cisco_proto {
    pub interval: c_uint,
    pub timeout:  c_uint,
}

//////////////////////////
// include/uapi/linux/if.h
//////////////////////////

pub const IFNAMSIZ : usize = 16;
pub const IFALIASZ : usize = 256;

pub type net_device_flags = c_int;
pub const IFF_UP          : net_device_flags = 1<<0;
pub const IFF_BROADCAST   : net_device_flags = 1<<1;
pub const IFF_DEBUG       : net_device_flags = 1<<2;
pub const IFF_LOOPBACK    : net_device_flags = 1<<3;
pub const IFF_POINTOPOINT : net_device_flags = 1<<4;
pub const IFF_NOTRAILERS  : net_device_flags = 1<<5;
pub const IFF_RUNNING     : net_device_flags = 1<<6;
pub const IFF_NOARP       : net_device_flags = 1<<7;
pub const IFF_PROMISC     : net_device_flags = 1<<8;
pub const IFF_ALLMULTI    : net_device_flags = 1<<9;
pub const IFF_MASTER      : net_device_flags = 1<<10;
pub const IFF_SLAVE       : net_device_flags = 1<<11;
pub const IFF_MULTICAST   : net_device_flags = 1<<12;
pub const IFF_PORTSEL     : net_device_flags = 1<<13;
pub const IFF_AUTOMEDIA   : net_device_flags = 1<<14;
pub const IFF_DYNAMIC     : net_device_flags = 1<<15;
pub const IFF_LOWER_UP    : net_device_flags = 1<<16;
pub const IFF_DORMANT     : net_device_flags = 1<<17;
pub const IFF_ECHO        : net_device_flags = 1<<18;

pub const IFF_VOLATILE : net_device_flags =
        IFF_LOOPBACK | IFF_POINTOPOINT | IFF_BROADCAST | IFF_ECHO |
		IFF_MASTER | IFF_SLAVE | IFF_RUNNING | IFF_LOWER_UP | IFF_DORMANT;

pub const IF_GET_IFACE            : c_int = 0x0001;
pub const IF_GET_PROTO            : c_int = 0x0002;
pub const IF_IFACE_V35            : c_int = 0x1000;
pub const IF_IFACE_V24            : c_int = 0x1001;
pub const IF_IFACE_X21            : c_int = 0x1002;
pub const IF_IFACE_T1             : c_int = 0x1003;
pub const IF_IFACE_E1             : c_int = 0x1004;
pub const IF_IFACE_SYNC_SERIAL    : c_int = 0x1005;
pub const IF_IFACE_X21D           : c_int = 0x1006;
pub const IF_PROTO_HDLC           : c_int = 0x2000;
pub const IF_PROTO_PPP            : c_int = 0x2001;
pub const IF_PROTO_CISCO          : c_int = 0x2002;
pub const IF_PROTO_FR             : c_int = 0x2003;
pub const IF_PROTO_FR_ADD_PVC     : c_int = 0x2004;
pub const IF_PROTO_FR_DEL_PVC     : c_int = 0x2005;
pub const IF_PROTO_X25            : c_int = 0x2006;
pub const IF_PROTO_HDLC_ETH       : c_int = 0x2007;
pub const IF_PROTO_FR_ADD_ETH_PVC : c_int = 0x2008;
pub const IF_PROTO_FR_DEL_ETH_PVC : c_int = 0x2009;
pub const IF_PROTO_FR_PVC         : c_int = 0x200A;
pub const IF_PROTO_FR_ETH_PVC     : c_int = 0x200B;
pub const IF_PROTO_RAW            : c_int = 0x200C;

pub const IF_OPER_UNKNOWN        : c_int = 0;
pub const IF_OPER_NOTPRESENT     : c_int = 1;
pub const IF_OPER_DOWN           : c_int = 2;
pub const IF_OPER_LOWERLAYERDOWN : c_int = 3;
pub const IF_OPER_TESTING        : c_int = 4;
pub const IF_OPER_DORMANT        : c_int = 5;
pub const IF_OPER_UP             : c_int = 6;

pub const IF_LINK_MODE_DEFAULT : c_int = 0;
pub const IF_LINK_MODE_DORMANT : c_int = 1;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct ifmap {
    pub mem_start: c_ulong,
    pub mem_end:   c_ulong,
    pub base_addr: c_ushort,
    pub irq:       c_uchar,
    pub dma:       c_uchar,
    pub port:      c_uchar,
}

////////////////////////////
// include/uapi/linux/veth.h
////////////////////////////

pub const VETH_INFO_UNSPEC : u16 = 0;
pub const VETH_INFO_PEER   : u16 = 1;
pub const __VETH_INFO_MAX  : u16 = 2;
pub const VETH_INFO_MAX    : u16 = __VETH_INFO_MAX - 1;

/////////////////////////////
// include/uapi/linux/sched.h
/////////////////////////////

pub const CSIGNAL              : c_int = 0x000000ff;
pub const CLONE_VM             : c_int = 0x00000100;
pub const CLONE_FS             : c_int = 0x00000200;
pub const CLONE_FILES          : c_int = 0x00000400;
pub const CLONE_SIGHAND        : c_int = 0x00000800;
pub const CLONE_PTRACE         : c_int = 0x00002000;
pub const CLONE_VFORK          : c_int = 0x00004000;
pub const CLONE_PARENT         : c_int = 0x00008000;
pub const CLONE_THREAD         : c_int = 0x00010000;
pub const CLONE_NEWNS          : c_int = 0x00020000;
pub const CLONE_SYSVSEM        : c_int = 0x00040000;
pub const CLONE_SETTLS         : c_int = 0x00080000;
pub const CLONE_PARENT_SETTID  : c_int = 0x00100000;
pub const CLONE_CHILD_CLEARTID : c_int = 0x00200000;
pub const CLONE_DETACHED       : c_int = 0x00400000;
pub const CLONE_UNTRACED       : c_int = 0x00800000;
pub const CLONE_CHILD_SETTID   : c_int = 0x01000000;
pub const CLONE_NEWUTS         : c_int = 0x04000000;
pub const CLONE_NEWIPC         : c_int = 0x08000000;
pub const CLONE_NEWUSER        : c_int = 0x10000000;
pub const CLONE_NEWPID         : c_int = 0x20000000;
pub const CLONE_NEWNET         : c_int = 0x40000000;
pub const CLONE_IO             : c_int = 0x80000000;

pub const SCHED_NORMAL   : c_int = 0;
pub const SCHED_FIFO     : c_int = 1;
pub const SCHED_RR       : c_int = 2;
pub const SCHED_BATCH    : c_int = 3;
pub const SCHED_IDLE     : c_int = 5;
pub const SCHED_DEADLINE : c_int = 6;

pub const SCHED_RESET_ON_FORK      : c_int = 0x40000000;
pub const SCHED_FLAG_RESET_ON_FORK : c_int = 0x01;

////////////////////////////////
// include/uapi/linux/signalfd.h
////////////////////////////////

pub const SFD_CLOEXEC  : c_int = O_CLOEXEC;
pub const SFD_NONBLOCK : c_int = O_NONBLOCK;

#[repr(C)]
#[derive(Pod)]
pub struct signalfd_siginfo {
    pub ssi_signo    : __u32,
    pub ssi_errno    : __s32,
    pub ssi_code     : __s32,
    pub ssi_pid      : __u32,
    pub ssi_uid      : __u32,
    pub ssi_fd       : __s32,
    pub ssi_tid      : __u32,
    pub ssi_band     : __u32,
    pub ssi_overrun  : __u32,
    pub ssi_trapno   : __u32,
    pub ssi_status   : __s32,
    pub ssi_int      : __s32,
    pub ssi_ptr      : __u64,
    pub ssi_utime    : __u64,
    pub ssi_stime    : __u64,
    pub ssi_addr     : __u64,
    pub ssi_addr_lsb : __u16,
    pub __pad : [__u8; 46],
}
