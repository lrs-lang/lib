// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use cty_base::types::{c_char, c_int};
use syscall::{self};
use core::{mem};

#[cfg(no_libc)]
#[link(name = "lrs_crt")]
extern { }


#[no_mangle]
pub unsafe extern fn lrs_start_main(stack: *const usize) {
    extern {
        fn main(argc: c_int, argv: *const *const c_char);
        #[linkage = "extern_weak"] static __init_array_start: *const usize;
        #[linkage = "extern_weak"] static __init_array_end: *const usize;
    }

    let mut init_fn = __init_array_start;
    while init_fn != __init_array_end {
        if *init_fn != 0 && *init_fn != !0 {
            let f: extern fn() = mem::cast(*init_fn);
            f();
        }
        init_fn = init_fn.add(1);
    }

    let argc = *stack;
    let argv = stack.add(1);
    main(argc as c_int, argv as *const _);

    syscall::exit(0);
}
