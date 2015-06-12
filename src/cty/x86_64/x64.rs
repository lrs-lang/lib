// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(non_camel_case_types, raw_pointer_derive)]

pub use ::gen::{
    __kernel_long_t, __kernel_ulong_t,
};

pub const USER_POINTER_ALIGN : usize = 8;
pub const BITS_PER_C_ULONG : usize = 64;

pub type c_long  = i64;
pub type c_ulong = u64;

pub type user_size_t = ::c_ulong;

pub type __kernel_old_uid_t = ::c_ushort;
pub type __kernel_old_gid_t = ::c_ushort;
pub type __kernel_old_dev_t = ::c_ulong;

pub const __NR_rt_sigaction      : usize = 13;
pub const __NR_rt_sigreturn      : usize = 15;
pub const __NR_ioctl             : usize = 16;
pub const __NR_readv             : usize = 19;
pub const __NR_writev            : usize = 20;
pub const __NR_recvfrom          : usize = 45;
pub const __NR_sendmsg           : usize = 46;
pub const __NR_recvmsg           : usize = 47;
pub const __NR_setsockopt        : usize = 54;
pub const __NR_getsockopt        : usize = 55;
pub const __NR_execve            : usize = 59;
pub const __NR_ptrace            : usize = 101;
pub const __NR_rt_sigpending     : usize = 127;
pub const __NR_rt_sigtimedwait   : usize = 128;
pub const __NR_rt_sigqueueinfo   : usize = 129;
pub const __NR_sigaltstack       : usize = 131;
pub const __NR_uselib            : usize = 134;
pub const __NR__sysctl           : usize = 156;
pub const __NR_create_module     : usize = 174;
pub const __NR_get_kernel_syms   : usize = 177;
pub const __NR_query_module      : usize = 178;
pub const __NR_nfsservctl        : usize = 180;
pub const __NR_set_thread_area   : usize = 205;
pub const __NR_io_setup          : usize = 206;
pub const __NR_io_submit         : usize = 209;
pub const __NR_get_thread_area   : usize = 211;
pub const __NR_epoll_ctl_old     : usize = 214;
pub const __NR_epoll_wait_old    : usize = 215;
pub const __NR_timer_create      : usize = 222;
pub const __NR_vserver           : usize = 236;
pub const __NR_mq_notify         : usize = 244;
pub const __NR_kexec_load        : usize = 246;
pub const __NR_waitid            : usize = 247;
pub const __NR_set_robust_list   : usize = 273;
pub const __NR_get_robust_list   : usize = 274;
pub const __NR_vmsplice          : usize = 278;
pub const __NR_move_pages        : usize = 279;
pub const __NR_preadv            : usize = 295;
pub const __NR_pwritev           : usize = 296;
pub const __NR_rt_tgsigqueueinfo : usize = 297;
pub const __NR_recvmmsg          : usize = 299;
pub const __NR_sendmmsg          : usize = 307;
pub const __NR_process_vm_readv  : usize = 310;
pub const __NR_process_vm_writev : usize = 311;
pub const __NR_execveat          : usize = 322;

//////////////////////////////////////
// arch/x86/include/uapi/asm/siginfo.h
//////////////////////////////////////

pub const __ARCH_SI_PREAMBLE_SIZE: usize = 4 * ::BYTES_PER_INT;

pub use ::gen::{
    __ARCH_SI_CLOCK_T,
};
