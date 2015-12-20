// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_socket"]
#![crate_type = "lib"]
#![feature(custom_derive, associated_consts, const_fn)]
#![no_std]
#![allow(non_upper_case_globals)]

extern crate lrs_base       as base;
extern crate lrs_cty        as cty;
extern crate lrs_arch_fns   as arch_fns;
extern crate lrs_str_one    as str_one;
extern crate lrs_fmt        as fmt;
extern crate lrs_time_base  as time_base;
extern crate lrs_rv         as rv;
extern crate lrs_fd         as fd;
extern crate lrs_alloc      as alloc;
extern crate lrs_saturating as saturating;
extern crate lrs_io         as io;
extern crate lrs_syscall    as syscall;

pub use addr::{SockAddr, AddrType};
pub use addr::unix::{UnixSockAddr, UnixAddrType};
pub use addr::ipv4::{Ipv4Addr, Ipv4SockAddr, IPV4_SOCK_ADDR_SIZE};
pub use addr::ipv6::{Ipv6Addr, Ipv6SockAddr, IPV6_SOCK_ADDR_SIZE, Ipv6Scope};

mod std { pub use fmt::std::*; pub use cty; }

pub mod cmsg;
pub mod addr;
pub mod socket;
pub mod kind;
pub mod flags;
pub mod ip_proto;
pub mod nl_proto;
pub mod domain;
pub mod msg;
