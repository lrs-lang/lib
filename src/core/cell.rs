// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[lang="unsafe_cell"]
#[derive(Copy)]
pub struct Cell<T> {
    pub data: T,
}

impl<T> Cell<T> {
    pub fn ptr(&self) -> *mut T {
        &self.data as *const T as *mut T
    }
}
