// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use cty::{c_long};

pub use ::arch::common::{
    syscall0, syscall1, syscall2, syscall3, syscall4, syscall5, syscall6,
};

/// Syscall type
pub type SCT = c_long;
