// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_socket"]
#![crate_type = "lib"]
#![feature(plugin, no_std, custom_derive)]
#![plugin(lrs_core_plugin)]
#![no_std]

#[macro_use]
extern crate lrs_core     as core;
extern crate lrs_base     as base;
extern crate lrs_cty      as cty;
extern crate lrs_arch_fns as arch_fns;
extern crate lrs_str_one  as str_one;
extern crate lrs_fmt      as fmt;
extern crate lrs_rv       as rv;
extern crate lrs_fd       as fd;
extern crate lrs_syscall  as syscall;

#[prelude_import] use base::prelude::*;
use base::{error};
use cty::{
    AF_UNIX, AF_INET, AF_INET6, c_int, AF_UNSPEC, sa_family_t,
    SOCK_STREAM, SOCK_DGRAM, SOCK_SEQPACKET, SOCK_RAW, SOCK_RDM, SHUT_RD, SHUT_WR,
    SHUT_RDWR,
};
use syscall::{
    socket, bind, getsockname, getpeername, connect, close, shutdown, listen, sendto,
};
use fd::{FDContainer};


pub use addr::{SockAddr, AddrType};
pub use addr::unix::{UnixSockAddr, UnixAddrType};
pub use addr::ipv4::{Ipv4Addr, Ipv4SockAddr, IPV4_SOCK_ADDR_SIZE};
pub use addr::ipv6::{Ipv6Addr, Ipv6SockAddr, IPV6_SOCK_ADDR_SIZE, Ipv6Scope};

mod lrs { pub use fmt::lrs::*; pub use cty; }

mod addr;

#[derive(Copy, Eq)]
pub enum Type {
    Stream,
    Datagram,
    SeqPacket,
    Raw,
    Rdm,
}

impl Type {
    fn to_int(self) -> c_int {
        match self {
            Type::Stream => SOCK_STREAM,
            Type::Datagram => SOCK_DGRAM,
            Type::SeqPacket => SOCK_SEQPACKET,
            Type::Raw => SOCK_RAW,
            Type::Rdm => SOCK_RDM,
        }
    }
}

pub struct Socket {
    fd: c_int,
    owned: bool,
}

impl Socket {
    pub fn new(domain: c_int, kind: c_int, protocol: c_int) -> Result<Socket> {
        let fd = try!(rv!(socket(domain, kind, protocol), -> c_int));
        Ok(Socket { fd: fd, owned: true })
    }

    pub fn new_unix(kind: Type) -> Result<Socket> {
        Socket::new(AF_UNIX, kind.to_int(), 0)
    }

    pub fn new_ipv4(kind: Type) -> Result<Socket> {
        Socket::new(AF_INET, kind.to_int(), 0)
    }

    pub fn new_ipv4_raw(protocol: c_int) -> Result<Socket> {
        Socket::new(AF_INET, SOCK_RAW, protocol)
    }

    pub fn new_ipv6(kind: Type) -> Result<Socket> {
        Socket::new(AF_INET6, kind.to_int(), 0)
    }

    pub fn new_ipv6_raw(protocol: c_int) -> Result<Socket> {
        Socket::new(AF_INET6, SOCK_RAW, protocol)
    }

    pub fn bind<A>(&self, addr: A) -> Result
        where A: AsRef<SockAddr>,
    {
        rv!(bind(self.fd, addr.as_ref().as_ref()))
    }

    pub fn get_addr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut SockAddr> {
        let mut len = 0;
        try!(rv!(getsockname(self.fd, buf, &mut len)));
        if addr::type_supported(buf) {
            Ok(unsafe { SockAddr::from_mut_bytes_unchecked(&mut buf[..len]) })
        } else {
            Err(error::NotSupported)
        }
    }

    pub fn get_peer_addr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut SockAddr> {
        let mut len = 0;
        try!(rv!(getpeername(self.fd, buf, &mut len)));
        if addr::type_supported(buf) {
            Ok(unsafe { SockAddr::from_mut_bytes_unchecked(&mut buf[..len]) })
        } else {
            Err(error::NotSupported)
        }
    }

    pub fn connect<A>(&self, addr: A) -> Result
        where A: AsRef<SockAddr>,
    {
        rv!(connect(self.fd, addr.as_ref().as_ref()))
    }

    pub fn disconnect(&self) -> Result {
        rv!(connect(self.fd, (AF_UNSPEC as sa_family_t).as_ref()))
    }

    pub fn shutdown(&self) -> Result {
        rv!(shutdown(self.fd, SHUT_RDWR))
    }

    pub fn shutdown_incoming(&self) -> Result {
        rv!(shutdown(self.fd, SHUT_RD))
    }

    pub fn shutdown_outgoing(&self) -> Result {
        rv!(shutdown(self.fd, SHUT_WR))
    }

    pub fn listen(&self, backlog: u32) -> Result {
        rv!(listen(self.fd, backlog))
    }

    pub fn send(&self, buf: &[u8], flags: MsgFlags) -> Result<usize> {
        rv!(sendto(self.fd, buf, flags.to_int(), None), -> usize)
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        if self.owned {
            close(self.fd);
        }
    }
}

impl FDContainer for Socket {
    fn unwrap(self) -> c_int { self.fd }
    fn is_owned(&self) -> bool { self.owned }
    fn borrow(&self) -> c_int { self.fd }
    fn from_owned(fd: c_int) -> Socket { Socket { fd: fd, owned: true } }
    fn from_borrowed(fd: c_int) -> Socket { Socket { fd: fd, owned: false } }
}

#[derive(Pod, Eq)]
pub struct MsgFlags {
    flags: c_int,
}

impl MsgFlags {
    fn to_int(self) -> c_int {
        self.flags | cty::MSG_NOSIGNAL
    }
}

pub const MSG_CONFIRM:       MsgFlags = MsgFlags { flags: cty::MSG_CONFIRM   };
pub const MSG_DONT_ROUTE:    MsgFlags = MsgFlags { flags: cty::MSG_DONTROUTE };
pub const MSG_DONT_BLOCK:    MsgFlags = MsgFlags { flags: cty::MSG_DONTWAIT  };
pub const MSG_END_OF_RECORD: MsgFlags = MsgFlags { flags: cty::MSG_EOR       };
pub const MSG_MORE:          MsgFlags = MsgFlags { flags: cty::MSG_MORE      };
pub const MSG_OUT_OF_BAND:   MsgFlags = MsgFlags { flags: cty::MSG_OOB       };
