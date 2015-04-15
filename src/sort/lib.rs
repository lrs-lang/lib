// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_sort"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]

#[macro_use]
extern crate linux_core as core;

use core::prelude::*;

use core::{mem, slice};
use core::ops::{Ordering};
use core::cmp::{Ord};
use core::ops::Ordering::{Greater};

pub fn sort<T: Ord>(slice: &mut [T]) {
    sort_by(slice, |one, two| one.cmp(two));
}

pub fn sort_by<T, F: FnMut(&T, &T) -> Ordering>(slice: &mut [T], mut f: F) {
    if slice.len() < 2 {
        return;
    }
    unsafe { sort_by_int(slice, &mut f); }
}

unsafe fn sort_by_int<T, F: FnMut(&T, &T) -> Ordering>(slice: &mut [T], f: &mut F) {
    let pivot = &mut slice[0] as *mut T;
    let mut start = pivot;
    let mut end = pivot.add(slice.len() - 1);
    while start < end {
        while start <= end && f(&*start, &*pivot) != Greater {
            start = start.offset(1);
        }
        while start < end && f(&*end, &*pivot) == Greater {
            end = end.offset(-1);
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
    sort_by(slice::from_ptr(pivot, start_len), &mut *f);
    sort_by(slice::from_ptr(start.offset(1), slice.len() - start_len - 1), &mut *f);
}
