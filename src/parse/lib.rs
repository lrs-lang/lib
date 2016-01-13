// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_parse"]
#![crate_type = "lib"]
#![feature(custom_derive)]
#![no_std]

extern crate lrs_base as base;

use base::prelude::*;
use base::{error};

mod std { pub use base::std::*; }

pub use impls::int::{
    HexU8, HexU16, HexU32, HexU64, HexUsize, OctU8, OctU16, OctU32, OctU64, OctUsize,
    BinU8, BinU16, BinU32, BinU64, BinUsize,
};

mod impls {
    pub mod int;
    pub mod float;
}

/// Types that can be parsed.
pub trait Parse {
    /// Tries to parse the object.
    fn parse<P: Parsable>(&self) -> Result<P>;
}

impl Parse for [u8] { fn parse<P: Parsable>(&self) -> Result<P> { P::parse_bytes(self) } }
impl Parse for [i8] { fn parse<P: Parsable>(&self) -> Result<P> { P::parse_bytes(self.as_ref()) } }
impl Parse for str  { fn parse<P: Parsable>(&self) -> Result<P> { P::parse_bytes(self.as_ref()) } }

impl<'a, T: Parse+?Sized> Parse for &'a T {
    fn parse<P: Parsable>(&self) -> Result<P> {
        (**self).parse()
    }
}

/// Types that are parsable.
pub trait Parsable : Sized {
    /// Tries to parse an initial sequence of bytes as an object of this type.
    ///
    /// [argument, bytes]
    /// The bytes to be parsed.
    ///
    /// [return_value]
    /// Returns the object and the number of bytes consumed.
    fn parse_bytes_init(bytes: &[u8]) -> Result<(Self, usize)>;

    /// Tries to parse a byte slice as an object of this type.
    ///
    /// [argument, bytes]
    /// The bytes to be parsed.
    ///
    /// [return_value]
    /// Returns the object.
    ///
    /// = Remarks
    ///
    /// This fails if the whole sequence cannot be parsed, that is, if `parse_bytes_init`
    /// returns that not all of the bytes were parsed.
    fn parse_bytes(bytes: &[u8]) -> Result<Self> {
        match Self::parse_bytes_init(bytes) {
            Ok((v, l)) => {
                if l == bytes.len() {
                    Ok(v)
                } else {
                    Err(error::InvalidArgument)
                }
            },
            Err(e) => Err(e),
        }
    }
}
