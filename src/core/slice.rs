// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use mem::{self};
use repr::{Slice, Repr};
use ops::{Eq, Index, IndexMut, PartialOrd, Range, RangeTo, RangeFrom, RangeFull, FnMut,
          Ordering};
use cmp::{self, Ord};
use option::{Option};
use option::Option::{None, Some};
use iter::{Iterator};
use sort::{sort};
use marker::{Pod};
use data::{d8};

/// Creates a slice from a pointer and a length.
///
/// [argument, ptr]
/// The pointer to the first element of the slice.
///
/// [argument, len]
/// The number of elements in the slice.
///
/// [return_value]
/// Returns a slice made up of the arguments.
///
/// = Remarks
///
/// The arguments must be valid or the behavior is undefined.
pub unsafe fn from_ptr<'a, T>(ptr: *const T, len: usize) -> &'a mut [T] {
    mem::cast(Slice { ptr: ptr, len: len })
}

#[lang = "slice"]
impl<T> [T] {
    /// Returns the length of the slice.
    pub fn len(&self) -> usize {
        self.repr().len
    }

    /// Returns the starting address of the data.
    pub fn as_ptr(&self) -> *const T {
        self.repr().ptr
    }

    /// Returns the address of the first element in the slice.
    pub fn addr(&self) -> usize {
        self.as_ptr() as usize
    }

    /// Returns the starting address of the data.
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.repr().ptr as *mut T
    }

    // /// Returns a byte slice covering the same range as the slice.
    // pub fn as_bytes(&self) -> &[u8] {
    //     let len = self.len() * mem::size_of::<T>();
    //     unsafe { from_ptr(self.as_ptr() as *const _, len) }
    // }

    // /// Returns a mutable byte slice covering the same range as the slice.
    // pub fn as_mut_bytes(&mut self) -> &mut [u8]
    //     where T: Pod,
    // {
    //     let len = self.len() * mem::size_of::<T>();
    //     unsafe { from_ptr(self.as_ptr() as *const _, len) }
    // }

    /// Returns a byte slice covering the same range as the slice.
    pub fn as_data(&self) -> &[d8] {
        let len = self.len() * mem::size_of::<T>();
        unsafe { from_ptr(self.as_ptr() as *const _, len) }
    }

    /// Returns a mutable byte slice covering the same range as the slice.
    pub fn as_mut_data(&mut self) -> &mut [d8]
        where T: Pod,
    {
        let len = self.len() * mem::size_of::<T>();
        unsafe { from_ptr(self.as_ptr() as *const _, len) }
    }

    /// Creates an iterator over the elements of the slice.
    pub fn iter<'a>(&'a self) -> Items<'a, T> {
        Items { slice: self }
    }

    /// Creates a mutable iterator over the elements of the slice.
    pub fn iter_mut<'a>(&'a mut self) -> MutItems<'a, T> {
        MutItems { slice: self }
    }

    /// Returns the index of the first element in the slice which satisfies a predicate.
    ///
    /// [argument, f]
    /// The predicate.
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

    /// Returns the index of the last element in the slice which satisfies a predicate.
    ///
    /// [argument, f]
    /// The predicate.
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

    /// Performs a binary search to find an element in the slice which satisfies a
    /// predicate.
    ///
    /// [argument, f]
    /// The predicate.
    ///
    /// [return_value]
    /// Returns the found index in the first slot or the index at which an element that
    /// satisfies the predicate could be inserted in the second slot.
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

    /// Returns a reference to the last element in the slice (if any.)
    pub fn last(&self) -> Option<&T> {
        match self.len() {
            0 => None,
            n => Some(&self[n - 1]),
        }
    }

    /// Returns an iterator over the sub-slices which are separated by elements that
    /// satisfy a predicate.
    ///
    /// [argument, f]
    /// The predicate.
    pub fn split<'a, F>(&'a self, f: F) -> Split<'a, T, F>
        where F: FnMut(&T) -> bool,
    {
        Split { slice: self, f: f }
    }

    /// Splits the slice at an index.
    ///
    /// [argument, at]
    /// The index at which the slice is split.
    ///
    /// = Remarks
    ///
    /// The element at the index is the first element of the second slice.
    pub fn split_at<'a>(&'a self, at: usize) -> (&'a [T], &'a [T]) {
        let repr = self.repr();
        assert!(at <= repr.len);
        let left = unsafe { from_ptr(repr.ptr, at) };
        let right = unsafe { from_ptr(repr.ptr.add(at), repr.len - at) };
        (left, right)
    }

    /// Splits the mutable slice at an index.
    ///
    /// [argument, at]
    /// The index at which the slice is split.
    ///
    /// = Remarks
    ///
    /// The element at the index is the first element of the second slice.
    pub fn split_at_mut<'a>(&'a mut self, at: usize) -> (&'a mut [T], &'a mut [T]) {
        let repr = self.repr();
        assert!(at <= repr.len);
        let left = unsafe { from_ptr(repr.ptr, at) };
        let right = unsafe { from_ptr(repr.ptr.add(at), repr.len - at) };
        (left, right)
    }

    /// Checks whether the slice starts with another slice.
    ///
    /// [argument, other]
    /// The slice that this slice should start with.
    pub fn starts_with(&self, other: &[T]) -> bool
        where T: Eq,
    {
        if self.len() < other.len() {
            return false;
        }

        for i in 0..other.len() {
            if self[i] != other[i] {
                return false;
            }
        }

        true
    }

    /// Sorts the slice in-place.
    pub fn sort(&mut self)
        where T: Ord,
    {
        self.sort_by(|one, two| one.cmp(two));
    }

    /// Sorts the slice in-place with a comparison function.
    ///
    /// [argument, f]
    /// The comparison function.
    pub fn sort_by<F>(&mut self, mut f: F)
        where F: FnMut(&T, &T) -> Ordering
    {
        if mem::size_of::<T>() != 0 {
            unsafe { sort(self, &mut f); }
        }
    }

    /// Creates a subslice without checking the range for correctness.
    ///
    /// [argument, from]
    /// From where to slice.
    ///
    /// [argument, to]
    /// To where to slice.
    ///
    /// = Remarks
    ///
    /// If the arguments are invalid, the behavior is undefined.
    pub unsafe fn unchecked_slice(&self, from: usize, to: usize) -> &[T] {
        let len = to - from;
        let start = self.as_ptr().add(from);
        from_ptr(start, len)
    }

    /// Creates a mutable subslice without checking the range for correctness.
    ///
    /// [argument, from]
    /// From where to slice.
    ///
    /// [argument, to]
    /// To where to slice.
    ///
    /// = Remarks
    ///
    /// If the arguments are invalid, the behavior is undefined.
    pub unsafe fn unchecked_mut_slice(&mut self, from: usize, to: usize) -> &mut [T] {
        let len = to - from;
        let start = self.as_ptr().add(from);
        from_ptr(start, len)
    }

    /// Creates a subslice without checking the range for correctness.
    ///
    /// [argument, from]
    /// From where to slice.
    ///
    /// = Remarks
    ///
    /// If the argument are invalid, the behavior is undefined.
    pub unsafe fn unchecked_slice_from(&self, from: usize) -> &[T] {
        self.unchecked_slice(from, self.len())
    }

    /// Creates a mutable subslice without checking the range for correctness.
    ///
    /// [argument, from]
    /// From where to slice.
    ///
    /// = Remarks
    ///
    /// If the argument are invalid, the behavior is undefined.
    pub unsafe fn unchecked_mut_slice_from(&mut self, from: usize) -> &mut [T] {
        let len = self.len();
        self.unchecked_mut_slice(from, len)
    }

    /// Creates a mutable subslice without checking the range for correctness.
    ///
    /// [argument, to]
    /// To where to slice.
    ///
    /// = Remarks
    ///
    /// If the arguments are invalid, the behavior is undefined.
    pub unsafe fn unchecked_slice_to(&self, to: usize) -> &[T] {
        self.unchecked_slice(0, to)
    }

    /// Creates a mutable subslice without checking the range for correctness.
    ///
    /// [argument, to]
    /// To where to slice.
    ///
    /// = Remarks
    ///
    /// If the arguments are invalid, the behavior is undefined.
    pub unsafe fn unchecked_mut_slice_to(&mut self, to: usize) -> &mut [T] {
        self.unchecked_mut_slice(0, to)
    }

    /// Reverses the elements in the slice.
    pub fn reverse(&mut self) {
        let len = self.len();
        unsafe {
            for i in 0..len/2 {
                mem::swap(&mut *self.as_mut_ptr().add(i),
                          &mut *self.as_mut_ptr().add(len - i - 1));
            }
        }
    }
}

