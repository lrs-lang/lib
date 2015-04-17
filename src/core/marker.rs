// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ops::{Eq};
use clone::{Clone};

#[lang = "sized"]
pub trait Sized { }

impl Sized for .. { }

#[lang = "copy"]
pub trait Copy { }

#[lang = "sync"]
pub unsafe trait Sync { }

unsafe impl Sync for .. { }

impl<T> !Sync for *const T { }
impl<T> !Sync for *mut T { }

pub struct NoSync;

impl !Sync for NoSync { }

pub unsafe trait Send { }

unsafe impl Send for .. { }

impl<T> !Send for *const T { }
impl<T> !Send for *mut T { }

pub struct NoSend;

impl !Send for NoSend { }

#[lang = "phantom_data"]
pub struct PhantomData<T: ?Sized>;

impl<T> Copy for PhantomData<T> { }
impl<T> Clone for PhantomData<T> { fn clone(&self) -> PhantomData<T> { *self } }
impl<T> Eq for PhantomData<T> { fn eq(&self, _: &PhantomData<T>) -> bool { true } }
