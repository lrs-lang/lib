// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin)]
#![plugin(lrs_core_plugin)]

use std::process::{resource, resource_limit, set_resource_limit};

fn main() {
    println!("{:?}", set_resource_limit(0, resource::FileDescriptors, 0, 1000000));
    println!("{:?}", resource_limit(0, resource::FileDescriptors));
}
