// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use core::ops::{BitOr, Not, BitAnd};
use fmt::{Debug, Write};
use cty::{
    self, c_int,
};

/// MemMap flags.
#[derive(Pod, Eq)]
pub struct MemMapFlags(pub c_int);

impl BitOr for MemMapFlags {
    type Output = MemMapFlags;
    fn bitor(self, other: MemMapFlags) -> MemMapFlags {
        MemMapFlags(self.0 | other.0)
    }
}

impl BitAnd for MemMapFlags {
    type Output = MemMapFlags;
    fn bitand(self, other: MemMapFlags) -> MemMapFlags {
        MemMapFlags(self.0 & other.0)
    }
}

impl Not for MemMapFlags {
    type Output = MemMapFlags;
    fn not(self) -> MemMapFlags {
        MemMapFlags(!self.0)
    }
}

/// Dummy flag with all flags unset.
pub const MMAP_NONE: MemMapFlags = MemMapFlags(0);

macro_rules! create_flags {
    ($($(#[$meta:meta])* flag $name:ident = $val:ident;)*) => {
        $($(#[$meta])* pub const $name: MemMapFlags = MemMapFlags(cty::$val);)*

        impl Debug for MemMapFlags {
            fn fmt<W: Write>(&self, w: &mut W) -> Result {
                let mut first = true;
                $(
                    if self.0 & cty::$val != 0 {
                        if !first { try!(w.write(b"|")); }
                        first = false;
                        try!(w.write_all(stringify!($name).as_bytes()));
                    }
                )*
                if first { try!(w.write_all("MMAP_NONE".as_bytes())); }
                Ok(())
            }
        }
    }
}

create_flags! {
    // Exists only on x86
    // #[doc = "Put the mapping into the first 2GB of the address space.\n"]
    // #[doc = "= See also"]
    // #[doc = "* link:man:mmap(2) and MAP_32BIT therein"]
    // flag MMAP_32BIT = MAP_32BIT;

    #[doc = "Provide memory not backed by a file.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mmap(2) and MAP_ANONYMOUS therein"]
    flag MMAP_ANON = MAP_ANONYMOUS;

    #[doc = "Create a map at an exact place.\n"]
    #[doc = "= Remarks"]
    #[doc = "This flag is cannot be used safely. If you try using it via the safe \
             interface, the process will be aborted.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mmap(2) and MAP_FIXED therein"]
    flag MMAP_FIXED = MAP_FIXED;

    #[doc = "Allocate huge pages for this mapping.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mmap(2) and MAP_HUGETLB therein"]
    flag MMAP_HUGE_PAGES = MAP_HUGETLB;

    // Exists only on x86
    // #[doc = "Use 2MB pages for this mapping.\n"]
    // #[doc = "= Remarks"]
    // #[doc = "== Kernel versions"]
    // #[doc = "The minimum required kernel version is 3.8."]
    // #[doc = "= See also"]
    // #[doc = "* link:man:mmap(2) and MAP_HUGE_2MB therein"]
    // flag MMAP_HUGE_2MB = MAP_HUGE_2MB;

    // Exists only on x86
    // #[doc = "Use 1GB pages for this mapping.\n"]
    // #[doc = "= Remarks"]
    // #[doc = "== Kernel versions"]
    // #[doc = "The minimum required kernel version is 3.8."]
    // #[doc = "= See also"]
    // #[doc = "* link:man:mmap(2) and MAP_HUGE_1GB therein"]
    // flag MMAP_HUGE_1GB = MAP_HUGE_1GB;

    #[doc = "Lock the mapping into memory.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mmap(2) and MAP_LOCKED therein"]
    flag MMAP_LOCKED = MAP_LOCKED;

    #[doc = "Don't perform readahead.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mmap(2) and MAP_NONBLOCK therein"]
    flag MMAP_DONT_BLOCK = MAP_NONBLOCK;

    #[doc = "Don't reserve swap space for the mapping.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mmap(2) and MAP_NORESERVE therein"]
    flag MMAP_DONT_RESERVE = MAP_NORESERVE;

    #[doc = "Ensure that access of the mapping does not cause page faults.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mmap(2) and MAP_POPULATE therein"]
    flag MMAP_POPULATE = MAP_POPULATE;

    #[doc = "Don't clear anonymous pages.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mmap(2) and MAP_UNINITIALIZED therein"]
    flag MMAP_UNINITIALIZED = MAP_UNINITIALIZED;
}

impl MemMapFlags {
    /// Sets a flag.
    ///
    /// [argument, flag]
    /// The flag to be set.
    pub fn set(&mut self, flag: MemMapFlags) {
        self.0 |= flag.0
    }

    /// Clears a flag.
    ///
    /// [argument, flag]
    /// The flag to be cleared.
    pub fn unset(&mut self, flag: MemMapFlags) {
        self.0 &= !flag.0
    }

    /// Returns whether a flag is set.
    ///
    /// [argument, flag]
    /// The flag to be checked.
    pub fn is_set(&self, flag: MemMapFlags) -> bool {
        self.0 & flag.0 != 0
    }
}

/// Memory protection flags.
#[derive(Pod, Eq)]
pub struct MemProtFlags(pub c_int);

impl BitOr for MemProtFlags {
    type Output = MemProtFlags;
    fn bitor(self, other: MemProtFlags) -> MemProtFlags {
        MemProtFlags(self.0 | other.0)
    }
}

impl BitAnd for MemProtFlags {
    type Output = MemProtFlags;
    fn bitand(self, other: MemProtFlags) -> MemProtFlags {
        MemProtFlags(self.0 & other.0)
    }
}

impl Not for MemProtFlags {
    type Output = MemProtFlags;
    fn not(self) -> MemProtFlags {
        MemProtFlags(!self.0)
    }
}

/// Dummy flag with all flags unset.
pub const PROT_NONE: MemProtFlags = MemProtFlags(0);

macro_rules! create_flags {
    ($($(#[$meta:meta])* flag $name:ident = $val:ident;)*) => {
        $($(#[$meta])* pub const $name: MemProtFlags = MemProtFlags(cty::$val);)*

        impl Debug for MemProtFlags {
            fn fmt<W: Write>(&self, w: &mut W) -> Result {
                let mut first = true;
                $(
                    if self.0 & cty::$val != 0 {
                        if !first { try!(w.write(b"|")); }
                        first = false;
                        try!(w.write_all(stringify!($name).as_bytes()));
                    }
                )*
                if first { try!(w.write_all("PROT_NONE".as_bytes())); }
                Ok(())
            }
        }
    }
}

create_flags! {
    #[doc = "The memory region can be executed.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mmap(2) and PROT_EXEC therein"]
    flag PROT_EXEC = PROT_EXEC;

    #[doc = "The memory region can be read from.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mmap(2) and PROT_READ therein"]
    flag PROT_READ = PROT_READ;

    #[doc = "The memory region can be written to.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mmap(2) and PROT_WRITE therein"]
    flag PROT_WRITE = PROT_WRITE;
}

impl MemProtFlags {
    /// Sets a flag.
    ///
    /// [argument, flag]
    /// The flag to be set.
    pub fn set(&mut self, flag: MemProtFlags) {
        self.0 |= flag.0
    }

    /// Clears a flag.
    ///
    /// [argument, flag]
    /// The flag to be cleared.
    pub fn unset(&mut self, flag: MemProtFlags) {
        self.0 &= !flag.0
    }

    /// Returns whether a flag is set.
    ///
    /// [argument, flag]
    /// The flag to be checked.
    pub fn is_set(&self, flag: MemProtFlags) -> bool {
        self.0 & flag.0 != 0
    }
}

/// Flags for changing a mapping.
#[derive(Pod, Eq)]
pub struct MemReMapFlags(pub c_int);

impl BitOr for MemReMapFlags {
    type Output = MemReMapFlags;
    fn bitor(self, other: MemReMapFlags) -> MemReMapFlags {
        MemReMapFlags(self.0 | other.0)
    }
}

impl BitAnd for MemReMapFlags {
    type Output = MemReMapFlags;
    fn bitand(self, other: MemReMapFlags) -> MemReMapFlags {
        MemReMapFlags(self.0 & other.0)
    }
}

impl Not for MemReMapFlags {
    type Output = MemReMapFlags;
    fn not(self) -> MemReMapFlags {
        MemReMapFlags(!self.0)
    }
}

/// Dummy flag with all flags unset.
pub const MREMAP_NONE: MemReMapFlags = MemReMapFlags(0);

macro_rules! create_flags {
    ($($(#[$meta:meta])* flag $name:ident = $val:ident;)*) => {
        $($(#[$meta])* pub const $name: MemReMapFlags = MemReMapFlags(cty::$val);)*

        impl Debug for MemReMapFlags {
            fn fmt<W: Write>(&self, w: &mut W) -> Result {
                let mut first = true;
                $(
                    if self.0 & cty::$val != 0 {
                        if !first { try!(w.write(b"|")); }
                        first = false;
                        try!(w.write_all(stringify!($name).as_bytes()));
                    }
                )*
                if first { try!(w.write_all("MREMAP_NONE".as_bytes())); }
                Ok(())
            }
        }
    }
}

create_flags! {
    #[doc = "Move the mapping if it cannot be expanded in its current location.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mremap(2) and MREMAP_MAYMOVE therein"]
    flag MREMAP_MAY_MOVE = MREMAP_MAYMOVE;

    #[doc = "Create a map at an exact place.\n"]
    #[doc = "= Remarks"]
    #[doc = "This flag is cannot be used safely. If you try using it via the safe \
             interface, the process will be aborted.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mmap(2) and MREMAP_FIXED therein"]
    flag MREMAP_FIXED = MREMAP_FIXED;
}

impl MemReMapFlags {
    /// Sets a flag.
    ///
    /// [argument, flag]
    /// The flag to be set.
    pub fn set(&mut self, flag: MemReMapFlags) {
        self.0 |= flag.0
    }

    /// Clears a flag.
    ///
    /// [argument, flag]
    /// The flag to be cleared.
    pub fn unset(&mut self, flag: MemReMapFlags) {
        self.0 &= !flag.0
    }

    /// Returns whether a flag is set.
    ///
    /// [argument, flag]
    /// The flag to be checked.
    pub fn is_set(&self, flag: MemReMapFlags) -> bool {
        self.0 & flag.0 != 0
    }
}

/// Flags for synchronizing a file.
#[derive(Pod, Eq)]
pub struct MemSyncFlags(pub c_int);

impl BitOr for MemSyncFlags {
    type Output = MemSyncFlags;
    fn bitor(self, other: MemSyncFlags) -> MemSyncFlags {
        MemSyncFlags(self.0 | other.0)
    }
}

impl BitAnd for MemSyncFlags {
    type Output = MemSyncFlags;
    fn bitand(self, other: MemSyncFlags) -> MemSyncFlags {
        MemSyncFlags(self.0 & other.0)
    }
}

impl Not for MemSyncFlags {
    type Output = MemSyncFlags;
    fn not(self) -> MemSyncFlags {
        MemSyncFlags(!self.0)
    }
}

macro_rules! create_flags {
    ($($(#[$meta:meta])* flag $name:ident = $val:ident;)*) => {
        $($(#[$meta])* pub const $name: MemSyncFlags = MemSyncFlags(cty::$val);)*

        impl Debug for MemSyncFlags {
            fn fmt<W: Write>(&self, w: &mut W) -> Result {
                let mut first = true;
                $(
                    if self.0 & cty::$val != 0 {
                        if !first { try!(w.write(b"|")); }
                        first = false;
                        try!(w.write_all(stringify!($name).as_bytes()));
                    }
                )*
                if first { try!(w.write_all("invalid".as_bytes())); }
                Ok(())
            }
        }
    }
}

create_flags! {
    #[doc = "Move the mapping if it cannot be expanded in its current location.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mremap(2) and MREMAP_MAYMOVE therein"]
    flag MSYNC_ASYNC = MS_ASYNC;

    #[doc = "Move the mapping if it cannot be expanded in its current location.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mremap(2) and MREMAP_MAYMOVE therein"]
    flag MSYNC_SYNC = MS_SYNC;

    #[doc = "Move the mapping if it cannot be expanded in its current location.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mremap(2) and MREMAP_MAYMOVE therein"]
    flag MSYNC_INVALIDATE = MS_INVALIDATE;
}

impl MemSyncFlags {
    /// Sets a flag.
    ///
    /// [argument, flag]
    /// The flag to be set.
    pub fn set(&mut self, flag: MemSyncFlags) {
        self.0 |= flag.0
    }

    /// Clears a flag.
    ///
    /// [argument, flag]
    /// The flag to be cleared.
    pub fn unset(&mut self, flag: MemSyncFlags) {
        self.0 &= !flag.0
    }

    /// Returns whether a flag is set.
    ///
    /// [argument, flag]
    /// The flag to be checked.
    pub fn is_set(&self, flag: MemSyncFlags) -> bool {
        self.0 & flag.0 != 0
    }
}

/// Flags for locking pages in memory.
#[derive(Pod, Eq)]
pub struct MemLockFlags(pub c_int);

impl BitOr for MemLockFlags {
    type Output = MemLockFlags;
    fn bitor(self, other: MemLockFlags) -> MemLockFlags {
        MemLockFlags(self.0 | other.0)
    }
}

impl BitAnd for MemLockFlags {
    type Output = MemLockFlags;
    fn bitand(self, other: MemLockFlags) -> MemLockFlags {
        MemLockFlags(self.0 & other.0)
    }
}

impl Not for MemLockFlags {
    type Output = MemLockFlags;
    fn not(self) -> MemLockFlags {
        MemLockFlags(!self.0)
    }
}

macro_rules! create_flags {
    ($($(#[$meta:meta])* flag $name:ident = $val:ident;)*) => {
        $($(#[$meta])* pub const $name: MemLockFlags = MemLockFlags(cty::$val);)*

        impl Debug for MemLockFlags {
            fn fmt<W: Write>(&self, w: &mut W) -> Result {
                let mut first = true;
                $(
                    if self.0 & cty::$val != 0 {
                        if !first { try!(w.write(b"|")); }
                        first = false;
                        try!(w.write_all(stringify!($name).as_bytes()));
                    }
                )*
                if first { try!(w.write_all("invalid".as_bytes())); }
                Ok(())
            }
        }
    }
}

create_flags! {
    #[doc = "Lock all currently mapped pages.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mlockall(2) and MCL_CURRENT therein"]
    flag MLOCK_CURRENT = MCL_CURRENT;

    #[doc = "Lock all pages that will become mapped in the future.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:mlockall(2) and MCL_FUTURE therein"]
    flag MLOCK_FUTURE = MCL_FUTURE;
}

impl MemLockFlags {
    /// Sets a flag.
    ///
    /// [argument, flag]
    /// The flag to be set.
    pub fn set(&mut self, flag: MemLockFlags) {
        self.0 |= flag.0
    }

    /// Clears a flag.
    ///
    /// [argument, flag]
    /// The flag to be cleared.
    pub fn unset(&mut self, flag: MemLockFlags) {
        self.0 &= !flag.0
    }

    /// Returns whether a flag is set.
    ///
    /// [argument, flag]
    /// The flag to be checked.
    pub fn is_set(&self, flag: MemLockFlags) -> bool {
        self.0 & flag.0 != 0
    }
}
