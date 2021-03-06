// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use {Write, LowerHex, UpperHex, Debug, Display};

macro_rules! imp {
    ($ty:ident) => {
        impl<T: $ty, E: $ty> $ty for Result<T, E> {
            fn fmt<W: Write>(&self, w: &mut W) -> Result {
                match *self {
                    Ok(ref t) => {
                        try!(w.write_all(b"Ok(").ignore_ok());
                        try!(t.fmt(w));
                        w.write_all(b")").ignore_ok()
                    },
                    Err(ref e) => {
                        try!(w.write_all(b"Err(").ignore_ok());
                        try!(e.fmt(w));
                        w.write_all(b")").ignore_ok()
                    },
                }
            }
        }
    }
}

imp!(LowerHex);
imp!(UpperHex);
imp!(Debug);
imp!(Display);
