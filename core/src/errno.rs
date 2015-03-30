// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(non_upper_case_globals)]

use std::fmt::{self, Debug, Formatter};
use std::error::{FromError};
use std::io::{ErrorKind};

use cty::{c_int};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Errno(pub c_int);

macro_rules! create {
    ($($name:ident = ($val:expr, $str:expr),)*) => {
        $(pub const $name: Errno = Errno($val);)*

        impl Errno {
            pub fn as_str(self) -> &'static str {
                match self {
                    $($name => $str,)*
                    _ => "Unknown error",
                }
            }
        }

        impl Debug for Errno {
            fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
                match *self {
                    $($name => f.write_str(stringify!($name)),)*
                    x => write!(f, "Unknown({})", x.0),
                }
            }
        }
    }
}

create! {
    NotPermitted            = (1,   "Operation not permitted"),                             // EPERM
    DoesNotExist            = (2,   "No such file or directory"),                           // ENOENT
    NoSuchProcess           = (3,   "No process matches the specified process ID"),         // ESRCH
    Interrupted             = (4,   "Function call interrupted"),                           // EINTR
    InputOutput             = (5,   "Input/Output error"),                                  // EIO
    NoSuchDevice            = (6,   "No such device or address"),                           // ENXIO
    TooManyArguemnts        = (7,   "Argument list too long"),                              // E2BIG
    InvalidExecutable       = (8,   "Invalid executable file format"),                      // ENOEXEC
    BadFileDesc             = (9,   "Bad file descriptor"),                                 // EBADF
    NoChildProcesses        = (10,  "There are no child processes"),                        // ECHILD
    WouldBlock              = (11,  "Resource temporarily unavailable"),                    // EAGAIN
    NoMemory                = (12,  "No memory available"),                                 // ENOMEM
    AccessDenied            = (13,  "Permission denied"),                                   // EACCES
    InvalidPointer          = (14,  "Invalid pointer"),                                     // EFAULT
    NoBlockSpecialFile      = (15,  "Resource busy"),                                       // ENOTBLK
    ResourceBusy            = (16,  "Block special file required"),                         // EBUSY
    FileExists              = (17,  "File exists"),                                         // EEXIST
    CrossFileSystemLink     = (18,  "Attempted to link across file systems"),               // EXDEV
    WrongDeviceType         = (19,  "Wrong device type for operation"),                     // ENODEV
    NotADirectory           = (20,  "Directory required for operation"),                    // ENOTDIR
    IsADirectory            = (21,  "Directory not permitted in operation"),                // EISDIR
    InvalidArgument         = (22,  "Invalid argument"),                                    // EINVAL
    SystemFileLimit         = (23,  "Process file limit reached"),                          // ENFILE
    ProcessFileLimit        = (24,  "System file limit reached"),                           // EMFILE
    NotATerminal            = (25,  "Argument is not a terminal"),                          // ENOTTY
    ExecutableBusy          = (26,  "Trying to execute and write a file at the same time"), // ETXTBSY
    FileTooBig              = (27,  "File too big"),                                        // EFBIG
    DeviceFull              = (28,  "No space left on device"),                             // ENOSPC
    InvalidSeek             = (29,  "Invalid seek operation"),                              // ESPIPE
    ReadOnlyFileSystem      = (30,  "Operation not permitted on read-only file system"),    // EROFS
    TooManyLinks            = (31,  "Too many links"),                                      // EMLINK
    BrokenPipe              = (32,  "Broken pipe"),                                         // EPIPE
    DomainError             = (33,  "Domain error"),                                        // EDOM
    RangeError              = (34,  "Range error"),                                         // ERANGE
    DeadlockAvoided         = (35,  "Deadlock avoided"),
    PathTooLong             = (36,  "Path too long"),
    NoLocksAvailable        = (37,  "No locks available"),
    NotImplemented          = (38,  "Function not implemented"),
    NotEmpty                = (39,  "Directory not empty"),
    TooManySymlinks         = (40,  "Too many levels of symbolic links"),
    NoMessageOfType         = (42,  "No message of desired type"),
    IdentifierRemoved       = (43,  "Identifier removed"),
    ChannelOutOfRange       = (44,  "Channel number out of range"),
    Level2NotSync           = (45,  "Level 2 not synchronized"),
    Level3Halted            = (46,  "Level 3 halted"),
    Level3Reset             = (47,  "Level 3 reset"),
    LinkNumberOutOfRange    = (48,  "Link number out of range"),
    ProtoDriverNotAttached  = (49,  "Protocol driver not attached"),
    NoCSIStructAvailable    = (50,  "No CSI structure available"),
    Level2Halted            = (51,  "Level 2 halted"),
    InvalidExchange         = (52,  "Invalid exchange"),
    InvalidReqDesc          = (53,  "Invalid request descriptor"),
    ExchangeFull            = (54,  "Exchange full"),
    NoAnode                 = (55,  "No anode"),
    InvalidRequestCode      = (56,  "Invalid request code"),
    InvalidSlot             = (57,  "Invalid slot"),
    BadFontFileFormat       = (59,  "Bad font file format"),
    NotAStream              = (60,  "Device not a stream"),
    NoDataAvailable         = (61,  "No data available"),
    TimerExpired            = (62,  "Timer expired"),
    OutOfStreamsResources   = (63,  "Out of streams resources"),
    NotOnNetwork            = (64,  "Machine is not on the network"),
    PackageNotInstalled     = (65,  "Package not installed"),
    ObjectIsRemote          = (66,  "Object is remote"),
    LinkSevered             = (67,  "Link has been severed"),
    AdvertiseError          = (68,  "Advertise error"),
    SrmountError            = (69,  "Srmount error"),
    CommunitacionError      = (70,  "Communication error on send"),
    ProtocolError           = (71,  "Protocol error"),
    MultihopAttempted       = (72,  "Multihop attempted"),
    RFSError                = (73,  "RFS specific error"),
    NotADataMessage         = (74,  "Not a data message"),
    Overflow                = (75,  "Value too large for defined data type"),
    NotUnique               = (76,  "Name not unique on network"),
    BadFileDescState        = (77,  "File descriptor in bad state"),
    RemoteAddrChanged       = (78,  "Remote address changed"),
    SharedLibInaccessible   = (79,  "Can not access a needed shared library"),
    SharedLibCorrupted      = (80,  "Accessing a corrupted shared library"),
    LibSectionCorrupted     = (81,  ".lib section in a.out corrupted"),
    TooManySharedLibs       = (82,  "Attempting to link in too many shared libraries"),
    SharedLibExec           = (83,  "Cannot exec a shared library directly"),
    InvalidSequence         = (84,  "Invalid sequence"),
    Restart                 = (85,  "Interrupted system call should be restarted"),
    StreamPipeError         = (86,  "Streams pipe error"),
    TooManyUsers            = (87,  "Too many users"),
    NotASocket              = (88,  "Argument is not a socket"),
    NoDefaultDestination    = (89,  "Connectionless socket has no destination"),
    MessageSize             = (90,  "Message too large"),
    ProtoNotSupported       = (91,  "Protocol not supported by socket type"),
    OpNotSupported          = (92,  "Operation not supported by protocol"),
    ProtoNotSupported2      = (93,  "Protocol not supported by socket domain"),
    SocketTypeNotSupported  = (94,  "Socket type is not supported"),
    NotSupported            = (95,  "Operation not supported"),
    ProtoFamilyNotSupported = (96,  "Protocol family not supported"),
    AddrFamilyNotSupported  = (97,  "Address family not supported"),
    AddressInUse            = (98,  "Socket address already in use"),
    AddressNotAvailable     = (99,  "Socket address is not available"),
    NetworkDown             = (100, "Network is down"),
    NetworkUnreachable      = (101, "Remote network is unreachable"),
    HostCrashed             = (102, "Remote hast crashed"),
    ConnectionAborted       = (103, "Connection locally aborted"),
    ConnectionReset         = (104, "Connection closed"),
    KernelBuffersBusy       = (105, "All kernel I/O buffers are in use"),
    SocketConnected         = (106, "Socket is already connected"),
    SocketNotConnected      = (107, "Socket is not connected"),
    SocketShutDown          = (108, "Socket has shut down"),
    TooManyReferences       = (109, "Too many references"),
    SocketTimedOut          = (110, "Socket operation timed out"),
    ConnectionRefused       = (111, "Remote host is down"),
    HostDown                = (112, "Remote host is unreachable"),
    HostUnreachable         = (113, "Remote host refused connection"),
    AlreadyInProgress       = (114, "Operation already in progress"),
    OperationInitiated      = (115, "Operation initiated"),
    StaleFileHandle         = (116, "Stale file handle"),
    NeedsCleaning           = (117, "Structure needs cleaning"),
    NotXENIX                = (118, "Not a XENIX named type file"),
    NoXENIXSemaphores       = (119, "No XENIX semaphores available"),
    NamedTypeFile           = (120, "Is a named type file"),
    RemoteIOError           = (121, "Remote I/O error"),
    DiskQuota               = (122, "Disk quota exceeded"),
    NoMedium                = (123, "No medium found"),
    WrongMediumType         = (124, "Wrong medium type"),
    OperationCanceled       = (125, "Asynchronous operation canceled"),
    KeyNotAvailable         = (126, "Required key not available"),
    KeyExpired              = (127, "Key has expired"),
    KeyRevoked              = (128, "Key has been revoked"),
    KeyRejected             = (129, "Key was rejected by service"),
    OwnerDied               = (130, "Owner died"),
    IrrecoverableState      = (131, "State not recoverable"),
    RFKill                  = (132, "Operation not possible due to RF-kill"),
    HardwarePoison          = (133, "Memory page has hardware error"),

    RustError               = (5000, "Rust custom error"),
}

