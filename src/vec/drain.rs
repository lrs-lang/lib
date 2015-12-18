// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use {Vec};
use core::{mem, ptr};
use core::ops::{Range};
use core::marker::{Leak};
use alloc::{MemPool};

impl<T, H: ?Sized> Vec<T, H>
    where H: MemPool,
{
    /// Removes a range from a vector, making its elements available through an iterator.
    ///
    /// [argument, range]
    /// The range to remove.
    ///
    /// [return_value]
    /// Returns an iterator over the elements of the range.
    ///
    /// = Remarks
    ///
    /// If the range is not increasing or goes beyond the bounds of the vector, the
    /// process is aborted.
    pub fn drain<'a, R>(&mut self, range: R) -> Drainer<'a, T>
        where R: Into<Range<Option<usize>>>,
    {
        let Range { start, end } = range.into();
        let (start, end) = match (start, end) {
            (Some(s), Some(e)) => (s, e),
            (Some(s), None) => (s, self.len()),
            (None, Some(e)) => (0, e),
            (None, None) => (0, self.len()),
        };
        if start > end || end > self.len() {
            abort!();
        }
        let old_len = self.len();
        self.len -= end - start;
        if mem::size_of::<T>() != 0 {
            unsafe {
                Drainer {
                    start: self.ptr.get().add(start),
                    cur: self.ptr.get().add(start),
                    end: self.ptr.get().add(end),
                    vec_end: self.ptr.get().add(old_len),
                    _data: PhantomData,
                }
            }
        } else {
            Drainer {
                start: start as *const T,
                cur: start as *const T,
                end: end as *const T,
                vec_end: old_len as *const T,
                _data: PhantomData,
            }
        }
    }
}

/// An iterator over the elements of a subrange of the vector.
pub struct Drainer<'a, T> {
    start: *const T,
    cur: *const T,
    end: *const T,
    vec_end: *const T,
    _data: PhantomData<(&'a (), T)>,
}

impl<'a, T> !Leak for Drainer<'a, T> { }

impl<'a, T> Iterator for Drainer<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.cur != self.end {
            if mem::size_of::<T>() != 0 {
                unsafe {
                    self.cur = self.cur.add(1);
                    Some(ptr::read(self.cur.sub(1)))
                }
            } else {
                self.cur = (self.cur as usize + 1) as *const _;
                unsafe { Some(mem::unsafe_zeroed()) }
            }
        } else {
            None
        }
    }
}

impl<'a, T> Drop for Drainer<'a, T> {
    fn drop(&mut self) {
        if mem::needs_drop::<T>() {
            if mem::size_of::<T>() != 0 {
                while self.cur != self.end {
                    unsafe {
                        ptr::drop(self.cur as *mut T);
                        self.cur = self.cur.add(1);
                    }
                }
            } else {
                for _ in 0..(self.end as usize - self.cur as usize) {
                    unsafe { drop(mem::unsafe_zeroed::<T>()); }
                }
            }
        }

        if mem::size_of::<T>() != 0 {
            let len = (self.vec_end as usize - self.end as usize) / mem::size_of::<T>();
            if len != 0 {
                unsafe { ptr::memmove(self.start as *mut _, self.end, len); }
            }
        }
    }
}
