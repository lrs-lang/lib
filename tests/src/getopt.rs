// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin)]
#![plugin(lrs_core_plugin)]

use std::{env, getopt};

fn main() {
    let opts = [
        (Some('t'), Some("test"), false),
    ];
    let mut args = env::args();
    args.next();
    let mut getopts = getopt::Getopt::new(args, &opts);
    for (opt, arg) in getopts {
        println!("{:?} = {:?}", opt, arg);
    }
}
