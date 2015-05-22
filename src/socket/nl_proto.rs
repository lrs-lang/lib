// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use fmt::{Debug, Write};
use cty::{
    c_int,
    NETLINK_ROUTE, NETLINK_USERSOCK, NETLINK_FIREWALL, NETLINK_SOCK_DIAG, NETLINK_NFLOG,
    NETLINK_XFRM, NETLINK_SELINUX, NETLINK_ISCSI, NETLINK_AUDIT, NETLINK_FIB_LOOKUP,
    NETLINK_CONNECTOR, NETLINK_NETFILTER, NETLINK_IP6_FW, NETLINK_DNRTMSG,
    NETLINK_KOBJECT_UEVENT, NETLINK_GENERIC, NETLINK_SCSITRANSPORT, NETLINK_ECRYPTFS,
    NETLINK_RDMA, NETLINK_CRYPTO,
};

/// A Netlink protocol.
///
/// [field, 1]
/// The integer constant associated with the protocol.
///
/// = Remarks
///
/// :nlproto: link:lrs::socket::netlink::proto
///
/// See {nlproto} for pre-defined constants.
///
/// = See also
///
/// * {nlproto}
#[derive(Pod, Eq)]
pub struct Proto(pub c_int);

macro_rules! create {
    ($($name:ident = $val:expr, $doc:expr;)*) => {
        $(#[doc = $doc] pub const $name: Proto = Proto($val);)*

        impl Debug for Proto {
            fn fmt<W: Write>(&self, w: &mut W) -> Result {
                let s = match *self {
                    $($name => stringify!($name),)*
                    _ => "Unknown protocol",
                };
                w.write_all(s.as_bytes()).ignore_ok()
            }
        }
    }
}

create! {
    Route         = NETLINK_ROUTE          , "todo";
    UserSock      = NETLINK_USERSOCK       , "todo";
    Firewall      = NETLINK_FIREWALL       , "todo";
    InetDiag      = NETLINK_SOCK_DIAG      , "todo";
    NetfilterLog  = NETLINK_NFLOG          , "todo";
    Xfrm          = NETLINK_XFRM           , "todo";
    SeLinux       = NETLINK_SELINUX        , "todo";
    IScsi         = NETLINK_ISCSI          , "todo";
    Audit         = NETLINK_AUDIT          , "todo";
    FibLookup     = NETLINK_FIB_LOOKUP     , "todo";
    Connector     = NETLINK_CONNECTOR      , "todo";
    Netfilter     = NETLINK_NETFILTER      , "todo";
    Ipv6Fw        = NETLINK_IP6_FW         , "todo";
    Dnrtmsg       = NETLINK_DNRTMSG        , "todo";
    KobjectUevent = NETLINK_KOBJECT_UEVENT , "todo";
    Generic       = NETLINK_GENERIC        , "todo";
    ScsiTransport = NETLINK_SCSITRANSPORT  , "todo";
    EcryptFs      = NETLINK_ECRYPTFS       , "todo";
    Rdma          = NETLINK_RDMA           , "todo";
    Crypto        = NETLINK_CRYPTO         , "todo";
}
