// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_error"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(linux_core_plugin)]
#![no_std]
#![allow(non_upper_case_globals, non_camel_case_types)]

extern crate linux_core as core;

#[prelude_import] use core::prelude::*;
pub use arch::{nos};

mod linux {
    pub use ::core::linux::*;
}

mod gen;

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;

#[derive(Copy, Eq)]
pub struct Errno(pub arch::c_int);

macro_rules! create {
    ($($name:ident = ($val:expr, $str:expr),)*) => {
        $(pub const $name: Errno = Errno($val as arch::c_int);)*

        impl Errno {
            pub fn desc(self) -> &'static str {
                match self {
                    $($name => $str,)*
                    _ => "Unknown error",
                }
            }

            pub fn name(self) -> Option<&'static str> {
                match self {
                    $($name => Some(stringify!($name)),)*
                    _ => None,
                }
            }
        }
    }
}

create! {
    NotPermitted            = (nos::EPERM           , "Operation not permitted"),
    DoesNotExist            = (nos::ENOENT          , "No such file or directory"),
    NoSuchProcess           = (nos::ESRCH           , "No process matches the specified process ID"),
    Interrupted             = (nos::EINTR           , "Function call interrupted"),
    InputOutput             = (nos::EIO             , "Input/Output error"),
    NoSuchDevice            = (nos::ENXIO           , "No such device or address"),
    TooManyArguemnts        = (nos::E2BIG           , "Argument list too long"),
    InvalidExecutable       = (nos::ENOEXEC         , "Invalid executable file format"),
    BadFileDesc             = (nos::EBADF           , "Bad file descriptor"),
    NoChildProcesses        = (nos::ECHILD          , "There are no child processes"),
    WouldBlock              = (nos::EAGAIN          , "Resource temporarily unavailable"),
    NoMemory                = (nos::ENOMEM          , "No memory available"),
    AccessDenied            = (nos::EACCES          , "Permission denied"),
    InvalidPointer          = (nos::EFAULT          , "Invalid pointer"),
    NoBlockSpecialFile      = (nos::ENOTBLK         , "Resource busy"),
    ResourceBusy            = (nos::EBUSY           , "Block special file required"),
    FileExists              = (nos::EEXIST          , "File exists"),
    CrossFileSystemLink     = (nos::EXDEV           , "Attempted to link across file systems"),
    WrongDeviceType         = (nos::ENODEV          , "Wrong device type for operation"),
    NotADirectory           = (nos::ENOTDIR         , "Directory required for operation"),
    IsADirectory            = (nos::EISDIR          , "Directory not permitted in operation"),
    InvalidArgument         = (nos::EINVAL          , "Invalid argument"),
    SystemFileLimit         = (nos::ENFILE          , "Process file limit reached"),
    ProcessFileLimit        = (nos::EMFILE          , "System file limit reached"),
    NotATerminal            = (nos::ENOTTY          , "Argument is not a terminal"),
    ExecutableBusy          = (nos::ETXTBSY         , "Trying to execute and write a file at the same time"),
    FileTooBig              = (nos::EFBIG           , "File too big"),
    DeviceFull              = (nos::ENOSPC          , "No space left on device"),
    InvalidSeek             = (nos::ESPIPE          , "Invalid seek operation"),
    ReadOnlyFileSystem      = (nos::EROFS           , "Operation not permitted on read-only file system"),
    TooManyLinks            = (nos::EMLINK          , "Too many links"),
    BrokenPipe              = (nos::EPIPE           , "Broken pipe"),
    DomainError             = (nos::EDOM            , "Domain error"),
    RangeError              = (nos::ERANGE          , "Range error"),
    DeadlockAvoided         = (nos::EDEADLK         , "Deadlock avoided"),
    PathTooLong             = (nos::ENAMETOOLONG    , "Path too long"),
    NoLocksAvailable        = (nos::ENOLCK          , "No locks available"),
    NotImplemented          = (nos::ENOSYS          , "Function not implemented"),
    NotEmpty                = (nos::ENOTEMPTY       , "Directory not empty"),
    TooManySymlinks         = (nos::ELOOP           , "Too many levels of symbolic links"),
    NoMessageOfType         = (nos::ENOMSG          , "No message of desired type"),
    IdentifierRemoved       = (nos::EIDRM           , "Identifier removed"),
    ChannelOutOfRange       = (nos::ECHRNG          , "Channel number out of range"),
    Level2NotSync           = (nos::EL2NSYNC        , "Level 2 not synchronized"),
    Level3Halted            = (nos::EL3HLT          , "Level 3 halted"),
    Level3Reset             = (nos::EL3RST          , "Level 3 reset"),
    LinkNumberOutOfRange    = (nos::ELNRNG          , "Link number out of range"),
    ProtoDriverNotAttached  = (nos::EUNATCH         , "Protocol driver not attached"),
    NoCSIStructAvailable    = (nos::ENOCSI          , "No CSI structure available"),
    Level2Halted            = (nos::EL2HLT          , "Level 2 halted"),
    InvalidExchange         = (nos::EBADE           , "Invalid exchange"),
    InvalidReqDesc          = (nos::EBADR           , "Invalid request descriptor"),
    ExchangeFull            = (nos::EXFULL          , "Exchange full"),
    NoAnode                 = (nos::ENOANO          , "No anode"),
    InvalidRequestCode      = (nos::EBADRQC         , "Invalid request code"),
    InvalidSlot             = (nos::EBADSLT         , "Invalid slot"),
    BadFontFileFormat       = (nos::EBFONT          , "Bad font file format"),
    NotAStream              = (nos::ENOSTR          , "Device not a stream"),
    NoDataAvailable         = (nos::ENODATA         , "No data available"),
    TimerExpired            = (nos::ETIME           , "Timer expired"),
    OutOfStreamsResources   = (nos::ENOSR           , "Out of streams resources"),
    NotOnNetwork            = (nos::ENONET          , "Machine is not on the network"),
    PackageNotInstalled     = (nos::ENOPKG          , "Package not installed"),
    ObjectIsRemote          = (nos::EREMOTE         , "Object is remote"),
    LinkSevered             = (nos::ENOLINK         , "Link has been severed"),
    AdvertiseError          = (nos::EADV            , "Advertise error"),
    SrmountError            = (nos::ESRMNT          , "Srmount error"),
    CommunitacionError      = (nos::ECOMM           , "Communication error on send"),
    ProtocolError           = (nos::EPROTO          , "Protocol error"),
    MultihopAttempted       = (nos::EMULTIHOP       , "Multihop attempted"),
    RFSError                = (nos::EDOTDOT         , "RFS specific error"),
    NotADataMessage         = (nos::EBADMSG         , "Not a data message"),
    Overflow                = (nos::EOVERFLOW       , "Value too large for defined data type"),
    NotUnique               = (nos::ENOTUNIQ        , "Name not unique on network"),
    BadFileDescState        = (nos::EBADFD          , "File descriptor in bad state"),
    RemoteAddrChanged       = (nos::EREMCHG         , "Remote address changed"),
    SharedLibInaccessible   = (nos::ELIBACC         , "Can not access a needed shared library"),
    SharedLibCorrupted      = (nos::ELIBBAD         , "Accessing a corrupted shared library"),
    LibSectionCorrupted     = (nos::ELIBSCN         , ".lib section in a.out corrupted"),
    TooManySharedLibs       = (nos::ELIBMAX         , "Attempting to link in too many shared libraries"),
    SharedLibExec           = (nos::ELIBEXEC        , "Cannot exec a shared library directly"),
    InvalidSequence         = (nos::EILSEQ          , "Invalid sequence"),
    Restart                 = (nos::ERESTART        , "Interrupted system call should be restarted"),
    StreamPipeError         = (nos::ESTRPIPE        , "Streams pipe error"),
    TooManyUsers            = (nos::EUSERS          , "Too many users"),
    NotASocket              = (nos::ENOTSOCK        , "Argument is not a socket"),
    NoDefaultDestination    = (nos::EDESTADDRREQ    , "Connectionless socket has no destination"),
    MessageSize             = (nos::EMSGSIZE        , "Message too large"),
    ProtoNotSupported       = (nos::EPROTOTYPE      , "Protocol not supported by socket type"),
    OpNotSupported          = (nos::ENOPROTOOPT     , "Operation not supported by protocol"),
    ProtoNotSupported2      = (nos::EPROTONOSUPPORT , "Protocol not supported by socket domain"),
    SocketTypeNotSupported  = (nos::ESOCKTNOSUPPORT , "Socket type is not supported"),
    NotSupported            = (nos::EOPNOTSUPP      , "Operation not supported"),
    ProtoFamilyNotSupported = (nos::EPFNOSUPPORT    , "Protocol family not supported"),
    AddrFamilyNotSupported  = (nos::EAFNOSUPPORT    , "Address family not supported"),
    AddressInUse            = (nos::EADDRINUSE      , "Socket address already in use"),
    AddressNotAvailable     = (nos::EADDRNOTAVAIL   , "Socket address is not available"),
    NetworkDown             = (nos::ENETDOWN        , "Network is down"),
    NetworkUnreachable      = (nos::ENETUNREACH     , "Remote network is unreachable"),
    HostCrashed             = (nos::ENETRESET       , "Remote hast crashed"),
    ConnectionAborted       = (nos::ECONNABORTED    , "Connection locally aborted"),
    ConnectionReset         = (nos::ECONNRESET      , "Connection closed"),
    KernelBuffersBusy       = (nos::ENOBUFS         , "All kernel I/O buffers are in use"),
    SocketConnected         = (nos::EISCONN         , "Socket is already connected"),
    SocketNotConnected      = (nos::ENOTCONN        , "Socket is not connected"),
    SocketShutDown          = (nos::ESHUTDOWN       , "Socket has shut down"),
    TooManyReferences       = (nos::ETOOMANYREFS    , "Too many references"),
    SocketTimedOut          = (nos::ETIMEDOUT       , "Socket operation timed out"),
    ConnectionRefused       = (nos::ECONNREFUSED    , "Remote host is down"),
    HostDown                = (nos::EHOSTDOWN       , "Remote host is unreachable"),
    HostUnreachable         = (nos::EHOSTUNREACH    , "Remote host refused connection"),
    AlreadyInProgress       = (nos::EALREADY        , "Operation already in progress"),
    OperationInitiated      = (nos::EINPROGRESS     , "Operation initiated"),
    StaleFileHandle         = (nos::ESTALE          , "Stale file handle"),
    NeedsCleaning           = (nos::EUCLEAN         , "Structure needs cleaning"),
    NotXENIX                = (nos::ENOTNAM         , "Not a XENIX named type file"),
    NoXENIXSemaphores       = (nos::ENAVAIL         , "No XENIX semaphores available"),
    NamedTypeFile           = (nos::EISNAM          , "Is a named type file"),
    RemoteIOError           = (nos::EREMOTEIO       , "Remote I/O error"),
    DiskQuota               = (nos::EDQUOT          , "Disk quota exceeded"),
    NoMedium                = (nos::ENOMEDIUM       , "No medium found"),
    WrongMediumType         = (nos::EMEDIUMTYPE     , "Wrong medium type"),
    OperationCanceled       = (nos::ECANCELED       , "Asynchronous operation canceled"),
    KeyNotAvailable         = (nos::ENOKEY          , "Required key not available"),
    KeyExpired              = (nos::EKEYEXPIRED     , "Key has expired"),
    KeyRevoked              = (nos::EKEYREVOKED     , "Key has been revoked"),
    KeyRejected             = (nos::EKEYREJECTED    , "Key was rejected by service"),
    OwnerDied               = (nos::EOWNERDEAD      , "Owner died"),
    IrrecoverableState      = (nos::ENOTRECOVERABLE , "State not recoverable"),
    RFKill                  = (nos::ERFKILL         , "Operation not possible due to RF-kill"),
    HardwarePoison          = (nos::EHWPOISON       , "Memory page has hardware error"),
}
