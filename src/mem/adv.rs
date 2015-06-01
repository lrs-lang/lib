// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(non_upper_case_globals, non_camel_case_types)]

#[prelude_import] use base::prelude::*;
use core::ops::{Range};
use base::{error};
use fmt::{Debug, Write};
use cty::{
    self, c_int,
};
use syscall::{madvise};

#[derive(Pod, Eq)]
pub struct MemAdvice(pub c_int);

macro_rules! create {
    ($($(#[$meta:meta])* adv $name:ident = $val:ident;)*) => {
        $($(#[$meta])* pub const $name: MemAdvice = MemAdvice(cty::$val);)*

        impl Debug for MemAdvice {
            fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
                let s = match *self {
                    $($name => stringify!($name),)*
                    _ => return write!(w, "Unknown({})", self.0),
                };
                w.write_all(s.as_bytes()).ignore_ok()
            }
        }
    }
}

create! {
    #[doc = "No special treatment.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:madvise(2) and MADV_NORMAL therein"]
    adv Normal = MADV_NORMAL;

    #[doc = "Expect random access.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:madvise(2) and MADV_RANDOM therein"]
    adv Random = MADV_RANDOM;

    #[doc = "Expect sequential access.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:madvise(2) and MADV_SEQUENTIAL therein"]
    adv Sequential = MADV_SEQUENTIAL;

    #[doc = "Expect access in the near future.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:madvise(2) and MADV_WILLNEED therein"]
    adv WillNeed = MADV_WILLNEED;

    #[doc = "Discard these pages until the are accessed again (see Remarks).\n"]
    #[doc = "= Remarks"]
    #[doc = "Please note the discussion in the manpage regarding private anonymous \
             mappings. Using this function on random ranges will cause memory unsafety."]
    #[doc = "= See also"]
    #[doc = "* link:man:madvise(2) and MADV_DONTNEED therein"]
    adv DontNeed = MADV_DONTNEED;

    #[doc = "Free the given pages.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:madvise(2) and MADV_REMOVE therein"]
    adv Remove = MADV_REMOVE;

    #[doc = "Don't make these pages available to child processes.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:madvise(2) and MADV_DONTFORK therein"]
    adv DontFork = MADV_DONTFORK;

    #[doc = "Make these pages available to child processes.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:madvise(2) and MADV_DOFORK therein"]
    adv DoFork = MADV_DOFORK;

    #[doc = "Poison these pages.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:madvise(2) and MADV_POISON therein"]
    adv HwPoison = MADV_HWPOISON;

    #[doc = "Soft offline these pages.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:madvise(2) and MADV_SOFT_OFFLINE therein"]
    adv SoftOffline = MADV_SOFT_OFFLINE;

    #[doc = "Allow these pages to be merged with other pages with the same content.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:madvise(2) and MADV_MERGEABLE therein"]
    adv Mergeable = MADV_MERGEABLE;

    #[doc = "Don't allow these pages to be merged with other pages with the same \
             content.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:madvise(2) and MADV_UNMERGEABLE therein"]
    adv Unmergeable = MADV_UNMERGEABLE;

    #[doc = "Enable transparent huge pages for these pages.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:madvise(2) and MADV_HUGEPAGE therein"]
    adv HugePage = MADV_HUGEPAGE;

    #[doc = "Disable transparent huge pages for these pages.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:madvise(2) and MADV_NOHUGEPAGE therein"]
    adv NoHugePage = MADV_NOHUGEPAGE;

    #[doc = "Exclude these pages from coredumps.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:madvise(2) and MADV_DONTDUMP therein"]
    adv DontDump = MADV_DONTDUMP;

    #[doc = "Include these pages in coredumps.\n"]
    #[doc = "= See also"]
    #[doc = "* link:man:madvise(2) and MADV_DODUMP therein"]
    adv DoDump = MADV_DODUMP;
}

/// Advise the kernel of a certain memory usage pattern.
///
/// [argument, range]
/// The range for which the advice holds. Must be page-aligned.
///
/// [argument, advice]
/// The advice given.
///
/// = Remarks
///
/// The `DontNeed`, `Remove`, `DontFork` and `HwPoison` advices cannot be used safely.
/// Trying to use them with this interface causes a process abort.
///
/// = See also
///
/// * link:man:madvise(2)
pub fn advise(range: Range<usize>, advice: MemAdvice) -> Result {
    if range.start > range.end {
        return Err(error::InvalidArgument);
    }
    match advice {
        DontNeed | Remove | DontFork | HwPoison => abort!(),
        _ => { },
    }
    unsafe { rv!(madvise(range.start, range.end - range.start, advice.0)) }
}
