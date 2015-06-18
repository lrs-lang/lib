// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::marker::{Sized};
use into::{Into};

/// Types that have a default value.
pub trait Default: Sized {
    /// Returns the default value of this type.
    fn default() -> Self;
}

impl<T> Default for T where (): Into<T> {
    fn default() -> Self {
        ().into()
    }
}
