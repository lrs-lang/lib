// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use {mem};
use marker::{Copy};

#[allow(non_camel_case_types)]
pub struct d8(u8);

impl Copy for d8 { }
// unsafe impl Pod for d8 { }
// 
// #[fundamental]
// pub trait NotD8 { }
// impl NotD8 for .. { }
// impl !NotD8 for d8 { }

impl d8 {
    pub const fn new(byte: u8) -> d8 {
        d8(byte)
    }

    pub unsafe fn as_byte(&self) -> &u8 {
        &self.0
    }

    pub unsafe fn as_mut_byte(&mut self) -> &mut u8 {
        &mut self.0
    }
}

pub trait DataSlice {
    unsafe fn as_bytes(&self) -> &[u8];
    unsafe fn as_mut_bytes(&mut self) -> &mut [u8];
    fn align_for<T>(&self) -> &[d8];
    fn align_for_mut<T>(&mut self) -> &mut [d8];
}

impl DataSlice for [d8] {
    unsafe fn as_bytes(&self) -> &[u8] {
        mem::cast(self)
    }

    unsafe fn as_mut_bytes(&mut self) -> &mut [u8] {
        mem::cast(self)
    }

    fn align_for<T>(&self) -> &[d8] {
        unsafe { mem::cast(mem::align_for::<T>(self.as_bytes())) }
    }

    fn align_for_mut<T>(&mut self) -> &mut [d8] {
        unsafe { mem::cast(mem::align_for_mut::<T>(self.as_mut_bytes())) }
    }
}
