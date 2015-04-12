// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(non_camel_case_types, raw_pointer_derive)]

pub use ::gen::{
    __kernel_old_uid_t, __kernel_old_gid_t, __kernel_old_dev_t,
};

pub const USER_POINTER_ALIGN : usize = 4;
pub const BITS_PER_C_ULONG : usize = 32;

pub type c_long  = i32;
pub type c_ulong = u32;

pub type user_size_t = c_uint;

pub type __kernel_long_t  = ::c_longlong;
pub type __kernel_ulong_t = ::c_ulonglong;

pub const __NR_rt_sigaction      : usize = 512;
pub const __NR_rt_sigreturn      : usize = 513;
pub const __NR_ioctl             : usize = 514;
pub const __NR_readv             : usize = 515;
pub const __NR_writev            : usize = 516;
pub const __NR_recvfrom          : usize = 517;
pub const __NR_sendmsg           : usize = 518;
pub const __NR_recvmsg           : usize = 519;
pub const __NR_execve            : usize = 520;
pub const __NR_ptrace            : usize = 521;
pub const __NR_rt_sigpending     : usize = 522;
pub const __NR_rt_sigtimedwait   : usize = 523;
pub const __NR_rt_sigqueueinfo   : usize = 524;
pub const __NR_sigaltstack       : usize = 525;
pub const __NR_timer_create      : usize = 526;
pub const __NR_mq_notify         : usize = 527;
pub const __NR_kexec_load        : usize = 528;
pub const __NR_waitid            : usize = 529;
pub const __NR_set_robust_list   : usize = 530;
pub const __NR_get_robust_list   : usize = 531;
pub const __NR_vmsplice          : usize = 532;
pub const __NR_move_pages        : usize = 533;
pub const __NR_preadv            : usize = 534;
pub const __NR_pwritev           : usize = 535;
pub const __NR_rt_tgsigqueueinfo : usize = 536;
pub const __NR_recvmmsg          : usize = 537;
pub const __NR_sendmmsg          : usize = 538;
pub const __NR_process_vm_readv  : usize = 539;
pub const __NR_process_vm_writev : usize = 540;
pub const __NR_setsockopt        : usize = 541;
pub const __NR_getsockopt        : usize = 542;
pub const __NR_io_setup          : usize = 543;
pub const __NR_io_submit         : usize = 544;
pub const __NR_execveat          : usize = 545;
