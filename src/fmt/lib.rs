// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_fmt"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core as core;
extern crate linux_ty_one as ty_one;
extern crate linux_io as io;

#[prelude_import] use core::prelude::*;
#[prelude_import] use ty_one::prelude::*;
use io::{Write};

pub use impls::num::{format_u64};

pub mod linux {
    pub use ::ty_one::linux::*;
    pub mod fmt { pub use {LowerHex, UpperHex, Debug, Display}; }
}

mod impls {
    pub mod num;
    pub mod str;
    pub mod byte_str;
}

macro_rules! fmt_var {
    ($name:ident) => {
        pub trait $name {
            fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result;
        }

        impl<'a, T: $name+?Sized> $name for &'a T {
            fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result {
                (**self).fmt(w)
            }
        }

        impl<'a, T: $name+?Sized> $name for &'a mut T {
            fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result {
                (**self).fmt(w)
            }
        }
    }
}

fmt_var!(LowerHex);
fmt_var!(UpperHex);
fmt_var!(Debug);
fmt_var!(Display);

mod fmt {
    pub use {Debug, Display};
}

impl<T: Debug> Debug for [T] {
    fn fmt<W: Write+?Sized>(&self, mut w: &mut W) -> Result {
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

//fn main() {
//    use ty_one::byte_str::{AsByteStr};
//    use stdio::{Stdout};
//    let bs = b"al\xFFien".as_byte_str();
//    println!("ayy {:?} lmao", bs);
//}
