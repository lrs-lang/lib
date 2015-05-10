// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_getopt"]
#![crate_type = "lib"]
#![feature(plugin, no_std, custom_derive)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use]
extern crate lrs_core as core;
extern crate lrs_base as base;
extern crate lrs_str_one as str_one;

#[prelude_import] use base::prelude::*;
use str_one::{CStr, NoNullStr};

mod lrs { pub use base::lrs::*; }

pub struct Getopt<'a, I>
    where I: Iterator<Item=&'static CStr>,
{
    opts: &'a [(Option<char>, Option<&'static str>, bool)],
    cur: &'static CStr,
    args: I,
    num: usize,
}

impl<'a, I> Getopt<'a, I>
    where I: Iterator<Item=&'static CStr>,
{
    pub fn new(args: I,
               opts: &'a [(Option<char>, Option<&'static str>, bool)]) -> Getopt<'a, I> {
        Getopt {
            opts: opts,
            cur: CStr::empty(),
            args: args,
            num: 0,
        }
    }

    pub fn used(&self) -> usize {
        self.num
    }
}

impl<'a, I> Iterator for Getopt<'a, I>
    where I: Iterator<Item=&'static CStr>,
{
    type Item = (&'static NoNullStr, Option<&'static CStr>);
    fn next(&mut self) -> Option<(&'static NoNullStr, Option<&'static CStr>)> {
        if self.cur.len() > 0 {
            for &(short, _, optional) in self.opts {
                if short == Some(self.cur[0] as char) {
                    let arg = &self.cur[..1];
                    let opt = if self.cur.len() > 1 {
                        Some(&self.cur[1..])
                    } else if optional {
                        None
                    } else {
                        self.args.next()
                    };
                    self.cur = CStr::empty();
                    return Some((arg, opt));
                }
            }
            let rv = &self.cur[..1];
            self.cur = &self.cur[1..];
            return Some((rv, None));
        }
        
        if let Some(next) = self.args.next() {
            if next.len() < 2 || next[0] != b'-' {
                return None;
            }
            self.num += 1;
            if next == "--" {
                return None;
            }
            if next.starts_with("--") {
                let arg = &next[2..];
                for &(_, long, optional) in self.opts {
                    let long = match long {
                        Some(l) if arg.starts_with(l) => l,
                        _ => continue,
                    };
                    if optional {
                        if long.len() == arg.len() {
                            return Some((arg.as_ref(), None));
                        } else if arg[long.len()] == b'=' {
                            return Some((&arg[..long.len()], Some(&arg[long.len()+1..])));
                        }
                    } else if long.len() == arg.len() {
                        return Some((arg.as_ref(), self.args.next()));
                    } else if arg[long.len()] == b'=' {
                        return Some((&arg[..long.len()], Some(&arg[long.len()+1..])));
                    }
                }
                return Some((arg.as_ref(), None));
            }
            self.cur = &next[1..];
            return self.next();
        }

        None
    }
}
