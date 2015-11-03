// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::{mem};
use kind::{Kind};
use cty::{
    RTM_NEWLINK, RTM_DELLINK, RTM_GETLINK,
    RTM_NEWADDR, RTM_DELADDR, RTM_GETADDR,
    RTM_NEWROUTE, RTM_DELROUTE, RTM_GETROUTE,
    RTM_NEWNEIGH, RTM_DELNEIGH, RTM_GETNEIGH,
    RTM_NEWRULE, RTM_DELRULE, RTM_GETRULE,
    RTM_NEWQDISC, RTM_DELQDISC, RTM_GETQDISC,
    RTM_NEWTCLASS, RTM_DELTCLASS, RTM_GETTCLASS,
    RTM_NEWTFILTER, RTM_DELTFILTER, RTM_GETTFILTER,
};

pub const NewLink          : Kind = Kind(RTM_NEWLINK);
pub const DelLink          : Kind = Kind(RTM_DELLINK);
pub const GetLink          : Kind = Kind(RTM_GETLINK);
pub const NewAddr          : Kind = Kind(RTM_NEWADDR);
pub const DelAddr          : Kind = Kind(RTM_DELADDR);
pub const GetAddr          : Kind = Kind(RTM_GETADDR);
pub const NewRoute         : Kind = Kind(RTM_NEWROUTE);
pub const DelRoute         : Kind = Kind(RTM_DELROUTE);
pub const GetRoute         : Kind = Kind(RTM_GETROUTE);
pub const NewNeigh         : Kind = Kind(RTM_NEWNEIGH);
pub const DelNeigh         : Kind = Kind(RTM_DELNEIGH);
pub const GetNeigh         : Kind = Kind(RTM_GETNEIGH);
pub const NewRule          : Kind = Kind(RTM_NEWRULE);
pub const DelRule          : Kind = Kind(RTM_DELRULE);
pub const GetRule          : Kind = Kind(RTM_GETRULE);
pub const NewQueueDisc     : Kind = Kind(RTM_NEWQDISC);
pub const DelQueueDisc     : Kind = Kind(RTM_DELQDISC);
pub const GetQueueDisc     : Kind = Kind(RTM_GETQDISC);
pub const NewClass         : Kind = Kind(RTM_NEWTCLASS);
pub const DelClass         : Kind = Kind(RTM_DELTCLASS);
pub const GetClass         : Kind = Kind(RTM_GETTCLASS);
pub const NewTrafficFilter : Kind = Kind(RTM_NEWTFILTER);
pub const DelTrafficFilter : Kind = Kind(RTM_DELTFILTER);
pub const GetTrafficFilter : Kind = Kind(RTM_GETTFILTER);

#[repr(C)]
#[derive(Pod, Eq)]
pub struct IfInfoMsg {
    pub family: u8,
    pub ty:     u16,
    pub index:  i32,
    pub flags:  u32,
    pub change: u32,
}

impl AsRef<[u8]> for IfInfoMsg {
    fn as_ref(&self) -> &[u8] {
        mem::as_bytes(self)
    }
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct IfAddrMsg {
    pub family: u8,
    pub prefix: u8,
    pub flags:  u8,
    pub scope:  u8,
    pub index:  u32,
}

impl AsRef<[u8]> for IfAddrMsg {
    fn as_ref(&self) -> &[u8] {
        mem::as_bytes(self)
    }
}

#[repr(C)]
#[derive(Pod, Eq)]
pub struct RouteMsg {
    pub family:   u8,
    pub dst_len:  u8,
    pub src_len:  u8,
    pub tos:      u8,
    pub table:    u8,
    pub protocol: u8,
    pub scope:    u8,
    pub ty:       u8,
    pub flags:    u32,
}

impl AsRef<[u8]> for RouteMsg {
    fn as_ref(&self) -> &[u8] {
        mem::as_bytes(self)
    }
}

pub mod link_attr {
    use cty::{
        IFLA_UNSPEC, IFLA_ADDRESS, IFLA_BROADCAST, IFLA_IFNAME, IFLA_MTU, IFLA_LINK,
        IFLA_QDISC, IFLA_STATS, IFLA_COST, IFLA_PRIORITY, IFLA_MASTER, IFLA_WIRELESS,
        IFLA_PROTINFO, IFLA_TXQLEN, IFLA_MAP, IFLA_WEIGHT, IFLA_OPERSTATE, IFLA_LINKMODE,
        IFLA_LINKINFO, IFLA_NET_NS_PID, IFLA_IFALIAS, IFLA_NUM_VF, IFLA_VFINFO_LIST,
        IFLA_STATS64, IFLA_VF_PORTS, IFLA_PORT_SELF, IFLA_AF_SPEC, IFLA_GROUP,
        IFLA_NET_NS_FD, IFLA_EXT_MASK, IFLA_PROMISCUITY, IFLA_NUM_TX_QUEUES,
        IFLA_NUM_RX_QUEUES, IFLA_CARRIER, IFLA_PHYS_PORT_ID, IFLA_CARRIER_CHANGES,
        IFLA_PHYS_SWITCH_ID, IFLA_LINK_NETNSID,
    };

