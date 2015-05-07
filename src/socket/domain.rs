// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use cty::{
    c_int,
    AF_UNSPEC, AF_UNIX, AF_INET, AF_AX25, AF_IPX, AF_APPLETALK, AF_NETROM, AF_BRIDGE,
    AF_ATMPVC, AF_X25, AF_INET6, AF_ROSE, AF_DECnet, AF_NETBEUI, AF_SECURITY, AF_KEY,
    AF_NETLINK, AF_PACKET, AF_ASH, AF_ECONET, AF_ATMSVC, AF_RDS, AF_SNA, AF_IRDA,
    AF_PPPOX, AF_WANPIPE, AF_LLC, AF_IB, AF_CAN, AF_TIPC, AF_BLUETOOTH, AF_IUCV, AF_RXRPC,
    AF_ISDN, AF_PHONET, AF_IEEE802154, AF_CAIF, AF_ALG, AF_NFC, AF_VSOCK,
};
use fmt::{Debug, Write};

/// A socket domain/protocol family
///
/// This is a protocol family understood by the kernel. For direct access to the link
/// layer (TCP/IP model) use the `Packet` domain. See also `packet(7)`.
#[derive(Pod, Eq)]
pub struct Domain(pub c_int);

macro_rules! create {
    ($($name:ident = $val:expr, $doc:expr,)*) => {
        $(#[doc = $doc] pub const $name: Domain = Domain($val);)*

        impl Debug for Domain {
            fn fmt<W: Write>(&self, w: &mut W) -> Result {
                let s = match *self {
                    $($name => stringify!($name),)*
                    _ => "Unknown domain",
                };
                w.write_all(s.as_bytes()).ignore_ok()
            }
        }
    }
}

create! {
    Unspecified = AF_UNSPEC     , "Unspecified domain",
    Unix        = AF_UNIX       , "Unix domain",
    Ipv4        = AF_INET       , "Ipv4",
    Ax25        = AF_AX25       , "Amateur radio AX.25",
    Ipx         = AF_IPX        , "Internetwork Packet Exchange",
    AppleTalk   = AF_APPLETALK  , "AppleTalk",
    NetRom      = AF_NETROM     , "Amateur radio NET/ROM",
    Bridge      = AF_BRIDGE     , "Multiprotocol bridge",
    AtmPvc      = AF_ATMPVC     , "ATM PVCs",
    X25         = AF_X25        , "X.25 interface",
    Ipv6        = AF_INET6      , "Ipv6",
    Rose        = AF_ROSE       , "Amateur radio X.25 PLP",
    Decnet      = AF_DECnet     , "Decnet",
    NetBeui     = AF_NETBEUI    , "NetBEUI",
    Security    = AF_SECURITY   , "Security callback pseudo domain",
    Key         = AF_KEY        , "Security association database interface",
    Netlink     = AF_NETLINK    , "Kernel/Userspace communication",
    Packet      = AF_PACKET     , "Raw socket",
    Ash         = AF_ASH        , "Ash",
    Econet      = AF_ECONET     , "Acorn Econet",
    AtmSvc      = AF_ATMSVC     , "ATM SVCs",
    Rds         = AF_RDS        , "RDS",
    Sna         = AF_SNA        , "Linux SNA Project (nutters!)",
    Irda        = AF_IRDA       , "IRDA",
    Pppox       = AF_PPPOX      , "PPPoX",
    Wanpipe     = AF_WANPIPE    , "Wanpipe API",
    Llc         = AF_LLC        , "Linux LLC",
    Ib          = AF_IB         , "Native InfiniBand address",
    Can         = AF_CAN        , "Controller Area Network",
    Tipc        = AF_TIPC       , "TIPC sockets",
    Bluetooth   = AF_BLUETOOTH  , "Bluetooth",
    Iucv        = AF_IUCV       , "IUCV",
    Rxrpc       = AF_RXRPC      , "RxRPC",
    Isdn        = AF_ISDN       , "mISDN",
    Phonet      = AF_PHONET     , "Phonet",
    Ieee802154  = AF_IEEE802154 , "IEEE802154",
    Caif        = AF_CAIF       , "CAIF",
    Alg         = AF_ALG        , "Algorithm",
    Nfc         = AF_NFC        , "NFC",
    Vsock       = AF_VSOCK      , "vSockets",
}
