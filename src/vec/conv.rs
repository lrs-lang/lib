// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::marker::{Leak};
use core::{slice};
use bx::{Box};
use {Vec};
use alloc::{MemPool};
use str_one::{CStr, NoNullStr, ByteStr};

// Conversions
//
// &Vec<T, H> -> &[T]
// &mut Vec<T, H> -> &mut [T]
// Vec<T, H> -> Box<[T], H>
// Box<[T], H> -> Vec<T, H>
//
// &Vec<u8, H>     -> &ByteStr
// &Vec<u8, H>     -> Result<&str>
// &Vec<u8, H>     -> Result<&NoNullStr>
// &Vec<u8, H>     -> Result<&CStr>
// &mut Vec<u8, H> -> &mut ByteStr
// &mut Vec<u8, H> -> Result<&mut NoNullStr>
// &mut Vec<u8, H> -> Result<&mut CStr>
//
// &[T]        -> Result<Vec<T,  H>>
// &Vec<T, H2> -> Result<Vec<u8, H>>
// &str        -> Result<Vec<u8, H>>
// &ByteStr    -> Result<Vec<u8, H>>
// &NoNullStr  -> Result<Vec<u8, H>>
// &CStr       -> Result<Vec<u8, H>>

impl<T, H: ?Sized> AsRef<[T]> for Vec<T, H>
    where H: MemPool,
{
    fn as_ref(&self) -> &[T] {
        self.deref()
    }
}

impl<T, H: ?Sized> TryAsRef<[T]> for Vec<T, H>
    where H: MemPool,
{
    fn try_as_ref(&self) -> Result<&[T]> {
        Ok(self.deref())
    }
}

impl<T, H: ?Sized> AsMut<[T]> for Vec<T, H>
    where H: MemPool,
{
    fn as_mut(&mut self) -> &mut [T] {
        self.deref_mut()
    }
}

impl<T, H: ?Sized> TryAsMut<[T]> for Vec<T, H>
    where H: MemPool,
{
    fn try_as_mut(&mut self) -> Result<&mut [T]> {
        Ok(self.deref_mut())
    }
}

impl<T, H> Into<Box<[T], H>> for Vec<T, H>
    where H: MemPool,
          T: Leak,
{
    fn into(mut self) -> Box<[T], H> {
        self.shrink_to_fit().unwrap();
        unsafe {
            let (ptr, len, _, pool) = self.into_raw_parts();
            Box::from_raw_parts(slice::from_ptr(ptr, len), pool)
        }
    }
}

impl<T, H> OutOf<Box<[T], H>> for Vec<T, H>
    where H: MemPool,
          T: Leak,
{
    fn out_of(bx: Box<[T], H>) -> Vec<T, H> {
        unsafe {
            let (ptr, pool) = bx.into_raw_parts();
            let len = (*ptr).len();
            let ptr = (*ptr).as_mut_ptr();
            Vec::from_raw_parts(ptr, len, len, pool)
        }
    }
}

impl<H: ?Sized> AsRef<ByteStr> for Vec<u8, H>
    where H: MemPool,
{
    fn as_ref(&self) -> &ByteStr {
        self.deref().as_ref()
    }
}

impl<H: ?Sized> TryAsRef<ByteStr> for Vec<u8, H>
    where H: MemPool,
{
    fn try_as_ref(&self) -> Result<&ByteStr> {
        Ok(self.deref().as_ref())
    }
}

impl<H: ?Sized> AsMut<ByteStr> for Vec<u8, H>
    where H: MemPool,
{
    fn as_mut(&mut self) -> &mut ByteStr {
        self.deref_mut().as_mut()
    }
}

impl<H: ?Sized> TryAsMut<ByteStr> for Vec<u8, H>
    where H: MemPool,
{
    fn try_as_mut(&mut self) -> Result<&mut ByteStr> {
        Ok(self.deref_mut().as_mut())
    }
}

impl<H: ?Sized> TryAsRef<CStr> for Vec<u8, H>
    where H: MemPool,
{
    fn try_as_ref(&self) -> Result<&CStr> {
        self.deref().try_as_ref()
    }
}

impl<H: ?Sized> TryAsMut<CStr> for Vec<u8, H>
    where H: MemPool,
{
    fn try_as_mut(&mut self) -> Result<&mut CStr> {
        self.deref_mut().try_as_mut()
    }
}

impl<H: ?Sized> TryAsRef<str> for Vec<u8, H>
    where H: MemPool,
{
    fn try_as_ref(&self) -> Result<&str> {
        self.deref().try_as_ref()
    }
}

impl<H: ?Sized> TryAsRef<NoNullStr> for Vec<u8, H>
    where H: MemPool,
{
    fn try_as_ref(&self) -> Result<&NoNullStr> {
        self.deref().try_as_ref()
    }
}

impl<H: ?Sized> TryAsMut<NoNullStr> for Vec<u8, H>
    where H: MemPool,
{
    fn try_as_mut(&mut self) -> Result<&mut NoNullStr> {
        self.deref_mut().try_as_mut()
    }
}

impl<T, H, U = T> TryFrom<[T]> for Vec<U, H>
    where T: TryTo<U>,
          H: MemPool + OutOf,
{
    fn try_from(ts: &[T]) -> Result<Vec<U, H>> {
        let mut vec = try!(Vec::with_capacity(ts.len()));
        for t in ts {
            vec.push(try!(t.try_to()));
        }
        Ok(vec)
    }
}

impl<H> TryFrom<str> for Vec<u8, H>
    where H: MemPool + OutOf,
{
    fn try_from(ts: &str) -> Result<Vec<u8, H>> {
        let mut vec = try!(Vec::with_capacity(ts.len()));
        vec.push_all(ts.as_ref());
        Ok(vec)
    }
}

impl<H> TryFrom<ByteStr> for Vec<u8, H>
    where H: MemPool + OutOf,
{
    fn try_from(ts: &ByteStr) -> Result<Vec<u8, H>> {
        let bytes: &[u8] = ts.as_ref();
        bytes.try_to()
    }
}

impl<H> TryFrom<CStr> for Vec<u8, H>
    where H: MemPool + OutOf,
{
    fn try_from(ts: &CStr) -> Result<Vec<u8, H>> {
        let bytes: &[u8] = ts.as_ref();
        bytes.try_to()
    }
}

impl<H> TryFrom<NoNullStr> for Vec<u8, H>
    where H: MemPool + OutOf,
{
    fn try_from(ts: &NoNullStr) -> Result<Vec<u8, H>> {
        let bytes: &[u8] = ts.as_ref();
        bytes.try_to()
    }
}

impl<T, H2: ?Sized, U = T, H1 = H2> TryFrom<Vec<T, H2>> for Vec<U, H1>
    where T: TryTo<U>,
          H2: MemPool,
          H1: MemPool + OutOf,
{
    fn try_from(v: &Vec<T, H2>) -> Result<Vec<U, H1>> {
        (**v).try_to()
    }
}
