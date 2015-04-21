// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_rmo"]
#![crate_type = "lib"]
#![feature(plugin, no_std, optin_builtin_traits)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core    as core;
extern crate linux_base as base;
extern crate linux_fmt     as fmt;
extern crate linux_str_one as str_one;
extern crate linux_vec     as vec;
extern crate linux_str_two as str_two;

#[prelude_import] use base::prelude::*;
use base::rmo::{AsRef, AsMut};
use base::error::{Errno};
use core::ops::{Deref};

mod linux { pub use ::fmt::linux::*; }

mod impls {
    mod slice;
    mod byte_string;
    mod c_string;
    mod no_null_string;
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
