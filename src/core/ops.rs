// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use marker::{Copy, Sized, Unsize};
use option::{Option};
use option::Option::{Some};

/// Objects with a destructor.
#[lang = "drop"]
pub trait Drop {
    /// The destructor which will be called when the object is dropped.
    fn drop(&mut self);
}

/// Objects that implement the binary `+` operator.
#[lang = "add"]
pub trait Add<RHS=Self> {
    /// The output of the operator.
    type Output = Self;
    /// The method that will be called by the operator.
    ///
    /// [argument, rhs]
    /// The right-hand-side of the operator.
    fn add(self, rhs: RHS) -> Self::Output;
}

/// Objects that implement the binary `+=` operator.
#[lang = "add_assign"]
pub trait AddAssign<RHS=Self> {
    /// The method that will be called by the operator.
    ///
    /// [argument, rhs]
    /// The right-hand-side of the operator.
    fn add_assign(&mut self, rhs: RHS);
}

/// Objects that implement the binary `-` operator.
#[lang = "sub"]
pub trait Sub<RHS=Self> {
    /// The output of the operator.
    type Output = Self;
    /// The method that will be called by the operator.
    ///
    /// [argument, rhs]
    /// The right-hand-side of the operator.
    fn sub(self, rhs: RHS) -> Self::Output;
}

/// Objects that implement the binary `-=` operator.
#[lang = "sub_assign"]
pub trait SubAssign<RHS=Self> {
    /// The method that will be called by the operator.
    ///
    /// [argument, rhs]
    /// The right-hand-side of the operator.
    fn sub_assign(&mut self, rhs: RHS);
}

/// Objects that implement the binary `*` operator.
#[lang = "mul"]
pub trait Mul<RHS=Self> {
    /// The output of the operator.
    type Output = Self;
    /// The method that will be called by the operator.
    ///
    /// [argument, rhs]
    /// The right-hand-side of the operator.
    fn mul(self, rhs: RHS) -> Self::Output;
}

/// Objects that implement the binary `*=` operator.
#[lang = "mul_assign"]
pub trait MulAssign<RHS=Self> {
    /// The method that will be called by the operator.
    ///
    /// [argument, rhs]
    /// The right-hand-side of the operator.
    fn mul_assign(&mut self, rhs: RHS);
}

/// Objects that implement the binary `/` operator.
#[lang = "div"]
pub trait Div<RHS=Self> {
    /// The output of the operator.
    type Output = Self;
    /// The method that will be called by the operator.
    ///
    /// [argument, rhs]
    /// The right-hand-side of the operator.
    fn div(self, rhs: RHS) -> Self::Output;
}

/// Objects that implement the binary `/=` operator.
#[lang = "div_assign"]
pub trait DivAssign<RHS=Self> {
    /// The method that will be called by the operator.
    ///
    /// [argument, rhs]
    /// The right-hand-side of the operator.
    fn div_assign(&mut self, rhs: RHS);
}

/// Objects that implement the binary `%` operator.
#[lang = "rem"]
pub trait Rem<RHS=Self> {
    /// The output of the operator.
    type Output = Self;
    /// The method that will be called by the operator.
    ///
    /// [argument, rhs]
    /// The right-hand-side of the operator.
    fn rem(self, rhs: RHS) -> Self::Output;
}

/// Objects that implement the binary `%=` operator.
#[lang = "rem_assign"]
pub trait RemAssign<Rhs=Self> {
    /// The method that will be called by the operator.
    ///
    /// [argument, rhs]
    /// The right-hand-side of the operator.
    fn rem_assign(&mut self, Rhs);
}

/// Objects that implement the unary `-` operator.
#[lang = "neg"]
pub trait Neg {
    /// The output of the operator.
    type Output = Self;
    /// The method that will be called by the operator.
    fn neg(self) -> Self::Output;
}

/// Objects that implement the unary `!` operator.
#[lang = "not"]
pub trait Not {
    /// The output of the operator.
    type Output = Self;
    /// The method that will be called by the operator.
    fn not(self) -> Self::Output;
}

