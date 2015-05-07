// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Ordering comparisons
//!
//! = Description
//!
//! This module provides structures and functions that operate on objects in a partial or
//! total order.

pub use lrs_core::cmp::{
    PartialOrd, Ord, Ordering, min, min_ref, min_mut, max, max_ref, max_mut,
};