impl<T: PartialOrd> PartialOrd for [T] {
    fn partial_cmp(&self, other: &[T]) -> Option<Ordering> {
        let min_len = cmp::min(self.len(), other.len());
        for i in 0..min_len {
            match self[i].partial_cmp(&other[i]) {
                Some(Ordering::Equal) => { },
                r @ _ => return r,
            }
        }
        Some(self.len().cmp(&other.len()))
    }
}

impl<T: Ord> Ord for [T] {
    fn cmp(&self, other: &[T]) -> Ordering {
        let min_len = cmp::min(self.len(), other.len());
        for i in 0..min_len {
            match self[i].cmp(&other[i]) {
                Ordering::Equal => { },
                r @ _ => return r,
            }
        }
        self.len().cmp(&other.len())
    }
}

/// An iterator over an immutable slice.
pub struct Items<'a, T: 'a> {
    slice: &'a [T],
}

impl<'a, T> Iterator for Items<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        self.slice.next()
    }
}

/// An iterator over a mutable immutable slice.
pub struct MutItems<'a, T: 'a> {
    slice: &'a mut [T],
}

impl<'a, T> Iterator for MutItems<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<&'a mut T> {
        self.slice.next()
    }
}

/// An iterator over subslices.
pub struct Split<'a, T: 'a, F> {
    slice: &'a [T],
    f: F,
}

impl<'a, T, F> Iterator for Split<'a, T, F>
    where F: FnMut(&T) -> bool,
{
    type Item = &'a [T];
    fn next(&mut self) -> Option<&'a [T]> {
        if self.slice.len() == 0 {
            return None;
        }

        let mut i = 0;
        while i < self.slice.len() {
            if (self.f)(&self.slice[i]) {
                break;
            }
            i += 1;
        }

        if i < self.slice.len() {
            let (left, right) = self.slice.split_at(i + 1);
            self.slice = right;
            Some(&left[..left.len() - 1])
        } else {
            Some(mem::replace(&mut self.slice, &[]))
        }
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
        assert!(index.start <= index.end);
        assert!(index.end <= self.len());
        let len = index.end - index.start;
        let start = unsafe { self.as_ptr().add(index.start) };
        unsafe { from_ptr(start, len) }
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
        if self.addr() == other.addr() {
            return true;
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
