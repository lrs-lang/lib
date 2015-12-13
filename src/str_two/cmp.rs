// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::ops::{Eq, Deref};
use str_one::{ByteStr, CStr, NoNullStr};
use string::{String};
use c_string::{CString};
use alloc::{MemPool};

macro_rules! owned {
    ($one:ident, $two:ident) => {
        impl<H1, H2> Eq<$two<H2>> for $one<H1>
            where H1: MemPool,
                  H2: MemPool,
        {
            fn eq(&self, other: &$two<H2>) -> bool {
                self.deref() == other.deref()
            }
        }
    }
}

owned!(CString, CString);
owned!(CString, String);
owned!(String,  CString);
owned!(String,  String);

macro_rules! borrowed_no_str {
    ($one:ident, $two:ty) => {
        impl<H> Eq<$two> for $one<H>
            where H: MemPool,
        {
            fn eq(&self, other: &$two) -> bool {
                let deref: &[u8] = self.deref().deref();
                deref == other
            }
        }
    }
}

borrowed_no_str!(CString, ByteStr);
borrowed_no_str!(CString, NoNullStr);
borrowed_no_str!(CString, CStr);
borrowed_no_str!(CString, str);
borrowed_no_str!(CString, [u8]);

macro_rules! borrowed_str {
    ($one:ident, $two:ty) => {
        impl<H> Eq<$two> for $one<H>
            where H: MemPool,
        {
            fn eq(&self, other: &$two) -> bool {
                self.as_bytes() == other
            }
        }
    }
}

borrowed_str!(String, ByteStr);
borrowed_str!(String, NoNullStr);
borrowed_str!(String, CStr);
borrowed_str!(String, str);
borrowed_str!(String, [u8]);
