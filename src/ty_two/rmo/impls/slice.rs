// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use ty_one::prelude::*;
use vec::{Vec};
use rmo::{ToOwned};

impl<T: Copy> ToOwned for [T] {
    type Owned = Vec<T>;
    fn to_owned(&self) -> Result<Vec<T>> {
        let mut vec = try!(Vec::with_capacity(self.len()));
        vec.push_all(self);
        Ok(vec)
    }
}
