// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::alloc::{OncePool};

#[test]
fn debug_char() {
    let mut buf = [0; 30];
    let mut buf = Vec::with_pool(OncePool::new(buf.as_mut()));
    write!(&mut buf, "{:?}", 'a');
    write!(&mut buf, "{:?}", 'ä');
    write!(&mut buf, "{:?}", '日');
    test!(&*buf == "'a''\\u{e4}''\\u{65e5}'");
}

#[test]
fn display_char() {
    let mut buf = [0; 10];
    let mut buf = Vec::with_pool(OncePool::new(buf.as_mut()));
    write!(&mut buf, "{}", 'a');
    write!(&mut buf, "{}", 'ä');
    write!(&mut buf, "{}", '日');
    test!(&*buf == "aä日");
}

#[test]
fn debug_str() {
    let mut buf = [0; 30];
    let mut buf = Vec::with_pool(OncePool::new(buf.as_mut()));
    write!(&mut buf, "{:?}", "aä日");
    test!(&*buf == "\"a\\u{e4}\\u{65e5}\"");
}

#[test]
fn display_str() {
    let mut buf = [0; 30];
    let mut buf = Vec::with_pool(OncePool::new(buf.as_mut()));
    write!(&mut buf, "{}", "aä日");
    test!(&*buf == "aä日");
}
