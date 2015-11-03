// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use io::{Read};
use core::{mem, cmp};
use base::{error};

use super::{Zone};

macro_rules! rd {
    ($ip:expr, $t:ty, $sz:expr) => {
        if $ip.len() < $sz {
            Err(error::InvalidSequence)
        } else {
            let mut v = [0; $sz];
            let _ = $ip.read(&mut v);
            unsafe { Ok(mem::cast::<_, $t>(v).from_be()) }
        }
    }
}

fn read_u8(ip:  &mut &[u8]) -> Result<u8> { rd!(ip, u8,  1) }
fn read_i32(ip: &mut &[u8]) -> Result<i32> { rd!(ip, i32, 4) }
fn read_i64(ip: &mut &[u8]) -> Result<i64> { rd!(ip, i64, 8) }

trait TReader {
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
        return Err(error::InvalidSequence);
    }
    if &ip[..4] != "TZif" {
        return Err(error::InvalidSequence);
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

fn consume(buf: &mut &[u8], n: usize) {
    let min = cmp::min(buf.len(), n);
    *buf = &buf[min..];
}

fn parse_<T: TReader>(ip: &mut &[u8]) -> Result<Zone> {
    consume(ip, 20);

    let is_utc_indicators = try!(read_i32(ip)) as usize;
    let is_std_indicators = try!(read_i32(ip)) as usize;
    let num_leap_seconds  = try!(read_i32(ip)) as usize;
    let num_transitions   = try!(read_i32(ip)) as usize;
    let num_states        = try!(read_i32(ip)) as usize;
    let abbr_bytes        = try!(read_i32(ip)) as usize;

    if num_states == 0 {
        return Err(error::InvalidSequence);
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
            return Err(error::InvalidSequence);
        }
        transitions[i].1 = state;
    }

    for _ in 0..num_states {
        states.push((try!(read_i32(ip)) as i64, try!(read_u8(ip)) != 0));
        consume(ip, 1);
    }

    consume(ip, abbr_bytes);

    for _ in 0..num_leap_seconds {
        leap_seconds.push((try!(T::read_seconds(ip)), try!(read_i32(ip)) as i64));
    }

    consume(ip, is_std_indicators);
    consume(ip, is_utc_indicators);

    Ok(Zone {
        transitions:  transitions,
        states:       states,
        leap_seconds: leap_seconds,
    })
}

fn discard<T: TReader>(ip: &mut &[u8]) -> Result {
    consume(ip, 20);

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

    consume(ip, bytes);
    Ok(())
}
