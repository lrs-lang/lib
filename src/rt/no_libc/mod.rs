// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use cty::{c_char};

pub mod tls;
pub mod crt;

static mut ENVP: *const *const c_char = 0 as *const *const c_char;

pub fn raw_env() -> *const *const c_char {
    unsafe { ENVP }
}
