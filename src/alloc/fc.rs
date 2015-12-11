// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use {MemPool};

extern {
    #[link_name = "llvm.expect.i1"]
    fn expect(val: bool, expected: bool) -> bool;
}

#[derive(Copy)]
enum State {
    None,
    First,
    Second,
}

impl State {
    fn none(self) -> bool {
        match self {
            State::None => true,
            _ => false,
        }
    }
}

#[derive(Copy)]
pub struct FcPool<P1, P2>
    where P1: MemPool,
          P2: MemPool,
{
    state: State,
    p1: P1,
    p2: P2,
}

impl<P1, P2> Default for FcPool<P1, P2>
    where P1: MemPool+Default,
          P2: MemPool+Default,
{
    fn default() -> Self {
        FcPool {
            state: State::None,
            p1: P1::default(),
            p2: P2::default(),
        }
    }
}

impl<P1, P2> FcPool<P1, P2>
    where P1: MemPool,
          P2: MemPool,
{
    pub fn new(pool1: P1, pool2: P2) -> FcPool<P1, P2> {
        FcPool {
            state: State::None,
            p1: pool1,
            p2: pool2,
        }
    }
}

impl<P1, P2> MemPool for FcPool<P1, P2>
    where P1: MemPool,
          P2: MemPool,
{
    unsafe fn alloc(&mut self, size: usize, align: usize) -> Result<*mut u8> {
        if expect(self.state.none(), false) {
            if let Ok(ptr) = self.p1.alloc(size, align) {
                self.state = State::First;
                Ok(ptr)
            } else {
                self.state = State::Second;
                self.p2.alloc(size, align)
            }
        } else {
            if let State::First = self.state {
                self.p1.alloc(size, align)
            } else {
                self.p2.alloc(size, align)
            }
        }
    }

    unsafe fn free(&mut self, ptr: *mut u8, size: usize, align: usize) {
        if let State::First = self.state {
            self.p1.free(ptr, size, align)
        } else {
            self.p2.free(ptr, size, align)
        }
    }

    unsafe fn realloc(&mut self, ptr: *mut u8, oldsize: usize, newsize: usize,
                      align: usize) -> Result<*mut u8> {
        if let State::First = self.state {
            self.p1.realloc(ptr, oldsize, newsize, align)
        } else {
            self.p2.realloc(ptr, oldsize, newsize, align)
        }
    }
}
