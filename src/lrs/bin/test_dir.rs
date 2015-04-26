// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate lrs;

fn main() {
    for entry in lrs::dir::iter(".", None) {
        println!("{:?}", entry);
    }

    //lrs::dir::walk(".", |entry| {
    //    println!("{:?}", entry);
    //    if entry.name.as_bytes()[0] != b'.' && entry.ty == lrs::dir::Type::Directory {
    //        println!("RECURSING");
    //        lrs::dir::WalkOp::Recurse
    //    } else {
    //        lrs::dir::WalkOp::Continue
    //    }
    //});
}
