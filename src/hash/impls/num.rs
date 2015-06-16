// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use {Hash, Hasher};
use wrapping::{W8, W16, W32, W64, Wsize};

macro_rules! impl_num {
    ($ty:ty, $hfn:ident, $wfn:ident) => {
        impl Hash for $ty {
            fn stateful_hash<H: Hasher>(&self, h: &mut H) {
                h.$wfn(*self)
            }

            fn stateful_hash_slice<H: Hasher>(val: &[Self], h: &mut H) {
                h.write_bytes(val.as_ref());
            }

            fn hash<H: Hasher>(&self, seed: H::Digest) -> H::Digest {
                H::$hfn(*self, seed)
            }

            fn hash_slice<H: Hasher>(val: &[Self], seed: H::Digest) -> H::Digest {
                H::hash_bytes(val.as_ref(), seed)
            }
        }
    }
}

impl_num!(u8,    hash_u8,    write_u8);
impl_num!(u16,   hash_u16,   write_u16);
impl_num!(u32,   hash_u32,   write_u32);
impl_num!(u64,   hash_u64,   write_u64);
impl_num!(usize, hash_usize, write_usize);
impl_num!(i8,    hash_i8,    write_i8);
impl_num!(i16,   hash_i16,   write_i16);
impl_num!(i32,   hash_i32,   write_i32);
impl_num!(i64,   hash_i64,   write_i64);
impl_num!(isize, hash_isize, write_isize);

macro_rules! impl_wnum {
    ($ty:ty, $hfn:ident, $wfn:ident) => {
        impl Hash for $ty {
            fn stateful_hash<H: Hasher>(&self, h: &mut H) {
                h.$wfn(**self)
            }

            fn stateful_hash_slice<H: Hasher>(val: &[Self], h: &mut H)
                where Self: Sized
            {
                h.write_bytes(val.as_ref());
            }

            fn hash<H: Hasher>(&self, seed: H::Digest) -> H::Digest {
                H::$hfn(**self, seed)
            }

            fn hash_slice<H: Hasher>(val: &[Self], seed: H::Digest) -> H::Digest
                where Self: Sized
            {
                H::hash_bytes(val.as_ref(), seed)
            }
        }
    }
}

impl_wnum!(W8,    hash_u8,    write_u8);
impl_wnum!(W16,   hash_u16,   write_u16);
impl_wnum!(W32,   hash_u32,   write_u32);
impl_wnum!(W64,   hash_u64,   write_u64);
impl_wnum!(Wsize, hash_usize, write_usize);
