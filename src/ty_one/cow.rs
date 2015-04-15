// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::prelude::*;

pub trait AsRef<Target: ?Sized> {
    fn as_ref(&self) -> &Target;
}

pub trait AsMut<Target: ?Sized> : AsRef<Target> {
    fn as_mut(&mut self) -> &mut Target;
}

pub trait Into<T> {
    fn into(&self) -> T;
}

pub enum Cow<'a, Owned: AsRef<Borrowed>, Borrowed: Into<Owned>+'a+?Sized> {
    Borrowed(&'a Borrowed),
    Owned(Owned),
}
