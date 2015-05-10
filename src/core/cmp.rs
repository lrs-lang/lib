// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use marker::{Sized};
use ops::Ordering::{Less, Equal, Greater};

pub use ops::{PartialOrd};
pub use ops::{Ordering};

/// Objects that are part of a total order.
///
/// = Remarks
///
/// :po: link:lrs::cmp::PartialOrd[PartialOrd]
///
/// This is different from {po} in that two elements can always be ordered: They are
/// either equal or one is smaller than the other.
///
/// Since `Ord<Rhs>: PartialOrd<Rhs>`, it is recommended for `PartialOrd` to be
/// implemented in terms of `Ord`:
///
/// ----
/// impl PartialOrd<T> for U {
///     fn partial_cmp(&self, other: &T) -> Option<Ordering> {
///         Some(self.cmp(other))
///     }
/// }
/// ----
///
/// Note that this is merely a library trait and does not influence the comparison
/// operators `<`, `<=`, etc. which always use the `PartialOrd` implementation.
///
/// Note that you are not forced to implement this trait reflexively or transitively which
/// is, in general, impossible due to coherence rules imposed by the compiler.
///
/// = Examples
///
/// Integers, strings, etc.
///
/// = See also
///
/// * {po}
/// * link:http://en.wikipedia.org/wiki/Total_order
pub trait Ord<Rhs: ?Sized = Self> : PartialOrd<Rhs> {
    /// Compares two objects.
    ///
    /// [argument, other]
    /// The object to be compared to `self`.
    ///
    /// [return_value]
    /// Returns the ordering between `self` and `other`.
    ///
    /// = Remarks
    ///
    /// The returned ordering is in the form `self Ordering other`, e.g., `self
    /// Ordering::Less other`.
    fn cmp(&self, other: &Rhs) -> Ordering;
}

/// = Remarks
///
/// This is implemented by dereferencing both arguments and comparing the results.
impl<'a, 'b, U, T: Ord<U>+?Sized> Ord<&'b U> for &'a T {
    fn cmp(&self, other: &&'b U) -> Ordering {
        (**self).cmp(*other)
    }
}

/// Calculates the minimum of two values.
///
/// [argument, one]
/// The first value.
///
/// [argument, two]
/// The second value.
///
/// [return_value]
/// Returns the smaller one of the two values.
///
/// = Remarks
///
/// :copy: link:lrs::marker::Copy[Copy]
///
/// If the comparison of both objects returns that they are equal, the first one is
/// returned.
///
/// Since this function takes ownership of the objects and drops one of them, it is mostly
/// useful for type that implement {copy}.
///
/// = Examples
///
/// ----
/// assert!(min(1, 2) == 1);
/// ----
///
/// = See also
///
/// * {copy}
/// * link:lrs::cmp::min_ref
/// * link:lrs::cmp::min_mut
pub fn min<T: Ord>(one: T, two: T) -> T {
    match one.cmp(&two) {
        Less | Equal => one,
        _ => two,
    }
}

/// Calculates the minimum of two values by reference.
///
/// [argument, one]
/// The first value.
///
/// [argument, two]
/// The second value.
///
/// [return_value]
/// The returns the smaller one of the two values.
///
/// = Remarks
///
/// If the comparison of both objects returns that they are equal, the first one is
/// returned.
///
/// = Examples
///
/// ----
/// let x = &1;
/// let y = &2;
/// assert!(min(x, y) == x);
/// ----
///
/// = See also
///
/// * link:lrs::cmp::min
/// * link:lrs::cmp::min_mut
pub fn min_ref<'a, T: Ord+?Sized>(one: &'a T, two: &'a T) -> &'a T {
    match one.cmp(two) {
        Less | Equal => one,
        _ => two,
    }
}

/// Calculates the minimum of two values by mutable reference.
///
/// [argument, one]
/// The first value.
///
/// [argument, two]
/// The second value.
///
/// [return_value]
/// The returns the smaller one of the two values.
///
/// = Remarks
///
/// If the comparison of both objects returns that they are equal, the first one is
/// returned.
///
/// = Examples
///
/// ----
/// let x = &mut 1;
/// let y = &mut 2;
/// *min(x, y) = 3;
/// assert!(*x == 3);
/// ----
///
/// = See also
///
/// * link:lrs::cmp::min
/// * link:lrs::cmp::min_ref
pub fn min_mut<'a, T: Ord+?Sized>(one: &'a mut T, two: &'a mut T) -> &'a mut T {
    match one.cmp(two) {
        Less | Equal => one,
        _ => two,
    }
}

/// Calculates the maximum of two values.
///
/// [argument, one]
/// The first value.
///
/// [argument, two]
/// The second value.
///
/// [return_value]
/// Returns the larger one of the two values.
///
/// = Remarks
///
/// :copy: link:lrs::marker::Copy[Copy]
///
/// If the comparison of both objects returns that they are equal, the first one is
/// returned.
///
/// Since this function takes ownership of the objects and drops one of them, it is mostly
/// useful for type that implement {copy}.
///
/// = Examples
///
/// ----
/// assert!(max(1, 2) == 2);
/// ----
///
/// = See also
///
/// * {copy}
/// * link:lrs::cmp::max_ref
/// * link:lrs::cmp::max_mut
pub fn max<T: Ord>(one: T, two: T) -> T {
    match one.cmp(&two) {
        Greater | Equal => one,
        _ => two,
    }
}

/// Calculates the maximum of two values by reference.
///
/// [argument, one]
/// The first value.
///
/// [argument, two]
/// The second value.
///
/// [return_value]
/// The returns the larger one of the two values.
///
/// = Remarks
///
/// If the comparison of both objects returns that they are equal, the first one is
/// returned.
///
/// = Examples
///
/// ----
/// let x = &1;
/// let y = &2;
/// assert!(max(x, y) == y);
/// ----
///
/// = See also
///
/// * link:lrs::cmp::max
/// * link:lrs::cmp::max_mut
pub fn max_ref<'a, T: Ord+?Sized>(one: &'a T, two: &'a T) -> &'a T {
    match one.cmp(two) {
        Greater | Equal => one,
        _ => two,
    }
}

/// Calculates the maximum of two values by mutable reference.
///
/// [argument, one]
/// The first value.
///
/// [argument, two]
/// The second value.
///
/// [return_value]
/// The returns the larger one of the two values.
///
/// = Remarks
///
/// If the comparison of both objects returns that they are equal, the first one is
/// returned.
///
/// = Examples
///
/// ----
/// let x = &mut 1;
/// let y = &mut 2;
/// *max(x, y) = 3;
/// assert!(*y == 3);
/// ----
///
/// = See also
///
/// * link:lrs::cmp::max
/// * link:lrs::cmp::max_ref
pub fn max_mut<'a, T: Ord+?Sized>(one: &'a mut T, two: &'a mut T) -> &'a mut T {
    match one.cmp(two) {
        Greater | Equal => one,
        _ => two,
    }
}