/// Objects that implement the binary `&` operator.
#[lang = "bitand"]
pub trait BitAnd<RHS = Self> {
    /// The output of the operator.
    type Output = Self;
    /// The method that will be called by the operator.
    ///
    /// [argument, rhs]
    /// The right-hand-side of the operator.
    fn bitand(self, rhs: RHS) -> Self::Output;
}

/// Objects that implement the binary `&=` operator.
#[lang = "bitand_assign"]
pub trait BitAndAssign<RHS=Self> {
    /// The method that will be called by the operator.
    ///
    /// [argument, rhs]
    /// The right-hand-side of the operator.
    fn bitand_assign(&mut self, rhs: RHS);
}

/// Objects that implement the binary `|` operator.
#[lang = "bitor"]
pub trait BitOr<RHS = Self> {
    /// The output of the operator.
    type Output = Self;
    /// The method that will be called by the operator.
    ///
    /// [argument, rhs]
    /// The right-hand-side of the operator.
    fn bitor(self, rhs: RHS) -> Self::Output;
}

/// Objects that implement the binary `|=` operator.
#[lang = "bitor_assign"]
pub trait BitOrAssign<RHS=Self> {
    /// The method that will be called by the operator.
    ///
    /// [argument, rhs]
    /// The right-hand-side of the operator.
    fn bitor_assign(&mut self, rhs: RHS);
}

/// Objects that implement the binary `^` operator.
#[lang = "bitxor"]
pub trait BitXor<RHS = Self> {
    /// The output of the operator.
    type Output = Self;
    /// The method that will be called by the operator.
    ///
    /// [argument, rhs]
    /// The right-hand-side of the operator.
    fn bitxor(self, rhs: RHS) -> Self::Output;
}

#[lang = "bitxor_assign"]
pub trait BitXorAssign<RHS=Self> {
    /// The method that will be called by the operator.
    ///
    /// [argument, rhs]
    /// The right-hand-side of the operator.
    fn bitxor_assign(&mut self, rhs: RHS);
}

/// Objects that implement the binary `<<` operator.
#[lang = "shl"]
pub trait Shl<RHS> {
    /// The output of the operator.
    type Output = Self;
    /// The method that will be called by the operator.
    ///
    /// [argument, rhs]
    /// The right-hand-side of the operator.
    fn shl(self, rhs: RHS) -> Self::Output;
}

/// Objects that implement the binary `<<=` operator.
#[lang = "shl_assign"]
pub trait ShlAssign<RHS> {
    /// The method that will be called by the operator.
    ///
    /// [argument, rhs]
    /// The right-hand-side of the operator.
    fn shl_assign(&mut self, rhs: RHS);
}

/// Objects that implement the binary `>>` operator.
#[lang = "shr"]
pub trait Shr<RHS> {
    /// The output of the operator.
    type Output = Self;
    /// The method that will be called by the operator.
    ///
    /// [argument, rhs]
    /// The right-hand-side of the operator.
    fn shr(self, rhs: RHS) -> Self::Output;
}

/// Objects that implement the binary `>>=` operator.
#[lang = "shr_assign"]
pub trait ShrAssign<RHS> {
    /// The method that will be called by the operator.
    ///
    /// [argument, rhs]
    /// The right-hand-side of the operator.
    fn shr_assign(&mut self, rhs: RHS);
}

/// Objects that implement the immutable subscript operator.
#[lang = "index"]
pub trait Index<Idx: ?Sized> {
    /// The output of the operator.
    type Output: ?Sized;
    /// The method that will be called by the operator.
    ///
    /// [argument, index]
    /// The index of the operator.
    fn index(&self, index: Idx) -> &Self::Output;
}

/// Objects that implement the mutable subscript operator.
#[lang = "index_mut"]
pub trait IndexMut<Idx: ?Sized>: Index<Idx> {
    /// The method that will be called by the operator.
    ///
    /// [argument, index]
    /// The index of the operator.
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output;
}

/// The type representing the unbounded `..` syntax.
#[lang = "range_full"]
#[derive(Eq)]
pub struct RangeFull;

