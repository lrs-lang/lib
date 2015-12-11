// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

pub trait To<T = Self>: TryTo<T> {
    fn to(&self) -> T;
}

impl<U: ?Sized, T = U> To<T> for U
    where T: From<U>
{
    fn to(&self) -> T {
        T::from(self)
    }
}

pub trait TryTo<T = Self> {
    fn try_to(&self) -> Result<T>;
}

impl<U: ?Sized, T = U> TryTo<T> for U
    where T: TryFrom<U>
{
    fn try_to(&self) -> Result<T> {
        T::try_from(self)
    }
}
