// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use core::prelude::*;
use core::ops::{Add};
use core::iter::{Iterator};

impl<T: Iterator> IteratorExt for T { }

pub trait IteratorExt : Iterator+Sized {
    fn sum(mut self) -> Option<Self::Item>
        where <Self as Iterator>::Item: Add<<Self as Iterator>::Item, Output=<Self as Iterator>::Item>
    {
        let mut sum = match self.next() {
            Some(e) => e,
            _ => return None,
        };
        for e in self {
            sum = sum + e;
        }
        Some(sum)
    }

    fn map<T, F>(self, f: F) -> Map<T, F, Self>
        where F: FnMut(Self::Item) -> T,
    {
        Map { iter: self, f: f }
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
