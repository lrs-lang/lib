// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::string::{AsCStr};
use std::getopt::{Getopt};
use std::iter::{IteratorExt};

macro_rules! cs {
    ($t:expr) => {
        concat!($t, "\0").as_cstr().unwrap()
    }
}

macro_rules! cmp {
    ($e:expr, $left:expr) => {
        match $e {
            Some((l, None)) if l == $left => { },
            _ => abort!(),
        }
    };
    ($e:expr, $left:expr, $right:expr) => {
        match $e {
            Some((l, r)) if l == $left && r.unwrap() == $right => { },
            _ => abort!(),
        }
    }
}

#[test]
fn test() {
    let opts = [
        (Some('a'), Some("aa"), false),
        (Some('b'), Some("bb"), true),
    ];
    let args = [
        cs!("-a"), cs!("a"),
        cs!("-aa"),
        cs!("--aa"), cs!("a"),
        cs!("--aa=a"),
        cs!("-b"),
        cs!("-bb"),
        cs!("--bb"),
        cs!("--bb=b"),
        cs!("-cde"),
        cs!("--test"),
        cs!("--"),
        cs!("test"),
    ];
    let mut g = Getopt::new(args.iter().map(|a| *a), &opts);

    cmp!(g.next(), "a", "a");
    cmp!(g.next(), "a", "a");
    cmp!(g.next(), "aa", "a");
    cmp!(g.next(), "aa", "a");
    cmp!(g.next(), "b");
    cmp!(g.next(), "b", "b");
    cmp!(g.next(), "bb");
    cmp!(g.next(), "bb", "b");
    cmp!(g.next(), "c");
    cmp!(g.next(), "d");
    cmp!(g.next(), "e");
    cmp!(g.next(), "test");

    assert!(g.next() == None);
    assert!(g.used() == args.len() - 1);
}