impl FromError<Errno> for ::std::io::Error {
    fn from_error(e: Errno) -> ::std::io::Error {
        ::std::io::Error::from_os_error(e.0)
    }
}

impl FromError<::std::io::Error> for Errno {
    fn from_error(e: ::std::io::Error) -> Errno {
        if let Some(num) = e.raw_os_error() {
            return Errno(num as c_int);
        }
        match e.kind() {
            ErrorKind::PermissionDenied               => NotPermitted,
            ErrorKind::NotFound                       => DoesNotExist,
            ErrorKind::ConnectionRefused              => ConnectionRefused,
            ErrorKind::ConnectionReset                => ConnectionReset,
            ErrorKind::ConnectionAborted              => ConnectionAborted,
            ErrorKind::NotConnected                   => SocketNotConnected,
            ErrorKind::AddrInUse                      => AddressInUse,
            ErrorKind::AddrNotAvailable               => AddressNotAvailable,
            ErrorKind::BrokenPipe                     => BrokenPipe,
            ErrorKind::AlreadyExists                  => FileExists,
            ErrorKind::WouldBlock                     => WouldBlock,
            ErrorKind::InvalidInput                   => InvalidArgument,
            ErrorKind::TimedOut                       => SocketTimedOut,
            ErrorKind::WriteZero                      => RustError,
            ErrorKind::Interrupted                    => Interrupted,
            ErrorKind::Other                          => RustError,
            _                                         => RustError,
        }
    }
}
