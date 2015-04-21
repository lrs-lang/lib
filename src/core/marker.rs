// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ops::{Eq};

/// Objects with a size known at compile time.
///
/// E.g. not `[T]` or `str` but `&[T]` and `&str`.
#[lang = "sized"]
pub trait Sized { }

impl Sized for .. { }

/// Objects that can safely be copied via `memcpy`.
#[lang = "copy"]
pub trait Copy { }

/// Objects that can safely be accessed immutable from multiple threads concurrently.
#[lang = "sync"]
pub unsafe trait Sync { }

unsafe impl Sync for .. { }

impl<T> !Sync for *const T { }
impl<T> !Sync for *mut T { }

/// A helper struct that can be embedded in other objects and makes them `!Sync`.
pub struct NoSync;

impl !Sync for NoSync { }

/// Objects whose ownership can safely be moved from one thread to another.
pub unsafe trait Send { }

unsafe impl Send for .. { }

impl<T> !Send for *const T { }
impl<T> !Send for *mut T { }

/// A helper struct that can be embedded in other objects and makes them `!Sync`.
pub struct NoSend;

impl !Send for NoSend { }

/// Objects that can be leaked without causing memory unsafety.
///
/// E.g. thread join guards cannot be leaked because the thread could retain references to
/// objects that have already been dropped.
pub unsafe trait Leak { }

unsafe impl Leak for .. { }

/// TODO: Document this.
#[lang = "phantom_data"]
pub struct PhantomData<T: ?Sized>;

impl<T> Copy for PhantomData<T> { }
impl<T> Eq for PhantomData<T> { fn eq(&self, _: &PhantomData<T>) -> bool { true } }
