// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Compiler intrinsics
//!
//! = Description
//!
//! This module provides direct access to compiler built-in functions. They are all unsafe
//! and have safe wrappers in other modules.

pub use lrs_core::intrinsics::{
    discriminant_value, abort, breakpoint, size_of, move_val_init, min_align_of,
    init_dropped, init, uninit, forget, transmute, needs_drop, offset, copy,
    copy_nonoverlapping, lrs_abort, unreachable,
};
