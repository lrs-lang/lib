// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(non_upper_case_globals, non_camel_case_types)]

#[prelude_import] use core::prelude::*;
pub use cty_base::{errno};
pub use cty_base::types::{c_int};

#[derive(Copy, Eq)]
pub struct Errno(pub c_int);

macro_rules! create {
    ($($name:ident = ($val:expr, $str:expr),)*) => {
        $(pub const $name: Errno = Errno($val as c_int);)*

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
    NotPermitted            = (errno::EPERM           , "Operation not permitted"),
    DoesNotExist            = (errno::ENOENT          , "No such file or directory"),
    NoSuchProcess           = (errno::ESRCH           , "No process matches the specified process ID"),
    Interrupted             = (errno::EINTR           , "Function call interrupted"),
    InputOutput             = (errno::EIO             , "Input/Output error"),
    NoSuchDevice            = (errno::ENXIO           , "No such device or address"),
    TooManyArguemnts        = (errno::E2BIG           , "Argument list too long"),
    InvalidExecutable       = (errno::ENOEXEC         , "Invalid executable file format"),
    BadFileDesc             = (errno::EBADF           , "Bad file descriptor"),
    NoChildProcesses        = (errno::ECHILD          , "There are no child processes"),
    WouldBlock              = (errno::EAGAIN          , "Resource temporarily unavailable"),
    NoMemory                = (errno::ENOMEM          , "No memory available"),
    AccessDenied            = (errno::EACCES          , "Permission denied"),
    InvalidPointer          = (errno::EFAULT          , "Invalid pointer"),
    NoBlockSpecialFile      = (errno::ENOTBLK         , "Resource busy"),
    ResourceBusy            = (errno::EBUSY           , "Block special file required"),
    FileExists              = (errno::EEXIST          , "File exists"),
    CrossFileSystemLink     = (errno::EXDEV           , "Attempted to link across file systems"),
    WrongDeviceType         = (errno::ENODEV          , "Wrong device type for operation"),
    NotADirectory           = (errno::ENOTDIR         , "Directory required for operation"),
    IsADirectory            = (errno::EISDIR          , "Directory not permitted in operation"),
    InvalidArgument         = (errno::EINVAL          , "Invalid argument"),
    SystemFileLimit         = (errno::ENFILE          , "Process file limit reached"),
    ProcessFileLimit        = (errno::EMFILE          , "System file limit reached"),
    NotATerminal            = (errno::ENOTTY          , "Argument is not a terminal"),
    ExecutableBusy          = (errno::ETXTBSY         , "Trying to execute and write a file at the same time"),
    FileTooBig              = (errno::EFBIG           , "File too big"),
    DeviceFull              = (errno::ENOSPC          , "No space left on device"),
    InvalidSeek             = (errno::ESPIPE          , "Invalid seek operation"),
    ReadOnlyFileSystem      = (errno::EROFS           , "Operation not permitted on read-only file system"),
    TooManyLinks            = (errno::EMLINK          , "Too many links"),
    BrokenPipe              = (errno::EPIPE           , "Broken pipe"),
    DomainError             = (errno::EDOM            , "Domain error"),
    RangeError              = (errno::ERANGE          , "Range error"),
    DeadlockAvoided         = (errno::EDEADLK         , "Deadlock avoided"),
    PathTooLong             = (errno::ENAMETOOLONG    , "Path too long"),
    NoLocksAvailable        = (errno::ENOLCK          , "No locks available"),
    NotImplemented          = (errno::ENOSYS          , "Function not implemented"),
    NotEmpty                = (errno::ENOTEMPTY       , "Directory not empty"),
    TooManySymlinks         = (errno::ELOOP           , "Too many levels of symbolic links"),
    NoMessageOfType         = (errno::ENOMSG          , "No message of desired type"),
    IdentifierRemoved       = (errno::EIDRM           , "Identifier removed"),
    ChannelOutOfRange       = (errno::ECHRNG          , "Channel number out of range"),
    Level2NotSync           = (errno::EL2NSYNC        , "Level 2 not synchronized"),
    Level3Halted            = (errno::EL3HLT          , "Level 3 halted"),
    Level3Reset             = (errno::EL3RST          , "Level 3 reset"),
    LinkNumberOutOfRange    = (errno::ELNRNG          , "Link number out of range"),
    ProtoDriverNotAttached  = (errno::EUNATCH         , "Protocol driver not attached"),
    NoCSIStructAvailable    = (errno::ENOCSI          , "No CSI structure available"),
    Level2Halted            = (errno::EL2HLT          , "Level 2 halted"),
    InvalidExchange         = (errno::EBADE           , "Invalid exchange"),
    InvalidReqDesc          = (errno::EBADR           , "Invalid request descriptor"),
    ExchangeFull            = (errno::EXFULL          , "Exchange full"),
    NoAnode                 = (errno::ENOANO          , "No anode"),
    InvalidRequestCode      = (errno::EBADRQC         , "Invalid request code"),
    InvalidSlot             = (errno::EBADSLT         , "Invalid slot"),
    BadFontFileFormat       = (errno::EBFONT          , "Bad font file format"),
    NotAStream              = (errno::ENOSTR          , "Device not a stream"),
    NoDataAvailable         = (errno::ENODATA         , "No data available"),
    TimerExpired            = (errno::ETIME           , "Timer expired"),
    OutOfStreamsResources   = (errno::ENOSR           , "Out of streams resources"),
    NotOnNetwork            = (errno::ENONET          , "Machine is not on the network"),
    PackageNotInstalled     = (errno::ENOPKG          , "Package not installed"),
    ObjectIsRemote          = (errno::EREMOTE         , "Object is remote"),
    LinkSevered             = (errno::ENOLINK         , "Link has been severed"),
    AdvertiseError          = (errno::EADV            , "Advertise error"),
    SrmountError            = (errno::ESRMNT          , "Srmount error"),
    CommunitacionError      = (errno::ECOMM           , "Communication error on send"),
    ProtocolError           = (errno::EPROTO          , "Protocol error"),
    MultihopAttempted       = (errno::EMULTIHOP       , "Multihop attempted"),
    RFSError                = (errno::EDOTDOT         , "RFS specific error"),
    NotADataMessage         = (errno::EBADMSG         , "Not a data message"),
    Overflow                = (errno::EOVERFLOW       , "Value too large for defined data type"),
    NotUnique               = (errno::ENOTUNIQ        , "Name not unique on network"),
    BadFileDescState        = (errno::EBADFD          , "File descriptor in bad state"),
    RemoteAddrChanged       = (errno::EREMCHG         , "Remote address changed"),
    SharedLibInaccessible   = (errno::ELIBACC         , "Can not access a needed shared library"),
    SharedLibCorrupted      = (errno::ELIBBAD         , "Accessing a corrupted shared library"),
    LibSectionCorrupted     = (errno::ELIBSCN         , ".lib section in a.out corrupted"),
    TooManySharedLibs       = (errno::ELIBMAX         , "Attempting to link in too many shared libraries"),
    SharedLibExec           = (errno::ELIBEXEC        , "Cannot exec a shared library directly"),
    InvalidSequence         = (errno::EILSEQ          , "Invalid sequence"),
    Restart                 = (errno::ERESTART        , "Interrupted system call should be restarted"),
    StreamPipeError         = (errno::ESTRPIPE        , "Streams pipe error"),
    TooManyUsers            = (errno::EUSERS          , "Too many users"),
    NotASocket              = (errno::ENOTSOCK        , "Argument is not a socket"),
    NoDefaultDestination    = (errno::EDESTADDRREQ    , "Connectionless socket has no destination"),
    MessageSize             = (errno::EMSGSIZE        , "Message too large"),
    ProtoNotSupported       = (errno::EPROTOTYPE      , "Protocol not supported by socket type"),
    OpNotSupported          = (errno::ENOPROTOOPT     , "Operation not supported by protocol"),
    ProtoNotSupported2      = (errno::EPROTONOSUPPORT , "Protocol not supported by socket domain"),
    SocketTypeNotSupported  = (errno::ESOCKTNOSUPPORT , "Socket type is not supported"),
    NotSupported            = (errno::EOPNOTSUPP      , "Operation not supported"),
    ProtoFamilyNotSupported = (errno::EPFNOSUPPORT    , "Protocol family not supported"),
    AddrFamilyNotSupported  = (errno::EAFNOSUPPORT    , "Address family not supported"),
    AddressInUse            = (errno::EADDRINUSE      , "Socket address already in use"),
    AddressNotAvailable     = (errno::EADDRNOTAVAIL   , "Socket address is not available"),
    NetworkDown             = (errno::ENETDOWN        , "Network is down"),
    NetworkUnreachable      = (errno::ENETUNREACH     , "Remote network is unreachable"),
    HostCrashed             = (errno::ENETRESET       , "Remote hast crashed"),
    ConnectionAborted       = (errno::ECONNABORTED    , "Connection locally aborted"),
    ConnectionReset         = (errno::ECONNRESET      , "Connection closed"),
    KernelBuffersBusy       = (errno::ENOBUFS         , "All kernel I/O buffers are in use"),
    SocketConnected         = (errno::EISCONN         , "Socket is already connected"),
    SocketNotConnected      = (errno::ENOTCONN        , "Socket is not connected"),
    SocketShutDown          = (errno::ESHUTDOWN       , "Socket has shut down"),
    TooManyReferences       = (errno::ETOOMANYREFS    , "Too many references"),
    SocketTimedOut          = (errno::ETIMEDOUT       , "Socket operation timed out"),
    ConnectionRefused       = (errno::ECONNREFUSED    , "Remote host is down"),
    HostDown                = (errno::EHOSTDOWN       , "Remote host is unreachable"),
    HostUnreachable         = (errno::EHOSTUNREACH    , "Remote host refused connection"),
    AlreadyInProgress       = (errno::EALREADY        , "Operation already in progress"),
    OperationInitiated      = (errno::EINPROGRESS     , "Operation initiated"),
    StaleFileHandle         = (errno::ESTALE          , "Stale file handle"),
    NeedsCleaning           = (errno::EUCLEAN         , "Structure needs cleaning"),
    NotXENIX                = (errno::ENOTNAM         , "Not a XENIX named type file"),
    NoXENIXSemaphores       = (errno::ENAVAIL         , "No XENIX semaphores available"),
    NamedTypeFile           = (errno::EISNAM          , "Is a named type file"),
    RemoteIOError           = (errno::EREMOTEIO       , "Remote I/O error"),
    DiskQuota               = (errno::EDQUOT          , "Disk quota exceeded"),
    NoMedium                = (errno::ENOMEDIUM       , "No medium found"),
    WrongMediumType         = (errno::EMEDIUMTYPE     , "Wrong medium type"),
    OperationCanceled       = (errno::ECANCELED       , "Asynchronous operation canceled"),
    KeyNotAvailable         = (errno::ENOKEY          , "Required key not available"),
    KeyExpired              = (errno::EKEYEXPIRED     , "Key has expired"),
    KeyRevoked              = (errno::EKEYREVOKED     , "Key has been revoked"),
    KeyRejected             = (errno::EKEYREJECTED    , "Key was rejected by service"),
    OwnerDied               = (errno::EOWNERDEAD      , "Owner died"),
    IrrecoverableState      = (errno::ENOTRECOVERABLE , "State not recoverable"),
    RFKill                  = (errno::ERFKILL         , "Operation not possible due to RF-kill"),
    HardwarePoison          = (errno::EHWPOISON       , "Memory page has hardware error"),
}
