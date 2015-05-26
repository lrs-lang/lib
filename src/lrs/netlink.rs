// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Netlink sockets.

pub use lrs_netlink::kind::{Kind};
pub use lrs_netlink::flags::{NlFlags};
pub use lrs_socket::nl_proto::{Proto};
pub use lrs_netlink::{MsgHeader, MsgError, Attr};

pub mod route {
    pub use lrs_netlink::route::{
        IfInfoMsg, IfAddrMsg, RouteMsg,
    };

    pub mod op {
        pub use lrs_netlink::route::{
            NewLink, DelLink, GetLink, NewAddr, DelAddr, GetAddr, NewRoute, DelRoute,
            GetRoute, NewNeigh, DelNeigh, GetNeigh, NewRule, DelRule, GetRule,
            NewQueueDisc, DelQueueDisc, GetQueueDisc, NewClass, DelClass, GetClass,
            NewTrafficFilter, DelTrafficFilter, GetTrafficFilter,
        };
    }

    pub mod link_attr {
        pub use lrs_netlink::route::link_attr::{
            Uspec, Address, Broadcast, IfName, Mtu, Link, QDisk, Stats, Cost, Priority,
            Master, Wireless, ProtInfo, TqxLen, Map, Weight, Operstate, LinkMode,
            LinkInfo, NetNsPid, IfAlias, NumVf, VfinfoList, Stats64, VfPorts, PortSelf,
            AfSpec, Group, NetNsFd, ExtMask, Promiscuity, NumTxQueues, NumRxQueues,
            Carrier, PhysPortId, CarrierChanges, PhysSwitchId, LinkNetnsid,
        };
    }

    pub mod link_info {
        pub use lrs_netlink::route::link_info::{
            Unspec, Kind, Data, XStats, SlaveKind, SlaveData,
        };
    }

    pub mod addr_attr {
        pub use lrs_netlink::route::addr_attr::{
            Unspec, Address, Local, Label, Broadcast, Anycast, CacheInfo, Multicast,
            Flags,
        };
    }

    pub mod veth_info {
        pub use lrs_netlink::route::veth_info::{
            Unspec, Peer,
        };
    }
}

pub mod fmt {
    pub use lrs_netlink::fmt::{
        NlBuf, NlMsg, NlAttr,
    };
}

pub mod parse {
    pub use lrs_netlink::parse::{
        MsgIter, MsgParser,
    };
}

pub mod kind {
    pub use lrs_netlink::kind::{
        NoOp, ErrorAck, Done,
    };
}

pub mod flags {
    pub use lrs_netlink::flags::{
        NLF_REQUEST, NLF_MULTI, NLF_ACK, NLF_ECHO, NLF_ROOT, NLF_ATOMIC, NLF_REPLACE,
        NLF_EXCL, NLF_CREATE, NLF_APPEND,
    };
}

/// Netlink protocol constants.
///
/// = Description
///
/// This module contains protocols for use over Netlink.
pub mod proto {
    pub use lrs_socket::nl_proto::{
        Route, UserSock, Firewall, InetDiag, NetfilterLog, Xfrm, SeLinux, IScsi,
        Audit, FibLookup, Connector, Netfilter, Ipv6Fw, Dnrtmsg, KobjectUevent,
        Generic, ScsiTransport, EcryptFs, Rdma, Crypto,
    };
}
