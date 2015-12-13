// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_rmo"]
#![crate_type = "lib"]
#![feature(no_std)]
#![no_std]

extern crate lrs_base     as base;
extern crate lrs_fmt      as fmt;
extern crate lrs_str_one  as str_one;
extern crate lrs_vec      as vec;
extern crate lrs_str_two  as str_two;
extern crate lrs_alloc    as alloc;
extern crate lrs_arch_fns as arch_fns;

use base::prelude::*;

mod std { pub use ::fmt::std::*; }

mod impls {
    mod c_string;
}

/// A container that contains either a borrowed, a mutably borrowed, or an owned version
/// of a type.
pub enum Rmo<'a, Ref: ?Sized+'a, Owned>
    where Owned: AsRef<Ref>,
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
    Owned(Owned),
}

pub trait ToRmo<D, Ref: ?Sized, Owned>
    where Owned: AsRef<Ref>,
{
    fn to_rmo_with<'a>(&'a self, data: D) -> Result<Rmo<'a, Ref, Owned>>;

    fn to_rmo<'a>(&'a self) -> Result<Rmo<'a, Ref, Owned>>
        where D: OutOf,
    {
        self.to_rmo_with(D::out_of(()))
    }
}

impl<'a, D, Ref: ?Sized, Owned, T: ?Sized> ToRmo<D, Ref, Owned> for &'a T
    where T: ToRmo<D, Ref, Owned>,
          Owned: AsRef<Ref>,
{
    fn to_rmo_with<'b>(&'b self, data: D) -> Result<Rmo<'b, Ref, Owned>> {
        (**self).to_rmo_with(data)
    }
}

impl<'a, D, Ref: ?Sized, Owned, T: ?Sized> ToRmo<D, Ref, Owned> for &'a mut T
    where T: ToRmo<D, Ref, Owned>,
          Owned: AsRef<Ref>,
{
    fn to_rmo_with<'b>(&'b self, data: D) -> Result<Rmo<'b, Ref, Owned>> {
        (**self).to_rmo_with(data)
    }
}

impl<'a, Ref: ?Sized, Owned> Rmo<'a, Ref, Owned>
    where Ref: 'a,
          Owned: AsRef<Ref>,
{
    /// Returns a mutable reference to the contained object.
    ///
    /// = Remarks
    ///
    /// In the owned and mutably borrowed cases, this is a no-op and always succeeds. In
    /// the borrowed case, the object is first converted into its owned version.
    pub fn as_mut(&mut self) -> Result<&mut Ref>
        where Ref: TryTo<Owned>,
              Owned: AsMut<Ref>
    {
        if self.is_ref() {
            *self = Rmo::Owned(try!((**self).try_to()));
        }
        match *self {
            Rmo::Mut(ref mut m) => Ok(m),
            Rmo::Owned(ref mut o) => Ok(o.as_mut()),
            _ => abort!(),
        }
    }

    /// Turns the object into an owned variant.
    pub fn to_owned(&mut self) -> Result
        where Ref: TryTo<Owned>
    {
        if let Rmo::Owned(_) = *self {
            Ok(())
        } else {
            *self = Rmo::Owned(try!((**self).try_to()));
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
    pub fn into_owned(mut self) -> Owned
        where Ref: TryTo<Owned>
    {
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

impl<'a, Ref: ?Sized+'a, Owned> Deref for Rmo<'a, Ref, Owned>
    where Owned: AsRef<Ref>,
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
