// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use core::{mem};
use cty::{
    AF_INET6, sa_family_t, c_int, sockaddr_in6, in6_addr,
};
use base::{error};
use addr::{SockAddr};
use fmt::{Debug, Write};

/// The size of an Ipv6 socket address in bytes.
pub const IPV6_SOCK_ADDR_SIZE: usize = 28; // == size_of::<sockaddr_in6>().
                                           // XXX: See the comment in addr::ipv4

// Offsets of the port, address, etc. Same XXX applies.
const PORT_OFF: usize = 2;
const FLOW_OFF: usize = 4;
const ADDR_OFF: usize = 8;
const SCOP_OFF: usize = 24;

pub fn validate(bytes: &[u8]) -> Result<usize> {
    if bytes.len() < IPV6_SOCK_ADDR_SIZE {
        return Err(error::InvalidArgument);
    }
    let mut family: sa_family_t = 0;
    mem::copy(family.as_mut(), bytes);
    match family as c_int {
        AF_INET6 => Ok(IPV6_SOCK_ADDR_SIZE),
        _ => Err(error::InvalidArgument),
    }
}

/// An Ipv6 address.
///
/// = Remarks
///
/// The individual segments should be stored in host byte order. That is, `::1` is stored
/// as
///
/// ----
/// Ipv6Addr(0, 0, 0, 0, 0, 0, 0, 1)
/// ----
#[derive(Pod, Eq)]
pub struct Ipv6Addr(pub u16, pub u16, pub u16, pub u16,
                    pub u16, pub u16, pub u16, pub u16);

impl Ipv6Addr {
    /// Creates an Ipv6 address from segments in network byte order.
    pub fn from_be_bytes(bytes: [u16; 8]) -> Ipv6Addr {
        Ipv6Addr(
           bytes[0].from_be(), bytes[1].from_be(), bytes[2].from_be(), bytes[3].from_be(),
           bytes[4].from_be(), bytes[5].from_be(), bytes[6].from_be(), bytes[7].from_be(),
        )
    }

