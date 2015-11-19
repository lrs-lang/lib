// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use cty::{c_int, c_long};
use atomic::{AtomicCInt};
use rt::imp::{tls};

pub unsafe fn start_thread(func: unsafe extern fn(*mut u8) -> !, arg: *mut u8,
                           flags: c_int, stack: *mut u8, ctid: &AtomicCInt,
                           tp: *mut u8) -> c_long {
    extern {
        fn __start_thread(flags: c_int, stack: *mut u8, ptid: *mut c_int, arg: *mut u8,
                          ptp: *mut i32, func: unsafe extern fn(*mut u8) -> !,
                          ctid: *mut c_int) -> c_long;
    }
    let mut user_desc = tls::arch::magic_array(tp);
    __start_thread(flags, stack, 0 as *mut _, arg, user_desc.as_mut_ptr(), func,
                   ctid.unwrap())
}

pub unsafe fn stop_thread(stack_base: *mut u8, stack_size: usize,
                          tmp_stack: *mut u8) -> ! {
    extern {
        fn __stop_thread(stack_base: *mut u8, stack_size: usize, tmp_stack: *mut u8) -> !;
    }
    __stop_thread(stack_base, stack_size, tmp_stack)
}
