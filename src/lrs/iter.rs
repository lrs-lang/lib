// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Iterators
//!
//! TODO: Don't always write the same "this module provides [the obvious]" bullshit.

pub use lrs_core::iter::{
    Iterator, Empty, IntoIterator,
};
pub use lrs_iter::{
    repeat, Repeat, IteratorExt, Map,
};
