// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use base::{error};
use cty::{
    msghdr, c_void, iovec, c_int, AF_UNSPEC, sa_family_t, SHUT_RD, SHUT_WR, SHUT_RDWR,
    SOL_SOCKET, SO_ACCEPTCONN, SO_BINDTODEVICE, IFNAMSIZ, SO_BROADCAST, SO_DEBUG,
    SO_DOMAIN, MSG_CMSG_CLOEXEC, MSG_NOSIGNAL, SOCK_CLOEXEC,
};
use core::{num, slice};
use syscall::{
    socket, bind, getsockname, getpeername, connect, close, shutdown, listen, sendto,
    sendmsg, recvfrom, recvmsg, getsockopt, setsockopt,
};
use str_one::{ToCStr, CStr, AsMutCStr};
use fd::{FDContainer};
use rv::{retry};
use saturating::{SaturatingCast};

use addr::{self, SockAddr};
use cmsg::{CMsgIter};

use ip_proto::{self};
use domain::{self, Domain};
use kind::{self, Kind};
use msg::{self};
use flags::{self};

pub struct Socket {
    fd: c_int,
    owned: bool,
}

impl Socket {
    /// Creates a new socket.
    ///
    /// This is the most general constructor. There are simpler, more specialized
    /// constructors for the common cases of Unix, Ipv4, and Ipv6 sockets.
    pub fn new(domain: Domain, kind: Kind, protocol: c_int,
               flags: Option<flags::Flags>) -> Result<Socket> {
        let flags = flags.map(|f| f.0).unwrap_or(0);
        let ty = kind.0 | SOCK_CLOEXEC | flags;
        let fd = try!(rv!(socket(domain.0, ty, protocol), -> c_int));
        Ok(Socket { fd: fd, owned: true })
    }

    /// `Socket::new(domain::Unix, kind::Stream, 0, flags)`
    pub fn unix_stream(flags: Option<flags::Flags>) -> Result<Socket> {
        Socket::new(domain::Unix, kind::Stream, 0, flags)
    }

    /// `Socket::new(domain::Unix, kind::Datagram, 0, flags)`
    pub fn unix_datagram(flags: Option<flags::Flags>) -> Result<Socket> {
        Socket::new(domain::Unix, kind::Datagram, 0, flags)
    }

    /// `Socket::new(domain::Unix, kind::SeqPacket, 0, flags)`
    pub fn unix_seqpacket(flags: Option<flags::Flags>) -> Result<Socket> {
        Socket::new(domain::Unix, kind::SeqPacket, 0, flags)
    }

    /// `Socket::new(domain::Ipv4, kind::Stream, 0, flags)`
    pub fn ipv4_stream(flags: Option<flags::Flags>) -> Result<Socket> {
        Socket::new(domain::Ipv4, kind::Stream, 0, flags)
    }

    /// `Socket::new(domain::Ipv4, kind::Datagram, 0, flags)`
    pub fn ipv4_datagram(flags: Option<flags::Flags>) -> Result<Socket> {
        Socket::new(domain::Ipv4, kind::Datagram, 0, flags)
    }

    /// `Socket::new(domain::Ipv4, kind::Raw, proto.0 as c_int, flags)`
    pub fn ipv4_raw(proto: ip_proto::Proto,
                    flags: Option<flags::Flags>) -> Result<Socket> {
        Socket::new(domain::Ipv4, kind::Raw, proto.0 as c_int, flags)
    }

    /// `Socket::new(domain::Ipv6, kind::Stream, 0, flags)`
    pub fn ipv6_stream(flags: Option<flags::Flags>) -> Result<Socket> {
        Socket::new(domain::Ipv6, kind::Stream, 0, flags)
    }

    /// `Socket::new(domain::Ipv6, kind::Datagram, 0, flags)`
    pub fn ipv6_datagram(flags: Option<flags::Flags>) -> Result<Socket> {
        Socket::new(domain::Ipv6, kind::Datagram, 0, flags)
    }

    /// `Socket::new(domain::Ipv6, kind::Raw, proto.0 as c_int, flags)`
    pub fn ipv6_raw(proto: ip_proto::Proto,
                    flags: Option<flags::Flags>) -> Result<Socket> {
        Socket::new(domain::Ipv6, kind::Raw, proto.0 as c_int, flags)
    }

