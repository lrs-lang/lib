// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use base::error::{self, Errno};
use base::undef::{UndefState};
use cty::{
    msghdr, c_void, iovec, c_int, AF_UNSPEC, sa_family_t, SHUT_RD, SHUT_WR, SHUT_RDWR,
    SOL_SOCKET, SO_ACCEPTCONN, SO_BINDTODEVICE, IFNAMSIZ, SO_BROADCAST, SO_DEBUG,
    SO_DOMAIN, MSG_NOSIGNAL, SO_REUSEPORT, SO_REUSEADDR,
    SO_SNDTIMEO, SO_RCVTIMEO, SO_SNDBUF, SO_SNDBUFFORCE, SO_RCVBUF, SO_RCVBUFFORCE,
    timeval, SO_PRIORITY, SO_PEERCRED, SO_PEEK_OFF, SO_PASSCRED, SO_OOBINLINE,
    SO_MARK, SO_LINGER, SO_DONTROUTE, SO_KEEPALIVE, k_int, SO_ERROR, linger, INT_MAX,
    SO_PROTOCOL, SO_TIMESTAMPNS, SO_TYPE,
    IPPROTO_IP, IP_ADD_MEMBERSHIP, IP_DROP_MEMBERSHIP, IP_ADD_SOURCE_MEMBERSHIP,
    IP_DROP_SOURCE_MEMBERSHIP, IP_MULTICAST_ALL, IP_MULTICAST_TTL, IP_BLOCK_SOURCE,
    IP_UNBLOCK_SOURCE, in_addr, IP_RECVOPTS, IP_RETOPTS, IP_MULTICAST_LOOP, ip_mreqn,
    IP_OPTIONS, IP_TTL, IP_MTU, IP_HDRINCL, timespec, ip_mreq_source,
    IPPROTO_IPV6, IPV6_MTU, IPV6_MULTICAST_HOPS, IPV6_ADD_MEMBERSHIP,
    IPV6_DROP_MEMBERSHIP, IPV6_UNICAST_HOPS, IPV6_V6ONLY, AF_INET, IPV6_ADDRFORM,
    ipv6_mreq, in6_addr, IPV6_MULTICAST_LOOP,
    IPPROTO_TCP, TCP_CORK, IPPROTO_UDP, UDP_CORK,
};
use time_base::{Time};
use core::{slice, mem};
use syscall::{
    socket, bind, getsockname, getpeername, connect, close, shutdown, listen, sendto,
    sendmsg, recvfrom, recvmsg, getsockopt, setsockopt, ioctl_siocgstampns, ioctl_siocinq,
    ioctl_siocoutq, accept4,
};
use str_one::{CStr, NoNullStr};
use fd::{FdContainer};
use rv::{retry};
use saturating::{SaturatingCast};

use addr::{self, SockAddr};
use addr::ipv4::{Ipv4Addr};
use addr::ipv6::{Ipv6Addr};
use cmsg::{CMsgIter, Credentials};
use time_base::{self};

use ip_proto::{self};
use nl_proto::{self};
use domain::{self, Domain};
use kind::{self, Kind};
use msg::{MsgFlags};
use flags::{SockFlags};

/// A Socket
///
/// = Remarks
///
/// This type provides access to the Linux socket interface.
///
/// = See also
///
/// * link:man:socket(7)
pub struct Socket {
    fd: c_int,
    owned: bool,
}

impl Socket {
    /// Creates a new socket.
    ///
    /// = Remarks
    ///
    /// This is the most general constructor. There are simpler, more specialized
    /// constructors for the common cases of Unix, Ipv4, and Ipv6 sockets.
    ///
    /// = See also
    ///
    /// * link:man:socket(2)
    pub fn new(domain: Domain, kind: Kind, protocol: c_int,
               flags: SockFlags) -> Result<Socket> {
        let ty = flags.0 | kind.0;
        let fd = try!(rv!(socket(domain.0, ty, protocol), -> c_int));
        Ok(Socket { fd: fd, owned: true })
    }

    /// Creates a Unix/Tcp socket.
    ///
    /// = Remarks
    ///
    /// This is equivalent to
    ///
    /// ----
    /// Socket::new(domain::Unix, kind::Stream, 0, flags)
    /// ----
    ///
    /// = See also
    ///
    /// * link:man:unix(7)
    /// * link:man:tcp(7)
    pub fn unix_stream(flags: SockFlags) -> Result<Socket> {
        Socket::new(domain::Unix, kind::Stream, 0, flags)
    }

    /// Creates a Unix/Udp socket.
    ///
    /// = Remarks
    ///
    /// This is equivalent to
    ///
    /// ----
    /// Socket::new(domain::Unix, kind::Datagram, 0, flags)
    /// ----
    ///
    /// = See also
    ///
    /// * link:man:unix(7)
    /// * link:man:tcp(7)
    pub fn unix_datagram(flags: SockFlags) -> Result<Socket> {
        Socket::new(domain::Unix, kind::Datagram, 0, flags)
    }

    /// Creates a Unix/SeqPacket socket.
    ///
    /// = Remarks
    ///
    /// This is equivalent to
    ///
    /// ----
    /// Socket::new(domain::Unix, kind::SeqPacket, 0, flags)
    /// ----
    ///
    /// = See also
    ///
    /// * link:man:unix(7)
    pub fn unix_seqpacket(flags: SockFlags) -> Result<Socket> {
        Socket::new(domain::Unix, kind::SeqPacket, 0, flags)
    }

    /// Creates an Ipv4/Tcp socket.
    ///
    /// = Remarks
    ///
    /// This is equivalent to
    ///
    /// ----
    /// Socket::new(domain::Ipv4, kind::Stream, 0, flags)
    /// ----
    ///
    /// = See also
    ///
    /// * link:man:ip(7)
    /// * link:man:tcp(7)
    pub fn ipv4_stream(flags: SockFlags) -> Result<Socket> {
        Socket::new(domain::Ipv4, kind::Stream, 0, flags)
    }

    /// Creates an Ipv4/Udp socket.
    ///
    /// = Remarks
    ///
    /// This is equivalent to
    ///
    /// ----
    /// Socket::new(domain::Ipv4, kind::Datagram, 0, flags)
    /// ----
    ///
    /// = See also
    ///
    /// * link:man:ip(7)
    /// * link:man:udp(7)
    pub fn ipv4_datagram(flags: SockFlags) -> Result<Socket> {
        Socket::new(domain::Ipv4, kind::Datagram, 0, flags)
    }

    /// Creates a raw Ipv4 socket.
    ///
    /// = Remarks
    ///
    /// This is equivalent to
    ///
    /// ----
    /// Socket::new(domain::Ipv4, kind::Raw, proto.0 as c_int, flags)
    /// ----
    ///
    /// = See also
    ///
    /// * link:man:ip(7)
    /// * link:man:raw(7)
    pub fn ipv4_raw(proto: ip_proto::Proto, flags: SockFlags) -> Result<Socket> {
        Socket::new(domain::Ipv4, kind::Raw, proto.0 as c_int, flags)
    }

    /// Creates an Ipv6/Tcp socket.
    ///
    /// = Remarks
    ///
    /// This is equivalent to
    ///
    /// ----
    /// Socket::new(domain::Ipv6, kind::Stream, 0, flags)
    /// ----
    ///
    /// = See also
    ///
    /// * link:man:ipv6(7)
    /// * link:man:tcp(7)
    pub fn ipv6_stream(flags: SockFlags) -> Result<Socket> {
        Socket::new(domain::Ipv6, kind::Stream, 0, flags)
    }

    /// Creates an Ipv6/Udp socket.
    ///
    /// = Remarks
    ///
    /// This is equivalent to
    ///
    /// ----
    /// Socket::new(domain::Ipv6, kind::Datagram, 0, flags)
    /// ----
    ///
    /// = See also
    ///
    /// * link:man:ipv6(7)
    /// * link:man:udp(7)
    pub fn ipv6_datagram(flags: SockFlags) -> Result<Socket> {
        Socket::new(domain::Ipv6, kind::Datagram, 0, flags)
    }

    /// Creates a raw Ipv6 socket.
    ///
    /// = Remarks
    ///
    /// This is equivalent to
    ///
    /// ----
    /// Socket::new(domain::Ipv6, kind::Raw, proto.0 as c_int, flags)
    /// ----
    ///
    /// = See also
    ///
    /// * link:man:ipv6(7)
    /// * link:man:raw(7)
    pub fn ipv6_raw(proto: ip_proto::Proto, flags: SockFlags) -> Result<Socket> {
        Socket::new(domain::Ipv6, kind::Raw, proto.0 as c_int, flags)
    }

