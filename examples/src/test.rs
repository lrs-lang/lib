// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::env::{self};
use std::string::{CStr, AsByteStr};
use std::process::{exit};
use std::file::{self, Type};
use std::file::mode::{MODE_SET_USER_ID, MODE_SET_GROUP_ID};
use std::iter::{IteratorExt};

use PathCondition::*;
use IntegerCondition::*;

#[derive(Eq)]
enum PathCondition {
    BlockSpecial,
    CharacterSpecial,
    Directory,
    Exists,
    Regular,
    GroupIDFlag,
    SymLink,
    FIFO,
    Readable,
    Socket,
    NonEmpty,
    UserIDFlag,
    Writable,
    Executable,
}

enum IntegerCondition {
    Equal,
    Unequal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

fn main() {
    let num_args = match env::arg_count() {
        1 => exit(1),
        v @ 2...6 => v,
        _ => exit(2),
    };

    let mut args = [CStr::empty(); 6];
    env::args().collect_into(&mut args);

    let args = match args[0].as_byte_str().as_ref() {
        b"[" => match args[num_args-1].as_byte_str().as_ref() {
            b"]" => &args[1..num_args-1],
            _ => exit(2),
        },
        _ => &args[1..num_args],
    };

    let rv = match args.len() {
        1 => one(args),
        2 => two(args),
        3 => three(args),
        4 => four(args),
        _ => exit(2),
    };

    exit(1 - rv as u8);
}

fn one(args: &[&CStr]) -> bool {
    args[0].len() > 0
}

fn two(args: &[&CStr]) -> bool {
    match args[0].as_byte_str().as_ref() {
        b"!" => !one(&args[1..]),
        b"-b" => path(args[1], BlockSpecial),
        b"-c" => path(args[1], CharacterSpecial),
        b"-d" => path(args[1], Directory),
        b"-e" => path(args[1], Exists),
        b"-f" => path(args[1], Regular),
        b"-g" => path(args[1], GroupIDFlag),
        b"-h" => path(args[1], SymLink),
        b"-L" => path(args[1], SymLink),
        b"-n" => one(&args[1..]),
        b"-p" => path(args[1], FIFO),
        b"-r" => path(args[1], Readable),
        b"-S" => path(args[1], Socket),
        b"-s" => path(args[1], NonEmpty),
        b"-t" => isatty(args[1]),
        b"-u" => path(args[1], UserIDFlag),
        b"-w" => path(args[1], Writable),
        b"-x" => path(args[1], Executable),
        b"-z" => !one(&args[1..]),
        _ => false,
    }
}

fn three(args: &[&CStr]) -> bool {
    match args[1].as_byte_str().as_ref() {
        b"=" => args[0] == args[2],
        b"!=" => args[0] != args[2],
        b"-eq" => integers(args[0], args[2], Equal),
        b"-ne" => integers(args[0], args[2], Unequal),
        b"-gt" => integers(args[0], args[2], Greater),
        b"-ge" => integers(args[0], args[2], GreaterEqual),
        b"-lt" => integers(args[0], args[2], Less),
        b"-le" => integers(args[0], args[2], LessEqual),
        _ => match args[0].as_byte_str().as_ref() {
            b"!" => !two(&args[1..]),
            _ => false,
        }
    }
}

fn four(args: &[&CStr]) -> bool {
    match args[0].as_byte_str().as_ref() {
        b"!" => !three(&args[1..]),
        _ => false,
    }
}

fn integers(a: &CStr, b: &CStr, cond: IntegerCondition) -> bool {
    let (a, b): (isize, isize) = match (a.parse(), b.parse()) {
        (Ok(a), Ok(b)) => (a, b),
        _ => return false,
    };
    match cond {
        Equal        => a == b,
        Unequal      => a != b,
        Greater      => a >  b,
        GreaterEqual => a >= b,
        Less         => a <  b,
        LessEqual    => a <= b,
    }
}

fn isatty(fd: &CStr) -> bool {
    // TODO
    false
}

fn path(path: &CStr, cond: PathCondition) -> bool {
    if cond == SymLink {
        return match file::info_no_follow(path) {
            Ok(f) => f.file_type() == Type::SymLink,
            _ => false,
        }
    }

    let access_mode = match cond {
        Readable   => Some("r--".parse().unwrap()),
        Writable   => Some("-w-".parse().unwrap()),
        Executable => Some("--x".parse().unwrap()),
        _ => None,
    };

    if let Some(mode) = access_mode {
        return file::can_access(path, mode) == Ok(true);
    }

    let info = match file::info(path) {
        Ok(i) => i,
        _ => return false,
    };

    match cond {
        Exists           => true,
        NonEmpty         => info.size() > 0,
        GroupIDFlag      => info.mode().is_set(MODE_SET_GROUP_ID),
        UserIDFlag       => info.mode().is_set(MODE_SET_USER_ID),
        BlockSpecial     => info.file_type() == Type::BlockDevice,
        CharacterSpecial => info.file_type() == Type::CharDevice,
        Directory        => info.file_type() == Type::Directory,
        Regular          => info.file_type() == Type::File,
        FIFO             => info.file_type() == Type::FIFO,
        Socket           => info.file_type() == Type::Socket,
        _                => false,
    }
}
