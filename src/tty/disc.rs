// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(non_upper_case_globals, non_camel_case_types)]

#[prelude_import] use base::prelude::*;
use fmt::{Debug, Write};
use cty::{
    self, c_int,
};

#[derive(Pod, Eq)]
pub struct LineDiscipline(pub c_int);

macro_rules! create {
    ($($(#[$meta:meta])* disc $name:ident = $val:ident;)*) => {
        $($(#[$meta])* pub const $name: LineDiscipline = LineDiscipline(cty::$val);)*

        impl Debug for LineDiscipline {
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
    disc Tty         = N_TTY;
    disc Slip        = N_SLIP;
    disc Mouse       = N_MOUSE;
    disc Ppp         = N_PPP;
    disc Strip       = N_STRIP;
    disc Ax25        = N_AX25;
    disc X25         = N_X25;
    disc SixPack     = N_6PACK;
    disc Masc        = N_MASC;
    disc R3964       = N_R3964;
    disc ProfibusFdl = N_PROFIBUS_FDL;
    disc Irda        = N_IRDA;
    disc Smsblock    = N_SMSBLOCK;
    disc Hdlc        = N_HDLC;
    disc SyncPpp     = N_SYNC_PPP;
    disc Hci         = N_HCI;
    disc GigasetM101 = N_GIGASET_M101;
    disc Slcan       = N_SLCAN;
    disc Pps         = N_PPS;
    disc V253        = N_V253;
    disc Caif        = N_CAIF;
    disc Gsm0710     = N_GSM0710;
    disc TiWl        = N_TI_WL;
    disc TraceSink   = N_TRACESINK;
    disc TraceRouter = N_TRACEROUTER;
}