    /// Binds the socket to an address.
    pub fn bind<A>(&self, addr: A) -> Result
        where A: AsRef<[u8]>,
    {
        rv!(bind(self.fd, addr.as_ref().as_ref()))
    }

    /// Returns the address the socket is bound to.
    pub fn get_addr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut SockAddr> {
        let mut len = 0;
        try!(rv!(getsockname(self.fd, buf, &mut len)));
        if addr::type_supported(buf) {
            Ok(unsafe { SockAddr::from_mut_bytes_unchecked(&mut buf[..len]) })
        } else {
            Err(error::NotSupported)
        }
    }

    /// Returns the address of the peer this socket is connected to if any.
    pub fn get_peer_addr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut SockAddr> {
        let mut len = 0;
        try!(rv!(getpeername(self.fd, buf, &mut len)));
        if addr::type_supported(buf) {
            Ok(unsafe { SockAddr::from_mut_bytes_unchecked(&mut buf[..len]) })
        } else {
            Err(error::NotSupported)
        }
    }

    /// Connects this socket to a peer.
    ///
    /// For connection-less sockets, this only sets the default address for packets
    /// where the destination has not been specified.
    pub fn connect<A>(&self, addr: A) -> Result
        where A: AsRef<[u8]>,
    {
        rv!(connect(self.fd, addr.as_ref()))
    }

    /// Disconnects a connected, connection-less socket.
    ///
    /// Note that this does not work for connection-based sockets.
    pub fn disconnect(&self) -> Result {
        rv!(connect(self.fd, (AF_UNSPEC as sa_family_t).as_ref()))
    }

    /// Shuts down the receiving and sending ends of a connected socket.
    pub fn shutdown(&self) -> Result {
        rv!(shutdown(self.fd, SHUT_RDWR))
    }

    /// Shuts down the receiving end of a connected socket.
    pub fn shutdown_incoming(&self) -> Result {
        rv!(shutdown(self.fd, SHUT_RD))
    }

    /// Shuts down the sending end of a socket.
    pub fn shutdown_outgoing(&self) -> Result {
        rv!(shutdown(self.fd, SHUT_WR))
    }

    /// Marks this socket as accepting incoming connections.
    ///
    /// The backlock determins the maximum number of pending requests. Once this buffer is
    /// full, new requests will be automatically rejected.
    pub fn listen(&self, backlog: u32) -> Result {
        rv!(listen(self.fd, backlog))
    }

    /// Sends a message on a connected socket.
    ///
    /// Returns the number of bytes sent.
    pub fn send(&self, buf: &[u8], flags: msg::Flags) -> Result<usize> {
        let flags = flags.0 | MSG_NOSIGNAL;
        retry(|| sendto(self.fd, buf, flags, None)).map(|v| v as usize)
    }

    /// Sends a message on a connection-less socket.
    ///
    /// Returns the number of bytes sent.
    pub fn send_to<A>(&self, buf: &[u8], addr: A, flags: msg::Flags) -> Result<usize>
        where A: AsRef<[u8]>,
    {
        let flags = flags.0 | MSG_NOSIGNAL;
        retry(|| sendto(self.fd, buf, flags, Some(addr.as_ref()))).map(|v| v as usize)
    }

    /// Like `send` but gathers its data from multiple buffers.
    ///
    /// Returns the number of bytes sent.
    pub fn gather_send(&self, buf: &[&[u8]], flags: msg::Flags) -> Result<usize> {
        let addr: &[u8] = &[];
        let ctrl: &[u8] = &[];
        self.send_ctrl_to(buf, addr, ctrl, flags)
    }

    /// Like `gather_send` but also allows to specify the address of the destination.
    ///
    /// Returns the number of bytes sent.
    pub fn gather_send_to<A>(&self, buf: &[&[u8]], addr: A,
                             flags: msg::Flags) -> Result<usize>
        where A: AsRef<[u8]>,
    {
        let addr = addr.as_ref();
        let ctrl: &[u8] = &[];
        self.send_ctrl_to(buf, addr, ctrl, flags)
    }

