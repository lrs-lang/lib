// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_fmt"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(lrs_core_plugin)]
#![no_std]

extern crate lrs_base as base;
extern crate lrs_io as io;

use base::prelude::*;

pub use io::{Write};

pub mod std {
    pub use base::std::*;
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

macro_rules! impl_slice {
    ($name:ident, $fmt:expr) => {
        impl<T: $name> $name for [T] {
            fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
                try!(write!(w, "["));
                if self.len() > 0 {
                    for el in &self[..self.len() - 1] {
                        try!(write!(w, concat!($fmt, ", "), el));
                    }
                    try!(write!(w, $fmt, &self[self.len() - 1]));
                }
                write!(w, "]");
                Ok(())
            }
        }
    }
}

impl_slice!(Debug, "{:?}");
impl_slice!(LowerHex, "{:x}");
impl_slice!(UpperHex, "{:X}");
