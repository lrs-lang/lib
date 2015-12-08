// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_netlink"]
#![crate_type = "lib"]
#![feature(plugin, no_std, custom_derive)]
#![plugin(lrs_core_plugin)]
#![no_std]
#![allow(non_upper_case_globals)] 

extern crate lrs_base       as base;
extern crate lrs_cty        as cty;
extern crate lrs_alloc      as alloc;
extern crate lrs_vec        as vec;
extern crate lrs_fmt        as fmt_;

mod std { pub use fmt_::std::*; pub use cty; }

use base::prelude::*;
use base::{error};
use cty::{c_int};
use fmt_::{Debug, Write};

pub mod kind;
pub mod flags;
pub mod fmt;
pub mod route;
pub mod parse;

#[repr(C)]
#[derive(Pod, Eq)]
pub struct MsgHeader {
    pub len:      u32,
    pub ty:       kind::Kind,
    pub flags:    flags::NlFlags,
    pub sequence: u32,
    pub port:     u32,
}

impl Debug for MsgHeader {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        write!(w, "MsgHeader {{ len: {}, ty: {:?}, flags: {:?}, sequence: {}, port: {} \
                   }}",
               self.len, self.ty, self.flags, self.sequence, self.port)
    }
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct MsgError {
    pub error:  c_int,
    pub header: MsgHeader,
}

impl Debug for MsgError {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        if self.error == 0 {
            write!(w, "MsgError {{ ack, header: {:?} }}", self.header)
        } else {
            write!(w, "MsgError {{ error: {:?}, header: {:?} }}",
                   error::Errno(-self.error), self.header)
        }
    }
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct Attr {
    pub len: u16,
    pub ty: u16,
}
