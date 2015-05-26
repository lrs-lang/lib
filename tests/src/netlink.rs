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
use lrs::socket::{Socket, domain, kind};
use lrs::socket::flags::{SOCK_NONE};
use lrs::socket::msg::{MSG_NONE};

use lrs::netlink::proto::{self};
use lrs::netlink::fmt::{NlBuf};
use lrs::netlink::flags::{NLF_REQUEST, NLF_CREATE, NLF_EXCL, NLF_ACK};
use lrs::netlink::route::{self, IfInfoMsg};
use lrs::netlink::parse::{MsgIter, MsgParser};
use lrs::netlink::{MsgError};

fn main() {
    let mut buf: NlBuf = NlBuf::new();
    {
        let flags = NLF_REQUEST | NLF_CREATE | NLF_EXCL | NLF_ACK;
        let mut msg = buf.new_msg(route::op::NewLink, flags, 1, 0).unwrap();
        let head: IfInfoMsg = mem::zeroed();
        msg.add_raw(head.as_ref());
        msg.add_string(route::link_attr::IfName, "veth0");
        {
            let mut attr = msg.add_nested(route::link_attr::LinkInfo).unwrap();
            attr.add_string(route::link_info::Kind, "veth");
            {
                let mut attr = attr.add_nested(route::link_info::Data).unwrap();
                {
                    let mut attr = attr.add_nested(route::veth_info::Peer).unwrap();
                    attr.add_raw(head.as_ref());
                    attr.add_string(route::link_attr::IfName, "veth1");
                }
            }
        }
    }

    let socket = Socket::netlink(proto::Route, SOCK_NONE).unwrap();
    socket.send(buf.as_ref(), MSG_NONE).unwrap();

    let mut buf = [0u32; 1024];
    let buf = buf.as_mut();
    let len = socket.recv(buf, MSG_NONE).unwrap();
    for (head, data) in MsgIter::new(&buf[..len], None) {
        let mut parser = MsgParser::new(data).unwrap();
        println!("{:?}, {:?}", head, parser.peek_raw::<MsgError>());
    }
}
