// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Private};
use base::prelude::*;

pub use self::var::{mem_size};

#[path = "var1.rs"] mod var;

pub const DTVR_ALIGN: usize = 8;
pub const DTVR_SIZE: usize = 16;

#[repr(C)]
pub struct ArchPrivate {
}

pub unsafe fn place_tls(mem: *mut u8) -> (*mut Private, *mut u8) {
    var::place_tls(mem)
}

pub unsafe fn set_tp(tls: *mut u8) -> Result {
    // We don't even have to use a syscall on aarch64. The kernel will save this
    // register when clone(2) is called or during a context switch.
    asm!("msr tpidr_el0,$0" : : "r"(tls) : : "volatile");
    Ok(())
}

pub unsafe fn get_private() -> *mut Private {
    let addr;
    asm!("mrs $0,tpidr_el0" : "=r"(addr));
    var::get_private(addr)
}
