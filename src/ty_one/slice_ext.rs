// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::{Iterator};

pub trait SliceExt {
    fn split<'a>(&'a self) -> Split<'a>;
}

impl SliceExt for str {
    fn split<'a, F>(&'a self, f: F) -> Split<'a, F>
        F: FnMut(
    {
        Split { val: self }
    }
}

struct Split<'a> {
    val: &'a str,

}

impl Iterator for Split
