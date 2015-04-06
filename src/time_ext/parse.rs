// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::io::{BufRead, Read};
use std::num::{Int};
use std::marker::{PhantomFn};
use std::{mem};

use super::{Zone};

use core::result::{Result};
use core::errno::{self};

macro_rules! rd {
    ($ip:expr, $t:ty, $sz:expr) => {
        if $ip.len() < $sz {
            Err(errno::InvalidSequence)
        } else {
            let mut v = [0; $sz];
            let _ = $ip.read(&mut v);
            unsafe { Ok(Int::from_be(mem::transmute::<_, $t>(v))) }
        }
    }
}

fn read_u8(ip:  &mut &[u8]) -> Result<u8> { rd!(ip, u8,  1) }
fn read_i32(ip: &mut &[u8]) -> Result<i32> { rd!(ip, i32, 4) }
fn read_i64(ip: &mut &[u8]) -> Result<i64> { rd!(ip, i64, 8) }

trait TReader: PhantomFn<Self> {
    fn read_seconds(ip: &mut &[u8]) -> Result<i64>;
    fn seconds_width() -> usize;
}

struct T32Reader;

impl TReader for T32Reader {
    fn read_seconds(ip: &mut &[u8]) -> Result<i64> {
        read_i32(ip).map(|v| v as i64)
    }
    fn seconds_width() -> usize { 4 }
}

struct T64Reader;

impl TReader for T64Reader {
    fn read_seconds(ip: &mut &[u8]) -> Result<i64> {
        read_i64(ip)
    }
    fn seconds_width() -> usize { 8 }
}

pub fn parse(ip: &mut &[u8]) -> Result<Zone> {
    if ip.len() < 5 {
        return Err(errno::InvalidSequence);
    }
    if &ip[..4] != b"TZif" {
        return Err(errno::InvalidSequence);
    }
    let version = match ip[4] {
        0 => 1,
        _ => 2,
    };
    if version > 1 {
        try!(discard::<T32Reader>(ip));
        parse_::<T64Reader>(ip)
    } else {
        parse_::<T32Reader>(ip)
    }
}

fn parse_<T: TReader>(ip: &mut &[u8]) -> Result<Zone> {
    ip.consume(20);

    let is_utc_indicators = try!(read_i32(ip)) as usize;
    let is_std_indicators = try!(read_i32(ip)) as usize;
    let num_leap_seconds  = try!(read_i32(ip)) as usize;
    let num_transitions   = try!(read_i32(ip)) as usize;
    let num_states        = try!(read_i32(ip)) as usize;
    let abbr_bytes        = try!(read_i32(ip)) as usize;

    if num_states == 0 {
        return Err(errno::InvalidSequence);
    }

    let mut transitions  = vec!();
    let mut states       = vec!();
    let mut leap_seconds = vec!();

    for _ in 0..num_transitions {
        transitions.push((try!(T::read_seconds(ip)), 0));
    }

    for i in 0..num_transitions {
        let state = try!(read_u8(ip)) as usize;
        if state >= num_states {
            return Err(errno::InvalidSequence);
        }
        transitions[i].1 = state;
    }

    for _ in 0..num_states {
        states.push((try!(read_i32(ip)) as i64, try!(read_u8(ip)) != 0));
        ip.consume(1);
    }

    ip.consume(abbr_bytes);

    for _ in 0..num_leap_seconds {
        leap_seconds.push((try!(T::read_seconds(ip)), try!(read_i32(ip)) as i64));
    }

    ip.consume(is_std_indicators);
    ip.consume(is_utc_indicators);

    Ok(Zone {
        transitions:  transitions,
        states:       states,
        leap_seconds: leap_seconds,
    })
}

fn discard<T: TReader>(ip: &mut &[u8]) -> Result<()> {
    ip.consume(20);

    let is_utc_indicators = try!(read_i32(ip)) as usize;
    let is_std_indicators = try!(read_i32(ip)) as usize;
    let num_leap_seconds  = try!(read_i32(ip)) as usize;
    let num_transitions   = try!(read_i32(ip)) as usize;
    let num_states        = try!(read_i32(ip)) as usize;
    let abbr_bytes        = try!(read_i32(ip)) as usize;

    let bytes =   num_transitions * T::seconds_width()
                + num_transitions * 1
                + num_states * (4 + 1 + 1)
                + abbr_bytes * 1
                + num_leap_seconds * (T::seconds_width() + 4)
                + is_utc_indicators * 1
                + is_std_indicators * 1;

    ip.consume(bytes);
    Ok(())
}
