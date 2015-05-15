// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use lrs_socket::domain::{Domain};
pub use lrs_socket::kind::{Kind};
pub use lrs_socket::flags::{Flags};
pub use lrs_socket::socket::{Socket};
pub use lrs_socket::addr::{SockAddr, AddrType};

/// Kernel domain constants
///
/// = Description
///
/// This module contains domain constants understood by the kernel.
pub mod domain {
    pub use lrs_socket::domain::{
        Unspecified, Unix, Ipv4, Ax25, Ipx, AppleTalk, NetRom, Bridge, AtmPvc, X25, Ipv6,
        Rose, Decnet, NetBeui, Security, Key, Netlink, Packet, Ash, Econet, AtmSvc, Rds,
        Sna, Irda, Pppox, Wanpipe, Llc, Ib, Can, Tipc, Bluetooth, Iucv, Rxrpc, Isdn,
        Phonet, Ieee802154, Caif, Alg, Nfc, Vsock,
    };
}

/// Ip sockets.
pub mod ip {
    pub use lrs_socket::ip_proto::{Proto};
    pub use lrs_socket::addr::ipv4::{Ipv4Addr, Ipv4SockAddr};
    pub use lrs_socket::addr::ipv6::{Ipv6Addr, Ipv6SockAddr, Ipv6Scope};

    /// IP protocol constants.
    ///
    /// = Description
    ///
    /// This module contains IANA-registered protocols for use over IP.
    pub mod proto {
        pub use lrs_socket::ip_proto::{
            HopByHop, InternetControlMessage, InternetGroupManagement, GatewayToGateway,
            Ipv4Encapsulation, Stream, Tcp, Cbt, ExteriorGateway, InteriorGateway,
            BbnRccMonitoring, NetworkVoice, Pup, Argus, Emcon, CrossNetDebugger, Chaos,
            Udp, Multiplexing, Dcn, HostMonitoring, PacketRadioMeasurement, XeroxIdp,
            Trunk1, Trunk2, Leaf1, Leaf2, ReliableData, InternetReliableTransaction, Iso4,
            BulkDataTransfer, MfeNetworkServices, MeritInternodal,
            DatagramCongestionControl, ThirdPartyConnect, InterDomainPolicyRouting, Xtp,
            DatagramDelivery, IdprControlMessageTransport, TpPPTransport, IlTransport,
            Ipv6Encapsulation, SourceDemandRouting, Ipv6Routing, Ipv6Fragment,
            InterDomainRouting, Reservation, GenericRoutingEncapsulation,
            DynamicSourceRouting, Bna, EncapSecurityPayload, AuthenticationHeader,
            IntegratedNetLayerSecurity, Swipe, NbmaAddressResolution, IpMobility,
            TransportLayerSecurity, Skip, Ipv6InternetControlMessage, Ipv6NoNextHeader,
            Ipv6DestinationOptions, Cftp, SatnetExpak, Kryptolan, RemoteVirtualDisk,
            InternetPluribusPacketCore, SatnetMonitoring, Visa, InternetPacketCoreUtility,
            CompterProtocolNetworkExecutive, ComputerProtocolHeartBeat, WangSpanNetwork,
            PacketVideoProtocol, BackroomSatnetMonitoring, SunNd, WidebandMonitoring,
            WidebandExpak, IsoIp, Vmtp, SecureVmtp, Vines, IpTrafficManager, NsfnetIgp,
            DissimilarGateway, Tcf, Eigrp, Ospfigp, SpriteRpc, LocusAddressResolution,
            MulticastTransport, Ax25, IpWithinIpEncapsulation,
            MobileInternetworkingControl, SemaphoreCommunicationsSec,
            EthernetWithinIpEncapsulation, EncapsulationHeader, Gmtp,
            IpsilonFlowManagement, PnniOverIp, ProtocolIndependentMulticast, Aris, Scps,
            Qnx, ActiveNetworks, IpPayloadCompression, SitaraNetworks, CompaqPeer,
            IpxInIp, VirtualRouterRedundancy, PgnReliableTransport, LayerTwoTunneling,
            DiiDataExchange, InteractiveAgentTransfer, ScheduleTransfer, SpectraLinkRadio,
            Uti, SimpleMessage, Sm, PerformanceTransparency, Fire, CombatRadioTransport,
            CombatRadioUserDatagram, Sscopmce, Iplt, SecurePacketShield,
            PrivateIpEncapsulation, StreamControlTransmission, FibreChannel, UdpLite,
            MplsInIp, Manet, HostIdentity, Shim6, WrappedEncapsulatingSecurity,
            RobustHeaderCompression, Raw,
        };
    }
}

/// Unix domain sockets.
pub mod unix {
    pub use lrs_socket::addr::unix::{UnixSockAddr};
}

/// Socket type constants.
pub mod kind {
    pub use lrs_socket::kind::{
        Stream, Datagram, Raw, Rdm, SeqPacket, Dccp,
    };
}

/// Control messages.
pub mod cmsg {
    pub use lrs_socket::cmsg::{
        CMsgBuf, CMsgIter, CMsg, Credentials,
    };
}

/// Socket flags.
///
/// = Description
///
/// These flags can be used when creating sockets.
pub mod flags {
    pub use lrs_socket::flags::{
        None, NonBlocking, CloseOnExec,
    };
}

/// Per-message flags
///
/// = Description
///
/// These are flags that can be set per receive/send call and which will be returned on
/// received messages.
pub mod msg {
    pub use lrs_socket::msg::{Flags};

    pub use lrs_socket::msg::{
        None, Confirm, DontRoute, DontBlock, EndOfRecord, More, OutOfBand,
        Peek, RealSize, WaitAll, CMsgTruncated,
    };
}
