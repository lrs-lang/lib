// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_fmt"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use]
extern crate lrs_core as core;
extern crate lrs_base as base;
extern crate lrs_io as io;

#[prelude_import] use base::prelude::*;

pub use io::{Write};

pub mod lrs {
    pub use ::base::lrs::*;
    pub mod fmt { pub use {LowerHex, UpperHex, Debug, Display}; }
}

pub mod impls {
    pub mod num;
    pub mod str;
    pub mod option;
    pub mod boolean;
    pub mod result;
    pub mod errno;
    pub mod unit;
    pub mod tuple;
}

macro_rules! fmt_var {
    ($($(#[$meta:meta])* ty $name:ident)*) => {
        $(
            $(#[$meta])*
            pub trait $name {
                /// Formats the object into the writer.
                fn fmt<W: Write>(&self, w: &mut W) -> Result;
            }

            impl<'a, T: $name+?Sized> $name for &'a T {
                fn fmt<W: Write>(&self, w: &mut W) -> Result {
                    (**self).fmt(w)
                }
            }

            impl<'a, T: $name+?Sized> $name for &'a mut T {
                fn fmt<W: Write>(&self, w: &mut W) -> Result {
                    (**self).fmt(w)
                }
            }
        )*
    }
}

fmt_var! {
    #[doc = "Objects that can be formatted in a \"lower hex\" form."]
    ty LowerHex

    #[doc = "Objects that can be formatted in a \"upper hex\" form."]
    ty UpperHex

    #[doc = "Objects that can be formatted in a \"debug\" form."]
    ty Debug

    #[doc = "Objects that can be formatted in a \"display\" form."]
    ty Display
}

mod fmt {
    pub use {Debug, Display};
}

impl<T: Debug> Debug for [T] {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        try!(write!(w, "["));
        if self.len() > 0 {
            for el in &self[..self.len() - 1] {
                try!(write!(w, "{:?}, ", el));
            }
            try!(write!(w, "{:?}", &self[self.len() - 1]));
        }
        write!(w, "]");
        Ok(())
    }
}
