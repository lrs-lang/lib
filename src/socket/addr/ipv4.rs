// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use core::{mem};
use base::{error};
use cty::{
    AF_INET, sa_family_t, c_int, sockaddr_in, in_addr,
};
use addr::{SockAddr};
use fmt::{Debug, Write};

pub const IPV4_SOCK_ADDR_SIZE: usize = 16; // == size_of::<sockaddr_in>()
                                           // XXX: On some platforms there might be
                                           // unnatural padding between the fields. We
                                           // should have a static assert that this size
                                           // is actually correct.

// Offsets of the port and address. Same XXX applies.
const PORT_OFF: usize = 2;
const ADDR_OFF: usize = 4;

pub fn validate(bytes: &[u8]) -> Result<usize> {
    if bytes.len() < IPV4_SOCK_ADDR_SIZE {
        return Err(error::InvalidArgument);
    }
    let mut family: sa_family_t = 0;
    mem::copy(family.as_mut(), bytes);
    match family as c_int {
        AF_INET => Ok(IPV4_SOCK_ADDR_SIZE),
        _ => Err(error::InvalidArgument),
    }
}

#[derive(Pod, Eq)]
pub struct Ipv4Addr(pub u8, pub u8, pub u8, pub u8);

impl Ipv4Addr {
    pub fn from_bytes(bytes: [u8; 4]) -> Ipv4Addr {
        Ipv4Addr(bytes[0], bytes[1], bytes[2], bytes[3])
    }

    pub fn from_be(addr: u32) -> Ipv4Addr {
        unsafe { Ipv4Addr::from_bytes(mem::cast(addr)) }
    }

    pub fn to_bytes(self) -> [u8; 4] {
        [self.0, self.1, self.2, self.3]
    }

    pub fn to_be(self) -> u32 {
        unsafe { mem::cast(self.to_bytes()) }
    }

    /// `0.0.0.0/8`
    pub fn is_current(self) -> bool {
        self.0 == 0
    }

    /// `10.0.0.0/8` or `172.16.0.0/12` or `192.168.0.0/16`
    pub fn is_private(self) -> bool {
        (self.0 == 10) || (self.0 == 172 && self.1 & 0b1111_0000 == 16) ||
            (self.0 == 192 && self.1 == 168)
    }

    /// `100.64.0.0/10`
    pub fn is_shared(self) -> bool {
        self.0 == 100 && self.1 & 0b1100_0000 == 64
    }

    /// `127.0.0.0/8`
    pub fn is_loopback(self) -> bool {
        self.0 == 127
    }

    /// `169.254.0.0/16`
    pub fn is_link_local(self) -> bool {
        self.0 == 169 && self.1 == 254
    }

    /// `192.88.99.0/24`
    pub fn is_6to4(self) -> bool {
        self.0 == 192 && self.1 == 88 && self.2 == 99
    }

    /// `224.0.0.0/4`
    pub fn is_multicast(self) -> bool {
        self.0 & 0b1111_0000 == 224
    }

    /// `255.255.255.255`
    pub fn broadcast() -> Ipv4Addr {
        Ipv4Addr(255, 255, 255, 255)
    }

    /// `0.0.0.0`
    pub fn any() -> Ipv4Addr {
        Ipv4Addr(0, 0, 0, 0)
    }
}

pub struct Ipv4SockAddr { data: [u8] }

impl Ipv4SockAddr {
    pub fn from_bytes(bytes: &[u8]) -> Result<&Ipv4SockAddr> {
        validate(bytes).map(|l| unsafe { mem::cast(&bytes[..l]) })
    }

    pub fn from_mut_bytes(bytes: &mut [u8]) -> Result<&mut Ipv4SockAddr> {
        validate(bytes).map(|l| unsafe { mem::cast(&mut bytes[..l]) })
    }

    pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &Ipv4SockAddr {
        mem::cast(bytes)
    }

    pub unsafe fn from_mut_bytes_unchecked(bytes: &mut [u8]) -> &mut Ipv4SockAddr {
        mem::cast(bytes)
    }

    pub fn from_addr_port(bytes: &mut [u8], addr: Ipv4Addr,
                          port: u16) -> Result<&mut Ipv4SockAddr> {
        if bytes.len() < IPV4_SOCK_ADDR_SIZE {
            return Err(error::NoMemory);
        }
        let addr = sockaddr_in {
            sin_family: AF_INET as sa_family_t,
            sin_port: port.to_be(),
            sin_addr: in_addr { s_addr: addr.to_be() },
            .. mem::zeroed()
        };
        mem::copy(bytes, mem::as_bytes(&addr));
        Ok(unsafe { mem::cast(&mut bytes[..IPV4_SOCK_ADDR_SIZE]) })
    }

    pub fn addr(&self) -> Ipv4Addr {
        let mut addr = 0;
        mem::copy(addr.as_mut(), &self.data[ADDR_OFF..]);
        Ipv4Addr::from_be(addr)
    }

    pub fn set_addr(&mut self, addr: Ipv4Addr) {
        let addr = addr.to_be();
        mem::copy(&mut self.data[ADDR_OFF..], addr.as_ref());
    }

    pub fn port(&self) -> u16 {
        let mut port: u16 = 0;
        mem::copy(port.as_mut(), &self.data[PORT_OFF..]);
        port.from_be()
    }

    pub fn set_port(&mut self, port: u16) {
        mem::copy(&mut self.data[PORT_OFF..], port.to_be().as_ref());
    }
}

impl AsRef<[u8]> for Ipv4SockAddr {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl AsRef<SockAddr> for Ipv4SockAddr {
    fn as_ref(&self) -> &SockAddr {
        unsafe { mem::cast(self) }
    }
}

impl AsMut<SockAddr> for Ipv4SockAddr {
    fn as_mut(&mut self) -> &mut SockAddr {
        unsafe { mem::cast(self) }
    }
}

impl Debug for Ipv4Addr {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        write!(w, "{}.{}.{}.{}", self.0, self.1, self.2, self.3)
    }
}

impl Debug for Ipv4SockAddr {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        let addr = self.addr();
        let port = self.port();
        write!(w, "Ipv4SockAddr {{ {:?}:{} }}", addr, port)
    }
}
