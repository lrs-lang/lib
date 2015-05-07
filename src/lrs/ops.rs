// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Operator traits
//!
//! = Description
//!
//! This module contains traits and structures that are used via special symbols, e.g.,
//! `+`, `==`, or `..`.

pub use lrs_core::ops::{
    Drop, Add, Sub, Mul, Div, Rem, Neg, Not, BitAnd, BitOr, Shl, Shr, Index, IndexMut,
    RangeFull, Range, RangeFrom, RangeTo, Deref, DerefMut, Eq, PartialOrd, Fn, FnMut,
    FnOnce,
};
