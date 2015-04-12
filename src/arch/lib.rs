// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_arch"]
#![crate_type = "lib"]
#![feature(asm)]
#![allow(trivial_numeric_casts, trivial_casts, non_upper_case_globals, dead_code)]

pub mod cty;
pub mod syscall;
