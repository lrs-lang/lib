// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::{mem};

pub struct CStr {
    inner: [u8]
}

impl CStr {
    pub unsafe fn from_nt_slice(slice: &[u8]) -> &CStr {
        mem::cast(slice)
    }

    pub unsafe fn from_nt_slice_mut(slice: &mut [u8]) -> &mut CStr {
        mem::cast(slice)
    }

    // pub fn as_ptr(&self) -> *const u8 {
    //     self.inner.as_ptr() as *const u8
    // }

    pub fn as_bytes(&self) -> &[u8] {
        let bytes = self.as_bytes_with_null();
        &bytes[..bytes.len() - 1]
    }

    pub fn as_bytes_with_null(&self) -> &[u8] {
        unsafe { mem::cast(&self.inner) }
    }
}
