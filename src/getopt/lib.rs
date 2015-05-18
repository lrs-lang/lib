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

/// A simple argument parser.
///
/// = See also
///
/// * link:lrs::getopt::Getopt::new for a complete description
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
    /// Creates a new parser.
    ///
    /// [argument, args]
    /// An iterator over the command line arguments.
    ///
    /// [argument, opts]
    /// The arguments that take (optional) parameters.
    ///
    /// = Remarks
    ///
    /// The args argument should start at the first real argument, not at the program
    /// name. See the example below.
    ///
    /// The opts argument has the following structure: (short name, long name, optional).
    /// The short name is the character used for parsing of POSIX-style (`-a`) options.
    /// The long name is the string used for parsing GNU-style (`--argument`) options. The
    /// short name *must* be in the ASCII set. The optional parameter defines whether the
    /// parameter of this argument is optional.
    ///
    /// The opts argument should thus only contain those arguments that take an (optional)
    /// parameter.
    ///
    /// The parsing proceeds as follows:
    ///
    /// * When an argument of the form `--` is encountered, the parsing stops.
    /// * When an argument shorter than two characters or an argument that doesn't start
    ///   with a `-` is encountered, the parsing stops.
    /// **
    /// {
    /// If an argument starting with `--` is encountered, a long argument is parsed as
    /// follows:
    ///
    /// * If it starts with one of the arguments that take an (optional) parameter and the
    ///   character after the argument name is a `=`, then the part after the `=` is
    ///   returned as the parameter. (For example: `--argument=yo`)
    /// * If it is exactly one of the arguments that take an optional parameter, then
    ///   `None` is returned as the parameter. (For example: `--argument`)
    /// * If it is exactly one of the arguments that take a non-optional parameter, then
    ///   the next argument (if any) is returned as the parameter. (For example:
    ///   `--argument yo`)
    /// * Otherwise the argument is returned as a whole.
    /// }
    /// **
    /// {
    /// Otherwise, if an argument starting with a `-` is encountered, each byte is
    /// parsed as follows:
    ///
    /// * If the byte is one of the arguments that take an (optional) parameter and we're
    ///   not at the end of the current argument, then the rest of the current argument is
    ///   returned as the parameter. (For example: `-ayo`)
    /// * If the byte is one of the arguments that take an optional parameter and we're at
    ///   the end of the current argument, then `None` is returned as the parameter. (For
    ///   example `-a`)
    /// * If the byte is one of the arguments that take a non-optional parameter and we're
    ///   at the end of the current argument, then the next argument (if any) is returned
    ///   as the parameter. (For example: `-a yo`)
    /// * Otherwise the byte is returned as the argument and parsing continues at the next
    ///   byte.
    /// }
    ///
    /// = Examples
    ///
    /// The following example contains the code for parsing argument of a program with the
    /// following help message:
    ///
    /// ----
    /// Usage: my_program [OPTIONS]*
    ///
    /// Options:
    ///   -a, --arg[=PARAMETER]
    ///   -b PARAMETER
    ///       --something-else
    ///   -c
    /// ----
    ///
    /// The code:
    ///
    /// ----
    /// let mut args = env::args();
    /// args.next(); // skip program name
    ///
    /// let params = [
    ///     (Some('a'), Some("arg"), true),
    ///     (Some('b'), None, false),
    /// ];
    ///
    /// for (arg, param) in Getopt::new(args, &params) {
    ///     match arg.as_ref() {
    ///         b"a" | b"arg" => {
    ///             if let Some(param) = param {
    ///                 // ...
    ///             } else {
    ///                 // ...
    ///             }
    ///         },
    ///         b"b" => {
    ///             if let Some(param) = param {
    ///                 // ...
    ///             } else {
    ///                 // errer: missing parameter
    ///             }
    ///         },
    ///         b"something-else" => {
    ///             // ...
    ///         }
    ///         b"c" => {
    ///             // ...
    ///         }
    ///         arg => {
    ///             // error: unexpected argument
    ///         }
    ///     }
    /// }
    /// ----
    ///
    /// This program can be invoked as follows:
    ///
    /// ----
    /// my_program -c
    /// my_program -cc
    /// my_program --something-else -c
    /// my_program -cbPARAMETER
    /// my_program -cb PARAMETER
    /// my_program b PARAMETER
    /// my_program --arg=PARAMETER
    /// my_program -aPARAMETER
    /// ----
    pub fn new(args: I,
               opts: &'a [(Option<char>, Option<&'static str>, bool)]) -> Getopt<'a, I> {
        Getopt {
            opts: opts,
            cur: CStr::empty(),
            args: args,
            num: 0,
        }
    }

    /// Returns the number of arguments used before parsing was stopped.
    ///
    /// = Remarks
    ///
    /// This can be used when parsing arguments that contain trailing data, e.g.,
    ///
    /// ----
    /// Usage: my_program [OPTIONS]* file_name
    /// ----
    ///
    /// = Examples
    ///
    /// ----
    /// let mut args = env::args();
    /// args.next(); // skip program name
    ///
    /// let mut getopts = Getopt::new(args, &[]);
    /// for (arg, param) in &mut getopts {
    ///     // ...
    /// }
    ///
    /// // Add 1 to getopts.used() because we have to skip the program name
    /// println!("file_name: {:?}", env::args().consume(1+getopts.used()).next());
    /// ----
    ///
    /// Note that both of the following invocations will print the correct file name:
    ///
    /// ----
    /// my_program my_file
    /// my_program -- my_file
    /// ----
    ///
    /// Prints
    ///
    /// ----
    /// file_name: "my_file"
    /// file_name: "my_file"
    /// ----
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
