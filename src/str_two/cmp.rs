// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::ops::{Eq, Deref};
use str_one::{ByteStr, CStr, NoNullStr};
use string::{String};
use byte_string::{ByteString};
use c_string::{CString};
use no_null_string::{NoNullString};
use alloc::{Allocator};

macro_rules! owned {
    ($one:ident, $two:ident) => {
        impl<'a, 'b, H1, H2> Eq<$two<'b, H2>> for $one<'a, H1>
            where H1: Allocator,
                  H2: Allocator,
        {
            fn eq(&self, other: &$two<'b, H2>) -> bool {
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

macro_rules! borrowed {
    ($one:ident, $two:ty) => {
        impl<'a, H> Eq<$two> for $one<'a, H>
            where H: Allocator,
        {
            fn eq(&self, other: &$two) -> bool {
                self.deref() == other
            }
        }
    }
}

borrowed!(ByteString,   ByteStr);
borrowed!(ByteString,   NoNullStr);
borrowed!(ByteString,   CStr);
borrowed!(ByteString,   str);
borrowed!(ByteString,   [u8]);
borrowed!(NoNullString, ByteStr);
borrowed!(NoNullString, NoNullStr);
borrowed!(NoNullString, CStr);
borrowed!(NoNullString, str);
borrowed!(NoNullString, [u8]);
borrowed!(CString,      ByteStr);
borrowed!(CString,      NoNullStr);
borrowed!(CString,      CStr);
borrowed!(CString,      str);
borrowed!(CString,      [u8]);
borrowed!(String,       ByteStr);
borrowed!(String,       NoNullStr);
borrowed!(String,       CStr);
borrowed!(String,       str);
borrowed!(String,       [u8]);
