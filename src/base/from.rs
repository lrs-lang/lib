// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

pub trait From<T: ?Sized = Self>: TryFrom<T>+Sized {
    fn from(t: &T) -> Self;
}

macro_rules! imp {
    ($ty:ident) => {
        impl From for $ty {
            fn from(t: &$ty) -> $ty {
                *t
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

impl From for () {
    fn from(_: &()) -> () { () }
}

impl<U, T = U> From<Option<T>> for Option<U>
    where T: To<U>,
{
    fn from(t: &Option<T>) -> Option<U> {
        match *t {
            Some(ref t) => Some(t.to()),
            _ => None,
        }
    }
}

impl<U0, T0 = U0> From<(T0,)> for (U0,)
    where T0: To<U0>,
{
    fn from(t: &(T0,)) -> (U0,) {
        (t.0.to(),)
    }
}

impl<U0, U1, T0 = U0, T1 = U1> From<(T0, T1)> for (U0,U1)
    where T0: To<U0>,
          T1: To<U1>,
{
    fn from(t: &(T0, T1)) -> (U0,U1) {
        (t.0.to(), t.1.to())
    }
}

impl<U0, U1, U2, T0 = U0, T1 = U1, T2 = U2> From<(T0, T1, T2)> for (U0,U1,U2)
    where T0: To<U0>,
          T1: To<U1>,
          T2: To<U2>,
{
    fn from(t: &(T0, T1, T2)) -> (U0,U1,U2) {
        (t.0.to(), t.1.to(), t.2.to())
    }
}

pub trait TryFrom<T: ?Sized = Self>: Sized {
    fn try_from(t: &T) -> Result<Self>;
}

impl<T> TryFrom for PhantomData<T> {
    fn try_from(t: &PhantomData<T>) -> Result<PhantomData<T>> {
        Ok(*t)
    }
}

macro_rules! mimp {
    ($ty:ident) => {
        impl TryFrom for $ty {
            fn try_from(t: &$ty) -> Result<$ty> {
                Ok(*t)
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

impl TryFrom for () {
    fn try_from(_: &()) -> Result<()> { Ok(()) }
}

impl<U, T = U> TryFrom<Option<T>> for Option<U>
    where T: TryTo<U>,
{
    fn try_from(t: &Option<T>) -> Result<Option<U>> {
        match *t {
            Some(ref t) => Ok(Some(try!(t.try_to()))),
            _ => Ok(None),
        }
    }
}

impl<U0, T0 = U0> TryFrom<(T0,)> for (U0,)
    where T0: TryTo<U0>,
{
    fn try_from(t: &(T0,)) -> Result<(U0,)> {
        Ok((try!(t.0.try_to()),))
    }
}

impl<U0, U1, T0 = U0, T1 = U1> TryFrom<(T0, T1)> for (U0,U1)
    where T0: TryTo<U0>,
          T1: TryTo<U1>,
{
    fn try_from(t: &(T0,T1)) -> Result<(U0,U1)> {
        Ok((try!(t.0.try_to()), try!(t.1.try_to())))
    }
}

impl<U0, U1, U2, T0 = U0, T1 = U1, T2 = U2> TryFrom<(T0, T1, T2)> for (U0,U1,U2)
    where T0: To<U0>,
          T1: To<U1>,
          T2: To<U2>,
{
    fn try_from(t: &(T0, T1, T2)) -> Result<(U0,U1,U2)> {
        Ok((t.0.to(), t.1.to(), t.2.to()))
    }
}
