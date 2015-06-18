// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use prelude::*;

/// Objects that can be duplicated.
///
/// = Remarks
///
/// Duplication might not succeed (e.g. out of memory) in which case an error is returned.
pub trait Clone {
    /// Clones the value.
    fn clone(&self) -> Self;
}

impl<T> Clone for PhantomData<T> {
    fn clone(&self) -> PhantomData<T> {
        *self
    }
}

macro_rules! imp {
    ($ty:ident) => {
        impl Clone for $ty {
            fn clone(&self) -> $ty {
                *self
            }
        }
    }
}

imp!(u8);
imp!(u16);
imp!(u32);
imp!(u64);
imp!(usize);
imp!(i8);
imp!(i16);
imp!(i32);
imp!(i64);
imp!(isize);
imp!(bool);

impl Clone for () {
    fn clone(&self) -> () { () }
}

impl<T0> Clone for (T0,)
    where T0: Clone,
{
    fn clone(&self) -> (T0,) {
        (self.0.clone(),)
    }
}

impl<T0, T1> Clone for (T0,T1)
    where T0: Clone,
          T1: Clone,
{
    fn clone(&self) -> (T0,T1) {
        (self.0.clone(),self.1.clone())
    }
}

/// Objects that can be duplicated.
///
/// = Remarks
///
/// Duplication might not succeed (e.g. out of memory) in which case an error is returned.
pub trait MaybeClone {
    /// Clones the value.
    fn maybe_clone(&self) -> Result<Self>;
}

impl<T: Clone> MaybeClone for T {
    fn maybe_clone(&self) -> Result<Self> {
        Ok(self.clone())
    }
}
