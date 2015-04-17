// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use core::prelude::*;
use core::cell::{Cell};

pub struct CopyCell<T: Copy> {
    data: Cell<T>,
}

impl<T: Copy> CopyCell<T> {
    pub fn new(data: T) -> CopyCell<T> {
        CopyCell { data: Cell { data: data } }
    }

    pub fn get(&self) -> T {
        self.data.data
    }

    pub fn set(&self, data: T) {
        unsafe { *self.data.ptr() = data }
    }
}