/// The type representing the bounded `M..N` syntax.
#[lang = "range"]
#[derive(Eq)]
pub struct Range<Idx> {
    /// The left-hand-side of the operator.
    pub start: Idx,
    /// The right-hand-side of the operator.
    pub end: Idx,
}

/// The type representing the half-bounded `M..` syntax.
#[lang = "range_from"]
#[derive(Eq)]
pub struct RangeFrom<Idx> {
    /// The left-hand-side of the operator.
    pub start: Idx,
}

/// The type representing the half-bounded `..N` syntax.
#[lang = "range_to"]
#[derive(Eq)]
pub struct RangeTo<Idx> {
    /// The right-hand-side of the operator.
    pub end: Idx,
}

/// Objects that implement the immutable dereference operator.
#[lang = "deref"]
pub trait Deref {
    /// The output of the operator.
    type Target: ?Sized;
    /// The method that will be called by the operator.
    fn deref(&self) -> &Self::Target;
}

impl<'a, T> Deref for &'a T {
    type Target = T;
    fn deref(&self) -> &T { *self }
}

// TODO: This is wrong. DerefMut should not depend on Deref. E.g. Mutex can implement
// DerefMut but not Deref. We could change it here but method resolution in the compiler
// doesn't work in that case. Change it once we fork the compiler.
/// Objects that implement the mutable dereference operator.
#[lang = "deref_mut"]
pub trait DerefMut: Deref {
    /// The method that will be called by the operator.
    fn deref_mut<'a>(&'a mut self) -> &'a mut Self::Target;
}

impl<'a, T> Deref for &'a mut T {
    type Target = T;
    fn deref(&self) -> &T { *self }
}

impl<'a, T> DerefMut for &'a mut T {
    fn deref_mut(&mut self) -> &mut T { *self }
}

/// Objects that implement the binary `==` and `!=` operators.
#[lang = "eq"]
pub trait Eq<Rhs: ?Sized = Self> {
    /// The method that will be called by the `==` operator.
    ///
    /// [argument, other]
    /// The right-hand-side of the operator.
    fn eq(&self, other: &Rhs) -> bool;
    /// The method that will be called by the `==` operator.
    ///
    /// [argument, other]
    /// The right-hand-side of the operator.
    fn ne(&self, other: &Rhs) -> bool { !self.eq(other) }
}

impl<'a, 'b, U: ?Sized, T: Eq<U>+?Sized> Eq<&'b U> for &'a T {
    fn eq(&self, other: &&U) -> bool { (**self).eq(*other) }
}

impl<'a, 'b, U: ?Sized, T: Eq<U>+?Sized> Eq<&'b U> for &'a mut T {
    fn eq(&self, other: &&U) -> bool { (**self).eq(*other) }
}

impl<'a, 'b, U: ?Sized, T: Eq<U>+?Sized> Eq<&'b mut U> for &'a T {
    fn eq(&self, other: &&mut U) -> bool { (**self).eq(*other) }
}

impl<'a, 'b, U: ?Sized, T: Eq<U>+?Sized> Eq<&'b mut U> for &'a mut T {
    fn eq(&self, other: &&mut U) -> bool { (**self).eq(*other) }
}

/// Result of a comparison of two values.
#[derive(Eq)]
pub enum Ordering {
    /// The first value is smaller than the second.
    Less,
    /// The values are equal.
    Equal,
    /// The second value is larger than the firest.
    Greater,
}
impl Copy for Ordering { }

/// Objects that implement the binary `<`, `<=`, `>`, and `>=` operators.
#[lang = "ord"]
pub trait PartialOrd<Rhs: ?Sized = Self> : Eq<Rhs> {
    /// Attempts a comparison between the object and another one.
    ///
    /// [argument, other]
    /// The right-hand-side of the operation.
    ///
    /// [return_value]
    /// Returns the result of the comparison, if any.
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;

