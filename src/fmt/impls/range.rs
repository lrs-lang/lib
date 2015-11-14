// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::ops::{Range, RangeTo, RangeFrom, RangeFull};
use base::result::{Result};
use {Debug, Write};

impl<T> Debug for Range<T>
    where T: Debug,
{
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        try!(self.start.fmt(w));
        try!(w.write_all(b".."));
        self.end.fmt(w)
    }
}

impl<T> Debug for RangeTo<T>
    where T: Debug,
{
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        try!(w.write_all(b".."));
        self.end.fmt(w)
    }
}

impl<T> Debug for RangeFrom<T>
    where T: Debug,
{
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        try!(self.start.fmt(w));
        w.write_all(b"..").ignore_ok()
    }
}

impl Debug for RangeFull {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        w.write_all(b"..").ignore_ok()
    }
}
