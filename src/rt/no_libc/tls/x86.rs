// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use r_syscall::arch::{set_thread_area};
use super::{Private};
use base::prelude::*;

pub use self::var::{mem_size};

#[path = "var2.rs"] mod var;

#[repr(C)]
pub struct ArchPrivate {
    tp: *mut u8,
}

pub unsafe fn place_tls(mem: *mut u8) -> (*mut Private, *mut u8) {
    let (private, tp) = var::place_tls(mem);
    (*private).arch.tp = tp;
    (private, tp)
}

pub unsafe fn set_tp(tls: *mut u8) -> Result {
    static mut ID: i32 = -1;

    /* Collapsed value of the bitfield:
       .seg_32bit = 1
       .contents = 0
       .read_exec_only = 0
       .limit_in_pages = 1
       .seg_not_present = 0
       .useable = 1 */
    let mut user_desc = [ID, tls as usize as i32, 0xfffff, 0x51];
    match rv!(set_thread_area(user_desc.as_mut_ptr() as *mut _)) {
        Ok(()) => {
            ID = user_desc[0];
            asm!("movw ${0:w},%gs" : : "q"(ID*8 + 3) : : "volatile");
            Ok(())
        },
        e => e,
    }
}
