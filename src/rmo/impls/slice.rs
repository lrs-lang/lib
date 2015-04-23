// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use vec::{SVec};
use {ToOwned};

impl<T: Copy> ToOwned for [T] {
    type Owned = SVec<T>;
    fn to_owned(&self) -> Result<SVec<T>> {
        let mut vec = try!(SVec::with_capacity(self.len()));
        vec.push_all(self);
        Ok(vec)
    }
}
