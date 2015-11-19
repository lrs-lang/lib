// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use r_syscall::arch::{arch_prctl};
use cty::{ARCH_SET_FS, k_ulong};
use base::prelude::*;

use super::{Private};
pub use self::var::{mem_size};

#[path = "var2.rs"] mod var;

#[repr(C)]
pub struct ArchPrivate {
    /// Points to itself. Required by the ABI.
    tp: *mut u8,
}

pub unsafe fn place_tls(mem: *mut u8) -> (*mut Private, *mut u8) {
    let (private, tp) = var::place_tls(mem);
    (*private).arch.tp = tp;
    (private, tp)
}

pub unsafe fn set_tp(tls: *mut u8) -> Result {
    rv!(arch_prctl(ARCH_SET_FS, tls as k_ulong))
}

pub unsafe fn get_private() -> *mut Private {
    let addr;
    asm!("mov %fs:0,$0" : "=r"(addr));
    var::get_private(addr)
}
