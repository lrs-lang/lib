// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::ops::{Eq, Deref};
use str_one::{ByteStr, CStr, NoNullStr};
use string::{String};
use byte_string::{ByteString};
use c_string::{CString};
use no_null_string::{NoNullString};
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

owned!(ByteString,   ByteString);
owned!(ByteString,   NoNullString);
owned!(ByteString,   CString);
owned!(ByteString,   String);
owned!(NoNullString, ByteString);
owned!(NoNullString, NoNullString);
owned!(NoNullString, CString);
owned!(NoNullString, String);
owned!(CString,      ByteString);
owned!(CString,      NoNullString);
owned!(CString,      CString);
owned!(CString,      String);
owned!(String,       ByteString);
owned!(String,       NoNullString);
owned!(String,       CString);
owned!(String,       String);

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

borrowed_no_str!(ByteString,   ByteStr);
borrowed_no_str!(ByteString,   NoNullStr);
borrowed_no_str!(ByteString,   CStr);
borrowed_no_str!(ByteString,   str);
borrowed_no_str!(ByteString,   [u8]);
borrowed_no_str!(NoNullString, ByteStr);
borrowed_no_str!(NoNullString, NoNullStr);
borrowed_no_str!(NoNullString, CStr);
borrowed_no_str!(NoNullString, str);
borrowed_no_str!(NoNullString, [u8]);
borrowed_no_str!(CString,      ByteStr);
borrowed_no_str!(CString,      NoNullStr);
borrowed_no_str!(CString,      CStr);
borrowed_no_str!(CString,      str);
borrowed_no_str!(CString,      [u8]);

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

borrowed_str!(String,       ByteStr);
borrowed_str!(String,       NoNullStr);
borrowed_str!(String,       CStr);
borrowed_str!(String,       str);
borrowed_str!(String,       [u8]);
