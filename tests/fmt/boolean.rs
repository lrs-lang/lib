// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

macro_rules! tt {
    ($fmt:expr, $name:ident) => {
        #[test]
        fn $name() {
            let mut buf = [0; 10];
            let mut buf = Vec::buffered(&mut buf);
            write!(&mut buf, $fmt, true).unwrap();
            test!(&*buf == "true");

            buf.truncate(0);
            write!(&mut buf, $fmt, false).unwrap();
            test!(&*buf == "false");
        }
    }
}

tt!("{:?}", debug);
tt!("{}", display);
