// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use mem::{self};
use repr::{Slice, Repr};
use ops::{Eq, Index, IndexMut, Range, RangeTo, RangeFrom, RangeFull, FnMut, Ordering};
use option::{Option};
use option::Option::{None, Some};
use iter::{Iterator};

pub unsafe fn from_ptr<'a, T>(ptr: *const T, len: usize) -> &'a mut [T] {
    mem::cast(Slice { ptr: ptr, len: len })
}

#[lang = "slice"]
impl<T> [T] {
    pub fn len(&self) -> usize {
        self.repr().len
    }

    pub fn as_ptr(&self) -> *const T {
        self.repr().ptr
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.repr().ptr as *mut T
    }

    pub fn iter<'a>(&'a self) -> Items<'a, T> {
        Items { slice: self }
    }

    pub fn find<F>(&self, mut f: F) -> Option<usize>
        where F: FnMut(&T) -> bool
    {
        for i in 0..self.len() {
            if f(&self[i]) {
                return Some(i);
            }
        }
        None
    }

    pub fn find_reverse<F>(&self, mut f: F) -> Option<usize>
        where F: FnMut(&T) -> bool
    {
        for i in 0..self.len() {
            if f(&self[self.len() - i - 1]) {
                return Some(self.len() - i - 1);
            }
        }
        None
    }

    pub fn find_binary<F>(&self, mut f: F) -> (Option<usize>, usize)
        where F: FnMut(&T) -> Ordering,
    {
        let mut left = 0;
        let mut right = self.len();
        while left < right {
            let pos = (left + right) / 2;
            match f(&self[pos]) {
                Ordering::Equal => return (Some(pos), pos),
                Ordering::Less => left = pos + 1,
                Ordering::Greater => right = pos,
            }
        }
        (None, left)
    }
}

pub struct Items<'a, T: 'a> {
    slice: &'a [T],
}

impl<'a, T> Iterator for Items<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        self.slice.next()
    }
}

/////////
// Index impls
/////////

impl<T> Index<usize> for [T] {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        if index >= self.len() { abort!(); }
        unsafe { &*self.as_ptr().add(index) }
    }
}

impl<T> IndexMut<usize> for [T] {
    fn index_mut(&mut self, index: usize) -> &mut T {
        if index >= self.len() { abort!(); }
        unsafe { &mut *self.as_mut_ptr().add(index) }
    }
}

impl<T> Index<Range<usize>> for [T] {
    type Output = [T];

    fn index(&self, index: Range<usize>) -> &[T] {
        assert!(index.start <= index.end);
        assert!(index.end <= self.len());
        let len = index.end - index.start;
        let start = unsafe { self.as_ptr().add(index.start) };
        unsafe { from_ptr(start, len) }
    }
}

impl<T> IndexMut<Range<usize>> for [T] {
    fn index_mut(&mut self, index: Range<usize>) -> &mut [T] {
        unsafe { mem::cast(self.index(index)) }
    }
}

impl<T> Index<RangeTo<usize>> for [T] {
    type Output = [T];

    fn index(&self, index: RangeTo<usize>) -> &[T] {
        self.index(0..index.end)
    }
}

impl<T> IndexMut<RangeTo<usize>> for [T] {
    fn index_mut(&mut self, index: RangeTo<usize>) -> &mut [T] {
        self.index_mut(0..index.end)
    }
}

impl<T> Index<RangeFrom<usize>> for [T] {
    type Output = [T];

    fn index(&self, index: RangeFrom<usize>) -> &[T] {
        self.index(index.start..self.len())
    }
}

impl<T> IndexMut<RangeFrom<usize>> for [T] {
    fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut [T] {
        let len = self.len();
        self.index_mut(index.start..len)
    }
}

impl<T> Index<RangeFull> for [T] {
    type Output = [T];

    fn index(&self, _: RangeFull) -> &[T] {
        self
    }
}

impl<T> IndexMut<RangeFull> for [T] {
    fn index_mut(&mut self, _: RangeFull) -> &mut [T] {
        self
    }
}

impl<T: Eq> Eq for [T] {
    fn eq(&self, other: &[T]) -> bool {
        if self.len() != other.len() {
            return false;
        }
        let mut idx = 0;
        while idx < self.len() {
            if self[idx] != other[idx] {
                return false;
            }
            idx += 1;
        }
        true
    }
}

impl<'a, T> Iterator for &'a [T] {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        if self.len() > 0 {
            let first = unsafe { &*self.as_ptr() };
            *self = &self[1..];
            Some(first)
        } else {
            None
        }
    }
}

impl<'a, T> Iterator for &'a mut [T] {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<&'a mut T> {
        if self.len() > 0 {
            let first = unsafe { &mut *self.as_mut_ptr() };
            let slf = mem::replace(self, &mut []);
            *self = &mut slf[1..];
            Some(first)
        } else {
            None
        }
    }
}
