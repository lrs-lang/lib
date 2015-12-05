// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use option::{Option};
use option::Option::{Some, None};
use marker::{Sized, PhantomData};
use ops::{FnMut};

/// Objects that can be iterated over.
#[lang = "iterator"]
pub trait Iterator {
    /// The type that the iterator yields.
    type Item;

    /// Yields the next value.
    ///
    /// [return_value]
    /// Returns the next value or `None` if no more values are available.
    fn next(&mut self) -> Option<Self::Item>;

    /// Yields the first value that satisfies a predicate.
    ///
    /// [argument, pred]
    /// The predicate which returns `true` iff it is satisfied by a value.
    ///
    /// [return_value]
    /// The first value that satisfies the predicate.
    ///
    /// = Remarks
    ///
    /// The values that don't satisfy the predicate are lost.
    fn find<P>(&mut self, mut pred: P) -> Option<Self::Item>
        where P: FnMut(&mut Self::Item) -> bool,
    {
        while let Some(mut i) = self.next() {
            if pred(&mut i) {
                return Some(i);
            }
        }
        None
    }

    /// Yields the first value that satisfies a predicate.
    ///
    /// [argument, pred]
    /// The predicate which returns the value it is satisfied with.
    ///
    /// [return_value]
    /// The first value that satisfies the predicate.
    fn find_move<P>(&mut self, mut pred: P) -> Option<Self::Item>
        where P: FnMut(Self::Item) -> Option<Self::Item>,
    {
        while let Some(i) = self.next() {
            let rv = pred(i);
            if rv.is_some() {
                return rv;
            }
        }
        None
    }

    /// Returns whether any element in the iterator satisfies a predicate.
    ///
    /// [argument, pred]
    /// The predicate which returns `true` iff it is satisfied by a value.
    ///
    /// [return_value]
    /// Whether any element satisfies the predicate.
    ///
    /// = Remarks
    ///
    /// The values before and including the first element that satisfies the predicate
    /// are lost.
    fn any<P>(&mut self, pred: P) -> bool
        where P: FnMut(&mut Self::Item) -> bool,
    {
        self.find(pred).is_some()
    }
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
/// = Remarks
///
/// This is the trait used by `for` loops. The object on the right-hand-side has to
/// implement this trait. All iterators implement this trait and simply return themselves.
pub trait IntoIterator {
    /// The type that the iterator yields.
    type Item;

    /// The type of the iterator this object can be turned into.
    type IntoIter: Iterator<Item=Self::Item>;

    /// Turns the object into an iterator.
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
