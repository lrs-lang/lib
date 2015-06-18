// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use core::{mem};

pub mod dense;

pub trait Bucket<K, V> {
    fn is_empty(&self) -> bool;
    fn is_deleted(&self) -> bool;
    fn is_set(&self) -> bool;

    unsafe fn set_empty(&mut self);
    unsafe fn set_deleted(&mut self);
    unsafe fn copy(&mut self, other: &Self);

    unsafe fn set(&mut self, key: K, value: V);
    unsafe fn replace(&mut self, key: K, value: V) -> (K, V);
    unsafe fn remove(&mut self) -> (K, V);
    unsafe fn key(&self) -> &K;
    unsafe fn mut_key(&mut self) -> &mut K;
    unsafe fn value(&self) -> &V;
    unsafe fn mut_value(&mut self) -> &mut V;
}

pub struct EmptyBucket<'a, K: 'a, V: 'a, B: 'a>
    where B: Bucket<K, V>
{
    bucket: &'a mut B,
    _marker: PhantomData<(K, V)>,
}

impl<'a, K, V, B> EmptyBucket<'a, K, V, B>
    where B: Bucket<K, V>
{
    pub unsafe fn from_bucket(bucket: &'a mut B) -> Self {
        // debug_assert!(bucket.is_empty());
        EmptyBucket { bucket: bucket, _marker: PhantomData }
    }

    pub fn set(self, key: K, value: V) -> MutSetBucket<'a, K, V, B> {
        unsafe {
            self.bucket.set(key, value);
            MutSetBucket::from_bucket(self.bucket)
        }
    }
}

pub struct SetBucket<'a, K: 'a, V: 'a, B: 'a>
    where B: Bucket<K, V>
{
    bucket: &'a B,
    _marker: PhantomData<(K, V)>,
}

impl<'a, K, V, B> SetBucket<'a, K, V, B>
    where B: Bucket<K, V>
{
    pub unsafe fn from_bucket(bucket: &'a B) -> Self {
        // debug_assert!(bucket.is_set());
        SetBucket { bucket: bucket, _marker: PhantomData }
    }

    pub fn key(&self) -> &'a K {
        unsafe { self.bucket.key() }
    }

    pub fn value(&self) -> &'a V {
        unsafe { self.bucket.value() }
    }
}

pub struct MutSetBucket<'a, K: 'a, V: 'a, B: 'a>
    where B: Bucket<K, V>
{
    bucket: &'a mut B,
    _marker: PhantomData<(K, V)>,
}

impl<'a, K, V, B> MutSetBucket<'a, K, V, B>
    where B: Bucket<K, V>
{
    pub unsafe fn from_bucket(bucket: &'a mut B) -> Self {
        // debug_assert!(bucket.is_set());
        MutSetBucket { bucket: bucket, _marker: PhantomData }
    }

    pub fn mut_value(self) -> &'a mut V {
        unsafe { self.bucket.mut_value() }
    }

    pub fn remove(self) -> (EmptyBucket<'a, K, V, B>, K, V) {
        unsafe {
            let (k, v) = self.bucket.remove();
            let b = EmptyBucket::from_bucket(self.bucket);
            (b, k, v)
        }
    }
}

impl<'a, K, V, B> Deref for MutSetBucket<'a, K, V, B>
    where B: Bucket<K, V>
{
    type Target = SetBucket<'a, K, V, B>;
    fn deref(&self) -> &SetBucket<'a, K, V, B> {
        unsafe { mem::cast(self) }
    }
}
