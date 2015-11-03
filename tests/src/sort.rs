// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin)]
#![plugin(lrs_core_plugin)]

use std::string::{AsByteStr};
use std::{mem, slice};
use std::cmp::{Ordering};
use std::cmp::Ordering::{Greater};
use std::ops::{FnMut};
use std::fmt::{Debug, Write};

struct T(&'static [u8]);

impl Debug for T {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        self.0.as_byte_str().fmt(w)
    }
}

fn main() {
    let x: &mut [u8] = &mut [1, 2, 0, 0];
    xsort(x);
    for y in &*x {
        println!("{:?}", y);
    }
}

pub fn xsort<T>(s: &mut [T])
    where T: Ord + Debug,
{
    sort_by(s, |one, two| one.cmp(two));
}

pub fn sort_by<T, F>(s: &mut [T], mut f: F)
    where F: FnMut(&T, &T) -> Ordering,
          T: Debug,
{
    unsafe { sort(s, &mut f); }
}

pub unsafe fn sort<T, F>(slice: &mut [T], f: &mut F)
    where F: FnMut(&T, &T) -> Ordering,
          T: Debug,
{
    if slice.len() < 2 {
        return;
    }

    println!("{:?}", slice);
    println!("{:?}", slice[0]);
    let pivot = &mut slice[0] as *mut T;
    let mut start = pivot.add(1);
    let mut end = pivot.add(slice.len() - 1);
    while start <= end {
        while start <= end && f(&*start, &*pivot) != Greater {
            println!("{:?} <= {:?}", &*start, &*pivot);
            start = start.offset(1);
        }
        if start <= end {
            println!("{:?} > {:?}", &*start, &*pivot);
        }
        while start < end && f(&*end, &*pivot) == Greater {
            println!("{:?} > {:?}", &*end, &*pivot);
            end = end.offset(-1);
        }
        if start < end {
            println!("{:?} <= {:?}", &*end, &*pivot);
        }
        if start < end {
            mem::swap(&mut *start, &mut *end);
            start = start.offset(1);
            end = end.offset(-1);
        }
    }
    start = start.offset(-1);
    if pivot < start {
        mem::swap(&mut *pivot, &mut *start);
    }
    let start_len = (start as usize - pivot as usize) / mem::size_of::<T>();

    let one = slice::from_ptr(pivot          , start_len                  );
    let two = slice::from_ptr(start.offset(1), slice.len() - start_len - 1);
    println!("{:?}", one);
    println!("{:?}\n", two);
    sort(slice::from_ptr(pivot          , start_len                  ), &mut *f);
    sort(slice::from_ptr(start.offset(1), slice.len() - start_len - 1), &mut *f);
}
