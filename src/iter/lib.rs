// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_iter"]
#![crate_type = "lib"]
#![feature(plugin, no_std, optin_builtin_traits)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core as core;
extern crate linux_base as base;
extern crate linux_vec as vec;

#[prelude_import] use base::prelude::*;
use core::ops::{Add};
use core::iter::{Iterator};
use vec::{Vec};

/// Returns an iterator which yields the provided value indefinitely.
pub fn repeat<T: Copy>(val: T) -> Repeat<T> {
    Repeat { val: val }
}

/// See `repeat`.
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
    fn collect(self) -> Vec<'static, Self::Item> {
        let mut vec = Vec::new();
        vec.extend(self);
        vec
    }

    /// Sums all elements in the iterator, starting with `start`.
    fn sum(self, start: Self::Item) -> Self::Item
        where <Self as Iterator>::Item: Add<Self::Item, Output=Self::Item>
    {
        let mut sum = start;
        for e in self { sum = sum + e; }
        sum
    }

    /// Returns a new iterator where `f` is applied to all elements.
    fn map<T, F>(self, f: F) -> Map<T, F, Self>
        where F: FnMut(Self::Item) -> T,
    {
        Map { iter: self, f: f }
    }

    fn filter<F>(self, f: F) -> Filter<F, Self>
        where F: FnMut(&Self::Item) -> bool,
    {
        Filter { iter: self, f: f }
    }

    /// Runs the iterator and places the elements into the buffer until the buffer or the
    /// iterator are exhausted.
    ///
    /// Returns the number of elements stored in the buffer.
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

/// See `map`.
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

/// See `filter`.
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
