// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::rc::{Rc};
use std::marker::{Leak};

#[inline(never)]
fn foo<T: Leak>(t: &Rc<T>) {
    drop(t.clone());  
}

fn main() {
    foo(&Rc::new().unwrap().set(1usize));
}
