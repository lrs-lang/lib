// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::{mem};

pub struct CStr {
    inner: [u8]
}

// This is quite pathetic and should be implemented at a higher level but w_syscall wants
// it so we do what we can here and 

impl CStr {
    pub fn as_bytes(&self) -> &[u8] {
        let bytes = self.as_bytes_with_null();
        &bytes[..bytes.len() - 1]
    }

    pub fn as_bytes_with_null(&self) -> &[u8] {
        unsafe { mem::cast(&self.inner) }
    }

    pub fn empty() -> &'static CStr {
        static EMPTY: [u8; 1] = [0];
        unsafe { mem::cast(&EMPTY[..]) }
    }
}
