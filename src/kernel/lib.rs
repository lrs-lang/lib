// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_kernel"]
#![crate_type = "lib"]
#![feature(const_fn)]
#![no_std]

extern crate lrs_base as base;
extern crate lrs_cty as cty;
extern crate lrs_syscall as syscall;
extern crate lrs_parse as parse;
extern crate lrs_atomic as atomic;
extern crate lrs_arch_fns as arch_fns;

use base::prelude::*;
use core::{mem};
use base::{error};
use syscall::{uname};
use atomic::{Atomic};
use parse::{Parsable};

mod std { pub use base::std::*; }

static mut MAJOR: u8 = 0;
static mut MINOR: u8 = 0;

fn init() {
    static STATUS: Atomic<u8> = Atomic::new(0);
    const NONE: u8 = 0;
    const WORKING: u8 = 1;
    const DONE: u8 = 2;

    if STATUS.load_acquire() == DONE {
        return;
    }

    if STATUS.compare_exchange_monotonic(NONE, WORKING) == NONE {
        let mut name = mem::zeroed();
        if uname(&mut name) == 0 {
            let release = unsafe { mem::as_data(&name.release).as_bytes() };
            if let Ok((major, bytes)) = u8::parse_bytes_init(release) {
                if let Ok((minor, _)) = u8::parse_bytes_init(&release[bytes+1..]) {
                    unsafe {
                        MAJOR = major;
                        MINOR = minor;
                    }
                }
            }
        }
        STATUS.store_release(DONE);
    } else {
        while STATUS.load_acquire() != DONE {
            arch_fns::spin();
        }
    }
}

/// Returns the major and minor version of the kernel we're running on.
pub fn version() -> Result<(u8, u8)> {
    init();
    unsafe {
        match (MAJOR, MINOR) {
            (0, _) => Err(error::ResourceBusy),
            r => Ok(r),
        }
    }
}

