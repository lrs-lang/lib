// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use core::prelude::*;
use core::ops::{Deref};

pub trait AsRef<Target: ?Sized> {
    fn as_ref(&self) -> &Target;
}

pub trait AsMut<Target: ?Sized> {
    fn as_mut(&mut self) -> &mut Target;
}

pub trait ToOwned {
    type Owned;
    fn to_owned(&self) -> Self::Owned;
}

pub enum Cow<'a, Ref: ToOwned+'a+?Sized>
    where Ref::Owned: AsRef<Ref>
{
    Ref(&'a Ref),
    Owned(Ref::Owned),
}

impl<'a, Ref: ToOwned+'a+?Sized> Cow<'a, Ref>
    where Ref::Owned: AsRef<Ref>
{
    pub fn as_mut(&mut self) -> &mut Ref::Owned {
        if self.is_ref() {
            *self = Cow::Owned(self.deref().to_owned());
        }
        match *self {
            Cow::Owned(ref mut o) => o,
            _ => abort!(),
        }
    }

    pub fn into_owned(self) -> Ref::Owned {
        match self {
            Cow::Ref(r) => r.to_owned(),
            Cow::Owned(o) => o,
        }
    }

    fn is_ref(&self) -> bool {
        match *self {
            Cow::Ref(_) => true,
            _ => false,
        }
    }
}

impl<'a, Ref: ToOwned+'a+?Sized> Deref for Cow<'a, Ref>
    where Ref::Owned: AsRef<Ref>
{
    type Target = Ref;
    fn deref(&self) -> &Ref {
        match *self {
            Cow::Ref(r) => r,
            Cow::Owned(ref o) => o.as_ref(),
        }
    }
}
