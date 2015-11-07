// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

macro_rules! tt {
    ($fmt:expr, $name:ident, $some:expr, $e1:expr, $e2:expr) => {
        #[test]
        fn $name() {
            let mut buf = [0; 20];
            let mut buf = Vec::buffered(&mut buf);
            write!(buf, $fmt, $some);
            test!(&*buf == $e1);
            buf.truncate(0);
            write!(buf, $fmt, None::<u8>);
            test!(&*buf == $e2);
        }
    }
}

tt!("{:?}", debug, Some("ä"), "Some(\"\\u{e4}\")", "None");
tt!("{}", display, Some("ä"), "Some(ä)", "None");
tt!("{:x}", lower_hex, Some(255u8), "Some(ff)", "None");
tt!("{:X}", upper_hex, Some(255u8), "Some(FF)", "None");