    /// Creates a new Netlink socket.
    ///
    /// = Remarks
    ///
    /// This is equivalent to
    ///
    /// ----
    /// Socket::new(domain::Netlink, kind::Raw, proto.0 as c_int, flags)
    /// ----
    ///
    /// = See also
    ///
    /// * link:man:netlink(7)
    pub fn netlink(proto: nl_proto::Proto, flags: SockFlags) -> Result<Socket> {
        Socket::new(domain::Netlink, kind::Raw, proto.0, flags)
    }

    /// Binds the socket to an address.
    ///
    /// [argument, addr]
    /// The address the socket will be bound to.
    ///
    /// = See also
    ///
    /// * link:man:bind(2)
    pub fn bind<A: ?Sized>(&self, addr: &A) -> Result
        where A: AsRef<[u8]>,
    {
        rv!(bind(self.fd, addr.as_ref().as_ref()))
    }

    /// Retrieves the address the socket is bound to.
    ///
    /// [argument, buf]
    /// The buffer in which the address will be stored.
    ///
    /// [return_value]
    /// Returns the address the socket is bound to.
    ///
    /// = See also
    ///
    /// * link:man:getsockname(2)
    pub fn get_addr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut SockAddr> {
        let mut len = 0;
        try!(rv!(getsockname(self.fd, buf.as_mut(), &mut len)));
        if addr::type_supported(buf) {
            Ok(unsafe { SockAddr::from_mut_bytes_unchecked(&mut buf[..len]) })
        } else {
            Err(error::NotSupported)
        }
    }

    /// Retrieves the address of the peer this socket is connected to, if any.
    ///
    /// [argument, buf]
    /// The buffer in which the address will be stored.
    ///
    /// [return_value]
    /// Returns the address of the peer this socket is connected to, if any.
    ///
    /// = See also
    ///
    /// * link:man:getpeername(2)
    pub fn get_peer_addr<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut SockAddr> {
        let mut len = 0;
        try!(rv!(getpeername(self.fd, buf.as_mut(), &mut len)));
        if addr::type_supported(buf) {
            Ok(unsafe { SockAddr::from_mut_bytes_unchecked(&mut buf[..len]) })
        } else {
            Err(error::NotSupported)
        }
    }

    /// Connects this socket to a peer.
    ///
    /// [argument, addr]
    /// The address of the peer.
    ///
    /// = Remarks
    ///
    /// For connection-less sockets, this only sets the default address for packets
    /// where the destination has not been specified.
    ///
    /// = See also
    ///
    /// * link:man:connect(2)
    pub fn connect<A: ?Sized>(&self, addr: &A) -> Result
        where A: AsRef<[u8]>,
    {
        rv!(connect(self.fd, addr.as_ref().as_ref()))
    }

    /// Disconnects a connected, connection-less socket.
    ///
    /// = Remarks
    ///
    /// This does not work for connection-based sockets.
    ///
    /// = See also
    ///
    /// * link:man:connect(2)
    pub fn disconnect(&self) -> Result {
        rv!(connect(self.fd, (AF_UNSPEC as sa_family_t).as_ref()))
    }

    /// Shuts down the receiving and sending ends of a connected socket.
    ///
    /// = See also
    ///
    /// * link:man:shutdown(2)
    pub fn shutdown(&self) -> Result {
        rv!(shutdown(self.fd, SHUT_RDWR))
    }

    /// Shuts down the receiving end of a connected socket.
    ///
    /// = See also
    ///
    /// * link:man:shutdown(2)
    pub fn shutdown_incoming(&self) -> Result {
        rv!(shutdown(self.fd, SHUT_RD))
    }

    /// Shuts down the sending end of a socket.
    ///
    /// = See also
    ///
    /// * link:man:shutdown(2)
    pub fn shutdown_outgoing(&self) -> Result {
        rv!(shutdown(self.fd, SHUT_WR))
    }

    /// Marks this socket as accepting incoming connections.
    ///
    /// [argument, backlog]
    /// The maximum number of pending connection requests.
    ///
    /// = Remarks
    ///
    /// When `backlog` many requests are waiting to be accepted, new requests will
    /// automatically be rejected.
    ///
    /// = See also
    ///
    /// * link:man:listen(2)
    pub fn listen(&self, backlog: u32) -> Result {
        rv!(listen(self.fd, backlog))
    }

