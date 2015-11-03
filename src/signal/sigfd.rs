// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use base::prelude::*;
use core::{mem};
use base::undef::{UndefState};
use cty::{c_int};
use syscall::{close, read, signalfd4};
use fd::{FDContainer};
use rv::{retry};
use {Sigset};
use signals::{Signal};

/// Information about a signal received via a `Sigfd`.
#[repr(C)]
#[derive(Pod)]
pub struct SigfdInfo {
    /// The signal number.
    pub signo    : u32,
    /// Unused.
    pub errno    : i32,
    /// Signal code.
    pub code     : i32,
    /// Sending process id.
    pub pid      : u32,
    /// Real user id of the sending process.
    pub uid      : u32,
    /// Ready file descriptor.
    pub fd       : i32,
    /// Timer id.
    pub tid      : u32,
    /// Band event.
    pub band     : u32,
    /// Timer overrruns.
    pub overrun  : u32,
    /// Trap number.
    pub trapno   : u32,
    /// Exit status.
    pub status   : i32,
    /// Integer.
    pub int      : i32,
    /// Pointer.
    pub ptr      : u64,
    /// User CPU time consumed.
    pub utime    : u64,
    /// System CPU time consumed.
    pub stime    : u64,
    /// Address that generated the signal.
    pub addr     : u64,
    /// Least significant bit of address.
    pub addr_lsb : u16,
    pub padding : [u8; 46],
}

impl SigfdInfo {
    /// Returns the signal that generated the information.
    pub fn signal(&self) -> Signal {
        Signal(self.signo as u8)
    }

    /// Creates a new, empty SigfdInfo.
    pub fn new() -> SigfdInfo {
        mem::zeroed()
    }
}

/// A signalfd.
pub struct Sigfd {
    fd: c_int,
    owned: bool,
}

impl Sigfd {
    /// Creates a new signalfd.
    ///
    /// [argument, set]
    /// The set of signals to watch.
    ///
    /// [agument, flags]
    /// Flags used when creating the file descriptor.
    ///
    /// [return_value]
    /// Returns a new signalfd.
    pub fn new(set: Sigset, flags: flags::SigfdFlags) -> Result<Sigfd> {
        let fd = try!(rv!(signalfd4(-1, &set.data, flags.0), -> c_int));
        Ok(Sigfd { fd: fd, owned: true })
    }

    /// Sets the mask of monitored signals.
    ///
    /// [argument, set]
    /// The new set of monitored signals.
    pub fn set_mask(&self, set: Sigset) -> Result {
        rv!(signalfd4(self.fd, &set.data, 0))
    }

    /// Reads a number of signals from the signalfd.
    ///
    /// [argument, buf]
    /// The buffer in which the signals will be stored.
    ///
    /// [return_value]
    /// Returns a slice of received signals.
    pub fn read<'a>(&self, buf: &'a mut [SigfdInfo]) -> Result<&'a mut [SigfdInfo]> {
        let len = try!(retry(|| read(self.fd, buf.as_mut_bytes())));
        let num = len as usize / mem::size_of::<SigfdInfo>();
        Ok(&mut buf[..num])
    }
}

unsafe impl UndefState for Sigfd {
    fn num() -> usize { bool::num() }

    unsafe fn set_undef(val: *mut Sigfd, n: usize) {
        bool::set_undef(&mut (*val).owned, n);
    }

    unsafe fn is_undef(val: *const Sigfd, n: usize) -> bool {
        bool::is_undef(&(*val).owned, n)
    }
}

impl Drop for Sigfd {
    fn drop(&mut self) {
        if self.owned {
            close(self.fd);
        }
    }
}

impl FDContainer for Sigfd {
    fn unwrap(self) -> c_int {
        let fd = self.fd;
        mem::forget(self);
        fd
    }

    fn is_owned(&self) -> bool {
        self.owned
    }

    fn borrow(&self) -> c_int {
        self.fd
    }

    fn from_owned(fd: c_int) -> Sigfd {
        Sigfd { fd: fd, owned: true }
    }

    fn from_borrowed(fd: c_int) -> Sigfd {
        Sigfd { fd: fd, owned: false }
    }
}

pub mod flags {
    use base::prelude::*;
    use cty::{
        c_int, SFD_NONBLOCK, SFD_CLOEXEC,
    };
    use fmt::{Debug, Write};
    use core::ops::{BitOr, BitAnd, Not};

    /// Flags that can be used when creating a Sigfd.
    ///
    /// [field, 1]
    /// The integer constant associated with the flags.
    ///
    /// = Remarks
    ///
    /// :flags: link:lrs::signal::fd::flags
    ///
    /// See {flags} for pre-defined constants.
    ///
    /// = See also
    ///
    /// * {flags}
    #[derive(Pod, Eq)]
    pub struct SigfdFlags(pub c_int);

    impl BitAnd for SigfdFlags {
        type Output = SigfdFlags;
        fn bitand(self, rhs: SigfdFlags) -> SigfdFlags { SigfdFlags(self.0 & rhs.0) }
    }

    impl BitOr for SigfdFlags {
        type Output = SigfdFlags;
        fn bitor(self, rhs: SigfdFlags) -> SigfdFlags { SigfdFlags(self.0 | rhs.0) }
    }

    impl Not for SigfdFlags {
        type Output = SigfdFlags;
        fn not(self) -> SigfdFlags { SigfdFlags(!self.0) }
    }

    /// Dummy flag with all flags unset.
    pub const SIGFD_NONE: SigfdFlags = SigfdFlags(0);


    macro_rules! create {
        ($($(#[$meta:meta])* flag $name:ident = $val:expr;)*) => {
            $($(#[$meta])*  pub const $name: SigfdFlags = SigfdFlags($val);)*

            /// = Remarks
            ///
            /// This prints the flags as a comma-separated list.
            impl Debug for SigfdFlags {
                fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
                    let raw = self.0;
                    const KNOWN_FLAGS: c_int = 0 $(| $val)*;
                    if raw & !KNOWN_FLAGS != 0 {
                        return write!(w, "0x{:x}", raw as u32);
                    }
                    let mut first = true;
                    $(
                        if raw & $val != 0 {
                            if !first { try!(w.write(b",")); }
                            first = false;
                            try!(w.write_all(stringify!($name).as_bytes()));
                        }
                    )*
                    let _ = first;
                    Ok(())
                }
            }
        }
    }

    create! {
        #[doc = "Sets the file descriptor to non-blocking"]
        #[doc = ""]
        #[doc = "= See also"]
        #[doc = ""]
        #[doc = "* link:man:signalfd(2) and SFD_NONBLOCK therein"]
        flag SIGFD_DONT_BLOCK = SFD_NONBLOCK;

        #[doc = "Sets the close-on-exec flag on the file descriptor"]
        #[doc = ""]
        #[doc = "= Remarks"]
        #[doc = ""]
        #[doc = "This flag will always be set."]
        #[doc = ""]
        #[doc = "= See also"]
        #[doc = ""]
        #[doc = "* link:man:signalfd(2) and SFD_CLOEXEC therein"]
        flag SIGFD_CLOSE_ON_EXEC = SFD_CLOEXEC;
    }
}
