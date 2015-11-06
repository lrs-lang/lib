// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::env::{args, arg_count, env};

fn main() {
    println!("Have {} args:", arg_count());
    for arg in args() {
        println!("{:?}", arg);
    }
    println!("");
    println!("environment:");
    for arg in env() {
        println!("{:?}", arg);
    }
}
