// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::marker::{Sized};
use conv::out_of::{OutOf};

/// Types that have a default value.
pub trait Default: OutOf+Sized {
    /// Returns the default value of this type.
    fn default() -> Self;
}

impl<T> Default for T
    where T: OutOf
{
    fn default() -> T {
        T::out_of(())
    }
}
