// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use prelude::*;

/// Objects that can be duplicated.
///
/// Duplication might not succeed (e.g. out of memory) in which case an error is returned.
pub trait Clone {
    /// Clones the value.
    fn clone(&self) -> Result<Self>;
}

impl<T> Clone for PhantomData<T> {
    fn clone(&self) -> Result<PhantomData<T>> {
        Ok(*self)
    }
}

macro_rules! imp {
    ($ty:ident) => {
        impl Clone for $ty {
            fn clone(&self) -> Result<$ty> {
                Ok(*self)
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
    fn clone(&self) -> Result<()> { Ok(()) }
}

impl<T0> Clone for (T0,)
    where T0: Clone,
{
    fn clone(&self) -> Result<(T0,)> {
        Ok((try!(self.0.clone()),))
    }
}

impl<T0, T1> Clone for (T0,T1)
    where T0: Clone,
          T1: Clone,
{
    fn clone(&self) -> Result<(T0,T1)> {
        Ok((try!(self.0.clone()),try!(self.1.clone())))
    }
}
