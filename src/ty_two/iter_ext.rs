// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::iter::{Iterator};
use vec::{Vec};

pub trait IteratorExt2 : Iterator {
    fn collect(self) -> Vec<Self::Item>;
}

impl<T: Iterator> IteratorExt2 for T {
    fn collect(self) -> Vec<Self::Item> {
        let mut vec = Vec::new();
        vec.extend(self);
        vec
    }
}
