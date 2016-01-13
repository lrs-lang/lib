// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use io::{Write};
use {Debug, Display};

impl Debug for f32 {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        Debug::fmt(&(*self as f64), w)
    }
}

impl Display for f32 {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        Debug::fmt(self, w)
    }
}

impl Display for f64 {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        Debug::fmt(self, w)
    }
}

impl Debug for f64 {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        let mut val = *self;
        if val < 0.0 {
            try!(w.write_all(b"-"));
            val = val.abs();
        }
        try!(Debug::fmt(&(val as u64), w));
        try!(w.write_all(b"."));
        val -= val as u64 as f64;
        val *= 1_000_000_000_000.0;
        Debug::fmt(&(val as u64), w)
    }
}
