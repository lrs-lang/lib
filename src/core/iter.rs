// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use option::{Option};
use marker::{Sized, PhantomData};

/// Objects that can be iterated over.
#[lang = "iterator"]
pub trait Iterator {
    /// The type that the iterator yields.
    type Item;
    /// Returns the next value of `None` if the iterator is finished.
    fn next(&mut self) -> Option<Self::Item>;
}

impl<'a, T: Iterator+?Sized> Iterator for &'a mut T {
    type Item = T::Item;
    fn next(&mut self) -> Option<T::Item> { (**self).next() }
}

/// An iterator that yields no values.
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

/// Objects that can be turned into iterators.
///
/// This is the trait used by `for` loops. The object on the right-hand-side has to
/// implement this trait. All iterators implement this trait and simply return themselves.
///
/// The unit `()` implements this trait with the target `Empty<()>`.
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
