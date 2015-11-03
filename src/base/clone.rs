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

impl<T> Clone for Option<T>
    where T: Clone,
{
    fn clone(&self) -> Option<T> {
        match *self {
            Some(ref t) => Some(t.clone()),
            _ => None,
        }
    }
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

impl<T0, T1, T2> Clone for (T0,T1,T2)
    where T0: Clone,
          T1: Clone,
          T2: Clone,
{
    fn clone(&self) -> (T0,T1,T2) {
        (self.0.clone(),self.1.clone(),self.2.clone())
    }
}

/// Objects that can be duplicated.
///
/// = Remarks
///
/// Duplication might not succeed (e.g. out of memory) in which case an error is returned.
pub trait MaybeClone: Sized {
    /// Clones the value.
    fn maybe_clone(&self) -> Result<Self>;
}

macro_rules! mimp {
    ($ty:ident) => {
        impl MaybeClone for $ty {
            fn maybe_clone(&self) -> Result<$ty> {
                Ok(*self)
            }
        }
    }
}

mimp!(u8);
mimp!(u16);
mimp!(u32);
mimp!(u64);
mimp!(usize);
mimp!(i8);
mimp!(i16);
mimp!(i32);
mimp!(i64);
mimp!(isize);
mimp!(bool);

impl MaybeClone for () {
    fn maybe_clone(&self) -> Result<()> { Ok(()) }
}

impl<T> MaybeClone for Option<T>
    where T: MaybeClone,
{
    fn maybe_clone(&self) -> Result<Option<T>> {
        match *self {
            Some(ref t) => Ok(Some(try!(t.maybe_clone()))),
            _ => Ok(None),
        }
    }
}

impl<T0> MaybeClone for (T0,)
    where T0: MaybeClone,
{
    fn maybe_clone(&self) -> Result<(T0,)> {
        Ok((try!(self.0.maybe_clone()),))
    }
}

impl<T0, T1> MaybeClone for (T0,T1)
    where T0: MaybeClone,
          T1: MaybeClone,
{
    fn maybe_clone(&self) -> Result<(T0,T1)> {
        Ok((try!(self.0.maybe_clone()), try!(self.1.maybe_clone())))
    }
}
