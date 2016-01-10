// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use {VarArg};
use core::{ptr, mem};

#[repr(C)]
pub struct VarArgs {
    stack: *mut u8,
}

impl VarArgs {
    pub unsafe fn get<T: VarArg>(&mut self) -> T {
        let val = ptr::read(self.stack as *mut T);
        match mem::size_of::<T>() {
            0...4 => self.stack = self.stack.add(4),
            l => self.stack = self.stack.add(l),
        }
        val
    }
}
