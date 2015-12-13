// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

/// Objects that can be turned into other objects.
pub trait Into<T> {
    /// Turns the object into another object.
    fn into(self) -> T;
}

impl<U, T> Into<T> for U
    where T: OutOf<U>
{
    fn into(self) -> T {
        T::out_of(self)
    }
}
