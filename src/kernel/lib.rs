// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_kernel"]
#![crate_type = "lib"]
#![feature(plugin, no_std, custom_derive)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use]
extern crate lrs_core as core;
extern crate lrs_base as base;
extern crate lrs_cty as cty;
extern crate lrs_syscall as syscall;
extern crate lrs_parse as parse;
extern crate lrs_atomic as atomic;
extern crate lrs_arch_fns as arch_fns;

#[prelude_import] use base::prelude::*;
use core::{mem};
use base::{error};
use syscall::{uname};
use atomic::{AtomicU8, ATOMIC_U8_INIT};
use parse::{Parsable};

mod lrs { pub use base::lrs::*; }

static mut MAJOR: u8 = 0;
static mut MINOR: u8 = 0;

fn init() {
    static STATUS: AtomicU8 = ATOMIC_U8_INIT;

    if STATUS.load_weak() == 2 {
        return;
    }
    
    if STATUS.compare_exchange(0, 1) == 0 {
        let mut name = mem::zeroed();
        if uname(&mut name) == 0 {
            let release = mem::as_bytes(&name.release);
            if let Ok((major, bytes)) = u8::parse_bytes_init(release) {
                if let Ok((minor, _)) = u8::parse_bytes_init(&release[bytes+1..]) {
                    unsafe {
                        MAJOR = major;
                        MINOR = minor;
                    }
                }
            }
        }
        STATUS.store(2);
    } else {
        while STATUS.load_weak() != 2 {
            arch_fns::spin();
        }
    }
}

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
    ($($name:ident >= $major:expr,$minor:expr)+) => {
        $(
            pub fn $name() -> bool {
                init();
                unsafe { MAJOR > $major || (MAJOR == $major && MINOR >= $minor) }
            }
        )+
    }
}

kver! {
    has_bpf                      >= 3,18
    has_execveat                 >= 3,19
    has_finit_module             >= 3,8
    has_getrandom                >= 3,17
    has_kcmp                     >= 3,5
    has_kexec_file_load          >= 3,17
    has_memfd_create             >= 3,17
    has_process_vm_readv         >= 3,2
    has_process_vm_writev        >= 3,2
    has_renameat2                >= 3,15
    has_sched_getattr            >= 3,14
    has_sched_setattr            >= 3,14
    has_seccomp                  >= 3,17
    has_o_tmpfile                >= 3,11
    has_seek_data                >= 3,1
    has_seek_hole                >= 3,1
    has_falloc_fl_collapse_range >= 3,15
    has_falloc_fl_zero_range     >= 3,15
    has_tfd_ioc_set_ticks        >= 3,17
    has_epollwakeup              >= 3,5
}
