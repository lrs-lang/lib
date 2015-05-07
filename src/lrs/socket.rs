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

pub mod ip {
    pub use lrs_socket::ip_proto::{Proto};

    /// IP protocol constants
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

pub mod kind {
    pub use lrs_socket::kind::{
        Stream, Datagram, Raw, Rdm, SeqPacket, Dccp,
    };
}

pub mod cmsg {
    pub use lrs_socket::cmsg::{
        CMsgBuf, CMsgIter, CMsg, Credentials,
    };
}

pub mod flags {
    pub use lrs_socket::flags::{
        None, NonBlocking,
    };
}

/// Per-message flags
///
/// = Description
///
/// These are flags that can be set per receive/send call and which will be returned on
/// received messages.
///
/// # Confirm
///
/// #### In function calls
///
/// Informs the link layer that we've successfully received messages from a peer and that
/// the link layer doesn't have to refresh the MAC address of the peer. This flag can only
/// be set when sending messages via UDP or raw packets.
///
/// # DontRoute
///
/// #### In function calls
///
/// Tells the system only to send the message if we are directly connected to the peer.
///
/// # DontBlock
///
/// Function calls with this flag will never block.
///
/// ## EndOfRecord
///
/// #### In function calls
///
/// When sending on `SeqPacket` sockets, this flag marks the end of a single record.
///
/// #### In messages
///
/// Messages with this flag mark the end of a record.
///
/// # More
///
/// #### In function calls
///
/// When using this flag while sending, the system will not send the message until the
/// next sending call without this flag set. This can be used to assemble UDP messages.
///
/// # OutOfBand
///
/// #### In function calls
///
/// Sends out-of-band data.
///
/// #### In messages
///
/// Messages with this flag contain out-of-band data.
///
/// # ErrorQueue
///
/// #### In function calls
///
/// This flag can be used when receiving messages. In response, the kernel will attach
/// enqueued socket errors as control messages.
///
/// #### In messages
///
/// No data but a control message containing an error was received.
///
/// # Peek
///
/// #### In function calls
///
/// When receiving messages with this call, the message will remain in the kernel buffer
/// and can be received again with another receive call.
///
/// # RealSize
///
/// #### In function calls
///
/// When receiving messages on UDP or raw sockets, this flag makes the kernel return the
/// un-truncated size of truncated messages.
///
/// #### In messages
///
/// When a message with this flag is received, a part of the message has been discarded
/// because the provided buffer was too small.
///
/// #### Example
///
/// ```
/// use lrs::socket::msg::{RealSize};
///
/// let socket = {
///     // Create a datagram socket
/// };
///
/// let mut buf = [0; 128];
/// let (size, _, _, flags) = socket.recv_msg(&mut buf, &mut [], &mut [],
///                                           RealSize).unwrap();
/// if flags.is_set(RealSize) {
///     println!("Received a truncated message: buffer size: {}, real size: {}",
///              buf.len(), size);
/// }
/// ```
pub mod msg {
    pub use lrs_socket::msg::{Flags};

    pub use lrs_socket::msg::{
        None, Confirm, DontRoute, DontBlock, EndOfRecord, More, OutOfBand, ErrorQueue,
        Peek, RealSize, WaitAll, WaitForOne, CMsgRealSize, Probe, FastOpen,
    };
}
