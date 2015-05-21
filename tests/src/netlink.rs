// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin, no_std)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use] extern crate lrs;
extern crate lrs_cty;
mod core { pub use lrs::core::*; }
#[prelude_import] use lrs::prelude::*;

use lrs::{mem};
use lrs::fd::{STDOUT};
use lrs::socket::netlink::{NlBuf};
use lrs::socket::{Socket, domain, kind, msg};
use lrs::socket::flags::{SOCK_NONE};
use lrs_cty::{
    RTM_NEWLINK,
    NLM_F_REQUEST, NLM_F_ACK, NLM_F_CREATE, NLM_F_EXCL,
    ifinfomsg,
    IFLA_IFNAME, IFLA_LINKINFO,
    IFLA_INFO_KIND, IFLA_INFO_DATA, VETH_INFO_PEER,
    NETLINK_ROUTE,
};

fn main() {
    let mut buf: NlBuf = NlBuf::new();
    let flags = NLM_F_REQUEST | NLM_F_CREATE | NLM_F_EXCL;
    {
        let mut msg = buf.new_msg(0, RTM_NEWLINK, flags, 1, 0).unwrap();
        let head: ifinfomsg = mem::zeroed();
        msg.add_raw(mem::as_bytes(&head));
        msg.add_string(IFLA_IFNAME, "veth0");
        {
            let mut attr = msg.add_nested(0, IFLA_LINKINFO).unwrap();
            attr.add_string(IFLA_INFO_KIND, "veth");
            {
                let mut attr = attr.add_nested(0, IFLA_INFO_DATA).unwrap();
                {
                    let mut attr = attr.add_nested(0, VETH_INFO_PEER).unwrap();
                    attr.add_raw(mem::as_bytes(&head));
                    attr.add_string(IFLA_IFNAME, "enp2s0f0");
                }
            }
        }
    }

    let socket = Socket::new(domain::Netlink, kind::Raw, NETLINK_ROUTE,
                             SOCK_NONE).unwrap();
    socket.send(buf.as_ref(), msg::None).unwrap();
}
