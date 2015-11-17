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
    /// Points to itself. Required by the ABI.
    tp: *mut u8,
}

pub unsafe fn place_tls(mem: *mut u8) -> (*mut Private, *mut u8) {
    let (private, tp) = var::place_tls(mem);
    (*private).arch.tp = tp;
    (private, tp)
}

pub unsafe fn set_tp(tls: *mut u8) -> Result {
    static mut ID: i32 = -1;

    // The set_thread_area syscall is a bit too powerful for our needs but all that exists
    // on x86. The argument is actually a struct but that struct has a bitfield so it's
    // better to just put all of the calculated values in an array and to pass that.
    //
    // Field 1:: This field contains the ID of the GDT entry that will be used for our
    // storage. Linux gives us up to three of those entries and if we pass the ID -1 it
    // will find a valid ID itself and write that one back into field 1. After the first
    // successful call to this function (when the main thread's tp is set), the ID field
    // above is set and we'll continue to use that one.
    //
    // Field 2:: The thread pointer.
    //
    // Field 3:: The limit field contains the size of the memory that can be accessed
    // through the segment register. We write 0xfffff which, when counted as pages, is
    // 4GB.
    //
    // Field 4:: The bitfield with the following settings:
    //       seg_32bit = 1
    //       contents = 0
    //       read_exec_only = 0
    //       limit_in_pages = 1
    //       seg_not_present = 0
    //       useable = 1
    //
    // Afterwards we set the %gs register to access our segment.

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
