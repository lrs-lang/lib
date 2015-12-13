// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::ops::{Index, IndexMut, Range, RangeFrom, RangeTo, RangeFull};
use {ByteStr};

impl Index<usize> for ByteStr {
    type Output = u8;
    fn index(&self, idx: usize) -> &u8 {
        &self.0[idx]
    }
}

impl IndexMut<usize> for ByteStr {
    fn index_mut(&mut self, idx: usize) -> &mut u8 {
        &mut self.0[idx]
    }
}

impl Index<RangeFull> for ByteStr {
    type Output = ByteStr;
    fn index(&self, _: RangeFull) -> &ByteStr { self }
}

impl IndexMut<RangeFull> for ByteStr {
    fn index_mut(&mut self, _: RangeFull) -> &mut ByteStr { self }
}

impl Index<RangeTo<usize>> for ByteStr {
    type Output = ByteStr;
    fn index(&self, idx: RangeTo<usize>) -> &ByteStr {
        self.0[idx].as_ref()
    }
}

impl IndexMut<RangeTo<usize>> for ByteStr {
    fn index_mut(&mut self, idx: RangeTo<usize>) -> &mut ByteStr {
        self.0[idx].as_mut()
    }
}

impl Index<RangeFrom<usize>> for ByteStr {
    type Output = ByteStr;
    fn index(&self, idx: RangeFrom<usize>) -> &ByteStr {
        self.0[idx].as_ref()
    }
}

impl IndexMut<RangeFrom<usize>> for ByteStr {
    fn index_mut(&mut self, idx: RangeFrom<usize>) -> &mut ByteStr {
        self.0[idx].as_mut()
    }
}

impl Index<Range<usize>> for ByteStr {
    type Output = ByteStr;
    fn index(&self, idx: Range<usize>) -> &ByteStr {
        self.0[idx].as_ref()
    }
}

impl IndexMut<Range<usize>> for ByteStr {
    fn index_mut(&mut self, idx: Range<usize>) -> &mut ByteStr {
        self.0[idx].as_mut()
    }
}
