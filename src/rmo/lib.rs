// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_rmo"]
#![crate_type = "lib"]
#![feature(plugin, no_std, optin_builtin_traits)]
#![plugin(lrs_core_plugin)]
#![no_std]

extern crate lrs_base    as base;
extern crate lrs_fmt     as fmt;
extern crate lrs_str_one as str_one;
extern crate lrs_vec     as vec;
extern crate lrs_str_two as str_two;
extern crate lrs_alloc   as alloc;

use base::prelude::*;
use alloc::{MemPool};

mod std { pub use ::fmt::std::*; }

mod impls {
    mod slice;
    mod byte_string;
    mod c_string;
    mod no_null_string;
}

// We'd like to define ToOwned together with AsRef and AsMut down below but coherence
// rules seem to make this impossible.

/// Objects that can be converted into an owned version.
pub trait ToOwned<H = alloc::Heap>
    where H: MemPool,
{
    /// The type of the owned version.
    type Owned;

    /// Converts the object into its owned version.
    fn to_owned(&self) -> Result<Self::Owned>
        where H: Default,
    {
        self.to_owned_with_pool(H::default())
    }

    /// Converts the object into its owned version.
    fn to_owned_with_pool(&self, pool: H) -> Result<Self::Owned>;
}

/// A container that contains either a borrowed, a mutably borrowed, or an owned version
/// of a type.
pub enum Rmo<'a, Ref: ?Sized+'a, H = alloc::Heap>
    where H: MemPool+Default,
          Ref: ToOwned<H>,
          Ref::Owned: AsRef<Ref>,
{
    /// The borrowed case.
    ///
    /// [field, 1]
    /// A borrowed reference.
    Ref(&'a Ref),

    /// The mutably borrowed case.
    ///
    /// [field, 1]
    /// A mutably borrowed reference.
    Mut(&'a mut Ref),

    /// The owned case.
    ///
    /// [field, 1]
    /// An owned object.
    Owned(Ref::Owned),
}

impl<'a, Ref: ?Sized+'a, H> Rmo<'a, Ref, H>
    where H: MemPool+Default,
          Ref: ToOwned<H>,
          Ref::Owned: AsRef<Ref>,
{
    /// Returns a mutable reference to the contained object.
    ///
    /// = Remarks
    ///
    /// In the owned and mutably borrowed cases, this is a no-op and always succeeds. In
    /// the borrowed case, the object is first converted into its owned version.
    pub fn as_mut(&mut self) -> Result<&mut Ref> where Ref::Owned: AsMut<Ref> {
        if self.is_ref() {
            *self = Rmo::Owned(try!((**self).to_owned()));
        }
        match *self {
            Rmo::Mut(ref mut m) => Ok(m),
            Rmo::Owned(ref mut o) => Ok(o.as_mut()),
            _ => abort!(),
        }
    }

    /// Turns the object into an owned variant.
    pub fn to_owned(&mut self) -> Result {
        if let Rmo::Owned(_) = *self {
            Ok(())
        } else {
            *self = Rmo::Owned(try!((**self).to_owned()));
            Ok(())
        }
    }

    /// Unwraps the owned variant of this object.
    ///
    /// = Remarks
    ///
    /// :to: link:lrs::rmo::Rmo::to_owned[to_owned]
    ///
    /// If the object does not contain an owned variant, the method first tries to convert
    /// it into an owned variant. If this fails, the process is aborted. To avoid this,
    /// first use the {to} method.
    ///
    /// = See also
    ///
    /// * {to}
    pub fn into_owned(mut self) -> Ref::Owned {
        self.to_owned().unwrap();
        if let Rmo::Owned(o) = self {
            o
        } else {
            abort!()
        }
    }

    fn is_ref(&self) -> bool {
        match *self {
            Rmo::Ref(_) => true,
            _ => false,
        }
    }
}

impl<'a, Ref: ?Sized+'a, H> Deref for Rmo<'a, Ref, H>
    where H: MemPool+Default,
          Ref: ToOwned<H>,
          Ref::Owned: AsRef<Ref>,
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