    /// Creates an Ipv6 address from segments in host byte order.
    pub fn from_bytes(bytes: [u16; 8]) -> Ipv6Addr {
        Ipv6Addr(
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7],
        )
    }

    /// Transforms the address into an array of segments in network byte order.
    pub fn to_be_bytes(&self) -> [u16; 8] {
        [self.0.to_be(), self.1.to_be(), self.2.to_be(), self.3.to_be(),
         self.4.to_be(), self.5.to_be(), self.6.to_be(), self.7.to_be()]
    }

    /// Transforms the address into an array of segments in host byte order.
    pub fn to_bytes(&self) -> [u16; 8] {
        [self.0, self.1, self.2, self.3,
         self.4, self.5, self.6, self.7]
    }

    /// Creates the unspecified address `::`.
    pub fn unspecified() -> Ipv6Addr {
        Ipv6Addr::from_bytes([0, 0, 0, 0, 0, 0, 0, 0])
    }

    /// Creates the loopback address `::1`.
    pub fn loopback() -> Ipv6Addr {
        Ipv6Addr::from_bytes([0, 0, 0, 0, 0, 0, 0, 1])
    }

    /// Compares the address to the link local prefix `fe80::/10`.
    pub fn is_link_local(&self) -> bool {
        self.0 & 0b1111_1111_1100_0000 == 0xfe80
    }

    /// Compares the address to the unique local prefix `fc00::/7`.
    pub fn is_unique_local(&self) -> bool {
        self.0 & 0b1111_1110_0000_0000 == 0xfc00
    }

    /// Compares the address to the prefix `::/96`.
    pub fn is_ipv4_compatible(&self) -> bool {
        self.0 == 0 && self.1 == 0 && self.2 == 0 && self.3 == 0 && self.4 == 0
            && self.5 == 0
    }

    /// Compares the address to the Ipv4-mapped prefix `::ffff:0:0/96`.
    pub fn is_ipv4_mapped(&self) -> bool {
        self.0 == 0 && self.1 == 0 && self.2 == 0 && self.3 == 0 && self.4 == 0
            && self.5 == 0xffff
    }

    /// Compares the address to the Ipv4-translated prefix `::ffff:0:0:0/96`.
    pub fn is_ipv4_translated(&self) -> bool {
        self.0 == 0 && self.1 == 0 && self.2 == 0 && self.3 == 0 && self.4 == 0xffff
            && self.5 == 0
    }

    /// Compares the address to the 6to4 prefix `2002::/16`.
    pub fn is_6to4(&self) -> bool {
        self.0 == 0x2002
    }

    /// Compares the address to the Teredo prefix `2001::/32`.
    pub fn is_teredo(&self) -> bool {
        self.0 == 0x2001 && self.1 == 0
    }

    /// Compares the address to the BWMG prefix `2001:2::/48`.
    pub fn is_bmwg(&self) -> bool {
        self.0 == 0x2001 && self.1 == 2 && self.2 == 0
    }

    /// Compares the address to the ORCHIDv2 prefix `2001:20::/28`.
    pub fn is_orchidv2(&self) -> bool {
        self.0 == 0x2001 && self.1 & 0b1111_1111_1111_0000 == 0x20
    }

    /// Compares the address to the documentation prefix `2001:db8::/32`.
    pub fn is_documentation(&self) -> bool {
        self.0 == 0x2001 && self.1 == 0xdb8
    }

    /// Compares the address to the discard prefix `0100::/64`.
    pub fn is_discard(&self) -> bool {
        self.0 == 0x0100 && self.1 == 0 && self.2 == 0 && self.3 == 0
    }

    /// Compares the address to the multicast prefix `ff00::/8`.
    pub fn is_multicast(&self) -> bool {
        self.0 & 0b1111_1111_0000_0000 == 0xff00
    }

    /// Creates the well-known "All nodes" multicast address with a certain scope.
    ///
    /// [argument, scope]
    /// The scope of the multicast address.
    ///
    /// = Remarks
    ///
    /// The created address is `ff0?::1` where `?` depends on the scope.
    ///
    /// Only the `Interface` and `Link` scopes are valid.
    pub fn multicast(scope: Ipv6Scope) -> Ipv6Addr {
        Ipv6Addr::from_bytes([scope.to_multicast_prefix(), 0, 0, 0, 0, 0, 0, 0x1])
    }

    /// Creates the well-known "All routers" multicast address with a certain scope.
    ///
    /// [argument, scope]
    /// The scope of the multicast address.
    ///
    /// = Remarks
    ///
    /// The created address is `ff0?::2` where `?` depends on the scope.
    ///
    /// Only the `Interface`, `Link`, and `Site` scopes are valid.
    pub fn router_multicast(scope: Ipv6Scope) -> Ipv6Addr {
        Ipv6Addr::from_bytes([scope.to_multicast_prefix(), 0, 0, 0, 0, 0, 0, 0x2])
    }

    /// Creates the well-known "All NTP servers" multicast address with a certain scope.
    ///
    /// [argument, scope]
    /// The scope of the multicast address.
    ///
    /// = Remarks
    ///
    /// The created address is `ff0?::101` where `?` depends on the scope.
    pub fn ntp_multicast(scope: Ipv6Scope) -> Ipv6Addr {
        Ipv6Addr::from_bytes([scope.to_multicast_prefix(), 0, 0, 0, 0, 0, 0, 0x101])
    }

    /// Creates the well-known "All DHCP servers" multicast address.
    ///
    /// = Remarks
    ///
    /// The created address is `ff05::1:3`.
    pub fn dhcp_server_multicast() -> Ipv6Addr {
        Ipv6Addr::from_bytes([0xff05, 0, 0, 0, 0, 0, 0x1, 0x3])
    }
}

