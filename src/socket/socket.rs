// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use base::error::{self, Errno};
use cty::{
    msghdr, c_void, iovec, c_int, AF_UNSPEC, sa_family_t, SHUT_RD, SHUT_WR, SHUT_RDWR,
    SOL_SOCKET, SO_ACCEPTCONN, SO_BINDTODEVICE, IFNAMSIZ, SO_BROADCAST, SO_DEBUG,
    SO_DOMAIN, MSG_CMSG_CLOEXEC, MSG_NOSIGNAL, SOCK_CLOEXEC, SO_REUSEPORT, SO_REUSEADDR,
    SO_SNDTIMEO, SO_RCVTIMEO, SO_SNDBUF, SO_SNDBUFFORCE, SO_RCVBUF, SO_RCVBUFFORCE,
    timeval, SO_PRIORITY, SO_PEERCRED, SO_PEEK_OFF, SO_PASSCRED, SO_OOBINLINE,
    SO_MARK, SO_LINGER, SO_DONTROUTE, SO_KEEPALIVE, k_int, SO_ERROR, linger, INT_MAX,
};
use time_base::{Time};
use core::{num, slice, mem};
use syscall::{
    socket, bind, getsockname, getpeername, connect, close, shutdown, listen, sendto,
    sendmsg, recvfrom, recvmsg, getsockopt, setsockopt,
};
use str_one::{ToCStr, CStr, AsMutCStr};
use fd::{FDContainer};
use rv::{retry};
use saturating::{SaturatingCast};

