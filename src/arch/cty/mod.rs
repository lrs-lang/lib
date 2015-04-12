// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(non_camel_case_types)]

pub use self::arch::*;

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
#[derive(Copy, Clone, Debug)]
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

// The linux headers define the seconds field as `c_long`, but `c_long` is 32 bit in the
// x32 ABI and `__kernel_long_t` is 64 bit. We use `__kernel_long_t` so that we don't have
// to convert during syscalls.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct timespec {
    pub tv_sec:  __kernel_time_t,
    pub tv_nsec: __kernel_long_t,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct timeval {
    pub tv_sec: __kernel_time_t,
    pub tv_usec: __kernel_suseconds_t,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct timezone {
    pub tz_minuteswest: c_int,
    pub tz_dsttime:     c_int,
}

pub const ITIMER_REAL    : c_int = 0;
pub const ITIMER_VIRTUAL : c_int = 1;
pub const ITIMER_PROF    : c_int = 2;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value:    timespec,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
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
#[derive(Copy, Clone, Debug)]
pub struct linux_dirent64 {
    pub d_ino:    u64,
    pub d_off:    i64,
    pub d_reclen: c_ushort,
    pub d_type:   c_uchar,
    pub d_name:   [c_char; 0],
}

pub const RLIM64_INFINITY: c_ulonglong = !0;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct rlimit64 {
    pub rlim_cur: __u64,
    pub rlim_max: __u64,
}

pub const SI_LOAD_SHIFT	: __kernel_ulong_t = 16;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
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

pub const F_SETLEASE          : c_int = F_LINUX_SPECIFIC_BASE+0;
pub const F_GETLEASE          : c_int = F_LINUX_SPECIFIC_BASE+1;
pub const F_CANCELLK          : c_int = F_LINUX_SPECIFIC_BASE+5;
pub const F_DUPFD_CLOEXEC     : c_int = F_LINUX_SPECIFIC_BASE+6;
pub const F_NOTIFY            : c_int = F_LINUX_SPECIFIC_BASE+2;
pub const F_SETPIPE_SZ        : c_int = F_LINUX_SPECIFIC_BASE+7;
pub const F_GETPIPE_SZ        : c_int = F_LINUX_SPECIFIC_BASE+8;
pub const F_ADD_SEALS         : c_int = F_LINUX_SPECIFIC_BASE+9;
pub const F_GET_SEALS         : c_int = F_LINUX_SPECIFIC_BASE+10;
pub const F_SEAL_SEAL         : c_int = 0x0001;
pub const F_SEAL_SHRINK       : c_int = 0x0002;
pub const F_SEAL_GROW         : c_int = 0x0004;
pub const F_SEAL_WRITE        : c_int = 0x0008;
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

fn BPF_CLASS (code: u8) -> u8 { code & 0x07 }
fn BPF_SIZE  (code: u8) -> u8 { code & 0x18 }
fn BPF_MODE  (code: u8) -> u8 { code & 0xe0 }
fn BPF_OP    (code: u8) -> u8 { code & 0xf0 }
fn BPF_SRC   (code: u8) -> u8 { code & 0x08 }

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
#[derive(Copy, Clone, Debug)]
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
#[derive(Copy, Clone, Debug)]
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
#[derive(Copy, Clone, Debug)]
pub struct __user_cap_header_struct {
    pub version: __u32,
    pub pid: c_int,
}

pub type cap_user_header_t = *mut __user_cap_header_struct;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
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
pub const XATTR_CAPS_SZ_1         : c_int = SIZEOF__le32*(1+2*VFS_CAP_U32_1);
pub const VFS_CAP_REVISION_2      : c_int = 0x02000000;
pub const VFS_CAP_U32_2           : c_int = 2;
pub const XATTR_CAPS_SZ_2         : c_int = SIZEOF__le32*(1+2*VFS_CAP_U32_2);
pub const XATTR_CAPS_SZ           : c_int = XATTR_CAPS_SZ_2;
pub const VFS_CAP_U32             : c_int = VFS_CAP_U32_2;
pub const VFS_CAP_REVISION        : c_int = VFS_CAP_REVISION_2;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct vfs_cap_data_array {
    pub permitted:   __le32,
    pub inheritable: __le32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct vfs_cap_data {
    pub magic_etc: __le32,
    pub data: [vfs_cap_data_array; VFS_CAP_U32],
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
#[derive(Copy, Clone, Debug)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: __kernel_size_t,
}

pub const UIO_FASTIOV : usize = 8;
pub const UIO_MAXIOV  : usize = 1024;

// socket.h

pub type __kernel_sa_family_t = c_ushort;

pub const _K_SS_MAXSIZE   : usize = 128;
pub const _K_SS_ALIGNSIZE : usize = USER_POINTER_ALIGN;

// XXX(WRONG) this needs an ((aligned(USER_POINTER_ALIGN))) attribute
// "systems language"
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct __kernel_sockaddr_storage {
    pub ss_family: __kernel_sa_family_t,
    pub __data:    [c_char; _K_SS_MAXSIZE - BYTES_PER_SHORT],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [k_char; 14],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct linger {
    pub l_onoff: k_int,
    pub l_linger: k_int,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct user_msghdr {
    pub msg_name:       *mut c_void,
    pub msg_namelen:    k_int,
    pub msg_iov:        *mut iovec,
    pub msg_iovlen:     __kernel_size_t,
    pub msg_control:    *mut c_void,
    pub msg_controllen: __kernel_size_t,
    pub msg_flags:      k_uint,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct mmsghdr {
    pub msg_hdr: user_msghdr,
    pub msg_len: k_uint,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct cmsghdr {
    pub cmsg_len:   __kernel_size_t,
    pub cmsg_level: k_int,
    pub cmsg_type:  k_int,
}

pub const SCM_RIGHTS      : c_int = 0x01;
pub const SCM_CREDENTIALS : c_int = 0x02;
pub const SCM_SECURITY    : c_int = 0x03;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
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
#[derive(Copy, Clone, Debug)]
struct timex {
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
#[derive(Copy, Clone, Debug)]
pub struct io_event {
    pub data: __u64,
    pub obj:  __u64,
    pub res:  __s64,
    pub res2: __s64,
}

#[cfg(target_endian = "little")]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
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
#[derive(Copy, Clone, Debug)]
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
#[derive(Copy, Clone, Debug)]
pub struct file_handle {
	pub handle_bytes: __u32,
	pub handle_type: k_int,
	pub f_handle: [c_uchar; 0],
}

// getcpu.h

#[repr(C)]
#[derive(Copy, Clone, Debug)]
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
#[derive(Copy, Clone, Debug)]
pub struct kexec_segment {
	buf: *const void,
	bufsz: size_t,
	mem: *const void,
	memsz: size_t,
}

// straight from fs/readdir.c

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct linux_dirent {
	pub d_ino:    k_ulong,
	pub d_off:    k_ulong,
	pub d_reclen: k_ushort,
    pub d_name: [k_char; 1],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct old_linux_dirent {
    pub d_ino:    k_ulong,
    pub d_offset: k_ulong,
    pub d_namlen: k_ushort,
    pub d_name: [k_char; 1],
}

// straight from mm/mmap.c

#[repr(C)]
#[derive(Copy, Clone, Debug)]
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
#[derive(Copy, Clone, Debug)]
pub struct mq_attr {
    pub mq_flags:	__kernel_long_t,
    pub mq_maxmsg:	__kernel_long_t,
    pub mq_msgsize:	__kernel_long_t,
    pub mq_curmsgs:	__kernel_long_t,
    pub __reserved: [__kernel_long_t; 4],
}

pub const NOTIFY_NONE       : c_int = 0;
pub const NOTIFY_WOKENUP    : c_int = 1;
pub const NOTIFY_REMOVED    : c_int = 2;
pub const NOTIFY_COOKIE_LEN : c_int = 32;

// ipc.h

pub const IPC_PRIVATE : __kernel_key_t = 0;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
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
#[derive(Copy, Clone, Debug)]
pub struct ipc_kludge {
	msgp: *mut msgbuf,
	msgtyp: c_long, // XXX: Maybe use k_long here?
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
#[derive(Copy, Clone, Debug)]
pub struct msqid_ds {
	pub msg_perm: ipc_perm,
	pub msg_first: *mut msg,
	pub msg_last:    *mut msg,
	pub msg_stime:   __kernel_time_t,
	pub msg_rtime:   __kernel_time_t,
	pub msg_ctime:   __kernel_time_t,
	pub msg_lcbytes: c_ulong,
	pub msg_lqbytes: c_ulong,
	pub msg_cbytes:  c_ushort,
	pub msg_qnum:    c_ushort,
	pub msg_qbytes:  c_ushort,
	pub msg_lspid:   __kernel_ipc_pid_t,
	pub msg_lrpid:   __kernel_ipc_pid_t,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct msgbuf {
	pub mtype: __kernel_long_t,
	pub mtext: [c_char; 1],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct msginfo {
	msgpool: c_int,
	msgmap:  c_int,
	msgmax:  c_int,
	msgmnb:  c_int,
	msgmni:  c_int,
	msgssz:  c_int,
	msgtql:  c_int,
	msgseg:  c_ushort,
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
#[derive(Copy, Clone, Debug)]
pub struct oldold_utsname {
    pub sysname:  [c_char; 9],
    pub nodename: [c_char; 9],
    pub release:  [c_char; 9],
    pub version:  [c_char; 9],
    pub machine:  [c_char; 9],
}

pub const __NEW_UTS_LEN : usize = 64;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct old_utsname {
    pub sysname:  [c_char; 65],
    pub nodename: [c_char; 65],
    pub release:  [c_char; 65],
    pub version:  [c_char; 65],
    pub machine:  [c_char; 65],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
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
#[derive(Copy, Clone, Debug)]
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
                                                ERF_SAMPLE_BRANCH_KERNEL |
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
#[derive(Copy, Clone, Debug)]
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

pub fn PERF_EVENT_IOC_ENABLE()     -> c_uint { _IO(b'$' as c_uint, 0) }
pub fn PERF_EVENT_IOC_DISABLE()    -> c_uint { _IO(b'$' as c_uint, 1) }
pub fn PERF_EVENT_IOC_REFRESH()    -> c_uint { _IO(b'$' as c_uint, 2) }
pub fn PERF_EVENT_IOC_RESET()      -> c_uint { _IO(b'$' as c_uint, 3) }
pub fn PERF_EVENT_IOC_PERIOD()     -> c_uint { _IOW::<__u64>(b'$' as c_uint, 4) }
pub fn PERF_EVENT_IOC_SET_OUTPUT() -> c_uint { _IO('$', 5) }
pub fn PERF_EVENT_IOC_SET_FILTER() -> c_uint { _IOW::<*mut c_char>(b'$' as c_uint, 6) }
pub fn PERF_EVENT_IOC_ID()         -> c_uint { _IOR::<*mut __u64>(b'$' as c_uint, 7 ) }

pub const PERF_IOC_FLAG_GROUP : c_uint = 1;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
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
#[derive(Copy, Clone, Debug)]
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
#[derive(Copy, Clone, Debug)]
pub struct perf_mem_data_src {
    pub val: __u64,
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
#[derive(Copy, Clone, Debug)]
pub struct perf_branch_entry {
    pub from: __u64,
    pub from: __u64,
	//__u64	mispred:1,
	//	predicted:1,
	//	in_tx:1,
	//	abort:1,
	//	reserved:60;
    __bitfield_one: __u64,
}
