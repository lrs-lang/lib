// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! An error type with corresponding constants.

pub use lrs_base::error::{Errno};

pub use lrs_base::error::{
    AccessDenied, AddrFamilyNotSupported, AddressInUse, AddressNotAvailable,
    AdvertiseError, AlreadyInProgress, BadFileDesc, BadFileDescState, BadFontFileFormat,
    BrokenPipe, ChannelOutOfRange, CommunitacionError, ConnectionAborted,
    ConnectionRefused, ConnectionReset, CrossFileSystemLink, DeadlockAvoided, DeviceFull,
    DiskQuota, DoesNotExist, DomainError, ExchangeFull, ExecutableBusy, FileExists,
    FileTooBig, HardwarePoison, HostCrashed, HostDown, HostUnreachable, IdentifierRemoved,
    InputOutput, Interrupted, InvalidArgument, InvalidExchange, InvalidExecutable,
    InvalidPointer, InvalidReqDesc, InvalidRequestCode, InvalidSeek, InvalidSequence,
    InvalidSlot, IrrecoverableState, IsADirectory, KernelBuffersBusy, KeyExpired,
    KeyNotAvailable, KeyRejected, KeyRevoked, Level2Halted, Level2NotSync, Level3Halted,
    Level3Reset, LibSectionCorrupted, LinkNumberOutOfRange, LinkSevered, MessageSize,
    MultihopAttempted, NamedTypeFile, NeedsCleaning, NetworkDown, NetworkUnreachable,
    NoAnode, NoBlockSpecialFile, NoCSIStructAvailable, NoChildProcesses, NoDataAvailable,
    NoDefaultDestination, NoLocksAvailable, NoMedium, NoMemory, NoMessageOfType,
    NoSuchDevice, NoSuchProcess, NoXENIXSemaphores, NotADataMessage, NotADirectory,
    NotASocket, NotAStream, NotATerminal, NotEmpty, NotImplemented, NotOnNetwork,
    NotPermitted, NotSupported, NotUnique, NotXENIX, ObjectIsRemote, OpNotSupported,
    OperationCanceled, OperationInitiated, OutOfStreamsResources, Overflow, OwnerDied,
    PackageNotInstalled, PathTooLong, ProcessFileLimit, ProtoDriverNotAttached,
    ProtoFamilyNotSupported, ProtoNotSupported, ProtoNotSupported2, ProtocolError, RFKill,
    RFSError, RangeError, ReadOnlyFileSystem, RemoteAddrChanged, RemoteIOError,
    ResourceBusy, Restart, SharedLibCorrupted, SharedLibExec, SharedLibInaccessible,
    SocketConnected, SocketNotConnected, SocketShutDown, SocketTimedOut,
    SocketTypeNotSupported, SrmountError, StaleFileHandle, StreamPipeError,
    SystemFileLimit, TimerExpired, TooManyArguemnts, TooManyLinks, TooManyReferences,
    TooManySharedLibs, TooManySymlinks, TooManyUsers, WouldBlock, WrongDeviceType,
    WrongMediumType,
};
