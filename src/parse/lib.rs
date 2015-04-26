// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_parse"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use]
extern crate lrs_core as core;
extern crate lrs_base as base;

#[prelude_import] use base::prelude::*;
use base::{error};

pub mod lrs {
    pub use ::base::lrs::*;
}

mod int;

/// Types that can be parsed.
pub trait Parse {
    /// Tries to parse the object as an object of type `P`.
    fn parse<P: Parsable>(&self) -> Result<P>;
}

impl Parse for [u8] { fn parse<P: Parsable>(&self) -> Result<P> { P::parse_bytes(self)  } }
impl Parse for [i8] { fn parse<P: Parsable>(&self) -> Result<P> { self.as_ref().parse() } }
impl Parse for str  { fn parse<P: Parsable>(&self) -> Result<P> { self.as_ref().parse() } }

impl<'a, T: Parse+?Sized> Parse for &'a T {
    fn parse<P: Parsable>(&self) -> Result<P> {
        (**self).parse()
    }
}

/// Types that are parsable.
pub trait Parsable : Sized {
    /// Tries to parse an initial sequence of bytes as an object of type `P`.
    ///
    /// Returns the object and the number of bytes consumed on success.
    fn parse_bytes_init(bytes: &[u8]) -> Result<(Self, usize)>;

    /// Like `parse_bytes_init` but returns an error if the whole slice wasn't consumed.
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
