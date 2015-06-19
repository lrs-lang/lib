// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use {Hash, Hasher};
use base::into::{Into};

impl Hash for char {
    fn stateful_hash<H: Hasher>(&self, h: &mut H) {
        h.write_u32(*self as u32);
    }

    fn stateful_hash_slice<H: Hasher>(val: &[Self], h: &mut H) {
        h.write_bytes(val.as_ref());
    }

    fn hash<H: Hasher, S: Into<H::Seed>>(&self, seed: S) -> H::Digest {
        H::hash_u32(*self as u32, seed)
    }

    fn hash_slice<H: Hasher, S: Into<H::Seed>>(val: &[Self], seed: S) -> H::Digest {
        H::hash_bytes(val.as_ref(), seed)
    }
}