    /// The method that will be called by the `<` operator.
    ///
    /// [argument, other]
    /// The right-hand-side of the operator.
    fn lt(&self, other: &Rhs) -> bool {
        self.partial_cmp(other) == Some(Ordering::Less)
    }

    /// The method that will be called by the `<=` operator.
    ///
    /// [argument, other]
    /// The right-hand-side of the operator.
    fn le(&self, other: &Rhs) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Less) | Some(Ordering::Equal) => true,
            _ => false,
        }
    }

    /// The method that will be called by the `>` operator.
    ///
    /// [argument, other]
    /// The right-hand-side of the operator.
    fn gt(&self, other: &Rhs) -> bool {
        self.partial_cmp(other) == Some(Ordering::Greater)
    }

    /// The method that will be called by the `>=` operator.
    ///
    /// [argument, other]
    /// The right-hand-side of the operator.
    fn ge(&self, other: &Rhs) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Greater) | Some(Ordering::Equal) => true,
            _ => false,
        }
    }
}

impl<'a, 'b, Rhs: ?Sized, Lhs: PartialOrd<Rhs>+?Sized> PartialOrd<&'a Rhs> for &'b Lhs {
    fn partial_cmp(&self, other: &&'a Rhs) -> Option<Ordering> {
        (**self).partial_cmp(*other)
    }
}

/// Objects that implement the immutable function call operator.
#[lang = "fn"]
#[rustc_paren_sugar]
pub trait Fn<Args> : FnMut<Args> {
    /// The method that is called by the operator.
    ///
    /// [argument, args]
    /// The arguments passed to the function.
    extern "rust-call" fn call(&self, args: Args) -> Self::Output;
}

/// Objects that implement the mutable function call operator.
#[lang = "fn_mut"]
#[rustc_paren_sugar]
pub trait FnMut<Args> : FnOnce<Args> {
    /// The method that is called by the operator.
    ///
    /// [argument, args]
    /// The arguments passed to the function.
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
}

/// Objects that implement the consuming function call operator.
#[lang = "fn_once"]
#[rustc_paren_sugar]
pub trait FnOnce<Args> {
    /// The return type of the function.
    type Output;
    /// The method that is called by the operator.
    ///
    /// [argument, args]
    /// The arguments passed to the function.
    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}

impl<'a, Args, F: Fn<Args>+?Sized> Fn<Args> for &'a F {
    extern "rust-call" fn call(&self, args: Args) -> F::Output {
        (**self).call(args)
    }
}

impl<'a, Args, F: Fn<Args>+?Sized> FnMut<Args> for &'a F {
    extern "rust-call" fn call_mut(&mut self, args: Args) -> F::Output {
        (**self).call(args)
    }
}

impl<'a, Args, F: Fn<Args>+?Sized> FnOnce<Args> for &'a F {
    type Output = F::Output;
    extern "rust-call" fn call_once(self, args: Args) -> F::Output {
        (*self).call(args)
    }
}

impl<'a, Args, F: FnMut<Args>+?Sized> FnMut<Args> for &'a mut F {
    extern "rust-call" fn call_mut(&mut self, args: Args) -> F::Output {
        (*self).call_mut(args)
    }
}

impl<'a, Args, F: FnMut<Args>+?Sized> FnOnce<Args> for &'a mut F {
    type Output = F::Output;
    extern "rust-call" fn call_once(self, args: Args) -> F::Output {
        (*self).call_mut(args)
    }
}

#[lang="coerce_unsized"]
pub trait CoerceUnsized<T> { }

impl<'a, T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<&'a mut U> for &'a mut T {}
impl<'a, 'b: 'a, T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<&'a U> for &'b mut T {}
impl<'a, T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<*mut U> for &'a mut T {}
impl<'a, T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<*const U> for &'a mut T {}
impl<'a, 'b: 'a, T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<&'a U> for &'b T {}
impl<'a, T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<*const U> for &'a T {}
impl<T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<*mut U> for *mut T {}
impl<T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<*const U> for *mut T {}
impl<T: ?Sized+Unsize<U>, U: ?Sized> CoerceUnsized<*const U> for *const T {}
