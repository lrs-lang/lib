// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use {Hash, Hasher};

macro_rules! impl_ptr {
    ($t:ty) => {
        impl<T> Hash for $t {
            fn stateful_hash<H: Hasher>(&self, h: &mut H) {
                h.write_usize(*self as usize)
            }

            fn stateful_hash_slice<H: Hasher>(val: &[Self], h: &mut H) {
                h.write_bytes(val.as_ref());
            }

            fn hash<H: Hasher, S: Into<H::Seed>>(&self, seed: S) -> H::Digest {
                H::hash_usize(*self as usize, seed)
            }

            fn hash_slice<H: Hasher, S: Into<H::Seed>>(val: &[Self],
                                                       seed: S) -> H::Digest {
                H::hash_bytes(val.as_ref(), seed)
            }
        }
    }
}

impl_ptr!(*const T);
impl_ptr!(*mut T);
