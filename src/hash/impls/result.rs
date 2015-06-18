// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use {Hash, Hasher};
use base::into::{Into};

impl<T: Hash, E: Hash> Hash for Result<T, E> {
    fn hash<H: Hasher, S: Into<H::Seed>>(&self, seed: S) -> H::Digest {
        match *self {
            Ok(ref o) => H::hash(o, seed),
            Err(ref e) => H::hash(e, seed),
        }
    }

    fn stateful_hash<H: Hasher>(&self, h: &mut H) {
        match *self {
            Ok(ref o) => o.stateful_hash(h),
            Err(ref e) => e.stateful_hash(h),
        }
    }
}
