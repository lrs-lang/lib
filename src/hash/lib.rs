// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_hash"]
#![crate_type = "lib"]
#![feature(plugin, no_std, const_fn, custom_derive)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use]
extern crate lrs_core as core;
extern crate lrs_base as base;
extern crate lrs_wrapping as wrapping;

mod lrs { pub use base::lrs::*; }

use base::prelude::*;

pub mod xx_hash;

pub trait Hash {
    fn stateful_hash<H: Hasher>(&self, h: &mut H);

    fn stateful_hash_slice<H: Hasher>(val: &[Self], h: &mut H)
        where Self: Sized
    {
        for el in val {
            el.stateful_hash(h);
        }
    }

    fn hash<H: Hasher>(&self, seed: H::Digest) -> H::Digest {
        let mut hasher = H::new(seed);
        self.stateful_hash(&mut hasher);
        hasher.digest()
    }

    fn hash_slice<H: Hasher>(val: &[Self], seed: H::Digest) -> H::Digest
        where Self: Sized
    {
        let mut hasher = H::new(seed);
        for el in val {
            el.stateful_hash(&mut hasher);
        }
        hasher.digest()
    }
}

pub trait Hasher {
    type Digest;

    // Stateful:

    fn new(seed: Self::Digest) -> Self;
    fn reset(&mut self, seed: Self::Digest);

    fn write_bytes (&mut self, val: &[u8] );
    fn write_u8    (&mut self, val: u8    ) { self.write_bytes(val.as_ref()); }
    fn write_u16   (&mut self, val: u16   ) { self.write_bytes(val.as_ref()); }
    fn write_u32   (&mut self, val: u32   ) { self.write_bytes(val.as_ref()); }
    fn write_u64   (&mut self, val: u64   ) { self.write_bytes(val.as_ref()); }
    fn write_usize (&mut self, val: usize ) { self.write_bytes(val.as_ref()); }
    fn write_i8    (&mut self, val: i8    ) { self.write_bytes(val.as_ref()); }
    fn write_i16   (&mut self, val: i16   ) { self.write_bytes(val.as_ref()); }
    fn write_i32   (&mut self, val: i32   ) { self.write_bytes(val.as_ref()); }
    fn write_i64   (&mut self, val: i64   ) { self.write_bytes(val.as_ref()); }
    fn write_isize (&mut self, val: isize ) { self.write_bytes(val.as_ref()); }

    fn digest(&self) -> Self::Digest;

    // Stateless:

    fn hash_bytes( val: &[u8], seed: Self::Digest) -> Self::Digest;
    fn hash_u8(    val: u8,    seed: Self::Digest) -> Self::Digest { Self::hash_bytes(val.as_ref(), seed) }
    fn hash_u16(   val: u16,   seed: Self::Digest) -> Self::Digest { Self::hash_bytes(val.as_ref(), seed) }
    fn hash_u32(   val: u32,   seed: Self::Digest) -> Self::Digest { Self::hash_bytes(val.as_ref(), seed) }
    fn hash_u64(   val: u64,   seed: Self::Digest) -> Self::Digest { Self::hash_bytes(val.as_ref(), seed) }
    fn hash_usize( val: usize, seed: Self::Digest) -> Self::Digest { Self::hash_bytes(val.as_ref(), seed) }
    fn hash_i8(    val: i8,    seed: Self::Digest) -> Self::Digest { Self::hash_bytes(val.as_ref(), seed) }
    fn hash_i16(   val: i16,   seed: Self::Digest) -> Self::Digest { Self::hash_bytes(val.as_ref(), seed) }
    fn hash_i32(   val: i32,   seed: Self::Digest) -> Self::Digest { Self::hash_bytes(val.as_ref(), seed) }
    fn hash_i64(   val: i64,   seed: Self::Digest) -> Self::Digest { Self::hash_bytes(val.as_ref(), seed) }
    fn hash_isize( val: isize, seed: Self::Digest) -> Self::Digest { Self::hash_bytes(val.as_ref(), seed) }
}

mod impls {
    mod num;
    mod slice;
    mod result;
    mod option;
    mod errno;
    mod bool;
    mod char;
    mod tuple;
    mod ptr;
}
