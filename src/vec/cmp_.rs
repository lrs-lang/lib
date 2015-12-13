// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use {Vec};
use alloc::{MemPool};
use core::cmp::{Eq};

impl<T, H1: ?Sized, H2: ?Sized> Eq<Vec<T, H1>> for Vec<T, H2>
    where T: Eq,
          H1: MemPool,
          H2: MemPool,
{
    fn eq(&self, other: &Vec<T, H1>) -> bool {
        self.deref().eq(other.deref())
    }
    fn ne(&self, other: &Vec<T, H1>) -> bool {
        self.deref().ne(other.deref())
    }
}

impl<T, H: ?Sized> Eq<[T]> for Vec<T, H>
    where T: Eq,
          H: MemPool,
{
    fn eq(&self, other: &[T]) -> bool {
        self.deref().eq(other)
    }
    fn ne(&self, other: &[T]) -> bool {
        self.deref().ne(other)
    }
}
