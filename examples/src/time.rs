// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::time::*;

fn main() {
    let clock = REAL;
    let now = clock.get_time().unwrap();

    let zone = Zone::local().unwrap();

    let exp = zone.expand(now);

    println!("{:?}", exp);
}
