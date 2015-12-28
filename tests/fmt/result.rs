// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::alloc::{OncePool};

macro_rules! tt {
    ($fmt:expr, $name:ident, $val:expr, $e1:expr, $e2:expr) => {
        #[test]
        fn $name() {
            let mut buf = [0; 20];
            let mut buf = Vec::with_pool(OncePool::new(buf.as_mut()));
            write!(buf, $fmt, Ok::<_, u8>($val));
            test!(&*buf == $e1);
            buf.truncate(0);
            write!(buf, $fmt, Err::<u8, _>($val));
            test!(&*buf == $e2);
        }
    }
}

tt!("{:?}", debug, "ä", "Ok(\"\\u{e4}\")", "Err(\"\\u{e4}\")");
tt!("{}", display, "ä", "Ok(ä)", "Err(ä)");
tt!("{:x}", lower_hex, 255u8, "Ok(ff)", "Err(ff)");
tt!("{:X}", upper_hex, 255u8, "Ok(FF)", "Err(FF)");
