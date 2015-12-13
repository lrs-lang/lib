// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

/// Objects that can be duplicated.
///
/// = Remarks
///
/// Duplication always succeeds.
pub trait Clone: TryClone+To {
    /// Clones the value.
    fn clone(&self) -> Self;
}

impl<T> Clone for T
    where T: To,
{
    fn clone(&self) -> Self {
        self.to()
    }
}

/// Objects that can be duplicated.
///
/// = Remarks
///
/// Duplication might not succeed (e.g. out of memory) in which case an error is returned.
pub trait TryClone: TryTo+Sized {
    /// Clones the value.
    fn try_clone(&self) -> Result<Self>;
}

impl<T> TryClone for T
    where T: TryTo,
{
    fn try_clone(&self) -> Result<Self> {
        self.try_to()
    }
}
