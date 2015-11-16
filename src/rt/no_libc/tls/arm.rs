// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::{mem};
use r_syscall::arch::{set_tls};
use base::prelude::*;
use super::{Private};

pub use self::var::{mem_size};

#[path = "var1.rs"] mod var;

pub const DTVR_ALIGN: usize = 4;
pub const DTVR_SIZE: usize = 8;

#[repr(C)]
pub struct ArchPrivate {
}

pub unsafe fn place_tls(mem: *mut u8) -> (*mut Private, *mut u8) {
    let (private, tp, _) = var::place_tls(mem);
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
    let kuser_get_tls = mem::cast::<usize, extern fn() -> *mut u8>(0xffff0fe0);
    kuser_get_tls()
}

#[no_mangle]
pub unsafe extern fn __aeabi_read_tp() -> *mut u8 {
    get_tp()
}
