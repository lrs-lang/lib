// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use {Write, LowerHex, UpperHex, Debug, Display};

macro_rules! imp {
    ($ty:ident) => {
        impl<T: $ty> $ty for Option<T> {
            fn fmt<W: Write>(&self, w: &mut W) -> Result {
                match *self {
                    Some(ref t) => {
                        try!(w.write_all(b"Some(").ignore_ok());
                        try!(t.fmt(w));
                        w.write_all(b")").ignore_ok()
                    },
                    _ => w.write_all(b"None").ignore_ok()
                }
            }
        }
    }
}

imp!(LowerHex);
imp!(UpperHex);
imp!(Debug);
imp!(Display);
