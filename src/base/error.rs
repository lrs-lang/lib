// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(non_upper_case_globals, non_camel_case_types)]

use core::option::{Option};
use core::option::Option::{Some, None};

pub use cty_base::errno:: {
    EPERM, ENOENT, ESRCH, EINTR, EIO, ENXIO, E2BIG, ENOEXEC, EBADF, ECHILD,
    EAGAIN, ENOMEM, EACCES, EFAULT, ENOTBLK, EBUSY, EEXIST, EXDEV, ENODEV, ENOTDIR,
    EISDIR, EINVAL, ENFILE, EMFILE, ENOTTY, ETXTBSY, EFBIG, ENOSPC, ESPIPE, EROFS,
    EMLINK, EPIPE, EDOM, ERANGE, EDEADLK, ENAMETOOLONG, ENOLCK, ENOSYS, ENOTEMPTY,
    ELOOP, ENOMSG, EIDRM, ECHRNG, EL2NSYNC, EL3HLT, EL3RST, ELNRNG, EUNATCH,
    ENOCSI, EL2HLT, EBADE, EBADR, EXFULL, ENOANO, EBADRQC, EBADSLT, EBFONT, ENOSTR,
    ENODATA, ETIME, ENOSR, ENONET, ENOPKG, EREMOTE, ENOLINK, EADV, ESRMNT, ECOMM,
    EPROTO, EMULTIHOP, EDOTDOT, EBADMSG, EOVERFLOW, ENOTUNIQ, EBADFD, EREMCHG,
    ELIBACC, ELIBBAD, ELIBSCN, ELIBMAX, ELIBEXEC, EILSEQ, ERESTART, ESTRPIPE,
    EUSERS, ENOTSOCK, EDESTADDRREQ, EMSGSIZE, EPROTOTYPE, ENOPROTOOPT,
    EPROTONOSUPPORT, ESOCKTNOSUPPORT, EOPNOTSUPP, EPFNOSUPPORT, EAFNOSUPPORT,
    EADDRINUSE, EADDRNOTAVAIL, ENETDOWN, ENETUNREACH, ENETRESET, ECONNABORTED,
    ECONNRESET, ENOBUFS, EISCONN, ENOTCONN, ESHUTDOWN, ETOOMANYREFS, ETIMEDOUT,
    ECONNREFUSED, EHOSTDOWN, EHOSTUNREACH, EALREADY, EINPROGRESS, ESTALE, EUCLEAN,
    ENOTNAM, ENAVAIL, EISNAM, EREMOTEIO, EDQUOT, ENOMEDIUM, EMEDIUMTYPE, ECANCELED,
    ENOKEY, EKEYEXPIRED, EKEYREVOKED, EKEYREJECTED, EOWNERDEAD, ENOTRECOVERABLE,
    ERFKILL, EHWPOISON,
};
pub use cty_base::types::{c_int};

/// A standard error
///
/// [field, 1]
/// The error number.
///
/// = Remarks
///
/// Error numbers between 0 and 4096 exclusive are reserved by the kernel. Error numbers
/// between 4096 and 9999 inclusive are reserved by lrs.
///
/// = See also
///
/// * link:man:errno(3)
/// * link:lrs::error which contains constants for the error numbers returned by the
///   kernel.
#[derive(Pod, Eq)]
pub struct Errno(pub c_int);

macro_rules! create {
    ($($name:ident = $val:expr, $str:expr,)*) => {
        $(#[doc = $str] pub const $name: Errno = Errno($val as c_int);)*

        impl Errno {
            /// Returns the name of the error in string form.
            pub fn name(self) -> Option<&'static str> {
                match self {
                    $($name => Some(stringify!($name)),)*
                    _ => None,
                }
            }

            /// Returns a longer description of the error.
            pub fn desc(self) -> &'static str {
                match self {
                    $($name => $str,)*
                    _ => "Unknown error",
                }
            }
        }
    }
}

