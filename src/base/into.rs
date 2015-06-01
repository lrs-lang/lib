// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use core::prelude::*;
pub use core::ops::{Range, RangeTo, RangeFrom, RangeFull};

/// Objects that can be turned into other objects.
pub trait Into<T> {
    /// Turns the object into another object.
    fn into(self) -> T;
}

impl<T> Into<T> for T {
    fn into(self) -> T {
        self
    }
}

impl<T> Into<Range<Option<T>>> for RangeTo<T> {
    fn into(self) -> Range<Option<T>> {
        Range { start: None, end: Some(self.end) }
    }
}

impl<T> Into<Range<Option<T>>> for RangeFrom<T> {
    fn into(self) -> Range<Option<T>> {
        Range { start: Some(self.start), end: None }
    }
}

impl<T> Into<Range<Option<T>>> for RangeFull {
    fn into(self) -> Range<Option<T>> {
        Range { start: None, end: None }
    }
}
