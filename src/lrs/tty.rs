// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use lrs_tty::{Tty, is_a_tty, hang_up};
pub use lrs_tty::attr::{TtyAttr, TtyInFlags, TtyOutFlags, TtyCtrlFlags, TtyLocFlags};
pub use lrs_tty::disc::{LineDiscipline};
pub use lrs_tty::key::{TtyKey};

pub mod flags {
    pub use lrs_tty::attr::{
        TTYIN_NONE, TTYIN_IGNORE_BREAK, TTYIN_BREAK_TO_INT, TTYIN_IGNORE_ERRORS,
        TTYIN_MARK_ERRORS, TTYIN_CHECK_INPUT, TTYIN_TO_LOWER, TTYIN_TO_ASCII,
        TTYIN_NL_TO_CR, TTYIN_IGNORE_CR, TTYIN_CR_TO_NL, TTYIN_OUTPUT_FLOW_CTRL,
        TTYIN_INPUT_FLOW_CTRL, TTYIN_ANY_RESTART, TTYIN_UTF8, TTYOUT_NONE, TTYOUT_PROCESS,
        TTYOUT_TO_UPPER, TTYOUT_NL_TO_CRNL, TTYOUT_CR_TO_NL, TTYOUT_NO_COL0_CR,
        TTYOUT_NO_CR, TTYCTRL_NONE, TTYCTRL_CSTOPB, TTYCTRL_CREAD, TTYCTRL_PARENB,
        TTYCTRL_PARODD, TTYCTRL_HUPCL, TTYCTRL_CLOCAL, TTYCTRL_CMSPAR, TTYCTRL_CRTSCTS,
        TTYLOC_NONE, TTYLOC_SIGNALS, TTYLOC_CANONICAL, TTYLOC_ECHO, TTYLOC_ERASE,
        TTYLOC_KILL, TTYLOC_ECHO_NL, TTYLOC_ECHO_ESCAPED, TTYLOC_ECHOPRT, TTYLOC_ECHOKE,
        TTYLOC_NOFLSH, TTYLOC_TOSTOP, TTYLOC_IEXTEN,
    };
}

pub mod line_disc {
    pub use lrs_tty::disc::{
        Tty, Slip, Mouse, Ppp, Strip, Ax25, X25, SixPack, Masc, R3964, ProfibusFdl, Irda,
        Smsblock, Hdlc, SyncPpp, Hci, GigasetM101, Slcan, Pps, V253, Caif, Gsm0710, TiWl,
        TraceSink, TraceRouter,
    };
}

pub mod key {
    pub use lrs_tty::key::{
        Interrupt, Quit, EraseChar, EraseLine, EndOfFile, Timeout, MinInput, StartOutput,
        StopOutput, Suspend, Reprint, EraseWord, Escape, EndOfLine, EndOfLine2,
    };
}