    /// Accepts a new connection on the socket.
    ///
    /// [argument, flags]
    /// The flags that will be set on the returned socket.
    ///
    /// = Remarks
    ///
    /// The flags argument will be used to construct the new socket, similar to how they
    /// are used in the constructors of the socket type.
    ///
    /// = See also
    ///
    /// * link:man:accept4(2)
    pub fn accept<'a>(&self, flags: SockFlags) -> Result<Socket> {
        let fd = try!(rv!(accept4(self.fd, None, &mut 0, flags.0), -> c_int));
        Ok(Socket { fd: fd, owned: true })
    }

    /// Accepts a new connection on the socket and returns the peer's address.
    ///
    /// [argument, addr]
    /// A buffer in which the address of the peer will be stored.
    ///
    /// [argument, flags]
    /// The flags that will be set on the returned socket.
    ///
    /// = Remarks
    ///
    /// :getpeeraddr: link:lrs::socket::Socket::get_peer_addr[get_peer_addr]
    ///
    /// If the address buffer is too small, and in particular if the address buffer is
    /// empty, the address of the peer will not be returned. It can still be obtained
    /// afterwards with the {getpeeraddr} method.
    ///
    /// The flags argument will be used to construct the new socket, similar to how they
    /// are used in the constructors of the socket type.
    ///
    /// = See also
    ///
    /// * link:man:accept4(2)
    pub fn accept_addr<'a>(&self, addr: &'a mut [u8],
                        flags: SockFlags) -> Result<(Result<&'a mut SockAddr>, Socket)> {
        let mut len = 0;
        let fd = try!(rv!(accept4(self.fd, Some(addr.as_mut()), &mut len, flags.0), -> c_int));
        let sock = Socket { fd: fd, owned: true };
        if len > addr.len() {
            Ok((Err(error::NoMemory), sock))
        } else {
            let addr = SockAddr::from_mut_bytes(&mut addr[..len]);
            Ok((addr, sock))
        }
    }

    /// Sends a message over a connected socket.
    ///
    /// [argument, buf]
    /// The buffer containing the message.
    ///
    /// [argument, flags]
    /// Flags that can change the behavior of the operation.
    ///
    /// [return_value]
    /// Returns the number of bytes sent.
    ///
    /// = See also
    ///
    /// * link:man:send(2)
    pub fn send(&self, buf: &[u8], flags: MsgFlags) -> Result<usize> {
        let flags = flags.0 | MSG_NOSIGNAL;
        retry(|| sendto(self.fd, buf.as_ref(), flags, None)).map(|v| v as usize)
    }

    /// Sends a message on a connection-less socket.
    ///
    /// [argument, buf]
    /// The buffer containing the message.
    ///
    /// [argument, addr]
    /// The destination of the message.
    ///
    /// [argument, flags]
    /// Flags that can change the behavior of the operation.
    ///
    /// [return_value]
    /// Returns the number of bytes sent.
    ///
    /// = See also
    ///
    /// * link:man:sendto(2)
    pub fn send_to<A: ?Sized>(&self, buf: &[u8], addr: &A,
                              flags: MsgFlags) -> Result<usize>
        where A: AsRef<[u8]>,
    {
        let flags = flags.0 | MSG_NOSIGNAL;
        retry(|| sendto(self.fd, buf.as_ref(), flags, Some(addr.as_ref().as_ref()))).map(|v| v as usize)
    }

    /// Gathers a message from multiple sources and sends it on a connected socket.
    ///
    /// [argument, buf]
    /// The buffers containing the message.
    ///
    /// [argument, flags]
    /// Flags that can change the behavior of the operation.
    ///
    /// [return_value]
    /// Returns the number of bytes sent.
    ///
    /// = See also
    ///
    /// * link:man:sendmsg(2)
    pub fn gather_send(&self, buf: &[&[u8]], flags: MsgFlags) -> Result<usize> {
        let addr: &[u8] = &[];
        let ctrl: &[u8] = &[];
        self.send_ctrl_to(buf, addr, ctrl, flags)
    }

    /// Gathers a message from multiple sources and sends it on a connection-less socket.
    ///
    /// [argument, buf]
    /// The buffers containing the message.
    ///
    /// [argument, addr]
    /// The destination of the message.
    ///
    /// [argument, flags]
    /// Flags that can change the behavior of the operation.
    ///
    /// [return_value]
    /// Returns the number of bytes sent.
    ///
    /// = See also
    ///
    /// * link:man:sendmsg(2)
    pub fn gather_send_to<A: ?Sized>(&self, buf: &[&[u8]], addr: &A,
                                     flags: MsgFlags) -> Result<usize>
        where A: AsRef<[u8]>,
    {
        let addr = addr.as_ref();
        let ctrl: &[u8] = &[];
        self.send_ctrl_to(buf, addr, ctrl, flags)
    }

    /// Gathers a message from multiple source and sends it together with a control
    /// message on a connected socket.
    ///
    /// [argument, buf]
    /// The buffers containing the message.
    ///
    /// [argument, ctrl]
    /// The control message.
    ///
    /// [argument, flags]
    /// Flags that can change the behavior of the operation.
    ///
    /// [return_value]
    /// Returns the number of bytes sent.
    ///
    /// = See also
    ///
    /// * link:man:sendmsg(2)
    pub fn send_ctrl<C: ?Sized>(&self, buf: &[&[u8]], ctrl: &C,
                                flags: MsgFlags) -> Result<usize>
        where C: AsRef<[u8]>,
    {
        let addr: &[u8] = &[];
        let ctrl = ctrl.as_ref();
        self.send_ctrl_to(buf, addr, ctrl, flags)
    }

    /// Gathers a message from multiple source and sends it together with a control
    /// message on a connection-less socket.
    ///
    /// [argument, buf]
    /// The buffers containing the message.
    ///
    /// [argument, addr]
    /// The destination of the message.
    ///
    /// [argument, ctrl]
    /// The control message.
    ///
    /// [argument, flags]
    /// Flags that can change the behavior of the operation.
    ///
    /// [return_value]
    /// Returns the number of bytes sent.
    ///
    /// = See also
    ///
    /// * link:man:sendmsg(2)
    pub fn send_ctrl_to<A: ?Sized, C: ?Sized>(&self, buf: &[&[u8]], addr: &A, ctrl: &C,
                                              flags: MsgFlags) -> Result<usize>
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

    /// Receives a message on a connected socket.
    ///
    /// [argument, buf]
    /// The buffer in which the message will be stored.
    ///
    /// [argument, flags]
    /// Flags that can change the behavior of the operation.
    ///
    /// [return_value]
    /// Returns the number of bytes received.
    ///
    /// = See also
    ///
    /// * link:man:recv(2)
    pub fn recv<'a>(&self, buf: &'a mut [u8], flags: MsgFlags) -> Result<usize> {
        let mut addr_len = 0;
        retry(|| recvfrom(self.fd, buf.as_mut(), flags.0, None,
                          &mut addr_len)).map(|v| v as usize)
    }

    /// Receives a message on a connection-less socket.
    ///
    /// [argument, buf]
    /// The buffer in which the message will be stored.
    ///
    /// [argument, addr_buf]
    /// The buffer in which the address of the sender will be stored.
    ///
    /// [argument, flags]
    /// Flags that can change the behavior of the operation.
    ///
    /// [return_value]
    /// Returns the number of bytes received and the address of the sender.
    ///
    /// = See also
    ///
    /// * link:man:recvfrom(2)
    pub fn recv_from<'a>(&self, buf: &mut [u8], addr_buf: &'a mut [u8],
                         flags: MsgFlags) -> Result<(usize, Option<&'a mut SockAddr>)> {
        let mut addr_len = 0;
        let buf_len = try!(retry(|| recvfrom(self.fd, buf.as_mut(), flags.0,
                                             Some(addr_buf.as_mut()),
                                             &mut addr_len)).map(|v| v as usize));
        let addr_buf = &mut addr_buf[..addr_len];
        let addr = match addr::type_supported(addr_buf) {
            true => unsafe { Some(SockAddr::from_mut_bytes_unchecked(&mut addr_buf[..])) },
            _ => None,
        };
        Ok((buf_len, addr))
    }

    /// Receives a message on a connected socket and stores it in multiple buffers.
    ///
    /// [argument, buf]
    /// The buffers in which the message will be stored.
    ///
    /// [argument, flags]
    /// Flags that can change the behavior of the operation.
    ///
    /// [return_value]
    /// Returns the number of bytes received.
    ///
    /// = See also
    ///
    /// * link:man:recvmsg(2)
    pub fn scatter_recv(&self, buf: &mut [&mut [u8]],
                        flags: MsgFlags) -> Result<usize> {
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

    /// Receives a message, control messages, and the senders address.
    ///
    /// [argument, buf]
    /// The buffers in which the message will be stored.
    ///
    /// [argument, addr]
    /// The buffer in which the address of the sender will be stored.
    ///
    /// [argument, ctrl]
    /// The buffer in which the control messages will be stored.
    ///
    /// [argument, flags]
    /// Flags that can change the behavior of the operation.
    ///
    /// [return_value]
    /// Returns the number of bytes received, the address of the sender, an iterator over
    /// the received control messages, and flags of the message.
    ///
    /// = See also
    ///
    /// * link:man:recvmsg(2)
    pub fn recv_msg<'a, 'b>(
        &self,
        buf: &mut [&mut [u8]],
        addr: &'a mut [u8],
        ctrl: &'b mut [u8],
        flags: MsgFlags,
        ) -> Result<(usize, Option<&'a mut SockAddr>, CMsgIter<'b>, MsgFlags)>
    {
        let (addr_ptr, addr_len) = match addr.len() {
            0 => (0 as *mut u8, 0),
            n => (addr.as_mut_ptr(), n),
        };

        let (mut ctrl_ptr, mut ctrl_len) = match ctrl.len() {
            0 => (0 as *mut u8, 0),
            n => (ctrl.as_mut_ptr(), n),
        };
        const PTR_MASK: usize = usize::bytes() - 1;
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
        let buf_len = try!(retry(|| recvmsg(self.fd, &mut msg,
                                            flags.0)).map(|v| v as usize));

        let addr_buf = &mut addr[..msg.msg_namelen as usize];
        let addr = match addr::type_supported(addr_buf) {
            true => unsafe { Some(SockAddr::from_mut_bytes_unchecked(&mut addr_buf[..])) },
            _ => None,
        };
        let ctrl_buf = unsafe { slice::from_ptr(ctrl_ptr, msg.msg_controllen as usize) };
        let iter = try!(CMsgIter::new(ctrl_buf));
        Ok((buf_len, addr, iter, MsgFlags(msg.msg_flags as c_int)))
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

    /// Retrieves whether this socket has been set up to accept connections.
    ///
    /// [return_value]
    /// Returns whether this socket has been set up to accept connections.
    ///
    /// = See also
    ///
    /// * link:man:socket(7) and SO_ACCEPTCONN therein
    pub fn accepts_connections(&self) -> Result<bool> {
        self.get_bool(SOL_SOCKET, SO_ACCEPTCONN)
    }

    /// Binds this socket to a device so that only packets from this device will be
    /// accepted.
    ///
    /// [argument, device]
    /// The device to which the socket will be bound.
    ///
    /// = See also
    ///
    /// * link:man:socket(7) and SO_BINDTODEVICE therein
    /// * link:lrs::socket::Socket::device
    pub fn bind_to_device<D: ?Sized>(&self, device: &D) -> Result
        where D: TryAsRef<NoNullStr>,
    {
        let mut buf = [0; IFNAMSIZ];
        let cstr = try!(device.try_as_ref());
        if buf.len() < cstr.len() + 1 {
            return Err(error::InvalidArgument);
        }
        mem::copy(&mut buf, cstr.as_ref());
        buf[cstr.len()] = 0;
        let cstr = unsafe { mem::cast(&buf[..cstr.len()]) };
        rv!(setsockopt(self.fd, SOL_SOCKET, SO_BINDTODEVICE, cstr))
    }

    /// Retrieves the name of the device this socket is bound to, if any.
    ///
    /// [argumnet, buf]
    /// The buffer in which the device name will be stored.
    ///
    /// [return_value]
    /// Returns the device to which the socket is bound.
    ///
    /// = See also
    ///
    /// * link:man:socket(7) and SO_BINDTODEVICE therein
    /// * link:lrs::socket::Socket::bind_to_device
    pub fn device<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut CStr> {
        let mut len = 0;
        try!(rv!(getsockopt(self.fd, SOL_SOCKET, SO_BINDTODEVICE, buf.as_mut(), &mut len)));
        buf[..len].try_as_mut()
    }

    /// Retrieves whether this socket allows broadcasting.
    ///
    /// [return_value]
    /// Returns whether this socket allows broadcasting.
    ///
    /// = See also
    ///
    /// * link:man:socket(7) and SO_BROADCAST therein
    /// * link:lrs::socket::Socket::set_broadcast
    pub fn is_broadcast(&self) -> Result<bool> {
        self.get_bool(SOL_SOCKET, SO_BROADCAST)
    }

    /// Enables or disables broadcasting for this socket.
    ///
    /// [argument, val]
    /// Whether whether broadcasting is enabled.
    ///
    /// = See also
    ///
    /// * link:man:socket(7) and SO_BROADCAST therein
    /// * link:lrs::socket::Socket::is_broadcast
    pub fn set_broadcast(&self, val: bool) -> Result {
        self.set_bool(SOL_SOCKET, SO_BROADCAST, val)
    }

    /// Retrieves whether this socket is in debug mode.
    ///
    /// [return_value]
    /// Returns whether this socket is in debug mode.
    ///
    /// = See also
    ///
    /// * link:man:socket(7) and SO_DEBUG therein
    /// * link:lrs::socket::Socket::set_debug
    pub fn is_debug(&self) -> Result<bool> {
        self.get_bool(SOL_SOCKET, SO_DEBUG)
    }

    /// Enables or disables debug mode for this socket.
    ///
    /// [argument, val]
    /// Whether debug mode is enabled.
    ///
    /// = See also
    ///
    /// * link:man:socket(7) and SO_DEBUG therein
    /// * link:lrs::socket::Socket::is_debug
    pub fn set_debug(&self, val: bool) -> Result {
        self.set_bool(SOL_SOCKET, SO_DEBUG, val)
    }

    /// Retrieves the domain of this socket.
    ///
    /// [return_value]
    /// Returns the domain of this socket.
    ///
    /// = See also
    ///
    /// * link:man:socket(7) and SO_DOMAIN therein
    pub fn domain(&self) -> Result<Domain> {
        let mut domain = 0;
        try!(rv!(getsockopt(self.fd, SOL_SOCKET, SO_DOMAIN, domain.as_mut(), &mut 0)));
        Ok(Domain(domain))
    }

    /// Retrieves the last pending socket error.
    ///
    /// [return_value]
    /// Returns the last socket error.
    ///
    /// = See also
    ///
    /// * link:man:socket(7) and SO_ERROR therein
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
    /// * link:man:socket(7) and SO_DONTROUTE therein
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
    /// * link:man:socket(7) and SO_DONTROUTE therein
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
    /// * link:man:socket(7) and SO_KEEPALIVE therein
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
    /// * link:man:socket(7) and SO_KEEPALIVE therein
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
    /// * link:man:socket(7) and SO_LINGER therein
    /// * link:lrs::socket::Socket::set_linger
    pub fn linger(&self) -> Result<Option<u32>> {
        let mut linger: linger = mem::zeroed();
        try!(rv!(getsockopt(self.fd, SOL_SOCKET, SO_LINGER, linger.as_mut(), &mut 0)));
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
    /// * link:man:socket(7) and SO_LINGER therein
    /// * link:lrs::socket::Socket::linger
    pub fn set_linger(&self, seconds: Option<u32>) -> Result {
        let linger = linger {
            l_onoff: seconds.is_some() as k_int,
            l_linger: seconds.unwrap_or(0).saturating_cast(),
        };
        rv!(setsockopt(self.fd, SOL_SOCKET, SO_LINGER, linger.as_ref()))
    }

    /// Retrieves the mark of the packets sent over this socket.
    ///
    /// [return_value]
    /// Returns the mark of the packets sent over this socket.
    ///
    /// = See also
    ///
    /// * link:man:socket(7) and SO_MARK therein
    /// * link:lrs::socket::Socket::set_mark
    pub fn mark(&self) -> Result<c_int> {
        let mut mark = 0;
        try!(rv!(getsockopt(self.fd, SOL_SOCKET, SO_MARK, mark.as_mut(), &mut 0)));
        Ok(mark)
    }

    /// Sets the mark of the packets sent over this socket.
    ///
    /// [argument, val]
    /// The mark of the packets sent over this socket.
    ///
    /// = See also
    ///
    /// * link:man:socket(7) and SO_MARK therein
    /// * link:lrs::socket::Socket::mark
    pub fn set_mark(&self, mark: c_int) -> Result {
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
    /// * link:man:socket(7) and SO_OOBINLINE therein
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
    /// * link:man:socket(7) and SO_OOBINLINE therein
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
    /// * link:man:socket(7) and SO_PASSCRED therein
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
    /// * link:man:socket(7) and SO_PASSCRED therein
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
    /// * link:man:socket(7) and SO_PEEK_OFF therein
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
    /// * link:man:socket(7) and SO_PEEK_OFF therein
    /// * link:lrs::socket::Socket::peek_offset
    pub fn set_peek_offset(&self, val: Option<usize>) -> Result {
        if let Some(val) = val {
            if val > INT_MAX as usize {
                Err(error::InvalidArgument)
            } else {
                let val = val as c_int;
                rv!(setsockopt(self.fd, SOL_SOCKET, SO_PEEK_OFF, val.as_ref()))
            }
        } else {
            let val: c_int = -1;
            rv!(setsockopt(self.fd, SOL_SOCKET, SO_PEEK_OFF, val.as_ref()))
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
    ///
    /// = See also
    ///
    /// * link:man:socket(7) and SO_PEEKCRED therein
    pub fn peer_credentials(&self) -> Result<Credentials> {
        let mut val: Credentials = mem::zeroed();
        try!(rv!(getsockopt(self.fd, SOL_SOCKET, SO_PEERCRED, val.as_mut(), &mut 0)));
        Ok(val)
    }

    /// Retrieves the priority of packets sent over this socket.
    ///
    /// [return_value]
    /// Returns the priority of packets sent over this socket.
    ///
    /// = See also
    ///
    /// * link:man:socket(7) and SO_PRIORITY therein
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
    /// * link:man:socket(7) and SO_PRIORITY therein
    /// * link:lrs::socket::Socket::priority
    pub fn set_priority(&self, val: u32) -> Result {
        if val > INT_MAX as u32 {
            return Err(error::InvalidArgument);
        }
        let val = val as c_int;
        rv!(setsockopt(self.fd, SOL_SOCKET, SO_PRIORITY, val.as_ref()))
    }

    /// Retrieves the protocol of this socket.
    ///
    /// [return_value]
    /// Returns the protocol of this socket.
    ///
    /// = See also
    ///
    /// * link:man:socket(7) and SO_PROTOCOL therein
    pub fn protocol(&self) -> Result<c_int> {
        let mut val = 0;
        try!(rv!(getsockopt(self.fd, SOL_SOCKET, SO_PROTOCOL, val.as_mut(), &mut 0)));
        Ok(val)
    }

    /// Retrieves the maximal receive buffer size of this socket.
    ///
    /// [return_value]
    /// Returns the maximal receive buffer size of this socket.
    ///
    /// = See also
    ///
    /// * link:man:socket(7) and SO_RCVBUF therein
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
    /// * link:man:socket(7) and SO_RCVBUF therein
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
    /// * link:man:socket(7) and SO_RCVBUFFORCE therein
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
        try!(rv!(getsockopt(self.fd, SOL_SOCKET, ty, time.as_mut(), &mut 0)));
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
            tv_usec: (val.nanoseconds / 1000).saturating_cast(),
        };
        rv!(setsockopt(self.fd, SOL_SOCKET, ty, time.as_ref()))
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
    /// * link:man:socket(7) and SO_RCVTIMEO therein
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
    /// * link:man:socket(7) and SO_RCVTIMEO therein
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
    /// * link:man:socket(7) and SO_SNDTIMEO therein
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
    /// * link:man:socket(7) and SO_SNDTIMEO therein
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
    /// * link:man:socket(7) and SO_SNDBUF therein
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
    /// * link:man:socket(7) and SO_SNDBUF therein
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
    /// * link:man:socket(7) and SO_SNDBUFFORCE therein
    /// * {send}
    /// * {ord}
    pub fn force_set_send_buffer_size(&self, size: usize) -> Result {
        if size > INT_MAX as usize {
            return Err(error::InvalidArgument);
        }
        let val = size as c_int;
        rv!(setsockopt(self.fd, SOL_SOCKET, SO_SNDBUFFORCE, val.as_ref()))
    }

    /// Retrieves whether this socket sends timestamp control messages.
    ///
    /// [return_value]
    /// Returns whether this socket sends timestamp control messages.
    ///
    /// = See also
    ///
    /// * link:man:socket(7) and SO_TIMESTAMPNS therein
    /// * link:lrs::socket::Socket::set_timestamp
    pub fn is_timestamp(&self) -> Result<bool> {
        self.get_bool(SOL_SOCKET, SO_TIMESTAMPNS)
    }

    /// Sets whether this socket sends timestamp control messages.
    ///
    /// [argument, val]
    /// Whether this socket sends timestamp control messages.
    ///
    /// = See also
    ///
    /// * link:man:socket(7) and SO_TIMESTAMPNS therein
    /// * link:lrs::socket::Socket::is_timestamp
    pub fn set_timestamp(&self, val: bool) -> Result {
        self.set_bool(SOL_SOCKET, SO_TIMESTAMPNS, val)
    }

    /// Retrieves the kind of the socket.
    ///
    /// [return_value]
    /// Returns the kind of the socket.
    ///
    /// = See also
    ///
    /// * link:man:socket(7) and SO_TYPE therein
    pub fn kind(&self) -> Result<Kind> {
        let mut val = 0;
        try!(rv!(getsockopt(self.fd, SOL_SOCKET, SO_TYPE, val.as_mut(), &mut 0)));
        Ok(Kind(val))
    }

    /// Retrieves the time the last packet was received.
    ///
    /// [return_value]
    /// Returns the time the last packet was received.
    ///
    /// = Remarks
    ///
    /// This must only be used if the `timestamp` option is not set because otherwise the
    /// return value will be incorrect.
    ///
    /// = See also
    ///
    /// * link:man:socket(7) and SIOCGSTAMPNS therein
    pub fn last_packet(&self) -> Result<Time> {
        let mut val: timespec = mem::zeroed();
        try!(rv!(ioctl_siocgstampns(self.fd, &mut val)));
        Ok(time_base::time_from_timespec(val))
    }

    /// Retrieves the amount of available data.
    ///
    /// [return_value]
    /// Returns the number of bytes available for reading.
    ///
    /// = Remarks
    ///
    /// For streaming sockets, this is the total number of bytes available. For datagram
    /// sockets, this is the size of the next message.
    ///
    /// = See also
    ///
    /// * link:man:socket(7) and SIOCINQ therein
    pub fn pending_input(&self) -> Result<usize> {
        let mut buf = 0;
        try!(rv!(ioctl_siocinq(self.fd, &mut buf)));
        Ok(buf)
    }

    /// Retrieves the amount of data not yet successfully sent.
    ///
    /// [return_value]
    /// Returns the number of bytes stored in the kernel for sending.
    ///
    /// = See also
    ///
    /// * link:man:socket(7) and SIOCOUTQ therein
    pub fn pending_output(&self) -> Result<usize> {
        let mut buf = 0;
        try!(rv!(ioctl_siocoutq(self.fd, &mut buf)));
        Ok(buf)
    }

    /// Retrieves the hop limit of Ipv4 packets sent over this socket.
    ///
    /// [return_value]
    /// Returns the hop limit of Ipv4 packets sent over this socket.
    ///
    /// = Remarks
    ///
    /// This option is only available for Ipv4 sockets.
    ///
    /// = See also
    ///
    /// * link:man:ip(7) and IP_TTL therein
    /// * link:lrs::socket::Socket::ipv4_set_hop_limit
    pub fn ipv4_hop_limit(&self) -> Result<u8> {
        let mut val: c_int = 0;
        try!(rv!(getsockopt(self.fd, IPPROTO_IP, IP_TTL, val.as_mut(), &mut 0)));
        Ok(val as u8)
    }

    /// Sets the hop limit of Ipv4 packets sent over this socket.
    ///
    /// [argument, val]
    /// The hop limit of Ipv4 packets sent over this socket.
    ///
    /// = Remarks
    ///
    /// This option is only available for Ipv4 sockets.
    ///
    /// = See also
    ///
    /// * link:man:ip(7) and IP_TTL therein
    /// * link:lrs::socket::Socket::ipv4_hop_limit
    pub fn ipv4_set_hop_limit(&self, val: u8) -> Result {
        let val = val as c_int;
        rv!(setsockopt(self.fd, IPPROTO_IP, IP_TTL, val.as_ref()))
    }

    /// Retrieves whether packets sent over this socket already have a header included.
    ///
    /// [return_value]
    /// Returns whether packets sent over this socket already have a header included.
    ///
    /// = Remarks
    ///
    /// This option is only available for raw Ipv4 sockets.
    ///
    /// = See also
    ///
    /// * link:man:ip(7) and IP_HDRINCL therein
    /// * link:lrs::socket::Socket::ipv4_set_header_included
    pub fn ipv4_is_header_included(&self) -> Result<bool> {
        self.get_bool(IPPROTO_IP, IP_HDRINCL)
    }

    /// Sets whether packets sent over this socket already have a header included.
    ///
    /// [argument, val]
    /// Whether packets sent over this socket already have a header included.
    ///
    /// = Remarks
    ///
    /// This option is only available for raw Ipv4 sockets.
    ///
    /// = See also
    ///
    /// * link:man:ip(7) and IP_HDRINCL therein
    /// * link:lrs::socket::Socket::ipv4_is_header_included
    pub fn ipv4_set_header_included(&self, val: bool) -> Result {
        self.set_bool(IPPROTO_IP, IP_HDRINCL, val)
    }

    /// Retrieves the Ipv4 options sent with each packet sent over this socket.
    ///
    /// [argument, buf]
    /// The buffer in which the options will be stored.
    ///
    /// [return_value]
    /// Returns the initial part of the buffer that has been filled with the options.
    ///
    /// = Remarks
    ///
    /// The options will use at most 40 bytes.
    ///
    /// = See also
    ///
    /// * link:man:ip(7) and IP_OPTIONS therein
    /// * link::lrs::socket::Socket::ipv4_set_options
    pub fn ipv4_options<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut [u8]> {
        let mut len = 0;
        try!(rv!(getsockopt(self.fd, IPPROTO_IP, IP_OPTIONS, buf.as_mut(), &mut len)));
        Ok(&mut buf[..len])
    }

    /// Sets the Ipv4 options sent with each packet sent over this socket.
    ///
    /// [argument, buf]
    /// The options to be sent on this socket.
    ///
    /// = Remarks
    ///
    /// The buffer should be at most 40 bytes long.
    ///
    /// = See also
    ///
    /// * link:man:ip(7) and IP_OPTIONS therein
    /// * link::lrs::socket::Socket::ipv4_options
    pub fn ipv4_set_options(&self, options: &[u8]) -> Result {
        rv!(setsockopt(self.fd, IPPROTO_IP, IP_OPTIONS, options.as_ref()))
    }

    /// Retrieves whether all received messages are accompanied by an options control
    /// message.
    ///
    /// [return_value]
    /// Returns whether all received messages are accompanied by an options control
    /// message.
    ///
    /// = Remarks
    ///
    /// This is not supported by stream sockets.
    ///
    /// = See also
    ///
    /// * link:man:ip(7) and IP_RECVOPTS therein
    /// * link::lrs::socket::Socket::ipv4_set_receive_options
    pub fn ipv4_is_receive_options(&self) -> Result<bool> {
        self.get_bool(IPPROTO_IP, IP_RECVOPTS)
    }

    /// Sets whether all received messages are accompanied by an options control
    /// message.
    ///
    /// [argument, val]
    /// Whether all received messages are accompanied by an options control message.
    ///
    /// = Remarks
    ///
    /// This is not supported by stream sockets.
    ///
    /// = See also
    ///
    /// * link:man:ip(7) and IP_RECVOPTS therein
    /// * link::lrs::socket::Socket::ipv4_is_receive_options
    pub fn ipv4_set_receive_options(&self, val: bool) -> Result {
        self.set_bool(IPPROTO_IP, IP_RECVOPTS, val)
    }

    /// Retrieves whether all received messages are accompanied by an options control
    /// message.
    ///
    /// [return_value]
    /// Returns whether all received messages are accompanied by an options control
    /// message.
    ///
    /// = Remarks
    ///
    /// This is not supported by stream sockets.
    ///
    /// = See also
    ///
    /// * link:man:ip(7) and IP_RETOPTS therein
    /// * link::lrs::socket::Socket::ipv4_set_receive_raw_options
    pub fn ipv4_is_receive_raw_options(&self) -> Result<bool> {
        self.get_bool(IPPROTO_IP, IP_RETOPTS)
    }

    /// Sets whether all received messages are accompanied by an options control
    /// message.
    ///
    /// [argument, val]
    /// Whether all received messages are accompanied by an options control message.
    ///
    /// = Remarks
    ///
    /// This is not supported by stream sockets.
    ///
    /// = See also
    ///
    /// * link:man:ip(7) and IP_RETOPTS therein
    /// * link::lrs::socket::Socket::ipv4_is_receive_raw_options
    pub fn ipv4_set_receive_raw_options(&self, val: bool) -> Result {
        self.set_bool(IPPROTO_IP, IP_RETOPTS, val)
    }

    /// Retrieves the MTU of this socket.
    ///
    /// [return_value]
    /// Returns the MTU of this socket.
    ///
    /// = Remarks
    ///
    /// Only valid for connected sockets.
    ///
    /// = See also
    ///
    /// * link:man:ip(7) and IP_MTU therein
    pub fn ipv4_mtu(&self) -> Result<usize> {
        let mut val: c_int = 0;
        try!(rv!(getsockopt(self.fd, IPPROTO_IP, IP_MTU, val.as_mut(), &mut 0)));
        Ok(val as usize)
    }

    /// Retrieves the hop limit of Ipv4 multicast packets sent over this socket.
    ///
    /// [return_value]
    /// Returns the hop limit of Ipv4 multicast packets sent over this socket.
    ///
    /// = Remarks
    ///
    /// This option is only available for Ipv4 sockets. The default value is `1` so that
    /// multicast packets don't leave the local network.
    ///
    /// = See also
    ///
    /// * link:man:ip(7) and IP_MULTICAST_TTL therein
    /// * link:lrs::socket::Socket::ipv4_set_multicast_hop_limit
    pub fn ipv4_multicast_hop_limit(&self) -> Result<u8> {
        let mut val: c_int = 0;
        try!(rv!(getsockopt(self.fd, IPPROTO_IP, IP_MULTICAST_TTL, val.as_mut(),
                            &mut 0)));
        Ok(val as u8)
    }

    /// Sets the hop limit of Ipv4 multicast packets sent over this socket.
    ///
    /// [argument, val]
    /// The hop limit of Ipv4 multicast packets sent over this socket.
    ///
    /// = Remarks
    ///
    /// This option is only available for Ipv4 sockets.
    ///
    /// = See also
    ///
    /// * link:man:ip(7) and IP_MULTICAST_TTL therein
    /// * link:lrs::socket::Socket::ipv4_multicast_hop_limit
    pub fn ipv4_set_multicast_hop_limit(&self, val: u8) -> Result {
        let val = val as c_int;
        rv!(setsockopt(self.fd, IPPROTO_IP, IP_MULTICAST_TTL, val.as_ref()))
    }

    /// Retrieves whether multicast packets sent from this socket are looped back to local
    /// sockets.
    ///
    /// [return_value]
    /// Returns whether multicast packets sent from this socket are looped back to local
    /// sockets.
    ///
    /// = Remarks
    ///
    /// The default value is `true`.
    ///
    /// = See also
    ///
    /// * link:man:ip(7) and IP_MULTICAST_LOOP therein
    /// * link:lrs::socket::Socket::ipv4_set_multicast_loop
    pub fn ipv4_is_multicast_loop(&self) -> Result<bool> {
        self.get_bool(IPPROTO_IP, IP_MULTICAST_LOOP)
    }

    /// Sets whether multicast packets sent from this socket are looped back to local
    /// sockets.
    ///
    /// [argument, val]
    /// Whether multicast packets sent from this socket are looped back to local sockets.
    ///
    /// = See also
    ///
    /// * link:man:ip(7) and IP_MULTICAST_LOOP therein
    /// * link:lrs::socket::Socket::ipv4_is_multicast_loop
    pub fn ipv4_set_multicast_loop(&self, val: bool) -> Result {
        self.set_bool(IPPROTO_IP, IP_MULTICAST_LOOP, val)
    }

    fn ipv4_multicast_membership_common(&self, multi_addr: Ipv4Addr,
                                        local_addr: Option<Ipv4Addr>,
                                        interface: Option<c_int>, ty: c_int) -> Result {
        let local_addr = local_addr.unwrap_or(Ipv4Addr::any());
        let interface = interface.unwrap_or(0);
        let mreqn = ip_mreqn {
            imr_multiaddr: in_addr { s_addr: multi_addr.to_be() },
            imr_address: in_addr { s_addr: local_addr.to_be() },
            imr_ifindex: interface,
        };
        rv!(setsockopt(self.fd, IPPROTO_IP, ty, mreqn.as_ref()))
    }

    /// Joins a multicast group.
    ///
    /// [argument, multi_addr]
    /// The address of the multicast group to join.
    ///
    /// [argument, local_addr]
    /// The local address that will join the group.
    ///
    /// [argument, interface]
    /// The local interface that will join the group.
    ///
    /// = Remarks
    ///
    /// If `local_addr` or `interface` are `None`, the system will choose appropriate
    /// values.
    ///
    /// = See also
    ///
    /// * link:man:ip(7) and IP_ADD_MEMBERSHIP therein
    /// * link:lrs::socket::Socket::ipv4_leave_multicast_group
    pub fn ipv4_join_multicast_group(&self, multi_addr: Ipv4Addr,
                                     local_addr: Option<Ipv4Addr>,
                                     interface: Option<c_int>) -> Result {
        self.ipv4_multicast_membership_common(multi_addr, local_addr, interface,
                                              IP_ADD_MEMBERSHIP)
    }

    /// Leaves a multicast group.
    ///
    /// [argument, multi_addr]
    /// The address of the multicast group to leave.
    ///
    /// [argument, local_addr]
    /// The local address that will leave the group.
    ///
    /// [argument, interface]
    /// The local interface that will leave the group.
    ///
    /// = Remarks
    ///
    /// If `local_addr` or `interface` are `None`, the system will find the correct
    /// values.
    ///
    /// = See also
    ///
    /// * link:man:ip(7) and IP_DROP_MEMBERSHIP therein
    /// * link:lrs::socket::Socket::ipv4_join_multicast_group
    pub fn ipv4_leave_multicast_group(&self, multi_addr: Ipv4Addr,
                                      local_addr: Option<Ipv4Addr>,
                                      interface: Option<c_int>) -> Result {
        self.ipv4_multicast_membership_common(multi_addr, local_addr, interface,
                                              IP_DROP_MEMBERSHIP)
    }

    fn ipv4_multicast_source_common(&self, multi_addr: Ipv4Addr,
                                    source_addr: Ipv4Addr,
                                    local_addr: Option<Ipv4Addr>,
                                    ty: c_int) -> Result {
        let local_addr = local_addr.unwrap_or(Ipv4Addr::any());
        let mreq_src = ip_mreq_source {
            imr_multiaddr: multi_addr.to_be(),
            imr_interface: local_addr.to_be(),
            imr_sourceaddr: source_addr.to_be(),
        };
        rv!(setsockopt(self.fd, IPPROTO_IP, ty, mreq_src.as_ref()))
    }

    /// Sets the socket up to accept multicast messages from a particular source.
    ///
    /// [argument, multi_addr]
    /// The address of the multicast group on which the messages are sent.
    ///
    /// [argument, source_addr]
    /// The address of the sender of the messages.
    ///
    /// [argument, local_addr]
    /// The local address that will accept the messages.
    ///
    /// = Remarks
    ///
    /// If `local_addr` is `None`, the system will choose an appropriate value.
    ///
    /// = See also
    ///
    /// * link:man:ip(7) and IP_ADD_SOURCE_MEMBERSHIP therein
    /// * link:lrs::socket::Socket::ipv4_remove_multicast_source
    pub fn ipv4_add_multicast_source(&self, multi_addr: Ipv4Addr,
                                     source_addr: Ipv4Addr,
                                     local_addr: Option<Ipv4Addr>) -> Result {
        self.ipv4_multicast_source_common(multi_addr, source_addr, local_addr,
                                          IP_ADD_SOURCE_MEMBERSHIP)
    }

    /// Removes a previously added source of multicast messages.
    ///
    /// [argument, multi_addr]
    /// The address of the multicast group on which the messages are sent.
    ///
    /// [argument, source_addr]
    /// The address of the sender of the messages.
    ///
    /// [argument, local_addr]
    /// The local address that was set up to accept the messages.
    ///
    /// = Remarks
    ///
    /// :iams: link:lrs::socket::Socket::ipv4_remove_multicast_source[ipv4_add_multicast_source]
    /// :ijmg: link:lrs::socket::Socket::ipv4_join_multicast_group[ipv4_join_multicast_group]
    /// :ibmp: link:lrs::socket::Socket::ipv4_block_multicast_peer[ipv4_block_multicast_peer]
    ///
    /// If `local_addr` is `None`, the system will choose an appropriate value.
    ///
    /// This is the inverse of {iams}. If you want to block messages from a group joined
    /// with {ijmg}, use {ibmp}.
    ///
    /// = See also
    ///
    /// * link:man:ip(7) and IP_DROP_SOURCE_MEMBERSHIP therein
    /// * {iams}
    /// * {ijmg}
    /// * {ibmp}
    pub fn ipv4_remove_multicast_source(&self, multi_addr: Ipv4Addr,
                                        source_addr: Ipv4Addr,
                                        local_addr: Option<Ipv4Addr>) -> Result {
        self.ipv4_multicast_source_common(multi_addr, source_addr, local_addr,
                                          IP_DROP_SOURCE_MEMBERSHIP)
    }

    /// Blocks messages from a peer in a multicast group.
    ///
    /// [argument, multi_addr]
    /// The address of the multicast group on which the messages are sent.
    ///
    /// [argument, peer_addr]
    /// The address of the peer which will be blocked.
    ///
    /// [argument, local_addr]
    /// The local address which is a member of the multicast group.
    ///
    /// = Remarks
    ///
    /// If `local_addr` is `None`, the system will choose an appropriate value.
    ///
    /// = See also
    ///
    /// * link:man:ip(7) and IP_BLOCK_SOURCE therein
    /// * link:lrs::socket::Socket::ipv4_unblock_multicast_peer
    pub fn ipv4_block_multicast_peer(&self, multi_addr: Ipv4Addr,
                                     peer_addr: Ipv4Addr,
                                     local_addr: Option<Ipv4Addr>) -> Result {
        self.ipv4_multicast_source_common(multi_addr, peer_addr, local_addr,
                                          IP_BLOCK_SOURCE)
    }

    /// Unblocks messages from a peer in a multicast group.
    ///
    /// [argument, multi_addr]
    /// The address of the multicast group on which the messages are sent.
    ///
    /// [argument, peer_addr]
    /// The address of the peer which will be unblocked.
    ///
    /// [argument, local_addr]
    /// The local address which is a member of the multicast group.
    ///
    /// = Remarks
    ///
    /// If `local_addr` is `None`, the system will choose an appropriate value.
    ///
    /// = See also
    ///
    /// * link:man:ip(7) and IP_UNBLOCK_SOURCE therein
    /// * link:lrs::socket::Socket::ipv4_block_multicast_peer
    pub fn ipv4_unblock_multicast_peer(&self, multi_addr: Ipv4Addr,
                                       peer_addr: Ipv4Addr,
                                       local_addr: Option<Ipv4Addr>) -> Result {
        self.ipv4_multicast_source_common(multi_addr, peer_addr, local_addr,
                                          IP_UNBLOCK_SOURCE)
    }

    /// Retrieves whether this socket receives all messages sent to any multicast groups
    /// that any socket on this system is a member of.
    ///
    /// [return_value]
    /// Returns whether this socket receives all messages sent to any multicast groups
    /// that any socket on this system is a member of.
    /// = See also
    ///
    /// * link:man:ip(7) and IP_MULTICAST_ALL therein
    pub fn ipv4_is_multicast_all(&self) -> Result<bool> {
        self.get_bool(IPPROTO_IP, IP_MULTICAST_ALL)
    }

    /// Sets whether this socket receives all messages sent to any multicast groups
    /// that any socket on this system is a member of.
    ///
    /// [argument, val]
    /// Whether this socket receives all messages sent to any multicast groups that any
    /// socket on this system is a member of.
    ///
    /// = See also
    ///
    /// * link:man:ip(7) and IP_MULTICAST_ALL therein
    pub fn ipv4_set_multicast_all(&self, val: bool) -> Result {
        self.set_bool(IPPROTO_IP, IP_MULTICAST_ALL, val)
    }

    /// Turns an Ipv6 socket into an Ipv4 socket.
    ///
    /// = Remarks
    ///
    /// This only works if the socket is connected to and bound to Ipv6-mapped Ipv4
    /// addresses.
    ///
    /// = See also
    ///
    /// * link:man:ipv6(7) and IPV6_ADDRFORM therein
    pub fn ipv6_into_ipv4(&self) -> Result {
        let val: c_int = AF_INET;
        rv!(setsockopt(self.fd, IPPROTO_IPV6, IPV6_ADDRFORM, val.as_ref()))
    }

    fn ipv6_multicast_membership_common(&self, multi_addr: Ipv6Addr,
                                        interface: Option<c_int>, ty: c_int) -> Result {
        let interface = interface.unwrap_or(0);
        let mreq = ipv6_mreq {
            ipv6mr_multiaddr: in6_addr { u6_addr16: multi_addr.to_be_bytes() },
            ipv6mr_ifindex: interface,
        };
        rv!(setsockopt(self.fd, IPPROTO_IPV6, ty, mreq.as_ref()))
    }


    /// Joins a multicast group.
    ///
    /// [argument, multi_addr]
    /// The address of the multicast group to join.
    ///
    /// [argument, interface]
    /// The local interface that will join the group.
    ///
    /// = Remarks
    ///
    /// If `interface` is `None`, the system will choose as appropriate value.
    ///
    /// = See also
    ///
    /// * link:man:ipv6(7) and IPV6_ADD_MEMBERSHIP therein
    /// * link:lrs::socket::Socket::ipv6_leave_multicast_group
    pub fn ipv6_join_multicast_group(&self, multi_addr: Ipv6Addr,
                                     interface: Option<c_int>) -> Result {
        self.ipv6_multicast_membership_common(multi_addr, interface,
                                              IPV6_ADD_MEMBERSHIP)
    }

    /// Leaves a multicast group.
    ///
    /// [argument, multi_addr]
    /// The address of the multicast group to leave.
    ///
    /// [argument, interface]
    /// The local interface that will leave the group.
    ///
    /// = Remarks
    ///
    /// If `interface` is `None`, the system will find the correct value.
    ///
    /// = See also
    ///
    /// * link:man:ipv6(7) and IPV6_DROP_MEMBERSHIP therein
    /// * link:lrs::socket::Socket::ipv6_join_multicast_group
    pub fn ipv6_leave_multicast_group(&self, multi_addr: Ipv6Addr,
                                 interface: Option<c_int>) -> Result {
        self.ipv6_multicast_membership_common(multi_addr, interface,
                                              IPV6_DROP_MEMBERSHIP)
    }

    /// Retrieves the MTU of this socket.
    ///
    /// [return_value]
    /// Returns the MTU of this socket.
    ///
    /// = Remarks
    ///
    /// Only valid for connected sockets.
    ///
    /// = See also
    ///
    /// * link:man:ip(7) and IP_MTU therein
    pub fn ipv6_mtu(&self) -> Result<u32> {
        let mut mtu: c_int = 0;
        try!(rv!(getsockopt(self.fd, IPPROTO_IPV6, IPV6_MTU, mtu.as_mut(), &mut 0)));
        Ok(mtu as u32)
    }

    /// Sets the MTU of this socket.
    ///
    /// [argument, val]
    /// The MTU of this socket.
    ///
    /// = See also
    ///
    /// * link:man:ipv6(7) and IPV6_MTU therein
    pub fn ipv6_set_mtu(&self, val: u32) -> Result {
        let mtu = val as c_int;
        rv!(setsockopt(self.fd, IPPROTO_IPV6, IPV6_MTU, mtu.as_ref()))
    }

    /// Retrieves the hop limit of multicast packets sent over this socket.
    ///
    /// [return_value]
    /// Returns the hop limit of multicast packets sent over this socket.
    ///
    /// = Remarks
    ///
    /// A `None` return value means that the default value is used.
    ///
    /// = See also
    ///
    /// * link:man:ipv6(7) and IPV6_MULTICAST_HOPS therein
    /// * link:lrs::socket::Socket::ipv6_set_multicast_hop_limit
    pub fn ipv6_multicast_hop_limit(&self) -> Result<Option<u8>> {
        let mut val: c_int = 0;
        try!(rv!(getsockopt(self.fd, IPPROTO_IPV6, IPV6_MULTICAST_HOPS, val.as_mut(),
                            &mut 0)));
        if val == -1 {
            Ok(None)
        } else {
            Ok(Some(val as u8))
        }
    }

    /// Sets the hop limit of multicast packets sent over this socket.
    ///
    /// [argument, val]
    /// The hop limit of multicast packets sent over this socket.
    ///
    /// = Remarks
    ///
    /// A `None` argument means that the default value is used.
    ///
    /// = See also
    ///
    /// * link:man:ipv6(7) and IPV6_MULTICAST_HOPS therein
    /// * link:lrs::socket::Socket::ipv6_multicast_hop_limit
    pub fn ipv6_set_multicast_hop_limit(&self, val: Option<u8>) -> Result {
        let val = val.map(|v| v as c_int).unwrap_or(-1);
        rv!(setsockopt(self.fd, IPPROTO_IPV6, IPV6_MULTICAST_HOPS, val.as_ref()))
    }

    /// Retrieves whether multicast packets sent from this socket are looped back to local
    /// sockets.
    ///
    /// [return_value]
    /// Returns whether multicast packets sent from this socket are looped back to local
    /// sockets.
    ///
    /// = Remarks
    ///
    /// The default value is `true`.
    ///
    /// = See also
    ///
    /// * link:man:ipv6(7) and IPV6_MULTICAST_LOOP therein
    /// * link:lrs::socket::Socket::ipv6_set_multicast_loop
    pub fn ipv6_is_multicast_loop(&self) -> Result<bool> {
        self.get_bool(IPPROTO_IPV6, IPV6_MULTICAST_LOOP)
    }

    /// Sets whether multicast packets sent from this socket are looped back to local
    /// sockets.
    ///
    /// [argument, val]
    /// Whether multicast packets sent from this socket are looped back to local sockets.
    ///
    /// = See also
    ///
    /// * link:man:ipv6(7) and IPV6_MULTICAST_LOOP therein
    /// * link:lrs::socket::Socket::ipv6_is_multicast_loop
    pub fn ipv6_set_multicast_loop(&self, val: bool) -> Result {
        self.set_bool(IPPROTO_IPV6, IPV6_MULTICAST_LOOP, val)
    }

    /// Retrieves the hop limit of packets sent over this socket.
    ///
    /// [return_value]
    /// Returns the hop limit of packets sent over this socket.
    ///
    /// = See also
    ///
    /// * link:man:ipv6(7) and IP_UNICAST_HOPS therein
    /// * link:lrs::socket::Socket::ipv6_set_hop_limit
    pub fn ipv6_hop_limit(&self) -> Result<Option<u8>> {
        let mut val: c_int = 0;
        try!(rv!(getsockopt(self.fd, IPPROTO_IPV6, IPV6_UNICAST_HOPS, val.as_mut(),
                            &mut 0)));
        if val == -1 {
            Ok(None)
        } else {
            Ok(Some(val as u8))
        }
    }

    /// Sets the hop limit of packets sent over this socket.
    ///
    /// [argument, val]
    /// The hop limit of packets sent over this socket.
    ///
    /// = See also
    ///
    /// * link:man:ipv6(7) and IPV6_UNICAST_HOPS therein
    /// * link:lrs::socket::Socket::ipv6_hop_limit
    pub fn ipv6_set_hop_limit(&self, val: Option<u8>) -> Result {
        let val = val.map(|v| v as c_int).unwrap_or(-1);
        rv!(setsockopt(self.fd, IPPROTO_IPV6, IPV6_UNICAST_HOPS, val.as_ref()))
    }

    /// Retrieves whether this socket only handles Ipv6 packets.
    ///
    /// [return_value]
    /// Returns whether this socket only handles Ipv6 packets.
    ///
    /// = See also
    ///
    /// * link:man:ipv6(7) and IPV6_V6ONLY therein
    /// * link:lrs::socket::Socket::ipv6_set_ipv6_only
    pub fn ipv6_is_ipv6_only(&self) -> Result<bool> {
        self.get_bool(IPPROTO_IPV6, IPV6_V6ONLY)
    }

    /// Sets whether this socket only handles Ipv6 packets.
    ///
    /// [argument, val]
    /// Whether this socket only handles Ipv6 packets.
    ///
    /// = Remarks
    ///
    /// If this option is set, then the socket will not send or receive Ipv4 packets.
    /// Otherwise, the kernel transparently translates between Ipv6 and Ipv4 packets for
    /// us. If this option is set, an Ipv4 and an Ipv6 socket can bind to the same port.
    ///
    /// = See also
    ///
    /// * link:man:ipv6(7) and IPV6_V6ONLY therein
    /// * link:lrs::socket::Socket::ipv6_is_ipv6_only
    pub fn ipv6_set_ipv6_only(&self, val: bool) -> Result {
        self.set_bool(IPPROTO_IPV6, IPV6_V6ONLY, val)
    }

    /// Retrieves the status of the `tcp_cork` option.
    ///
    /// [return_value]
    /// Returns the status of the `tcp_cork` option.
    ///
    /// = See also
    ///
    /// * link:man:tcp(7) and TCP_CORK therein
    /// * link:lrs::socket::Socket::tcp_set_cork
    pub fn tcp_is_cork(&self) -> Result<bool> {
        self.get_bool(IPPROTO_TCP, TCP_CORK)
    }

    /// Sets the status of the `tcp_cork` option.
    ///
    /// [argument, val]
    /// The status of the `tcp_cork` option.
    ///
    /// = See also
    ///
    /// * link:man:tcp(7) and TCP_CORK therein
    /// * link:lrs::socket::Socket::tcp_is_cork
    pub fn tcp_set_cork(&self, val: bool) -> Result {
        self.set_bool(IPPROTO_TCP, TCP_CORK, val)
    }

    /// Retrieves the status of the `udp_cork` option.
    ///
    /// [return_value]
    /// Returns the status of the `udp_cork` option.
    ///
    /// = See also
    ///
    /// * link:man:UDP(7) and UDP_CORK therein
    /// * link:lrs::socket::Socket::UDP_set_cork
    pub fn udp_is_cork(&self) -> Result<bool> {
        self.get_bool(IPPROTO_UDP, UDP_CORK)
    }

    /// Sets the status of the `udp_cork` option.
    ///
    /// [argument, val]
    /// The status of the `udp_cork` option.
    ///
    /// = See also
    ///
    /// * link:man:udp(7) and UDP_CORK therein
    /// * link:lrs::socket::Socket::udp_is_cork
    pub fn udp_set_cork(&self, val: bool) -> Result {
        self.set_bool(IPPROTO_UDP, UDP_CORK, val)
    }
}

unsafe impl UndefState for Socket {
    fn num() -> usize { bool::num() }

    unsafe fn set_undef(val: *mut Socket, n: usize) {
        bool::set_undef(&mut (*val).owned, n);
    }

    unsafe fn is_undef(val: *const Socket, n: usize) -> bool {
        bool::is_undef(&(*val).owned, n)
    }
}

/// = Remarks
///
/// This closes the socket, ignoring all errors.
impl Drop for Socket {
    fn drop(&mut self) {
        if self.owned {
            close(self.fd);
        }
    }
}

impl Into<c_int> for Socket {
    fn into(self) -> c_int {
        let fd = self.fd;
        mem::forget(self);
        fd
    }
}

impl FdContainer for Socket {
    fn is_owned(&self) -> bool { self.owned }
    fn borrow(&self) -> c_int { self.fd }
    fn from_owned(fd: c_int) -> Socket { Socket { fd: fd, owned: true } }
    fn from_borrowed(fd: c_int) -> Socket { Socket { fd: fd, owned: false } }
}
