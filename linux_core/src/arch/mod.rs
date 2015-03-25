// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod syscall {
    pub use super::arch::syscall::*;
}

pub mod cty {
    pub use super::arch::cty::*;
}

#[cfg(target_arch = "x86_64")]
#[path = "x86_64/mod.rs"]
mod arch;
