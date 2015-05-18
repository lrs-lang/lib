// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_iter"]
#![crate_type = "lib"]
#![feature(plugin, no_std, optin_builtin_traits)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use]
extern crate lrs_core as core;
extern crate lrs_base as base;
extern crate lrs_vec as vec;

#[prelude_import] use base::prelude::*;
use core::ops::{Add};
use core::iter::{Iterator};
use vec::{Vec};

/// Returns an iterator which yields a value indefinitely.
///
/// [argument, val]
/// The value to be repeated.
pub fn repeat<T: Copy>(val: T) -> Repeat<T> {
    Repeat { val: val }
}

/// An iterator that yields the same value indefinitely.
pub struct Repeat<T> {
    val: T,
}

impl<T> Iterator for Repeat<T>
    where T: Copy,
{
    type Item = T;
    fn next(&mut self) -> Option<T> { Some(self.val) }
}

impl<T: Iterator> IteratorExt for T { }

/// Extensions for the `Iterator` trait.
pub trait IteratorExt : Iterator+Sized {
    /// Collects all elements of the iterator into a vector.
    ///
    /// = Remarks
    ///
    /// If no space can be allocated for the elements, the process is aborted.
    fn collect(self) -> Vec<'static, Self::Item> {
        let mut vec = Vec::new();
        vec.extend(self);
        vec
    }

    /// Sums all elements in the iterator.
    ///
    /// [argument, start]
    /// The base value.
    fn sum(self, start: Self::Item) -> Self::Item
        where <Self as Iterator>::Item: Add<Self::Item, Output=Self::Item>
    {
        let mut sum = start;
        for e in self { sum = sum + e; }
        sum
    }

    /// Returns a new iterator that applies a function to all elements.
    ///
    /// [argument, f]
    /// The function that will be applied.
    fn map<T, F>(self, f: F) -> Map<T, F, Self>
        where F: FnMut(Self::Item) -> T,
    {
        Map { iter: self, f: f }
    }

    /// Returns a new iterator that filters elements via a function.
    ///
    /// [argument, f]
    /// The predicate.
    ///
    /// = Remarks
    ///
    /// An element is passed through if `f` returns true.
    fn filter<F>(self, f: F) -> Filter<F, Self>
        where F: FnMut(&Self::Item) -> bool,
    {
        Filter { iter: self, f: f }
    }

    /// Returns a new iterator that returns the number of the element in addition to the
    /// element.
    fn enumerate(self) -> Enumerate<Self> {
        Enumerate { iter: self, pos: 0 }
    }

    /// Removes a number of elements from the start of the iterator.
    ///
    /// [argument, n]
    /// The number of elements to remove.
    ///
    /// = Remarks
    ///
    /// This function calls `next` `n` times.
    fn consume(&mut self, n: usize) -> &mut Self {
        for _ in 0..n {
            self.next();
        }
        self
    }

    /// Places the elements of the iterator into a slice until the slice or the iterator
    /// are exhausted.
    ///
    /// [argument, buf]
    /// The buffer in which the elements will be placed.
    ///
    /// [return_value]
    /// Returns the number of elements placed in the buffer.
    fn collect_into(&mut self, mut buf: &mut [Self::Item]) -> usize {
        let mut count = 0;
        while buf.len() > 0 {
            let tmp = buf;
            tmp[0] = match self.next() {
                Some(v) => v,
                _ => break,
            };
            count += 1;
            buf = &mut tmp[1..];
        }
        count
    }
}

/// An iterator that wraps another iterator and applies a function to every element before
/// yielding it.
pub struct Map<T, F, I>
    where I: Iterator,
          F: FnMut(I::Item) -> T,
{
    iter: I,
    f: F,
}

impl<T, F, I> Iterator for Map<T, F, I>
    where I: Iterator,
          F: FnMut(I::Item) -> T,
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.iter.next().map(|i| (self.f)(i))
    }
}

/// An iterator that wraps another iterator and yields only those elements that satisfy a
/// predicate.
pub struct Filter<F, I>
    where I: Iterator,
          F: FnMut(&I::Item) -> bool,
{
    iter: I,
    f: F,
}

impl<F, I> Iterator for Filter<F, I>
    where I: Iterator,
          F: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> {
        loop {
            match self.iter.next() {
                Some(t) => if (self.f)(&t) {
                    return Some(t);
                },
                _ => return None,
            }
        }
    }
}

/// An iterator that wrap another iterator and returns the number of the returned value in
/// addition to the value.
pub struct Enumerate<I>
    where I: Iterator,
{
    iter: I,
    pos: usize,
}

impl<I> Iterator for Enumerate<I>
    where I: Iterator,
{
    type Item = (usize, I::Item);
    fn next(&mut self) -> Option<(usize, I::Item)> {
        loop {
            match self.iter.next() {
                Some(t) => {
                    let rv = Some((self.pos, t));
                    self.pos = self.pos.wrapping_add(1);
                    return rv;
                },
                _ => return None,
            }
        }
    }
}
