// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[allow(unused_imports)]
use {r_syscall, cty};
use base::prelude::*;

#[cfg(target_arch = "arm")]
pub unsafe fn set_tls(tls: *mut u8) -> Result {
    // Not really sure how tls works on 32 bit arm. glibc simply uses this syscall while
    // musl only uses it on armv7.
    rv!(r_syscall::arch::set_tls(tls))
}

#[cfg(target_arch = "aarch64")]
pub unsafe fn set_tls(tls: *mut u8) -> Result {
    // We don't even have to use a syscall on aarch64. The kernel will save this register
    // when clone(2) is called or during a context switch.
    asm!("msr tpidr_el0,$0" : : "r"(tls));
    Ok(())
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn set_tls(tls: *mut u8) -> Result {
    rv!(r_syscall::arch::arch_prctl(cty::ARCH_SET_FS, tls as cty::k_ulong))
}

#[cfg(target_arch = "x86")]
pub unsafe fn set_tls(tls: *mut u8) -> Result {
    static mut ID: i32 = -1;

    /* Collapsed value of the bitfield:
       .seg_32bit = 1
       .contents = 0
       .read_exec_only = 0
       .limit_in_pages = 1
       .seg_not_present = 0
       .useable = 1 */
    let mut user_desc = [ID, tls as usize as i32, 0xfffff, 0x51];
    match rv!(r_syscall::arch::set_thread_area(user_desc.as_mut_ptr() as *mut _)) {
        Ok(()) => {
            ID = user_desc[0];
            asm!("movw ${0:w},%gs" : : "q"(ID*8 + 3));
            Ok(())
        },
        e => e,
    }
}
