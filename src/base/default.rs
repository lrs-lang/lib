// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::marker::{Sized};
use to::{To};

/// Types that have a default value.
pub trait Default: Sized {
    /// Returns the default value of this type.
    fn default() -> Self;
}

impl<T> To<T> for ()
    where T: Default,
{
    fn to(&self) -> T {
        T::default()
    }
}

macro_rules! zero {
    ($ty:ty) => {
        impl Default for $ty {
            fn default() -> $ty {
                0
            }
        }
    }
}

zero!(u8    );
zero!(u16   );
zero!(u32   );
zero!(u64   );
zero!(usize );
zero!(i8    );
zero!(i16   );
zero!(i32   );
zero!(i64   );
zero!(isize );

impl<T> Default for Option<T> {
    fn default() -> Option<T> {
        None
    }
}