macro_rules! kver {
    ($($(#[$meta:meta])* fn $name:ident >= $major:expr,$minor:expr;)+) => {
        $(
            pub fn $name() -> bool {
                init();
                unsafe { MAJOR > $major || (MAJOR == $major && MINOR >= $minor) }
            }
        )+
    }
}

kver! {
    #[doc = "Returns whether this kernel version has the `bpf` system call.\n"]
    #[doc = "= Remarks"]
    #[doc = "This system call was introduced in 3.18.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:bpf(2)"]
    fn has_bpf >= 3,18;

    #[doc = "Returns whether this kernel version has the `execveat` system call.\n"]
    #[doc = "= Remarks"]
    #[doc = "This system call was introduced in 3.19.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:execveat(2)"]
    fn has_execveat >= 3,19;

    #[doc = "Returns whether this kernel version has the `finit_module` system call.\n"]
    #[doc = "= Remarks"]
    #[doc = "This system call was introduced in 3.8.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:finit_module(2)"]
    fn has_finit_module >= 3,8;

    #[doc = "Returns whether this kernel version has the `getrandom` system call.\n"]
    #[doc = "= Remarks"]
    #[doc = "This system call was introduced in 3.17.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:getrandom(2)"]
    fn has_getrandom >= 3,17;

    #[doc = "Returns whether this kernel version has the `kcmp` system call.\n"]
    #[doc = "= Remarks"]
    #[doc = "This system call was introduced in 3.5.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:kcmp(2)"]
    fn has_kcmp >= 3,5;

    #[doc = "Returns whether this kernel version has the `kexec_file_load` system \
             call.\n"]
    #[doc = "= Remarks"]
    #[doc = "This system call was introduced in 3.17.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:kexec_file_load(2)"]
    fn has_kexec_file_load >= 3,17;

    #[doc = "Returns whether this kernel version has the `memfd_create` system call.\n"]
    #[doc = "= Remarks"]
    #[doc = "This system call was introduced in 3.17.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:memfd_create(2)"]
    fn has_memfd_create >= 3,17;

    #[doc = "Returns whether this kernel version has the `process_vm_readv` system \
             call.\n"]
    #[doc = "= Remarks"]
    #[doc = "This system call was introduced in 3.2.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:process_vm_readv(2)"]
    fn has_process_vm_readv >= 3,2;

    #[doc = "Returns whether this kernel version has the `process_vm_writev` system \
             call.\n"]
    #[doc = "= Remarks"]
    #[doc = "This system call was introduced in 3.2.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:process_vm_writev(2)"]
    fn has_process_vm_writev >= 3,2;

    #[doc = "Returns whether this kernel version has the `renameat2` system call.\n"]
    #[doc = "= Remarks"]
    #[doc = "This system call was introduced in 3.15.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:renameat2(2)"]
    fn has_renameat2 >= 3,15;

    #[doc = "Returns whether this kernel version has the `sched_getattr` system call.\n"]
    #[doc = "= Remarks"]
    #[doc = "This system call was introduced in 3.14.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:sched_getattr(2)"]
    fn has_sched_getattr >= 3,14;

    #[doc = "Returns whether this kernel version has the `sched_settattr` system call.\n"]
    #[doc = "= Remarks"]
    #[doc = "This system call was introduced in 3.14.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:sched_setattr(2)"]
    fn has_sched_setattr >= 3,14;

    #[doc = "Returns whether this kernel version has the `seccomp` system call.\n"]
    #[doc = "= Remarks"]
    #[doc = "This system call was introduced in 3.17.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:seccomp(2)"]
    fn has_seccomp >= 3,17;

    #[doc = "Returns whether this kernel version has the `O_TMPFILE` flag.\n"]
    #[doc = "= Remarks"]
    #[doc = "This flag was introduced in 3.11.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:open(2)"]
    fn has_o_tmpfile >= 3,11;

    #[doc = "Returns whether this kernel version has the `SEEK_DATA` flag.\n"]
    #[doc = "= Remarks"]
    #[doc = "This flag was introduced in 3.1.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:lseek(2)"]
    fn has_seek_data >= 3,1;

    #[doc = "Returns whether this kernel version has the `SEEK_HOLE` flag.\n"]
    #[doc = "= Remarks"]
    #[doc = "This flag was introduced in 3.1.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:lseek(2)"]
    fn has_seek_hole >= 3,1;

    #[doc = "Returns whether this kernel version has the `FALLOC_FL_COLLAPSE_RANGE` \
             flag.\n"]
    #[doc = "= Remarks"]
    #[doc = "This flag was introduced in 3.15.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:fallocate(2)"]
    fn has_falloc_fl_collapse_range >= 3,15;

    #[doc = "Returns whether this kernel version has the `FALLOC_FL_ZERO_RANGE` flag.\n"]
    #[doc = "= Remarks"]
    #[doc = "This flag was introduced in 3.15.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:fallocate(2)"]
    fn has_falloc_fl_zero_range >= 3,15;

    #[doc = "Returns whether this kernel version has the `TFD_IOC_SET_TICKS` flag.\n"]
    #[doc = "= Remarks"]
    #[doc = "This flag was introduced in 3.17.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:timerfd_create(2)"]
    fn has_tfd_ioc_set_ticks >= 3,17;

    #[doc = "Returns whether this kernel version has the `EPOLLWAKEUP` flag.\n"]
    #[doc = "= Remarks"]
    #[doc = "This flag was introduced in 3.5.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:epoll_ctl(2)"]
    fn has_epollwakeup >= 3,5;

    #[doc = "Returns whether this kernel version supports the `O_DIRECT` flag for pipes.\
             \n"]
    #[doc = "= Remarks"]
    #[doc = "This flag was introduced in 3.4.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:pipe2(2)"]
    fn has_pipe_o_direct >= 3,4;

    #[doc = "Returns whether this kernel version has the `MAP_HUGE_2MB` flag.\n"]
    #[doc = "= Remarks"]
    #[doc = "This flag was introduced in 3.8.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mmap(2)"]
    fn has_map_huge_2mb >= 3,8;

    #[doc = "Returns whether this kernel version has the `MAP_HUGE_1GB` flag.\n"]
    #[doc = "= Remarks"]
    #[doc = "This flag was introduced in 3.8.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mmap(2)"]
    fn has_map_huge_1gb >= 3,8;

    #[doc = "Returns whether this kernel version has the `MADV_DONTDUMP` flag.\n"]
    #[doc = "= Remarks"]
    #[doc = "This flag was introduced in 3.4.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:madvise(2)"]
    fn has_madv_dontdump >= 3,4;

    #[doc = "Returns whether this kernel version has the `MADV_DODUMP` flag.\n"]
    #[doc = "= Remarks"]
    #[doc = "This flag was introduced in 3.4.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:madvise(2)"]
    fn has_madv_dodump >= 3,4;
}
