// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use fmt::{Debug, Write};

/// An IPv4/IPv6 protocol.
///
/// [field, 1]
/// The integer constant associated with the protocol.
///
/// = Remarks
///
/// This value corresponds to the protocol/next header field in an IPv4/IPv6 packet. For
/// example, TCP/IP uses the protocol `ip::proto::Tcp`.
///
/// :ipproto: link:lrs::socket::ip::proto
///
/// See {ipproto} for pre-defined constants.
///
/// = See also
///
/// * {ipproto}
#[derive(Pod, Eq)]
pub struct Proto(pub u8);

macro_rules! create {
    ($($name:ident = $val:expr, $doc:expr,)*) => {
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
    HopByHop                        = 0,   "IPv6 Hop-by-Hop Option",
    InternetControlMessage          = 1,   "Internet Control Message",
    InternetGroupManagement         = 2,   "Internet Group Management",
    GatewayToGateway                = 3,   "Gateway-to-Gateway",
    Ipv4Encapsulation               = 4,   "IPv4 encapsulation",
    Stream                          = 5,   "Stream",
    Tcp                             = 6,   "Transmission Control",
    Cbt                             = 7,   "CBT",
    ExteriorGateway                 = 8,   "Exterior Gateway Protocol",
    InteriorGateway                 = 9,   "Interior Gateway Protocol",
    BbnRccMonitoring                = 10,  "BBN RCC Monitoring",
    NetworkVoice                    = 11,  "Network Voice Protocol",
    Pup                             = 12,  "PUP",
    Argus                           = 13,  "ARGUS",
    Emcon                           = 14,  "EMCON",
    CrossNetDebugger                = 15,  "Cross Net Debugger",
    Chaos                           = 16,  "Chaos",
    Udp                             = 17,  "User Datagram",
    Multiplexing                    = 18,  "Multiplexing",
    Dcn                             = 19,  "DCN Measurement Subsystems",
    HostMonitoring                  = 20,  "Host Monitoring",
    PacketRadioMeasurement          = 21,  "Packet Radio Measurement",
    XeroxIdp                        = 22,  "XEROX NS IDP",
    Trunk1                          = 23,  "Trunk-1",
    Trunk2                          = 24,  "Trunk-2",
    Leaf1                           = 25,  "Leaf-1",
    Leaf2                           = 26,  "Leaf-2",
    ReliableData                    = 27,  "Reliable Data Protocol",
    InternetReliableTransaction     = 28,  "Internet Reliable Transaction",
    Iso4                            = 29,  "ISO Transport Protocol Class 4",
    BulkDataTransfer                = 30,  "Bulk Data Transfer Protocol",
    MfeNetworkServices              = 31,  "MFE Network Services Protocol",
    MeritInternodal                 = 32,  "MERIT Internodal Protocol",
    DatagramCongestionControl       = 33,  "Datagram Congestion Control Protocol",
    ThirdPartyConnect               = 34,  "Third Party Connect Protocol",
    InterDomainPolicyRouting        = 35,  "Inter-Domain Policy Routing Protocol",
    Xtp                             = 36,  "XTP",
    DatagramDelivery                = 37,  "Datagram Delivery Protocol",
    IdprControlMessageTransport     = 38,  "IDPR Control Message Transport Proto",
    TpPPTransport                   = 39,  "TP++ Transport Protocol",
    IlTransport                     = 40,  "IL Transport Protocol",
    Ipv6Encapsulation               = 41,  "IPv6 encapsulation",
    SourceDemandRouting             = 42,  "Source Demand Routing Protocol",
    Ipv6Routing                     = 43,  "Routing Header for IPv6",
    Ipv6Fragment                    = 44,  "Fragment Header for IPv6",
    InterDomainRouting              = 45,  "Inter-Domain Routing Protocol",
    Reservation                     = 46,  "Reservation Protocol",
    GenericRoutingEncapsulation     = 47,  "Generic Routing Encapsulation",
    DynamicSourceRouting            = 48,  "Dynamic Source Routing Protocol",
    Bna                             = 49,  "BNA",
    EncapSecurityPayload            = 50,  "Encap Security Payload",
    AuthenticationHeader            = 51,  "Authentication Header",
    IntegratedNetLayerSecurity      = 52,  "Integrated Net Layer Security",
    Swipe                           = 53,  "IP with Encryption",
    NbmaAddressResolution           = 54,  "NBMA Address Resolution Protocol",
    IpMobility                      = 55,  "IP Mobility",
    TransportLayerSecurity          = 56,  "Transport Layer Security Protocol",
    Skip                            = 57,  "SKIP",
    Ipv6InternetControlMessage      = 58,  "ICMP for IPv6",
    Ipv6NoNextHeader                = 59,  "No Next Header for IPv6",
    Ipv6DestinationOptions          = 60,  "Destination Options for IPv6",
    Cftp                            = 62,  "CFTP",
    SatnetExpak                     = 64,  "SATNET and Backroom EXPAK",
    Kryptolan                       = 65,  "Kryptolan",
    RemoteVirtualDisk               = 66,  "MIT Remote Virtual Disk Protocol",
    InternetPluribusPacketCore      = 67,  "Internet Pluribus Packet Core",
    SatnetMonitoring                = 69,  "SATNET Monitoring",
    Visa                            = 70,  "VISA Protocol",
    InternetPacketCoreUtility       = 71,  "Internet Packet Core Utility",
    CompterProtocolNetworkExecutive = 72,  "Computer Protocol Network Executive",
    ComputerProtocolHeartBeat       = 73,  "Computer Protocol Heart Beat",
    WangSpanNetwork                 = 74,  "Wang Span Network",
    PacketVideoProtocol             = 75,  "Packet Video Protocol",
    BackroomSatnetMonitoring        = 76,  "Backroom SATNET Monitoring",
    SunNd                           = 77,  "SUN ND PROTOCOL-Temporary",
    WidebandMonitoring              = 78,  "WIDEBAND Monitoring",
    WidebandExpak                   = 79,  "WIDEBAND EXPAK",
    IsoIp                           = 80,  "ISO Internet Protocol",
    Vmtp                            = 81,  "VMTP",
    SecureVmtp                      = 82,  "SECURE-VMTP",
    Vines                           = 83,  "VINES",
    IpTrafficManager                = 84,  "Internet Protocol Traffic Manager",
    NsfnetIgp                       = 85,  "NSFNET-IGP",
    DissimilarGateway               = 86,  "Dissimilar Gateway Protocol",
    Tcf                             = 87,  "TCF",
    Eigrp                           = 88,  "EIGRP",
    Ospfigp                         = 89,  "OSPFIGP",
    SpriteRpc                       = 90,  "Sprite RPC Protocol",
    LocusAddressResolution          = 91,  "Locus Address Resolution Protocol",
    MulticastTransport              = 92,  "Multicast Transport Protocol",
    Ax25                            = 93,  "AX.25 Frames",
    IpWithinIpEncapsulation         = 94,  "IP-within-IP Encapsulation Protocol",
    MobileInternetworkingControl    = 95,  "Mobile Internetworking Control Pro.",
    SemaphoreCommunicationsSec      = 96,  "Semaphore Communications Sec. Pro.",
    EthernetWithinIpEncapsulation   = 97,  "Ethernet-within-IP Encapsulation",
    EncapsulationHeader             = 98,  "Encapsulation Header",
    Gmtp                            = 100, "GMTP",
    IpsilonFlowManagement           = 101, "Ipsilon Flow Management Protocol",
    PnniOverIp                      = 102, "PNNI over IP",
    ProtocolIndependentMulticast    = 103, "Protocol Independent Multicast",
    Aris                            = 104, "ARIS",
    Scps                            = 105, "SCPS",
    Qnx                             = 106, "QNX",
    ActiveNetworks                  = 107, "Active Networks",
    IpPayloadCompression            = 108, "IP Payload Compression Protocol",
    SitaraNetworks                  = 109, "Sitara Networks Protocol",
    CompaqPeer                      = 110, "Compaq Peer Protocol",
    IpxInIp                         = 111, "IPX in IP",
    VirtualRouterRedundancy         = 112, "Virtual Router Redundancy Protocol",
    PgnReliableTransport            = 113, "PGM Reliable Transport Protocol",
    LayerTwoTunneling               = 115, "Layer Two Tunneling Protocol",
    DiiDataExchange                 = 116, "D-II Data Exchange (DDX)",
    InteractiveAgentTransfer        = 117, "Interactive Agent Transfer Protocol",
    ScheduleTransfer                = 118, "Schedule Transfer Protocol",
    SpectraLinkRadio                = 119, "SpectraLink Radio Protocol",
    Uti                             = 120, "UTI",
    SimpleMessage                   = 121, "Simple Message Protocol",
    Sm                              = 122, "SM",
    PerformanceTransparency         = 123, "Performance Transparency Protocol",
    Fire                            = 125, "FIRE",
    CombatRadioTransport            = 126, "Combat Radio Transport Protocol",
    CombatRadioUserDatagram         = 127, "Combat Radio User Datagram",
    Sscopmce                        = 128, "SSCOPMCE",
    Iplt                            = 129, "IPLT",
    SecurePacketShield              = 130, "Secure Packet Shield",
    PrivateIpEncapsulation          = 131, "Private IP Encapsulation within IP",
    StreamControlTransmission       = 132, "Stream Control Transmission Protocol",
    FibreChannel                    = 133, "Fibre Channel",
    UdpLite                         = 136, "UDPLite",
    MplsInIp                        = 137, "MPLS-in-IP",
    Manet                           = 138, "MANET Protocols",
    HostIdentity                    = 139, "Host Identity Protocol",
    Shim6                           = 140, "Shim6 Protocol",
    WrappedEncapsulatingSecurity    = 141, "Wrapped Encapsulating Security Payload",
    RobustHeaderCompression         = 142, "Robust Header Compression",
    Raw                             = 255, "Raw",
}
