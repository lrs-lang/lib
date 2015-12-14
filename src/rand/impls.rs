// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use {Gen, Rng};

impl Gen for bool {
    fn gen<G: Rng+?Sized>(g: &mut G) -> Result<Self> {
        let v = try!(u8::gen(g));
        Ok(v & 1 == 0)
    }
}

impl<T: Gen> Gen for Option<T> {
    fn gen<G: Rng+?Sized>(g: &mut G) -> Result<Self> {
        if try!(bool::gen(g)) {
            Ok(Some(try!(T::gen(g))))
        } else {
            Ok(None)
        }
    }
}

impl<T, E> Gen for Result<T, E>
    where T: Gen,
          E: Gen,
{
    fn gen<G: Rng+?Sized>(g: &mut G) -> Result<Self> {
        if try!(bool::gen(g)) {
            Ok(Ok(try!(T::gen(g))))
        } else {
            Ok(Err(try!(E::gen(g))))
        }
    }
}