    /// Like `gather_send` but also allows to send control messages.
    ///
    /// Returns the number of bytes sent.
    pub fn send_ctrl<C>(&self, buf: &[&[u8]], ctrl: C, flags: msg::Flags) -> Result<usize>
        where C: AsRef<[u8]>,
    {
        let addr: &[u8] = &[];
        let ctrl = ctrl.as_ref();
        self.send_ctrl_to(buf, addr, ctrl, flags)
    }

    /// Like `send_ctrl` but also allows to specify the address of the destination.
    ///
    /// Returns the number of bytes sent.
    pub fn send_ctrl_to<A, C>(&self, buf: &[&[u8]], addr: A, ctrl: C,
                              flags: msg::Flags) -> Result<usize>
        where A: AsRef<[u8]>,
              C: AsRef<[u8]>,
    {
        let addr = addr.as_ref();
        let (addr_ptr, addr_len) = match addr.len() {
            0 => (0 as *const u8, 0),
            n => (addr.as_ptr(), n),
        };
        let ctrl = ctrl.as_ref();
        let (ctrl_ptr, ctrl_len) = match ctrl.len() {
            0 => (0 as *const u8, 0),
            n => (ctrl.as_ptr(), n),
        };
        let msg = msghdr {
            msg_name:       addr_ptr as *mut c_void,
            msg_namelen:    addr_len.saturating_cast(),
            msg_iov:        buf.as_ptr() as *mut iovec,
            msg_iovlen:     buf.len().saturating_cast(),
            msg_control:    ctrl_ptr as *mut c_void,
            msg_controllen: ctrl_len.saturating_cast(),
            msg_flags:      0,
        };
        let flags = flags.0 | MSG_NOSIGNAL;
        retry(|| sendmsg(self.fd, &msg, flags)).map(|v| v as usize)
    }

