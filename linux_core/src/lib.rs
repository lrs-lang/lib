// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_core"]
#![crate_type = "lib"]
#![feature(std_misc, core, plugin, asm, io)]

pub mod cty;
pub mod arch;
pub mod syscall;
pub mod ext;
pub mod string;
pub mod result;
pub mod errno;
pub mod util;
