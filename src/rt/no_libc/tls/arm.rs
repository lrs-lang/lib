// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::{mem};
use r_syscall::arch::{set_tls};
use base::prelude::*;
use cty::{tls_index};
use super::{Private};

pub use self::var::{mem_size};

#[path = "var1.rs"] mod var;

pub const DTVR_ALIGN: usize = 4;
pub const DTVR_SIZE: usize = 8;

#[repr(C)]
pub struct ArchPrivate {
    static_block: *mut u8,
}

pub unsafe fn place_tls(mut mem: *mut u8) -> (*mut Private, *mut u8) {
    let (private, tp, static_block) = var::place_tls(mem);
    (*private).arch.static_block = static_block;
    (private, tp)
}

pub unsafe fn set_tp(tls: *mut u8) -> Result {
    // XXX should only check this once at startup.
    let kuser_helper_version = *(0xffff0ffc as *const i32);
    assert!(kuser_helper_version > 0);

    // Not really sure how tls works on 32 bit arm. glibc simply uses this syscall while
    // musl only uses it on armv7.
    rv!(set_tls(tls))
}

pub unsafe fn get_tp() -> *mut u8 {
    // Documentation/arm/kernel_user_helpers.txt in the linux source tree.
    let kuser_get_tls = mem::cast::<_, extern fn() -> *mut u8>(0xffff0fe0);
    kuser_get_tls()
}

#[no_mangle]
pub unsafe extern fn __aeabi_read_tp(ti: *const tls_index) -> *mut u8 {
    let tp = get_tp();
    let private = var::get_private(tp);
    (*private).arch.static_block.add((*ti).ti_offset)
}
