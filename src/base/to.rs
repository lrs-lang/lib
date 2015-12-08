// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

pub trait To<T = Self> {
    fn to(&self) -> T;
}

impl<T> To for PhantomData<T> {
    fn to(&self) -> PhantomData<T> {
        *self
    }
}

macro_rules! imp {
    ($ty:ident) => {
        impl To for $ty {
            fn to(&self) -> $ty {
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

impl To for () {
    fn to(&self) -> () { () }
}

impl<T, U> To<Option<U>> for Option<T>
    where T: To<U>,
{
    fn to(&self) -> Option<U> {
        match *self {
            Some(ref t) => Some(t.to()),
            _ => None,
        }
    }
}

impl<T0, U0> To<(U0,)> for (T0,)
    where T0: To<U0>,
{
    fn to(&self) -> (U0,) {
        (self.0.to(),)
    }
}

impl<T0, T1, U0, U1> To<(U0, U1)> for (T0,T1)
    where T0: To<U0>,
          T1: To<U1>,
{
    fn to(&self) -> (U0,U1) {
        (self.0.to(), self.1.to())
    }
}

impl<T0, T1, T2, U0, U1, U2> To<(U0, U1, U2)> for (T0,T1,T2)
    where T0: To<U0>,
          T1: To<U1>,
          T2: To<U2>,
{
    fn to(&self) -> (U0,U1,U2) {
        (self.0.to(), self.1.to(), self.2.to())
    }
}

pub trait TryTo<T = Self> {
    fn try_to(&self) -> Result<T>;
}

macro_rules! mimp {
    ($ty:ident) => {
        impl TryTo for $ty {
            fn try_to(&self) -> Result<$ty> {
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

impl TryTo for () {
    fn try_to(&self) -> Result<()> { Ok(()) }
}

impl<T, U> TryTo<Option<U>> for Option<T>
    where T: TryTo<U>,
{
    fn try_to(&self) -> Result<Option<U>> {
        match *self {
            Some(ref t) => Ok(Some(try!(t.try_to()))),
            _ => Ok(None),
        }
    }
}

impl<T0, U0> TryTo<(U0,)> for (T0,)
    where T0: TryTo<U0>,
{
    fn try_to(&self) -> Result<(U0,)> {
        Ok((try!(self.0.try_to()),))
    }
}

impl<T0, T1, U0, U1> TryTo<(U0, U1)> for (T0,T1)
    where T0: TryTo<U0>,
          T1: TryTo<U1>,
{
    fn try_to(&self) -> Result<(U0,U1)> {
        Ok((try!(self.0.try_to()), try!(self.1.try_to())))
    }
}
