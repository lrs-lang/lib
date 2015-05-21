// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use core::{mem};
use fmt::{Debug, Display, Write};
use vec::{Vec};
use alloc::{self, Allocator};

/// An owned UTF-8 string.
pub struct String<Heap = alloc::Heap>
    where Heap: Allocator,
{
    data: Vec<u8>,
    _marker: PhantomData<Heap>,
}

impl<H> Deref for String<H>
    where H: Allocator,
{
    type Target = str;
    fn deref(&self) -> &str {
        unsafe { mem::cast(self.data.deref()) }
    }
}

impl<H> Debug for String<H>
    where H: Allocator,
{
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        Debug::fmt(self.deref(), w)
    }
}

impl<H> Display for String<H>
    where H: Allocator,
{
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        Display::fmt(self.deref(), w)
    }
}
