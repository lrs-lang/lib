// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use cty_base::types::{c_char, c_int};
use syscall::{self};

#[no_mangle]
pub unsafe extern fn __lrs_start_main(argc: c_int, argv: *const *const c_char) {
    extern {
        fn main(argc: c_int, argv: *const *const c_char);
    }
    main(argc, argv);
    syscall::exit(0);
}
