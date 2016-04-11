// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_slice"]
#![crate_type = "lib"]
#![feature(optin_builtin_traits)]
#![no_std]

extern crate lrs_base as base;

use base::prelude::*;

/// Extensions for slices.
pub trait SliceExt<T> {
    fn last_to<U>(&self) -> Option<U>
        where T: To<U>;
}

impl<T> SliceExt<T> for [T] {
    fn last_to<U>(&self) -> Option<U>
        where T: To<U>,
    {
        match self.last() {
            Some(r) => Some(r.to()),
            _ => None,
        }
    }
}
