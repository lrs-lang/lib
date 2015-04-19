// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use core::prelude::*;
#[prelude_import] use prelude::*;
use {error};
use rmo::{AsRef};

mod int;

pub trait Parse {
    fn parse<P: Parsable>(&self) -> Result<P>;
}

impl Parse for [u8] { fn parse<P: Parsable>(&self) -> Result<P> { P::parse_bytes(self)    } }
impl Parse for [i8] { fn parse<P: Parsable>(&self) -> Result<P> { self.as_ref().parse() } }
impl Parse for str  { fn parse<P: Parsable>(&self) -> Result<P> { self.as_ref().parse() } }

impl<'a, T: Parse+?Sized> Parse for &'a T {
    fn parse<P: Parsable>(&self) -> Result<P> {
        (**self).parse()
    }
}

pub trait Parsable : Sized {
    fn parse_bytes_init(bytes: &[u8]) -> Result<(Self, usize)>;
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
