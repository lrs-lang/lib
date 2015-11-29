// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use {Node};
use core::{intrinsics};

// The longest path in a LLRB tree is bounded by 2*ln(n). Since n cannot be larger
// than the number addresses, we get the bound below.
#[cfg(target_pointer_width = "32")] pub const LONGEST_PATH: usize = 64;
#[cfg(target_pointer_width = "64")] pub const LONGEST_PATH: usize = 128;

pub struct Path<'a, U: Copy> {
    buf: [(&'a Node, U); LONGEST_PATH],
    pos: usize,
}

impl<'a, U: Copy> Path<'a, U> {
    /// Creates a new path.
    ///
    /// = Remarks
    ///
    /// The `reset` method must be called before the path is used or the behavior is
    /// undefnied.
    pub unsafe fn new() -> Path<'a, U> {
        intrinsics::uninit()
    }

    /// Resets the length of the path to `0`.
    pub fn reset(&mut self) {
        self.pos = 0;
    }

    /// Adds a node to the path.
    ///
    /// = Remarks
    ///
    /// This is unsafe because it doesn't check for overflow. The logest allowed path
    /// length is `LONGEST_PATH`.
    pub unsafe fn push(&mut self, node: &'a Node, ord: U) {
        *self.buf.as_mut_ptr().add(self.pos) = (node, ord);
        self.pos += 1;
    }

    /// Removes an element from the path.
    pub fn pop(&mut self) -> Option<(&'a Node, U)> {
        unsafe {
            if self.pos > 0 {
                self.pos -= 1;
                Some(*self.buf.as_ptr().add(self.pos))
            } else {
                None
            }
        }
    }
}
