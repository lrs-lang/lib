// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::alloc::{OncePool};

macro_rules! tt {
    ($fmt:expr, $name:ident, $val:expr, $res:expr) => {
        #[test] fn $name() {
            let mut buf = [0; 30];
            let mut buf = Vec::with_pool(OncePool::new(buf.as_mut()));
            write!(&mut buf, $fmt, $val);
            test!(&*buf == $res);
        }
    }
}

tt!("{:?}", debug_i8, -127i8, "-127");
tt!("{:?}", display_i8, -127i8, "-127");

tt!("{:?}", debug_i16, -127i16, "-127");
tt!("{:?}", display_i16, -127i16, "-127");

tt!("{:?}", debug_i32, -127i32, "-127");
tt!("{:?}", display_i32, -127i32, "-127");

tt!("{:?}", debug_i64, i64::min() + 1, "-9223372036854775807");
tt!("{:?}", display_i64, i64::min() + 1, "-9223372036854775807");

tt!("{:?}", debug_u8, 127u8, "127");
tt!("{:?}", display_u8, 127u8, "127");

tt!("{:?}", debug_u16, 127u16, "127");
tt!("{:?}", display_u16, 127u16, "127");

tt!("{:?}", debug_u32, 127u32, "127");
tt!("{:?}", display_u32, 127u32, "127");

tt!("{:?}", debug_u64, u64::max(), "18446744073709551615");
tt!("{:?}", display_u64, u64::max(), "18446744073709551615");

tt!("{:x}", lowerhex_u8, 127u8, "7f");
tt!("{:X}", upperhex_u8, 127u8, "7F");

tt!("{:x}", lowerhex_i16, 127u16, "7f");
tt!("{:X}", upperhex_i16, 127u16, "7F");

tt!("{:x}", lowerhex_i32, 127u32, "7f");
tt!("{:X}", upperhex_i32, 127u32, "7F");

tt!("{:x}", lowerhex_i64, u64::max(), "ffffffffffffffff");
tt!("{:X}", upperhex_i64, u64::max(), "FFFFFFFFFFFFFFFF");
