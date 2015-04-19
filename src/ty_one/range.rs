// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::ops::{Range};

pub trait BoundedRange<T> {
    fn to_range(self) -> Range<T>;
}

impl<T> BoundedRange<T> for Range<T> {
    fn to_range(self) -> Range<T> { self }
}