create! {
    NoError                 = 0               , "No error",
    NotPermitted            = EPERM           , "Operation not permitted",
    DoesNotExist            = ENOENT          , "No such file or directory",
    NoSuchProcess           = ESRCH           , "No process matches the specified process ID",
    Interrupted             = EINTR           , "Function call interrupted",
    InputOutput             = EIO             , "Input/Output error",
    NoSuchDevice            = ENXIO           , "No such device or address",
    TooManyArguemnts        = E2BIG           , "Argument list too long",
    InvalidExecutable       = ENOEXEC         , "Invalid executable file format",
    BadFileDesc             = EBADF           , "Bad file descriptor",
    NoChildProcesses        = ECHILD          , "There are no child processes",
    WouldBlock              = EAGAIN          , "Resource temporarily unavailable",
    NoMemory                = ENOMEM          , "No memory available",
    AccessDenied            = EACCES          , "Permission denied",
    InvalidPointer          = EFAULT          , "Invalid pointer",
    NoBlockSpecialFile      = ENOTBLK         , "Block special file required",
    ResourceBusy            = EBUSY           , "Resource busy",
    FileExists              = EEXIST          , "File exists",
    CrossFileSystemLink     = EXDEV           , "Attempted to link across filesystems",
    WrongDeviceType         = ENODEV          , "Wrong device type for operation",
    NotADirectory           = ENOTDIR         , "Directory required for operation",
    IsADirectory            = EISDIR          , "Directory not permitted in operation",
    InvalidArgument         = EINVAL          , "Invalid argument",
    SystemFileLimit         = ENFILE          , "Process file limit reached",
    ProcessFileLimit        = EMFILE          , "System file limit reached",
    NotATerminal            = ENOTTY          , "Argument is not a terminal",
    ExecutableBusy          = ETXTBSY         , "Trying to execute and write a file at the same time",
    FileTooBig              = EFBIG           , "File too big",
    DeviceFull              = ENOSPC          , "No space left on device",
    InvalidSeek             = ESPIPE          , "Invalid seek operation",
    ReadOnlyFileSystem      = EROFS           , "Operation not permitted on read-only filesystem",
    TooManyLinks            = EMLINK          , "Too many links",
    BrokenPipe              = EPIPE           , "Broken pipe",
    DomainError             = EDOM            , "Domain error",
    RangeError              = ERANGE          , "Range error",
    DeadlockAvoided         = EDEADLK         , "Deadlock avoided",
    PathTooLong             = ENAMETOOLONG    , "Path too long",
    NoLocksAvailable        = ENOLCK          , "No locks available",
    NotImplemented          = ENOSYS          , "Function not implemented",
    NotEmpty                = ENOTEMPTY       , "Directory not empty",
    TooManySymlinks         = ELOOP           , "Too many levels of symbolic links",
    NoMessageOfType         = ENOMSG          , "No message of desired type",
    IdentifierRemoved       = EIDRM           , "Identifier removed",
    ChannelOutOfRange       = ECHRNG          , "Channel number out of range",
    Level2NotSync           = EL2NSYNC        , "Level 2 not synchronized",
    Level3Halted            = EL3HLT          , "Level 3 halted",
    Level3Reset             = EL3RST          , "Level 3 reset",
    LinkNumberOutOfRange    = ELNRNG          , "Link number out of range",
    ProtoDriverNotAttached  = EUNATCH         , "Protocol driver not attached",
    NoCSIStructAvailable    = ENOCSI          , "No CSI structure available",
    Level2Halted            = EL2HLT          , "Level 2 halted",
    InvalidExchange         = EBADE           , "Invalid exchange",
    InvalidReqDesc          = EBADR           , "Invalid request descriptor",
    ExchangeFull            = EXFULL          , "Exchange full",
    NoAnode                 = ENOANO          , "No anode",
    InvalidRequestCode      = EBADRQC         , "Invalid request code",
    InvalidSlot             = EBADSLT         , "Invalid slot",
    BadFontFileFormat       = EBFONT          , "Bad font file format",
    NotAStream              = ENOSTR          , "Device not a stream",
    NoDataAvailable         = ENODATA         , "No data available",
    TimerExpired            = ETIME           , "Timer expired",
    OutOfStreamsResources   = ENOSR           , "Out of streams resources",
    NotOnNetwork            = ENONET          , "Machine is not on the network",
    PackageNotInstalled     = ENOPKG          , "Package not installed",
    ObjectIsRemote          = EREMOTE         , "Object is remote",
    LinkSevered             = ENOLINK         , "Link has been severed",
    AdvertiseError          = EADV            , "Advertise error",
    SrmountError            = ESRMNT          , "Srmount error",
    CommunitacionError      = ECOMM           , "Communication error on send",
    ProtocolError           = EPROTO          , "Protocol error",
    MultihopAttempted       = EMULTIHOP       , "Multihop attempted",
    RFSError                = EDOTDOT         , "RFS specific error",
    NotADataMessage         = EBADMSG         , "Not a data message",
    Overflow                = EOVERFLOW       , "Value too large for defined data type",
    NotUnique               = ENOTUNIQ        , "Name not unique on network",
    BadFileDescState        = EBADFD          , "File descriptor in bad state",
    RemoteAddrChanged       = EREMCHG         , "Remote address changed",
    SharedLibInaccessible   = ELIBACC         , "Can not access a needed shared library",
    SharedLibCorrupted      = ELIBBAD         , "Accessing a corrupted shared library",
    LibSectionCorrupted     = ELIBSCN         , ".lib section in a.out corrupted",
    TooManySharedLibs       = ELIBMAX         , "Attempting to link in too many shared libraries",
    SharedLibExec           = ELIBEXEC        , "Cannot exec a shared library directly",
    InvalidSequence         = EILSEQ          , "Invalid sequence",
    Restart                 = ERESTART        , "Interrupted system call should be restarted",
    StreamPipeError         = ESTRPIPE        , "Streams pipe error",
    TooManyUsers            = EUSERS          , "Too many users",
    NotASocket              = ENOTSOCK        , "Argument is not a socket",
    NoDefaultDestination    = EDESTADDRREQ    , "Connectionless socket has no destination",
    MessageSize             = EMSGSIZE        , "Message too large",
    ProtoNotSupported       = EPROTOTYPE      , "Protocol not supported by socket type",
    OpNotSupported          = ENOPROTOOPT     , "Operation not supported by protocol",
    ProtoNotSupported2      = EPROTONOSUPPORT , "Protocol not supported by socket domain",
    SocketTypeNotSupported  = ESOCKTNOSUPPORT , "Socket type is not supported",
    NotSupported            = EOPNOTSUPP      , "Operation not supported",
    ProtoFamilyNotSupported = EPFNOSUPPORT    , "Protocol family not supported",
    AddrFamilyNotSupported  = EAFNOSUPPORT    , "Address family not supported",
    AddressInUse            = EADDRINUSE      , "Socket address already in use",
    AddressNotAvailable     = EADDRNOTAVAIL   , "Socket address is not available",
    NetworkDown             = ENETDOWN        , "Network is down",
    NetworkUnreachable      = ENETUNREACH     , "Remote network is unreachable",
    HostCrashed             = ENETRESET       , "Remote hast crashed",
    ConnectionAborted       = ECONNABORTED    , "Connection locally aborted",
    ConnectionReset         = ECONNRESET      , "Connection closed",
    KernelBuffersBusy       = ENOBUFS         , "All kernel I/O buffers are in use",
    SocketConnected         = EISCONN         , "Socket is already connected",
    SocketNotConnected      = ENOTCONN        , "Socket is not connected",
    SocketShutDown          = ESHUTDOWN       , "Socket has shut down",
    TooManyReferences       = ETOOMANYREFS    , "Too many references",
    TimedOut                = ETIMEDOUT       , "Operation timed out",
    ConnectionRefused       = ECONNREFUSED    , "Remote host is down",
    HostDown                = EHOSTDOWN       , "Remote host is unreachable",
    HostUnreachable         = EHOSTUNREACH    , "Remote host refused connection",
    AlreadyInProgress       = EALREADY        , "Operation already in progress",
    OperationInitiated      = EINPROGRESS     , "Operation initiated",
    StaleFileHandle         = ESTALE          , "Stale file handle",
    NeedsCleaning           = EUCLEAN         , "Structure needs cleaning",
    NotXENIX                = ENOTNAM         , "Not a XENIX named type file",
    NoXENIXSemaphores       = ENAVAIL         , "No XENIX semaphores available",
    NamedTypeFile           = EISNAM          , "Is a named type file",
    RemoteIOError           = EREMOTEIO       , "Remote I/O error",
    DiskQuota               = EDQUOT          , "Disk quota exceeded",
    NoMedium                = ENOMEDIUM       , "No medium found",
    WrongMediumType         = EMEDIUMTYPE     , "Wrong medium type",
    OperationCanceled       = ECANCELED       , "Asynchronous operation canceled",
    KeyNotAvailable         = ENOKEY          , "Required key not available",
    KeyExpired              = EKEYEXPIRED     , "Key has expired",
    KeyRevoked              = EKEYREVOKED     , "Key has been revoked",
    KeyRejected             = EKEYREJECTED    , "Key was rejected by service",
    OwnerDied               = EOWNERDEAD      , "Owner died",
    IrrecoverableState      = ENOTRECOVERABLE , "State not recoverable",
    RFKill                  = ERFKILL         , "Operation not possible due to RF-kill",
    HardwarePoison          = EHWPOISON       , "Memory page has hardware error",
}
