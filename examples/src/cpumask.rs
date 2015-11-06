// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::thread::{CpuMask, cpus, set_cpus};

fn main() {
    println!("{:?}", set_cpus(0, CpuMask::new(&[1])));
    println!("{:?}", cpus(0, &mut [0; 1024]));
}
