// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use {mem, slice};
use ops::{Ordering, FnMut};
use ops::Ordering::{Greater};

pub unsafe fn sort<T, F>(slice: &mut [T], f: &mut F)
    where F: FnMut(&T, &T) -> Ordering,
{
    if slice.len() < 2 {
        return;
    }

    let pivot = &mut slice[0] as *mut T;
    let mut start = pivot.add(1);
    let mut end = pivot.add(slice.len() - 1);
    while start <= end {
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
        } else {
            break;
        }
    }
    start = start.offset(-1);
    if pivot < start {
        mem::swap(&mut *pivot, &mut *start);
    }
    let start_len = (start as usize - pivot as usize) / mem::size_of::<T>();

    sort(slice::from_ptr(pivot          , start_len                  ), &mut *f);
    sort(slice::from_ptr(start.offset(1), slice.len() - start_len - 1), &mut *f);
}
