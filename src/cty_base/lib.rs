// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_cty_base"]
#![crate_type = "lib"]
#![no_std]
#![allow(non_upper_case_globals, non_camel_case_types)]

pub use arch::{errno, types};

mod gen;

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;

#[cfg(target_arch = "arm")]
#[path = "arm.rs"]
mod arch;

#[cfg(target_arch = "aarch64")]
#[path = "aarch64.rs"]
mod arch;