use addr::{self, SockAddr};
use cmsg::{CMsgIter, Credentials};

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

    /// Returns the domain of this socket.
    pub fn domain(&self) -> Result<Domain> {
        let mut domain = 0;
        try!(rv!(getsockopt(self.fd, SOL_SOCKET, SO_DOMAIN, domain.as_mut(), &mut 0)));
        Ok(Domain(domain))
    }

    /// Retrieves the last pending socket error.
    ///
    /// [return_value]
    /// Returns the last socket error.
    pub fn error(&self) -> Result<Errno> {
        let mut error: c_int = 0;
        try!(rv!(getsockopt(self.fd, SOL_SOCKET, SO_ERROR, error.as_mut(), &mut 0)));
        Ok(Errno(error))
    }

    /// Retrieves whether this socket only sends packets to directly connected hosts.
    ///
    /// [return_value]
    /// Returns whether this socket only sends packets to directly connected hosts.
    ///
    /// = Remarks
    ///
    /// If this option is set, then the socket will not send packets via gateways.
    ///
    /// = See also
    ///
    /// * link:lrs::socket::Socket::set_dont_route
    pub fn is_dont_route(&self) -> Result<bool> {
        self.get_bool(SOL_SOCKET, SO_DONTROUTE)
    }

    /// Sets whether this socket only sends packets to directly connected hosts.
    ///
    /// [argument, val]
    /// Whether this socket only sends packets to directly connected hosts.
    ///
    /// = Remarks
    ///
    /// :dr: link:lrs::socket::msg::DontRoute[DontRoute]
    ///
    /// If this option is set, then the socket will not send packets via gateways. This
    /// option can also be set on a per-message basis with the {dr} flag.
    ///
    /// = See also
    ///
    /// * link:lrs::socket::Socket::is_dont_route
    /// * {dr}
    pub fn set_dont_route(&self, val: bool) -> Result {
        self.set_bool(SOL_SOCKET, SO_DONTROUTE, val)
    }

    /// Retrieves whether this socket sends keep-alive messages.
    ///
    /// [return_value]
    /// Returns whether this socket sends keep-alive messages.
    ///
    /// = Remarks
    ///
    /// This option only makes sense for connection-oriented protocols.
    ///
    /// = See also
    ///
    /// * link:lrs::socket::Socket::set_keep_alive
    pub fn is_keep_alive(&self) -> Result<bool> {
        self.get_bool(SOL_SOCKET, SO_KEEPALIVE)
    }

    /// Sets whether this socket sends keep-alive messages.
    ///
    /// [argument, val]
    /// Whether this socket sends keep-alive messages.
    ///
    /// = Remarks
    ///
    /// This option only makes sense for connection-oriented protocols.
    ///
    /// = See also
    ///
    /// * link:lrs::socket::Socket::is_keep_alive
    pub fn set_keep_alive(&self, val: bool) -> Result {
        self.set_bool(SOL_SOCKET, SO_KEEPALIVE, val)
    }

    /// Retrieves the linger setting of this socket.
    ///
    /// [return_value]
    /// Returns the number of seconds this socket will linger in the foreground or `None`.
    ///
    /// = See also
    ///
    /// * link:lrs::socket::Socket::set_linger
    pub fn linger(&self) -> Result<Option<u32>> {
        let mut linger: linger = mem::zeroed();
        try!(rv!(getsockopt(self.fd, SOL_SOCKET, SO_LINGER,
                            mem::as_mut_bytes(&mut linger), &mut 0)));
        if linger.l_onoff != 0 {
            Ok(Some(linger.l_linger as u32))
        } else {
            Ok(None)
        }
    }

    /// Sets the linger setting of this socket.
    ///
    /// [argument, seconds]
    /// For how long this socket will linger in the foreground, if at all.
    ///
    /// = Remarks
    ///
    /// :shut: link:lrs::socket::Socket::shutdown
    ///
    /// If this is set to anything but `None`, then closing the socket or calling :shut:
    /// will block for up to `seconds` seconds, waiting for all pending outgoing messages
    /// to be sent. Afterwards the socket lingers in the background.
    ///
    /// = See also
    ///
    /// * link:lrs::socket::Socket::linger
    pub fn set_linger(&self, seconds: Option<u32>) -> Result {
        let linger = linger {
            l_onoff: seconds.is_some() as k_int,
            l_linger: seconds.unwrap_or(0).saturating_cast(),
        };
        rv!(setsockopt(self.fd, SOL_SOCKET, SO_LINGER, mem::as_bytes(&linger)))
    }

    pub fn mark(&self) -> Result<u32> {
        let mut mark = 0;
        try!(rv!(getsockopt(self.fd, SOL_SOCKET, SO_MARK, mark.as_mut(), &mut 0)));
        Ok(mark)
    }

    pub fn set_mark(&self, mark: u32) -> Result {
        rv!(setsockopt(self.fd, SOL_SOCKET, SO_MARK, mark.as_ref()))
    }

    /// Retrieves whether this socket places out-of-band data directly into the data
    /// stream.
    ///
    /// [return_value]
    /// Returns whether this socket places out-of-band data directly into the data stream.
    ///
    /// = See also
    ///
    /// * link:lrs::socket::Socket::set_oob_inline
    pub fn is_oob_inline(&self) -> Result<bool> {
        self.get_bool(SOL_SOCKET, SO_OOBINLINE)
    }

    /// Sets whether this socket places out-of-band data directly into the data stream.
    ///
    /// [argument, val]
    /// Whether this socket places out-of-band data directly into the data stream.
    ///
    /// = Remarks
    ///
    /// :oob: link:lrs::socket::msg::OutOfBand[OutOfBand]
    ///
    /// If this option is not set, out-of-band data can only be received with the {oob}
    /// message option.
    ///
    /// = See also
    ///
    /// * link:lrs::socket::Socket::is_oob_inline
    /// * {oob}
    pub fn set_oob_inline(&self, val: bool) -> Result {
        self.set_bool(SOL_SOCKET, SO_OOBINLINE, val)
    }

    /// Retrieves whether this socket accepts credentials control messages.
    ///
    /// [return_value]
    /// Returns whether this socket accepts credentials control messages.
    ///
    /// = See also
    ///
    /// * link:lrs::socket::cmsg::Credentials
    /// * link:lrs::socket::Socket::set_receive_credentials
    pub fn is_receive_credentials(&self) -> Result<bool> {
        self.get_bool(SOL_SOCKET, SO_PASSCRED)
    }

    /// Sets whether this socket accepts credentials control messages.
    ///
    /// [argument, val]
    /// Whether this socket accepts credentials control messages.
    ///
    /// = See also
    ///
    /// * link:lrs::socket::cmsg::Credentials
    /// * link:lrs::socket::Socket::is_receive_credentials
    pub fn set_receive_credentials(&self, val: bool) -> Result {
        self.set_bool(SOL_SOCKET, SO_PASSCRED, val)
    }

    /// Retrieves the peek offset of this socket.
    ///
    /// [return_value]
    /// Returns the peek offset of this socket.
    ///
    /// = See also
    ///
    /// * link:lrs::socket::Socket::set_peek_offset
    pub fn peek_offset(&self) -> Result<Option<usize>> {
        let mut val: c_int = 0;
        try!(rv!(getsockopt(self.fd, SOL_SOCKET, SO_PEEK_OFF, val.as_mut(), &mut 0)));
        if val < 0 {
            Ok(None)
        } else {
            Ok(Some(val as usize))
        }
    }

    /// Sets the peek offset of this socket.
    ///
    /// [argument, val]
    /// The peek offset of this socket.
    ///
    /// = Remarks
    ///
    /// This is only implemented for Unix sockets. The argument must be representable in a
    /// `c_int`.
    ///
    /// = See also
    ///
    /// * link:lrs::socket::Socket::peek_offset
    pub fn set_peek_offset(&self, val: Option<usize>) -> Result {
        if val.is_none() {
            let val: c_int = -1;
            rv!(setsockopt(self.fd, SOL_SOCKET, SO_PEEK_OFF, val.as_ref()))
        } else {
            let val = val.unwrap();
            if val > INT_MAX as usize {
                Err(error::InvalidArgument)
            } else {
                let val = val as c_int;
                rv!(setsockopt(self.fd, SOL_SOCKET, SO_PEEK_OFF, val.as_ref()))
            }
        }
    }

    /// Retrieves the credentials of a connected peer.
    ///
    /// [return_value]
    /// Returns the credentials of the peer.
    ///
    /// = Remarks
    ///
    /// This is only implemented for Unix sockets.
    pub fn peer_credentials(&self) -> Result<Credentials> {
        let mut val = mem::zeroed();
        try!(rv!(getsockopt(self.fd, SOL_SOCKET, SO_PEERCRED, mem::as_mut_bytes(&mut val),
                            &mut 0)));
        Ok(val)
    }

    /// Retrieves the priority of packets sent over this socket.
    ///
    /// [return_value]
    /// Returns the priority of packets sent over this socket.
    ///
    /// = See also
    ///
    /// * link:lrs::socket::Socket::set_priority
    pub fn priority(&self) -> Result<u32> {
        let mut val: c_int = 0;
        try!(rv!(getsockopt(self.fd, SOL_SOCKET, SO_PRIORITY, val.as_mut(), &mut 0)));
        Ok(val as u32)
    }

    /// Sets the priority of packets sent over this socket.
    ///
    /// [argument, val]
    /// The priority of packets sent over this socket.
    ///
    /// = Remarks
    ///
    /// The argument must be representable in a `c_int`.
    ///
    /// = See also
    ///
    /// * link:lrs::socket::Socket::priority
    pub fn set_priority(&self, val: u32) -> Result {
        if val > INT_MAX as u32 {
            return Err(error::InvalidArgument);
        }
        let val = val as c_int;
        rv!(setsockopt(self.fd, SOL_SOCKET, SO_PRIORITY, val.as_ref()))
    }

    /// Retrieves the maximal receive buffer size of this socket.
    ///
    /// [return_value]
    /// Returns the maximal receive buffer size of this socket.
    ///
    /// = See also
    ///
    /// * link:lrs::socket::Socket::set_recv_buffer_size
    pub fn recv_buffer_size(&self) -> Result<usize> {
        let mut val: c_int = 0;
        try!(rv!(getsockopt(self.fd, SOL_SOCKET, SO_RCVBUF, val.as_mut(), &mut 0)));
        Ok(val as usize)
    }

    /// Sets the maximal receive buffer size of this socket.
    ///
    /// [argument, size]
    /// The maximal receive buffer size of this socket.
    ///
    /// = Remarks
    ///
    /// :recv: link:lrs::socket::Socket::recv_buffer_size[recv_buffer_size]
    ///
    /// The argument must be representable in a `c_int`. The value will be doubled by the
    /// kernel and this doubled value will be returned by a call to {recv}.
    ///
    /// = See also
    ///
    /// * {recv}
    /// * link:lrs::socket::Socket::force_set_recv_buffer_size
    pub fn set_recv_buffer_size(&self, size: usize) -> Result {
        if size > INT_MAX as usize {
            return Err(error::InvalidArgument);
        }
        let val = size as c_int;
        rv!(setsockopt(self.fd, SOL_SOCKET, SO_RCVBUF, val.as_ref()))
    }

    /// Sets the maximal receive buffer size of this socket, overriding normal system
    /// limits.
    ///
    /// [argument, size]
    /// The maximal receive buffer size of this socket.
    ///
    /// = Remarks
    ///
    /// :recv: link:lrs::socket::Socket::recv_buffer_size[recv_buffer_size]
    /// :ord: link:lrs::socket::Socket::set_recv_buffer_size[set_recv_buffer_size]
    ///
    /// The argument must be representable in a `c_int`. The value will be doubled by the
    /// kernel and this doubled value will be returned by a call to {recv}.
    ///
    /// This function is different from {ord} in that privileged processes can override
    /// the system limits imposed on the buffer size.
    ///
    /// = See also
    ///
    /// * {recv}
    /// * {ord}
    pub fn force_set_recv_buffer_size(&self, size: usize) -> Result {
        if size > INT_MAX as usize {
            return Err(error::InvalidArgument);
        }
        let val = size as c_int;
        rv!(setsockopt(self.fd, SOL_SOCKET, SO_RCVBUFFORCE, val.as_ref()))
    }

    fn common_timeout(&self, ty: c_int) -> Result<Option<Time>> {
        let mut time: timeval = mem::zeroed();
        try!(rv!(getsockopt(self.fd, SOL_SOCKET, ty, mem::as_mut_bytes(&mut time),
                            &mut 0)));
        if time.tv_sec == 0 && time.tv_usec == 0 {
            Ok(None)
        } else {
            Ok(Some(Time {
                seconds: time.tv_sec as i64,
                nanoseconds: time.tv_usec as i64 * 1000,
            }))
        }
    }

    fn set_common_timeout(&self, ty: c_int, val: Option<Time>) -> Result {
        let val = match val {
            Some(v) => v,
            _ => Time::seconds(0),
        };
        let time = timeval {
            tv_sec: val.seconds.saturating_cast(),
            tv_usec: val.nanoseconds / 1000,
        };
        rv!(setsockopt(self.fd, SOL_SOCKET, ty, mem::as_bytes(&time)))
    }

    /// Retrieves the timeout option of receiving operations.
    ///
    /// [return_value]
    /// Returns the timeout of receiving operations.
    ///
    /// = Remarks
    ///
    /// A return value of `None` implies that operations never time-out.
    ///
    /// = See also
    ///
    /// * link:lrs::socket::Socket::set_recv_timeout
    /// * link:lrs::socket::Socket::send_timeout
    pub fn recv_timeout(&self) -> Result<Option<Time>> {
        self.common_timeout(SO_RCVTIMEO)
    }

    /// Sets the timeout option of receiving operations.
    ///
    /// [argument, val]
    /// The timeout option of receiving operations.
    ///
    /// = Remarks
    ///
    /// A `None` argument implies that operations never time-out.
    ///
    /// = See also
    ///
    /// * link:lrs::socket::Socket::recv_timeout
    /// * link:lrs::socket::Socket::set_send_timeout
    pub fn set_recv_timeout(&self, val: Option<Time>) -> Result {
        self.set_common_timeout(SO_RCVTIMEO, val)
    }

    /// Retrieves the timeout option of sending operations.
    ///
    /// [return_value]
    /// Returns the timeout of sending operations.
    ///
    /// = Remarks
    ///
    /// A return value of `None` implies that operations never time-out.
    ///
    /// = See also
    ///
    /// * link:lrs::socket::Socket::set_send_timeout
    /// * link:lrs::socket::Socket::recv_timeout
    pub fn send_timeout(&self) -> Result<Option<Time>> {
        self.common_timeout(SO_SNDTIMEO)
    }

    /// Sets the timeout option of sending operations.
    ///
    /// [argument, val]
    /// The timeout option of sending operations.
    ///
    /// = Remarks
    ///
    /// A `None` argument implies that operations never time-out.
    ///
    /// = See also
    ///
    /// * link:lrs::socket::Socket::send_timeout
    /// * link:lrs::socket::Socket::set_recv_timeout
    pub fn set_send_timeout(&self, val: Option<Time>) -> Result {
        self.set_common_timeout(SO_SNDTIMEO, val)
    }

    pub fn is_reuse_addr(&self) -> Result<bool> {
        self.get_bool(SOL_SOCKET, SO_REUSEADDR)
    }

    pub fn set_reuse_addr(&self, val: bool) -> Result {
        self.set_bool(SOL_SOCKET, SO_REUSEADDR, val)
    }

    pub fn is_reuse_port(&self) -> Result<bool> {
        self.get_bool(SOL_SOCKET, SO_REUSEPORT)
    }

    pub fn set_reuse_port(&self, val: bool) -> Result {
        self.set_bool(SOL_SOCKET, SO_REUSEPORT, val)
    }

    /// Retrieves the maximal send buffer size of this socket.
    ///
    /// [return_value]
    /// Returns the maximal send buffer size of this socket.
    ///
    /// = See also
    ///
    /// * link:lrs::socket::Socket::set_send_buffer_size
    pub fn send_buffer_size(&self) -> Result<usize> {
        let mut val: c_int = 0;
        try!(rv!(getsockopt(self.fd, SOL_SOCKET, SO_SNDBUF, val.as_mut(), &mut 0)));
        Ok(val as usize)
    }

    /// Sets the maximal send buffer size of this socket.
    ///
    /// [argument, size]
    /// The maximal send buffer size of this socket.
    ///
    /// = Remarks
    ///
    /// :send: link:lrs::socket::Socket::send_buffer_size[send_buffer_size]
    ///
    /// The argument must be representable in a `c_int`. The value will be doubled by the
    /// kernel and this doubled value will be returned by a call to {send}.
    ///
    /// = See also
    ///
    /// * {send}
    /// * link:lrs::socket::Socket::force_set_send_buffer_size
    pub fn set_send_buffer_size(&self, size: usize) -> Result {
        if size > INT_MAX as usize {
            return Err(error::InvalidArgument);
        }
        let val = size as c_int;
        rv!(setsockopt(self.fd, SOL_SOCKET, SO_SNDBUF, val.as_ref()))
    }

    /// Sets the maximal send buffer size of this socket, overriding normal system
    /// limits.
    ///
    /// [argument, size]
    /// The maximal send buffer size of this socket.
    ///
    /// = Remarks
    ///
    /// :send: link:lrs::socket::Socket::send_buffer_size[send_buffer_size]
    /// :ord: link:lrs::socket::Socket::set_send_buffer_size[set_send_buffer_size]
    ///
    /// The argument must be representable in a `c_int`. The value will be doubled by the
    /// kernel and this doubled value will be returned by a call to {send}.
    ///
    /// This function is different from {ord} in that privileged processes can override
    /// the system limits imposed on the buffer size.
    ///
    /// = See also
    ///
    /// * {send}
    /// * {ord}
    pub fn force_set_send_buffer_size(&self, size: usize) -> Result {
        if size > INT_MAX as usize {
            return Err(error::InvalidArgument);
        }
        let val = size as c_int;
        rv!(setsockopt(self.fd, SOL_SOCKET, SO_SNDBUFFORCE, val.as_ref()))
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
