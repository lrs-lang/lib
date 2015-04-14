// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ops::{Eq};

macro_rules! array_impl {
    ($($size:expr)+) => {
        $(
            impl<T: Eq> Eq for [T; $size] {
                fn eq(&self, other: &[T; $size]) -> bool {
                    for i in 0usize..$size {
                        if self[i] != other[i] {
                            return false;
                        }
                    }
                    true
                }
            }
        )+
    }
}

array_impl!(0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28
            29 30 31 32);