/// The scope of an Ipv6 address.
///
/// = Remarks
///
/// This is currently only used for multicast addresses.
#[derive(Copy, Eq)]
pub enum Ipv6Scope {
    /// Interface-local scope.
    Interface,
    /// Link-local scope.
    Link,
    /// Admin-local scope.
    Admin,
    /// Site-local scope.
    Site,
    /// Organization-local scope.
    Organization,
    /// Global scope.
    Global,
}

impl Ipv6Scope {
    fn to_multicast_prefix(self) -> u16 {
        match self {
            Ipv6Scope::Interface    => 0xff01,
            Ipv6Scope::Link         => 0xff02,
            Ipv6Scope::Admin        => 0xff04,
            Ipv6Scope::Site         => 0xff05,
            Ipv6Scope::Organization => 0xff08,
            Ipv6Scope::Global       => 0xff0e,
        }
    }
}

/// An Ipv6 socket address.
pub struct Ipv6SockAddr { data: [u8] }

impl Ipv6SockAddr {
    /// Creates an Ipv6 address from given bytes.
    ///
    /// [argument, bytes]
    /// The bytes that contain the address.
    pub fn from_bytes(bytes: &[u8]) -> Result<&Ipv6SockAddr> {
        validate(bytes).map(|l| unsafe { mem::cast(&bytes[..l]) })
    }

    /// Creates a mutable Ipv6 address from given bytes.
    ///
    /// [argument, bytes]
    /// The bytes that contain the address.
    pub fn from_mut_bytes(bytes: &mut [u8]) -> Result<&mut Ipv6SockAddr> {
        validate(bytes).map(|l| unsafe { mem::cast(&mut bytes[..l]) })
    }

    /// Creates an Ipv6 address from given bytes without validating the contents.
    ///
    /// [argument, bytes]
    /// The bytes that contain the address.
    pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &Ipv6SockAddr {
        mem::cast(bytes)
    }

    /// Creates a mutable Ipv6 address from given bytes without validating the contents.
    ///
    /// [argument, bytes]
    /// The bytes that contain the address.
    pub unsafe fn from_mut_bytes_unchecked(bytes: &mut [u8]) -> &mut Ipv6SockAddr {
        mem::cast(bytes)
    }

    /// Creates a new Ipv6 socket address from an address and a port.
    ///
    /// [argument, bytes]
    /// The buffer in which the address will be stored.
    ///
    /// [argument, addr]
    /// The Ipv6 address of the socket.
    ///
    /// [argument, port]
    /// The port of the socket.
    pub fn from_addr_port(bytes: &mut [u8], addr: Ipv6Addr,
                          port: u16) -> Result<&mut Ipv6SockAddr> {
        if bytes.len() < IPV6_SOCK_ADDR_SIZE {
            return Err(error::NoMemory);
        }
        let addr = sockaddr_in6 {
            sin6_family: AF_INET6 as sa_family_t,
            sin6_port: port,
            sin6_addr: in6_addr { u6_addr16: addr.to_be_bytes() },
            .. mem::zeroed()
        };
        mem::copy(bytes, mem::as_bytes(&addr));
        Ok(unsafe { mem::cast(&mut bytes[..IPV6_SOCK_ADDR_SIZE]) })
    }

    /// Returns the Ipv6 address of an Ipv6 socket address.
    pub fn addr(&self) -> Ipv6Addr {
        let mut addr = [0; 8];
        mem::copy(addr.as_mut(), &self.data[ADDR_OFF..]);
        Ipv6Addr::from_be_bytes(addr)
    }

    /// Sets the Ipv6 address of an Ipv6 socket address.
    ///
    /// [argument, addr]
    /// The new address.
    pub fn set_addr(&mut self, addr: Ipv6Addr) {
        mem::copy(&mut self.data[ADDR_OFF..], addr.to_be_bytes().as_ref());
    }

    /// Returns the port of an Ipv6 socket address.
    pub fn port(&self) -> u16 {
        let mut port = 0;
        mem::copy(port.as_mut(), &self.data[PORT_OFF..]);
        port
    }