    pub fn recv<'a>(&self, buf: &'a mut [u8], flags: msg::Flags) -> Result<usize> {
        let mut addr_len = 0;
        retry(|| recvfrom(self.fd, buf, flags.0, None,
                          &mut addr_len)).map(|v| v as usize)
    }

    pub fn recv_from<'a>(&self, buf: &mut [u8], addr_buf: &'a mut [u8],
                         flags: msg::Flags) -> Result<(usize, Option<&'a mut SockAddr>)> {
        let mut addr_len = 0;
        let buf_len = try!(retry(|| recvfrom(self.fd, buf, flags.0, Some(addr_buf),
                                             &mut addr_len)).map(|v| v as usize));
        let addr_buf = &mut addr_buf[..addr_len];
        let addr = match addr::type_supported(addr_buf) {
            true => unsafe { Some(SockAddr::from_mut_bytes_unchecked(&mut addr_buf[..])) },
            _ => None,
        };
        Ok((buf_len, addr))
    }

    pub fn scatter_recv(&self, buf: &mut [&mut [u8]], flags: msg::Flags) -> Result<usize> {
        let mut msg = msghdr {
            msg_name:       0 as *mut c_void,
            msg_namelen:    0,
            msg_iov:        buf.as_mut_ptr() as *mut iovec,
            msg_iovlen:     buf.len().saturating_cast(),
            msg_control:    0 as *mut c_void,
            msg_controllen: 0,
            msg_flags:      0,
        };
        retry(|| recvmsg(self.fd, &mut msg, flags.0)).map(|v| v as usize)
    }

    pub fn recv_msg<'a, 'b>(
        &self,
        buf: &mut [&mut [u8]],
        addr: &'a mut [u8],
        ctrl: &'b mut [u8],
        flags: msg::Flags,
        ) -> Result<(usize, Option<&'a mut SockAddr>, CMsgIter<'b>, msg::Flags)>
    {
        let (addr_ptr, addr_len) = match addr.len() {
            0 => (0 as *mut u8, 0),
            n => (addr.as_mut_ptr(), n),
        };

        // Align the ctrl_ptr.
        let (mut ctrl_ptr, mut ctrl_len) = match ctrl.len() {
            0 => (0 as *mut u8, 0),
            n => (ctrl.as_mut_ptr(), n),
        };
        const PTR_MASK: usize = num::usize::BYTES - 1;
        let pad_ctrl_ptr = (ctrl_ptr as usize + PTR_MASK) & !PTR_MASK;
        if ctrl_len >= pad_ctrl_ptr - ctrl_ptr as usize {
            ctrl_ptr = pad_ctrl_ptr as *mut u8;
            ctrl_len -= pad_ctrl_ptr - ctrl_ptr as usize;
        } else {
            ctrl_len = 0;
        }

        let mut msg = msghdr {
            msg_name:       addr_ptr as *mut c_void,
            msg_namelen:    addr_len.saturating_cast(),
            msg_iov:        buf.as_ptr() as *mut iovec,
            msg_iovlen:     buf.len().saturating_cast(),
            msg_control:    ctrl_ptr as *mut c_void,
            msg_controllen: ctrl_len.saturating_cast(),
            msg_flags:      0,
        };
        let flags = flags.0 | MSG_CMSG_CLOEXEC;
        let buf_len = try!(retry(|| recvmsg(self.fd, &mut msg,
                                            flags)).map(|v| v as usize));

        let addr_buf = &mut addr[..msg.msg_namelen as usize];
        let addr = match addr::type_supported(addr_buf) {
            true => unsafe { Some(SockAddr::from_mut_bytes_unchecked(&mut addr_buf[..])) },
            _ => None,
        };
        let ctrl_buf = unsafe { slice::from_ptr(ctrl_ptr, msg.msg_controllen as usize) };
        let iter = CMsgIter::new(ctrl_buf).unwrap();
        Ok((buf_len, addr, iter, msg::Flags(msg.msg_flags as c_int)))
    }

    fn set_bool(&self, level: c_int, opt: c_int, val: bool) -> Result {
        let val = val as c_int;
        rv!(setsockopt(self.fd, level, opt, val.as_ref()))
    }

    fn get_bool(&self, level: c_int, opt: c_int) -> Result<bool> {
        let mut val: c_int = 0;
        try!(rv!(getsockopt(self.fd, level, opt, val.as_mut(), &mut 0)));
        Ok(val != 0)
    }

    /// Return whether this socket has been set up to accept connections.
    pub fn accepts_connections(&self) -> Result<bool> {
        self.get_bool(SOL_SOCKET, SO_ACCEPTCONN)
    }

    /// Binds this socket to a device so that only packets from this device will be
    /// accepted.
    pub fn bind_to_device<D>(&self, device: D) -> Result
        where D: ToCStr,
    {
        let mut buf = [0; IFNAMSIZ];
        let cstr = try!(device.to_cstr(&mut buf));
        rv!(setsockopt(self.fd, SOL_SOCKET, SO_BINDTODEVICE, cstr.as_ref()))
    }

    /// Returns the name of the device this socket is bound to if any.
    pub fn device<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        let mut len = 0;
        try!(rv!(getsockopt(self.fd, SOL_SOCKET, SO_BINDTODEVICE, buf, &mut len)));
        Ok(buf[..len].as_mut_cstr().unwrap())
    }

    /// Enables or disables broadcasting for this socket.
    pub fn set_broadcast(&self, val: bool) -> Result {
        self.set_bool(SOL_SOCKET, SO_BROADCAST, val)
    }

    /// Returns whether this socket allows broadcasting.
    pub fn is_broadcast(&self) -> Result<bool> {
        self.get_bool(SOL_SOCKET, SO_BROADCAST)
    }

    /// Enables or disables debug mode for this socket.
    pub fn set_debug(&self, val: bool) -> Result {
        self.set_bool(SOL_SOCKET, SO_DEBUG, val)
    }

    /// Returns whether this socket is in debug mode.
    pub fn is_debug(&self) -> Result<bool> {
        self.get_bool(SOL_SOCKET, SO_DEBUG)
    }

    /// Returns the domain of this socket, if any.
    pub fn domain(&self) -> Result<Domain> {
        let mut domain = 0;
        try!(rv!(getsockopt(self.fd, SOL_SOCKET, SO_DOMAIN, domain.as_mut(), &mut 0)));
        Ok(Domain(domain))
    }

    //pub fn error(&self) -> Result<Errno> {
    //    let mut error: c_int = 0;
    //    try!(rv!(
    //}
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
