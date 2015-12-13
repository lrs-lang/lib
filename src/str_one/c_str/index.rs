// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::{mem};
use core::ops::{Index, IndexMut, RangeFrom, RangeTo, Range, RangeFull};
use {CStr, NoNullStr};

impl Index<usize> for CStr {
    type Output = u8;
    fn index(&self, idx: usize) -> &u8 {
        &self.0[idx]
    }
}

impl Index<RangeFull> for CStr {
    type Output = CStr;
    fn index(&self, _: RangeFull) -> &CStr {
        self
    }
}

impl IndexMut<RangeFull> for CStr {
    fn index_mut(&mut self, _: RangeFull) -> &mut CStr {
        self
    }
}

impl Index<RangeFrom<usize>> for CStr {
    type Output = CStr;
    fn index(&self, idx: RangeFrom<usize>) -> &CStr {
        unsafe { mem::cast(&self.0[idx]) }
    }
}

impl IndexMut<RangeFrom<usize>> for CStr {
    fn index_mut(&mut self, idx: RangeFrom<usize>) -> &mut CStr {
        unsafe { mem::cast(&mut self.0[idx]) }
    }
}

impl Index<RangeTo<usize>> for CStr {
    type Output = NoNullStr;
    fn index(&self, idx: RangeTo<usize>) -> &NoNullStr {
        unsafe { mem::cast(&self.0[idx]) }
    }
}

impl IndexMut<RangeTo<usize>> for CStr {
    fn index_mut(&mut self, idx: RangeTo<usize>) -> &mut NoNullStr {
        unsafe { mem::cast(&mut self.0[idx]) }
    }
}

impl Index<Range<usize>> for CStr {
    type Output = NoNullStr;
    fn index(&self, idx: Range<usize>) -> &NoNullStr {
        unsafe { mem::cast(&self.0[idx]) }
    }
}

impl IndexMut<Range<usize>> for CStr {
    fn index_mut(&mut self, idx: Range<usize>) -> &mut NoNullStr {
        unsafe { mem::cast(&mut self.0[idx]) }
    }
}
