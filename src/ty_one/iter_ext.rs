// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use core::prelude::*;
use core::ops::{Add};
use core::iter::{Iterator};

pub fn repeat<T: Copy>(val: T) -> Repeat<T> {
    Repeat { val: val }
}

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

pub trait IteratorExt : Iterator+Sized {
    fn sum(self, start: Self::Item) -> Self::Item
        where <Self as Iterator>::Item: Add<Self::Item, Output=Self::Item>
    {
        let mut sum = start;
        for e in self { sum = sum + e; }
        sum
    }

    fn map<T, F>(self, f: F) -> Map<T, F, Self>
        where F: FnMut(Self::Item) -> T,
    {
        Map { iter: self, f: f }
    }

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
