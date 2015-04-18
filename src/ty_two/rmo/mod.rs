// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use core::prelude::*;
#[prelude_import] use ty_one::prelude::*;
use ty_one::error::{Errno};
use core::ops::{Deref};

mod impls {
    mod slice;
}

pub trait AsRef<Target: ?Sized> {
    fn as_ref(&self) -> &Target;
}

pub trait AsMut<Target: ?Sized> {
    fn as_mut(&mut self) -> &mut Target;
}

pub trait ToOwned {
    type Owned;
    fn to_owned(&self) -> Result<Self::Owned>;
}

pub enum Rmo<'a, Ref: ToOwned+'a+?Sized>
    where Ref::Owned: AsRef<Ref>
{
    Ref(&'a Ref),
    Mut(&'a mut Ref),
    Owned(Ref::Owned),
}

impl<'a, Ref: ToOwned+'a+?Sized> Rmo<'a, Ref>
    where Ref::Owned: AsRef<Ref>
{
    pub fn as_mut(&mut self) -> Result<&mut Ref> where Ref::Owned: AsMut<Ref> {
        if self.is_ref() {
            *self = Rmo::Owned(try!(self.deref().to_owned()));
        }
        match *self {
            Rmo::Mut(ref mut m) => Ok(m),
            Rmo::Owned(ref mut o) => Ok(o.as_mut()),
            _ => abort!(),
        }
    }

    pub fn into_owned(self) -> Result<Ref::Owned, (Errno, Rmo<'a, Ref>)> {
        match self {
            Rmo::Ref(r) => {
                match r.to_owned() {
                    Ok(o) => Ok(o),
                    Err(e) => Err((e, Rmo::Ref(r))),
                }
            },
            Rmo::Mut(r) => {
                match r.to_owned() {
                    Ok(o) => Ok(o),
                    Err(e) => Err((e, Rmo::Mut(r))),
                }
            },
            Rmo::Owned(o) => Ok(o),
        }
    }

    fn is_ref(&self) -> bool {
        match *self {
            Rmo::Ref(_) => true,
            _ => false,
        }
    }
}

impl<'a, Ref: ToOwned+'a+?Sized> Deref for Rmo<'a, Ref>
    where Ref::Owned: AsRef<Ref>
{
    type Target = Ref;
    fn deref(&self) -> &Ref {
        match *self {
            Rmo::Ref(r) => r,
            Rmo::Mut(ref m) => m,
            Rmo::Owned(ref o) => o.as_ref(),
        }
    }
}
