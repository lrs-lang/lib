// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use core::{mem};
use cty::{
    BYTES_PER_SHORT, AF_UNIX, AF_INET, AF_INET6, sa_family_t, c_int, sockaddr_in,
    sockaddr_in6,
};
use base::{error};

pub mod unix;
// pub mod ipv4;
// pub mod ipv6;

pub struct SockAddr { data: [u8] }

#[derive(Eq)]
pub enum AddrType {
    Unix,
    Ipv4,
    Ipv6,
}

pub fn maybe_addr_type(bytes: &[u8]) -> Result<AddrType> {
    if bytes.len() < BYTES_PER_SHORT { return Err(error::InvalidArgument); }
    let mut ty: sa_family_t = 0;
    mem::copy(ty.as_mut(), bytes);
    match ty as c_int {
        AF_UNIX  => Ok(AddrType::Unix),
        AF_INET  => Ok(AddrType::Ipv4),
        AF_INET6 => Ok(AddrType::Ipv6),
        _ => Err(error::InvalidArgument),
    }
}

pub fn addr_type(bytes: &[u8]) -> Result<AddrType> {
    match maybe_addr_type(bytes) {
        Ok(AddrType::Unix) if unix::validate(bytes).is_ok() => Ok(AddrType::Unix),
        // Ok(AddrType::Ipv4) if ipv4::validate(bytes).is_ok() => Ok(AddrType::Ipv4),
        // Ok(AddrType::Ipv6) if ipv6::validate(bytes).is_ok() => Ok(AddrType::Ipv6),
        _ => Err(error::InvalidArgument),
    }
}

impl SockAddr {
    pub fn from_bytes(bytes: &[u8]) -> Result<&SockAddr> {
        addr_type(bytes).map(|_| unsafe { mem::cast(bytes) })
    }

    pub fn from_mut_bytes(bytes: &mut [u8]) -> Result<&mut SockAddr> {
        addr_type(bytes).map(|_| unsafe { mem::cast(bytes) })
    }

    pub fn as_bytes     ( &    self ) -> &    [u8] { unsafe { mem::cast(self) } }
    pub fn as_mut_bytes ( &mut self ) -> &mut [u8] { unsafe { mem::cast(self) } }

    pub fn addr_type(&self) -> AddrType {
        maybe_addr_type(&self.data).unwrap()
    }

    pub fn as_unix(&self) -> Result<&unix::UnixSockAddr> {
        match self.addr_type() {
            AddrType::Unix => unix::UnixSockAddr::from_bytes(&self.data),
            _ => Err(error::InvalidArgument),
        }
    }

    pub fn as_mut_unix(&mut self) -> Result<&mut unix::UnixSockAddr> {
        match self.addr_type() {
            AddrType::Unix => unix::UnixSockAddr::from_mut_bytes(&mut self.data),
            _ => Err(error::InvalidArgument),
        }
    }

    pub fn as_ipv4(&self) -> Result<&ipv4::Ipv4SockAddr> {
        match self.addr_type() {
            AddrType::Ipv4 => ipv4::Ipv4SockAddr::from_bytes(&self.data),
            _ => Err(error::InvalidArgument),
        }
    }

    pub fn as_mut_ipv4(&mut self) -> Result<&mut ipv4::Ipv4SockAddr> {
        match self.addr_type() {
            AddrType::Ipv4 => ipv4::Ipv4SockAddr::from_mut_bytes(&mut self.data),
            _ => Err(error::InvalidArgument),
        }
    }

    // pub fn as_ipv6(&self) -> Result<&ipv6::Ipv6SockAddr> {
    //     match self.addr_type() {
    //         AddrType::Ipv6 => ipv6::Ipv6SockAddr::from_bytes(&self.data),
    //         _ => Err(error::InvalidArgument),
    //     }
    // }

    // pub fn as_mut_ipv6(&mut self) -> Result<&mut ipv6::Ipv6SockAddr> {
    //     match self.addr_type() {
    //         AddrType::Ipv6 => ipv6::Ipv6SockAddr::from_mut_bytes(&mut self.data),
    //         _ => Err(error::InvalidArgument),
    //     }
    // }
}