    /// Sets the port of an Ipv6 socket address.
    ///
    /// [argument, port]
    /// The new port.
    pub fn set_port(&mut self, port: u16) {
        mem::copy(&mut self.data[PORT_OFF..], port.to_be().as_ref());
    }

    /// Returns the flow label of an Ipv6 socket address.
    pub fn flow_label(&self) -> u32 {
        let mut label: u32 = 0;
        mem::copy(label.as_mut(), &self.data[FLOW_OFF..]);
        label.from_be()
    }

    /// Sets the flow label of an Ipv6 socket address.
    ///
    /// [argument, label]
    /// The new flow label.
    pub fn set_flow_label(&mut self, label: u32) {
        mem::copy(&mut self.data[FLOW_OFF..], label.to_be().as_ref());
    }

    /// Returns the scope id of an Ipv6 socket address.
    pub fn scope_id(&self) -> u32 {
        let mut id: u32 = 0;
        mem::copy(id.as_mut(), &self.data[SCOP_OFF..]);
        id.from_be()
    }

    /// Sets the scope id of an Ipv6 socket address.
    ///
    /// [argument, id]
    /// The new scope id.
    pub fn set_scope_id(&mut self, id: u32) {
        mem::copy(&mut self.data[SCOP_OFF..], id.to_be().as_ref());
    }
}

impl AsRef<[u8]> for Ipv6SockAddr {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl AsRef<SockAddr> for Ipv6SockAddr {
    fn as_ref(&self) -> &SockAddr {
        unsafe { mem::cast(self) }
    }
}

impl AsMut<SockAddr> for Ipv6SockAddr {
    fn as_mut(&mut self) -> &mut SockAddr {
        unsafe { mem::cast(self) }
    }
}

impl Debug for Ipv6Addr {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        let is_ipv4 = if self.is_ipv4_compatible() {
            try!(write!(w, "::"));
            true
        } else if self.is_ipv4_mapped() {
            try!(write!(w, "::ffff:"));
            true
        } else if self.is_ipv4_translated() {
            try!(write!(w, "::ffff:0:"));
            true
        } else {
            false
        };

        if is_ipv4 {
            let ab: [u8; 2] = unsafe { mem::cast(self.6.to_be()) };
            let cd: [u8; 2] = unsafe { mem::cast(self.6.to_be()) };
            return write!(w, "{}.{}.{}.{}", ab[0], ab[1], cd[0], cd[1]);
        }

        let bytes = self.to_bytes();

        // Find the longest sequence of zeros, favoring sequences at the end and start.
        let (start, end) = {
            let mut start = 0;
            let mut end = 0;
            let mut cur = 8;
            for i in 0..8usize {
                match bytes[i] {
                    0 if cur == 8 => cur = i,
                    0 => { },
                    _ => {
                        if cur != 8 && i - cur > end - start {
                            start = cur;
                            end = i;
                        }
                        cur = 8;
                    },
                }
            }
            if (8 - cur > end - start) || ((8 - cur == end - start) && start > 0) {
                start = cur;
                end = 8;
            }
            if end == start + 1 {
                (0, 0)
            } else {
                (start, end)
            }
        };

        if start == end {
            for i in 0..7usize {
                try!(write!(w, "{:x}:", bytes[i]))
            }
            try!(write!(w, "{:x}", bytes[7]))
        } else {
            if start == 0 {
                try!(write!(w, ":"));
            } else {
                for i in 0..start {
                    try!(write!(w, "{:x}:", bytes[i]))
                }
            }
            if end == 8 {
                try!(write!(w, ":"));
            } else {
                for i in end..8 {
                    try!(write!(w, ":{:x}", bytes[i]))
                }
            }
        }
        Ok(())
    }
}

impl Debug for Ipv6SockAddr {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        let addr = self.addr();
        let port = self.port();
        let flow = self.flow_label();
        let scope = self.scope_id();
        write!(w, "Ipv6SockAddr {{ addr: {:?}, port: {}, flow_label: {}, scope_id: {} }}",
               addr, port, flow, scope)
    }
}
