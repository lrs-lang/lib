// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
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
/// [field, 1]
/// The integer constant associated with the domain.
///
/// = Remarks
///
/// This is a protocol family understood by the kernel. For direct access to the link
/// layer (TCP/IP model) use the `Packet` domain. See also `packet(7)`.
///
/// :domains: link:lrs::socket::domain
///
/// See {domains} for pre-defined constants.
///
/// = See also
///
/// * {domains}
#[derive(Pod, Eq)]
pub struct Domain(pub c_int);

macro_rules! create {
    ($($(#[$meta:meta])* domain $name:ident = $val:expr;)*) => {
        $($(#[$meta])* pub const $name: Domain = Domain($val);)*

        impl Debug for Domain {
            fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
                let s = match *self {
                    $($name => stringify!($name),)*
                    _ => return write!(w, "Unknown({})", self.0),
                };
                w.write_all(s.as_bytes()).ignore_ok()
            }
        }
    }
}

create! {
    #[doc = "Unspecified domain"]
    domain Unspecified = AF_UNSPEC;
    #[doc = "Unix domain\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:unix(7)"]
    domain Unix = AF_UNIX;
    #[doc = "Ipv4\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:ip(7)"]
    domain Ipv4 = AF_INET;
    #[doc = "Amateur radio AX.25"]
    domain Ax25 = AF_AX25;
    #[doc = "Internetwork Packet Exchange"]
    domain Ipx = AF_IPX;
    #[doc = "AppleTalk\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:ddp(7)"]
    domain AppleTalk = AF_APPLETALK;
    #[doc = "Amateur radio NET/ROM"]
    domain NetRom = AF_NETROM;
    #[doc = "Multiprotocol bridge"]
    domain Bridge = AF_BRIDGE;
    #[doc = "ATM PVCs"]
    domain AtmPvc = AF_ATMPVC;
    #[doc = "X.25 interface\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:x25(7)"]
    domain X25 = AF_X25;
    #[doc = "Ipv6\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:ipv6(7)"]
    domain Ipv6 = AF_INET6;
    #[doc = "Amateur radio X.25 PLP"]
    domain Rose = AF_ROSE;
    #[doc = "Decnet"]
    domain Decnet = AF_DECnet;
    #[doc = "NetBEUI"]
    domain NetBeui = AF_NETBEUI;
    #[doc = "Security callback pseudo domain"]
    domain Security = AF_SECURITY;
    #[doc = "Security association database interface"]
    domain Key = AF_KEY;
    #[doc = "Kernel/Userspace communication\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:netlink(7)"]
    domain Netlink = AF_NETLINK;
    #[doc = "Raw socket"]
    domain Packet = AF_PACKET;
    #[doc = "Ash"]
    domain Ash = AF_ASH;
    #[doc = "Acorn Econet"]
    domain Econet = AF_ECONET;
    #[doc = "ATM SVCs"]
    domain AtmSvc = AF_ATMSVC;
    #[doc = "RDS"]
    domain Rds = AF_RDS;
    #[doc = "Linux SNA Project (nutters!)"]
    domain Sna = AF_SNA;
    #[doc = "IRDA"]
    domain Irda = AF_IRDA;
    #[doc = "PPPoX"]
    domain Pppox = AF_PPPOX;
    #[doc = "Wanpipe API"]
    domain Wanpipe = AF_WANPIPE;
    #[doc = "Linux LLC"]
    domain Llc = AF_LLC;
    #[doc = "Native InfiniBand address"]
    domain Ib = AF_IB;
    #[doc = "Controller Area Network"]
    domain Can = AF_CAN;
    #[doc = "TIPC sockets"]
    domain Tipc = AF_TIPC;
    #[doc = "Bluetooth"]
    domain Bluetooth = AF_BLUETOOTH;
    #[doc = "IUCV"]
    domain Iucv = AF_IUCV;
    #[doc = "RxRPC"]
    domain Rxrpc = AF_RXRPC;
    #[doc = "mISDN"]
    domain Isdn = AF_ISDN;
    #[doc = "Phonet"]
    domain Phonet = AF_PHONET;
    #[doc = "IEEE802154"]
    domain Ieee802154 = AF_IEEE802154;
    #[doc = "CAIF"]
    domain Caif = AF_CAIF;
    #[doc = "Algorithm"]
    domain Alg = AF_ALG;
    #[doc = "NFC"]
    domain Nfc = AF_NFC;
    #[doc = "vSockets"]
    domain Vsock = AF_VSOCK;
}
