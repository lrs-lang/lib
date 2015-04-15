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
extern crate linux_error as error;
extern crate linux_io as io;

#[prelude_import] use core::prelude::*;
use io::{Write};

pub use num::{format_u64};

mod num;
mod str;

pub type Result = core::prelude::Result<(), error::Errno>;

pub trait Debug {
    fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result;
}

impl<'a, T: Debug+?Sized> Debug for &'a T {
    fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result {
        (**self).fmt(w)
    }
}

impl<'a, T: Debug+?Sized> Debug for &'a mut T {
    fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result {
        (**self).fmt(w)
    }
}

pub trait Display {
    fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result;
}

impl<'a, T: Display+?Sized> Display for &'a T {
    fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result {
        (**self).fmt(w)
    }
}

impl<'a, T: Display+?Sized> Display for &'a mut T {
    fn fmt<W: Write+?Sized>(&self, w: &mut W) -> Result {
        (**self).fmt(w)
    }
}

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
//    extern {
//        fn write(fd: i32, ptr: *const u8, len: u64);
//    }
//    let mut buf = [0; 200];
//    write!(&mut buf[..], "hello {:?}\n", "wörld");
//    unsafe { write(1, buf.as_ptr(), 200); }
//}
