// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::{mem};
use cty::{
    BYTES_PER_SHORT, AF_UNIX, AF_INET, AF_INET6, sa_family_t, c_int,
};
use base::{error};
use fmt::{Debug, Write};

pub mod unix;
pub mod ipv4;
pub mod ipv6;

/// A socket address.
pub struct SockAddr { data: [u8] }

/// An address type.
#[derive(Eq)]
pub enum AddrType {
    /// A Unix socket address.
    Unix,
    /// An Ipv4 socket address.
    Ipv4,
    /// An Ipv6 socket address.
    Ipv6,
}

pub fn type_supported(bytes: &[u8]) -> bool {
    if bytes.len() < BYTES_PER_SHORT { return false; }
    let mut ty: sa_family_t = 0;
    mem::copy(ty.as_mut(), bytes);
    match ty as c_int {
        AF_UNIX | AF_INET | AF_INET6 => true,
        _ => false,
    }
}

fn addr_type(bytes: &[u8]) -> Result<AddrType> {
    if bytes.len() < BYTES_PER_SHORT { return Err(error::InvalidArgument); }
    let mut ty: sa_family_t = 0;
    mem::copy(ty.as_mut(), bytes);
    match ty as c_int {
        AF_UNIX  if unix::validate(bytes).is_ok() => Ok(AddrType::Unix),
        AF_INET  if ipv4::validate(bytes).is_ok() => Ok(AddrType::Ipv4),
        AF_INET6 if ipv6::validate(bytes).is_ok() => Ok(AddrType::Ipv6),
        _ => Err(error::InvalidArgument),
    }
}

impl SockAddr {
    /// Creates a socket address from a sequence of bytes.
    ///
    /// [argument, bytes]
    /// The buffer containing the address.
    ///
    /// = Remarks
    ///
    /// This fails if the socket address contains invalid data or an address that has
    /// not been wrapped.
    pub fn from_bytes(bytes: &[u8]) -> Result<&SockAddr> {
        addr_type(bytes).map(|_| unsafe { mem::cast(bytes) })
    }

    /// Creates a mutable socket address from a sequence of bytes.
    ///
    /// [argument, bytes]
    /// The buffer containing the address.
    ///
    /// = Remarks
    ///
    /// This fails if the socket address contains invalid data or an address that has
    /// not been wrapped.
    pub fn from_mut_bytes(bytes: &mut [u8]) -> Result<&mut SockAddr> {
        addr_type(bytes).map(|_| unsafe { mem::cast(bytes) })
    }

    /// Creates a socket address from a sequence of bytes without validation.
    ///
    /// [argument, bytes]
    /// The buffer containing the address.
    ///
    /// = Remarks
    ///
    /// If `bytes` does not contain a valid and wrapped socket address, the behavior is
    /// undefined.
    pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &SockAddr {
        mem::cast(bytes)
    }

    /// Creates a mutable socket address from a sequence of bytes without validation.
    ///
    /// [argument, bytes]
    /// The buffer containing the address.
    ///
    /// = Remarks
    ///
    /// If `bytes` does not contain a valid and wrapped socket address, the behavior is
    /// undefined.
    pub unsafe fn from_mut_bytes_unchecked(bytes: &mut [u8]) -> &mut SockAddr {
        mem::cast(bytes)
    }

    /// Returns the address type of the socket address.
    pub fn addr_type(&self) -> AddrType {
        let mut ty: sa_family_t = 0;
        mem::copy(ty.as_mut(), &self.data);
        match ty as c_int {
            AF_UNIX => AddrType::Unix,
            AF_INET => AddrType::Ipv4,
            _       => AddrType::Ipv6,
        }
    }

    /// Returns the socket address as a Unix socket address.
    ///
    /// = Remarks
    ///
    /// This fails if the address is not a Unix socket address.
    pub fn as_unix(&self) -> Result<&unix::UnixSockAddr> {
        match self.addr_type() {
            AddrType::Unix => unsafe {
                Ok(unix::UnixSockAddr::from_bytes_unchecked(&self.data))
            },
            _ => Err(error::InvalidArgument),
        }
    }

    /// Returns the socket address as a mutable Unix socket address.
    ///
    /// = Remarks
    ///
    /// This fails if the address is not a Unix socket address.
    pub fn as_mut_unix(&mut self) -> Result<&mut unix::UnixSockAddr> {
        match self.addr_type() {
            AddrType::Unix => unsafe {
                Ok(unix::UnixSockAddr::from_mut_bytes_unchecked(&mut self.data))
            },
            _ => Err(error::InvalidArgument),
        }
    }

    /// Returns the socket address as an Ipv4 socket address.
    ///
    /// = Remarks
    ///
    /// This fails if the address is not an Ipv4 socket address.
    pub fn as_ipv4(&self) -> Result<&ipv4::Ipv4SockAddr> {
        match self.addr_type() {
            AddrType::Ipv4 => unsafe {
                Ok(ipv4::Ipv4SockAddr::from_bytes_unchecked(&self.data))
            },
            _ => Err(error::InvalidArgument),
        }
    }

    /// Returns the socket address as a mutable Ipv4 socket address.
    ///
    /// = Remarks
    ///
    /// This fails if the address is not an Ipv4 socket address.
    pub fn as_mut_ipv4(&mut self) -> Result<&mut ipv4::Ipv4SockAddr> {
        match self.addr_type() {
            AddrType::Ipv4 => unsafe {
                Ok(ipv4::Ipv4SockAddr::from_mut_bytes_unchecked(&mut self.data))
            },
            _ => Err(error::InvalidArgument),
        }
    }

    /// Returns the socket address as an Ipv6 socket address.
    ///
    /// = Remarks
    ///
    /// This fails if the address is not an Ipv6 socket address.
    pub fn as_ipv6(&self) -> Result<&ipv6::Ipv6SockAddr> {
        match self.addr_type() {
            AddrType::Ipv6 => unsafe {
                Ok(ipv6::Ipv6SockAddr::from_bytes_unchecked(&self.data))
            },
            _ => Err(error::InvalidArgument),
        }
    }

    /// Returns the socket address as a mutable Ipv6 socket address.
    ///
    /// = Remarks
    ///
    /// This fails if the address is not an Ipv6 socket address.
    pub fn as_mut_ipv6(&mut self) -> Result<&mut ipv6::Ipv6SockAddr> {
        match self.addr_type() {
            AddrType::Ipv6 => unsafe {
                Ok(ipv6::Ipv6SockAddr::from_mut_bytes_unchecked(&mut self.data))
            },
            _ => Err(error::InvalidArgument),
        }
    }
}

impl AsRef<[u8]> for SockAddr {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

impl Debug for SockAddr {
    fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
        match self.addr_type() {
            AddrType::Unix => self.as_unix().unwrap().fmt(w),
            AddrType::Ipv4 => self.as_ipv4().unwrap().fmt(w),
            AddrType::Ipv6 => self.as_ipv6().unwrap().fmt(w),
        }
    }
}
