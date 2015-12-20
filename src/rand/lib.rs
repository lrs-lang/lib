// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_rand"]
#![crate_type = "lib"]
#![feature(no_std, custom_derive)]
#![no_std]

extern crate lrs_base as base;
extern crate lrs_io as io;
extern crate lrs_fmt as fmt;
#[cfg(not(freestanding))] extern crate lrs_cty as cty;
#[cfg(not(freestanding))] extern crate lrs_syscall as syscall;
#[cfg(not(freestanding))] extern crate lrs_rv as rv;
#[cfg(not(freestanding))] extern crate lrs_kernel as kernel;
#[cfg(not(freestanding))] extern crate lrs_file as file;

use base::prelude::*;
use base::{error};
use core::{mem};
use core::cmp::{Ordering};
use io::{Read};

#[cfg(not(freestanding))] pub use getrandom::{GetRandom, GetUrandom};
#[cfg(not(freestanding))] pub use devrandom::{DevRandom, DevUrandom};
pub use xorshift::{Xorshift};

mod std { pub use fmt::std::*; }

#[cfg(not(freestanding))] mod getrandom;
#[cfg(not(freestanding))] mod devrandom;
mod xorshift;

mod impls;

pub trait Gen: Sized {
    fn gen<G: Rng+?Sized>(g: &mut G) -> Result<Self>;
}

impl<T: Pod> Gen for T {
    fn gen<G: Rng+?Sized>(g: &mut G) -> Result<T> {
        let mut t: T = unsafe { mem::uninit() };
        if try!(g.read_all(t.as_mut())) < mem::size_of::<T>() {
            Err(error::WouldBlock)
        } else {
            Ok(t)
        }
    }
}

pub trait Rng: Read {
    fn gen<T: Gen>(&mut self) -> Result<T> {
        T::gen(self)
    }

    fn next_u32(&mut self) -> Result<u32> {
        self.gen()
    }

    fn shuffle<T>(&mut self, s: &mut [T]) {
        s.sort_by(|_, _| {
            let val = self.next_u32().unwrap();
            match val % 3 {
                0 => Ordering::Less,
                1 => Ordering::Equal,
                _ => Ordering::Greater,
            }
        });
    }
}
