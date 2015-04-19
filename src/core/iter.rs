// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use option::{Option};
use marker::{Sized, PhantomData};

#[lang = "iterator"]
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

impl<'a, T: Iterator+?Sized> Iterator for &'a mut T {
    type Item = T::Item;
    fn next(&mut self) -> Option<T::Item> { (**self).next() }
}

#[derive(Copy, Eq)]
pub struct Empty<T> { _data: PhantomData<T> }

impl<T> Empty<T> { pub fn new() -> Empty<T> { Empty { _data: PhantomData } } }

impl<T> Iterator for Empty<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> { Option::None }
}

impl<T> Iterator for Option<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> { self.take() }
}

pub trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item=Self::Item>;
    fn into_iter(self) -> Self::IntoIter;
}

impl<T: Iterator> IntoIterator for T {
    type Item = T::Item;
    type IntoIter = T;
    fn into_iter(self) -> T { self }
}

impl IntoIterator for () {
    type Item = ();
    type IntoIter = Empty<()>;
    fn into_iter(self) -> Empty<()> { Empty::new() }
}
