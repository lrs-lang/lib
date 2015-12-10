// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use vec::{Vec};
use {ToOwned};
use alloc::{MemPool};
use str_two::{String};

impl<T, H> ToOwned<H> for [T]
    where T: Copy,
          H: MemPool,
{
    type Owned = Vec<T, H>;
    fn to_owned_with_pool(&self, pool: H) -> Result<Vec<T, H>> {
        let mut vec = Vec::with_pool(pool);
        try!(vec.reserve(self.len()));
        vec.push_all(self);
        Ok(vec)
    }
}

impl<H> ToOwned<H> for str
    where H: MemPool,
{
    type Owned = String<H>;
    fn to_owned_with_pool(&self, pool: H) -> Result<String<H>> {
        let mut vec = Vec::with_pool(pool);
        try!(vec.reserve(self.len()));
        vec.push_all(self.as_bytes());
        unsafe {
            Ok(String::from_bytes_unchecked(vec))
        }
    }
}
