// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use cty::{
    c_int, SWAP_FLAG_PREFER, SWAP_FLAG_DISCARD,
};
use fmt::{Debug, Write};
use core::ops::{BitOr, BitAnd, Not};

/// Flags that can be used when creating a swap file/device.
///
/// [field, 1]
/// The integer constant associated with the flags.
///
/// = Remarks
///
/// :flags: link:lrs::swap::flags
///
/// See {flags} for pre-defined constants.
///
/// = See also
///
/// * {flags}
#[derive(Pod, Eq)]
pub struct SwapFlags(pub c_int);

impl BitAnd for SwapFlags {
    type Output = SwapFlags;
    fn bitand(self, rhs: SwapFlags) -> SwapFlags { SwapFlags(self.0 & rhs.0) }
}

impl BitOr for SwapFlags {
    type Output = SwapFlags;
    fn bitor(self, rhs: SwapFlags) -> SwapFlags { SwapFlags(self.0 | rhs.0) }
}

impl Not for SwapFlags {
    type Output = SwapFlags;
    fn not(self) -> SwapFlags { SwapFlags(!self.0) }
}

/// Dummy flag with all flags unset.
pub const SWAP_NONE: SwapFlags = SwapFlags(0);

macro_rules! create {
    ($($(#[$meta:meta])* flag $name:ident = $val:expr;)*) => {
        $($(#[$meta])*  pub const $name: SwapFlags = SwapFlags($val);)*

        impl Debug for SwapFlags {
            fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
                let raw = self.0;
                const KNOWN_FLAGS: c_int = 0 $(| $val)*;
                if raw & !KNOWN_FLAGS != 0 {
                    return write!(w, "0x{:x}", raw as u32);
                }
                let mut first = true;
                $(
                    if raw & $val != 0 {
                        if !first { try!(w.write(b"|")); }
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
    #[doc = "Set a higher than default preference.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:swapon(2) and SWAP_FLAG_PREFER therein"]
    flag SWAP_PREFER = SWAP_FLAG_PREFER;

    #[doc = "Discard freed swap pages before reusing them.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:swapon(2) and SWAP_FLAG_DISCARD therein"]
    flag SWAP_DISCARD = SWAP_FLAG_DISCARD;
}

