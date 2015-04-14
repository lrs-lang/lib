// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use marker::{Sized};
use option::{Option};
use option::Option::{Some};

#[lang = "drop"]
pub trait Drop {
    fn drop(&mut self);
}

#[lang = "add"]
pub trait Add<RHS=Self> {
    type Output;
    fn add(self, rhs: RHS) -> Self::Output;
}

#[lang = "sub"]
pub trait Sub<RHS=Self> {
    type Output;
    fn sub(self, rhs: RHS) -> Self::Output;
}

#[lang = "mul"]
pub trait Mul<RHS=Self> {
    type Output;
    fn mul(self, rhs: RHS) -> Self::Output;
}

#[lang = "div"]
pub trait Div<RHS=Self> {
    type Output;
    fn div(self, rhs: RHS) -> Self::Output;
}

#[lang = "rem"]
pub trait Rem<RHS=Self> {
    type Output = Self;
    fn rem(self, rhs: RHS) -> Self::Output;
}

#[lang = "neg"]
pub trait Neg {
    type Output;
    fn neg(self) -> Self::Output;
}

#[lang = "not"]
pub trait Not {
    type Output;
    fn not(self) -> Self::Output;
}

#[lang = "bitand"]
pub trait BitAnd<RHS=Self> {
    type Output;
    fn bitand(self, rhs: RHS) -> Self::Output;
}

#[lang = "bitor"]
pub trait BitOr<RHS=Self> {
    type Output;
    fn bitor(self, rhs: RHS) -> Self::Output;
}

#[lang = "bitxor"]
pub trait BitXor<RHS=Self> {
    type Output;
    fn bitxor(self, rhs: RHS) -> Self::Output;
}

#[lang = "shl"]
pub trait Shl<RHS> {
    type Output;
    fn shl(self, rhs: RHS) -> Self::Output;
}

#[lang = "shr"]
pub trait Shr<RHS> {
    type Output;
    fn shr(self, rhs: RHS) -> Self::Output;
}

#[lang = "index"]
pub trait Index<Idx: ?Sized> {
    type Output: ?Sized;
    fn index(&self, index: Idx) -> &Self::Output;
}

#[lang = "index_mut"]
pub trait IndexMut<Idx: ?Sized>: Index<Idx> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output;
}

#[lang = "range_full"]
pub struct RangeFull;

#[lang = "range"]
pub struct Range<Idx> {
    pub start: Idx,
    pub end: Idx,
}

#[lang = "range_from"]
pub struct RangeFrom<Idx> {
    pub start: Idx,
}

#[lang = "range_to"]
pub struct RangeTo<Idx> {
    pub end: Idx,
}

#[lang = "deref"]
pub trait Deref {
    type Target: ?Sized;
    fn deref(&self) -> &Self::Target;
}

impl<'a, T> Deref for &'a T {
    type Target = T;
    fn deref(&self) -> &T { *self }
}

#[lang = "deref_mut"]
pub trait DerefMut: Deref {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Self::Target;
}

impl<'a, T> Deref for &'a mut T {
    type Target = T;
    fn deref(&self) -> &T { *self }
}

impl<'a, T> DerefMut for &'a mut T {
    fn deref_mut(&mut self) -> &mut T { *self }
}

#[lang = "fn"]
pub trait Fn<Args> : FnMut<Args> {
    extern "rust-call" fn call(&self, args: Args) -> Self::Output;
}

#[lang = "fn_mut"]
pub trait FnMut<Args> : FnOnce<Args> {
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
}

#[lang = "fn_once"]
pub trait FnOnce<Args> {
    type Output;
    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}

#[lang = "eq"]
pub trait Eq<Rhs: ?Sized = Self> {
    fn eq(&self, other: &Rhs) -> bool;
    fn ne(&self, other: &Rhs) -> bool { !self.eq(other) }
}

impl<'a, T: Eq+?Sized> Eq for &'a T {
    fn eq(&self, other: &&T) -> bool { (*self).eq(*other) }
}

/// Result of a comparison of two values.
#[derive(Copy, Eq)]
pub enum Ordering {
    /// The first value is smaller than the second.
    Less,
    /// The values are equal.
    Equal,
    /// The second value is larger than the firest.
    Greater,
}

#[lang = "ord"]
pub trait PartialOrd<Rhs: ?Sized = Self> : Eq {
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;

    fn lt(&self, other: &Rhs) -> bool {
        self.partial_cmp(other) == Some(Ordering::Less)
    }

    fn le(&self, other: &Rhs) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Less) | Some(Ordering::Equal) => true,
            _ => false,
        }
    }

    fn gt(&self, other: &Rhs) -> bool {
        self.partial_cmp(other) == Some(Ordering::Greater)
    }

    fn ge(&self, other: &Rhs) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Greater) | Some(Ordering::Equal) => true,
            _ => false,
        }
    }
}