    pub const Uspec          : u16 = IFLA_UNSPEC;
    pub const Address        : u16 = IFLA_ADDRESS;
    pub const Broadcast      : u16 = IFLA_BROADCAST;
    pub const IfName         : u16 = IFLA_IFNAME;
    pub const Mtu            : u16 = IFLA_MTU;
    pub const Link           : u16 = IFLA_LINK;
    pub const QDisk          : u16 = IFLA_QDISC;
    pub const Stats          : u16 = IFLA_STATS;
    pub const Cost           : u16 = IFLA_COST;
    pub const Priority       : u16 = IFLA_PRIORITY;
    pub const Master         : u16 = IFLA_MASTER;
    pub const Wireless       : u16 = IFLA_WIRELESS;
    pub const ProtInfo       : u16 = IFLA_PROTINFO;
    pub const TqxLen         : u16 = IFLA_TXQLEN;
    pub const Map            : u16 = IFLA_MAP;
    pub const Weight         : u16 = IFLA_WEIGHT;
    pub const Operstate      : u16 = IFLA_OPERSTATE;
    pub const LinkMode       : u16 = IFLA_LINKMODE;
    pub const LinkInfo       : u16 = IFLA_LINKINFO;
    pub const NetNsPid       : u16 = IFLA_NET_NS_PID;
    pub const IfAlias        : u16 = IFLA_IFALIAS;
    pub const NumVf          : u16 = IFLA_NUM_VF;
    pub const VfinfoList     : u16 = IFLA_VFINFO_LIST;
    pub const Stats64        : u16 = IFLA_STATS64;
    pub const VfPorts        : u16 = IFLA_VF_PORTS;
    pub const PortSelf       : u16 = IFLA_PORT_SELF;
    pub const AfSpec         : u16 = IFLA_AF_SPEC;
    pub const Group          : u16 = IFLA_GROUP;
    pub const NetNsFd        : u16 = IFLA_NET_NS_FD;
    pub const ExtMask        : u16 = IFLA_EXT_MASK;
    pub const Promiscuity    : u16 = IFLA_PROMISCUITY;
    pub const NumTxQueues    : u16 = IFLA_NUM_TX_QUEUES;
    pub const NumRxQueues    : u16 = IFLA_NUM_RX_QUEUES;
    pub const Carrier        : u16 = IFLA_CARRIER;
    pub const PhysPortId     : u16 = IFLA_PHYS_PORT_ID;
    pub const CarrierChanges : u16 = IFLA_CARRIER_CHANGES;
    pub const PhysSwitchId   : u16 = IFLA_PHYS_SWITCH_ID;
    pub const LinkNetnsid    : u16 = IFLA_LINK_NETNSID;
}

pub mod link_info {
    pub use cty::{
        IFLA_INFO_UNSPEC, IFLA_INFO_KIND, IFLA_INFO_DATA, IFLA_INFO_XSTATS,
        IFLA_INFO_SLAVE_KIND, IFLA_INFO_SLAVE_DATA,
    };

    pub const Unspec    : u16 = IFLA_INFO_UNSPEC;
    pub const Kind      : u16 = IFLA_INFO_KIND;
    pub const Data      : u16 = IFLA_INFO_DATA;
    pub const XStats    : u16 = IFLA_INFO_XSTATS;
    pub const SlaveKind : u16 = IFLA_INFO_SLAVE_KIND;
    pub const SlaveData : u16 = IFLA_INFO_SLAVE_DATA;
}

pub mod addr_attr {
    pub use cty::{
        IFA_UNSPEC, IFA_ADDRESS, IFA_LOCAL, IFA_LABEL, IFA_BROADCAST, IFA_ANYCAST,
        IFA_CACHEINFO, IFA_MULTICAST, IFA_FLAGS,
    };

    pub const Unspec    : u16 = IFA_UNSPEC;
    pub const Address   : u16 = IFA_ADDRESS;
    pub const Local     : u16 = IFA_LOCAL;
    pub const Label     : u16 = IFA_LABEL;
    pub const Broadcast : u16 = IFA_BROADCAST;
    pub const Anycast   : u16 = IFA_ANYCAST;
    pub const CacheInfo : u16 = IFA_CACHEINFO;
    pub const Multicast : u16 = IFA_MULTICAST;
    pub const Flags     : u16 = IFA_FLAGS;
}

pub mod veth_info {
    pub use cty::{VETH_INFO_UNSPEC, VETH_INFO_PEER};

    pub const Unspec : u16 = VETH_INFO_UNSPEC;
    pub const Peer   : u16 = VETH_INFO_PEER;
}
