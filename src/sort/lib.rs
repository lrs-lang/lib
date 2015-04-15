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

use core::{mem, slice};
use core::cmp::{Ord};
use core::ops::Ordering::{Greater};

pub fn sort<T: Ord>(slice: &mut [T]) {
    if slice.len() < 2 {
        return;
    }
    unsafe { sort_int(slice); }
}

unsafe fn sort_int<T: Ord>(slice: &mut [T]) {
    let pivot = &mut slice[0] as *mut T;
    let mut start = pivot;
    let mut end = pivot.add(slice.len() - 1);
    while start < end {
        while start <= end && (*start).cmp(&*pivot) != Greater {
            start = start.offset(1);
        }
        while start < end && (*end).cmp(&*pivot) == Greater {
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
    sort(slice::from_ptr(pivot, start_len));
    sort(slice::from_ptr(start.offset(1), slice.len() - start_len - 1));
}
