// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use core::{mem};
use cty::{c_int, signalfd_siginfo};
use syscall::{close, read, signalfd4};
use fd::{FDContainer};
use rv::{retry};
use {Sigset};

#[repr(C)]
#[derive(Pod)]
pub struct SigfdData {
    pub signo    : u32,
    pub errno    : i32,
    pub code     : i32,
    pub pid      : u32,
    pub uid      : u32,
    pub fd       : i32,
    pub tid      : u32,
    pub band     : u32,
    pub overrun  : u32,
    pub trapno   : u32,
    pub status   : i32,
    pub int      : i32,
    pub ptr      : u64,
    pub utime    : u64,
    pub stime    : u64,
    pub addr     : u64,
    pub addr_lsb : u16,
    pub padding : [u8; 46],
}

pub struct Sigfd {
    fd: c_int,
    owned: bool,
}

impl Sigfd {
    pub fn new(set: Sigset, flags: flags::SigfdFlags) -> Result<Sigfd> {
        let fd = try!(rv!(signalfd4(-1, &set.data, flags.0), -> c_int));
        Ok(Sigfd { fd: fd, owned: true })
    }

    pub fn read(
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
    #[prelude_import] use base::prelude::*;
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
