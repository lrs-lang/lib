// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use cty_base::types::{c_char, c_int};
use syscall::{self};

#[cfg(no_libc)]
#[link(name = "lrs_crt")]
extern { }

#[no_mangle]
pub unsafe extern fn lrs_start_main(stack: *const usize) {
    extern {
        fn main(argc: c_int, argv: *const *const c_char);
    }

    let argc = *stack;
    let argv = stack.add(1);
    main(argc as c_int, argv as *const _);

    syscall::exit(0);
}
