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
pub trait Sync { }

impl Sync for .. { }

pub trait Send { }

impl Send for .. { }

#[lang = "phantom_data"]
pub struct PhantomData<T: ?Sized>;

impl<T> Copy for PhantomData<T> { }
impl<T> Clone for PhantomData<T> { fn clone(&self) -> PhantomData<T> { *self } }
impl<T> Eq for PhantomData<T> { fn eq(&self, _: &PhantomData<T>) -> bool { true } }
