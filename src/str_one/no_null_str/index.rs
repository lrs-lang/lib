// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::{mem};
use core::ops::{Index, IndexMut, Range, RangeFrom, RangeTo, RangeFull};
use {NoNullStr};

impl Index<usize> for NoNullStr {
    type Output = u8;
    fn index(&self, idx: usize) -> &u8 {
        &self.0[idx]
    }
}

impl Index<RangeFull> for NoNullStr {
    type Output = NoNullStr;
    fn index(&self, _: RangeFull) -> &NoNullStr { self }
}

impl IndexMut<RangeFull> for NoNullStr {
    fn index_mut(&mut self, _: RangeFull) -> &mut NoNullStr { self }
}

impl Index<RangeTo<usize>> for NoNullStr {
    type Output = NoNullStr;
    fn index(&self, idx: RangeTo<usize>) -> &NoNullStr {
        unsafe { mem::cast(&self.0[idx]) }
    }
}

impl IndexMut<RangeTo<usize>> for NoNullStr {
    fn index_mut(&mut self, idx: RangeTo<usize>) -> &mut NoNullStr {
        unsafe { mem::cast(&mut self.0[idx]) }
    }
}

impl Index<RangeFrom<usize>> for NoNullStr {
    type Output = NoNullStr;
    fn index(&self, idx: RangeFrom<usize>) -> &NoNullStr {
        unsafe { mem::cast(&self.0[idx]) }
    }
}

impl IndexMut<RangeFrom<usize>> for NoNullStr {
    fn index_mut(&mut self, idx: RangeFrom<usize>) -> &mut NoNullStr {
        unsafe { mem::cast(&mut self.0[idx]) }
    }
}

impl Index<Range<usize>> for NoNullStr {
    type Output = NoNullStr;
    fn index(&self, idx: Range<usize>) -> &NoNullStr {
        unsafe { mem::cast(&self.0[idx]) }
    }
}

impl IndexMut<Range<usize>> for NoNullStr {
    fn index_mut(&mut self, idx: Range<usize>) -> &mut NoNullStr {
        unsafe { mem::cast(&mut self.0[idx]) }
    }
}
