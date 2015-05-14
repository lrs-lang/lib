// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[prelude_import] use base::prelude::*;
use cty::{
    c_int,
    MSG_OOB, MSG_PEEK, MSG_DONTROUTE, MSG_CTRUNC, MSG_TRUNC, MSG_DONTWAIT,
    MSG_EOR, MSG_WAITALL, MSG_CONFIRM, MSG_ERRQUEUE, MSG_MORE, MSG_WAITFORONE,
    MSG_FASTOPEN, MSG_NOSIGNAL, MSG_CMSG_CLOEXEC,
};
use fmt::{Debug, Write};
use core::ops::{BitOr, BitAnd, Not};

/// Per-message flags.
///
/// [field, 1]
/// The integer constant representing the flags.
///
/// = Remarks
///
/// :msg: link:lrs::socket::msg
///
/// See {msg} for pre-defined constants.
///
/// = See also
///
/// * {msg}
#[derive(Pod, Eq)]
pub struct Flags(pub c_int);

impl BitAnd for Flags {
    type Output = Flags;
    fn bitand(self, rhs: Flags) -> Flags { Flags(self.0 & rhs.0) }
}

impl BitOr for Flags {
    type Output = Flags;
    fn bitor(self, rhs: Flags) -> Flags { Flags(self.0 | rhs.0) }
}

impl Not for Flags {
    type Output = Flags;
    fn not(self) -> Flags { Flags(!self.0) }
}

macro_rules! create {
    ($($(#[$meta:meta])* flag $name:ident = $val:expr;)*) => {
        $($(#[$meta])* pub const $name: Flags = Flags($val);)*

        /// = Remarks
        ///
        /// This prints the flags as a comma-separated list.
        impl Debug for Flags {
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
    #[doc = "No flags."]
    #[doc = ""]
    #[doc = "= Remarks"]
    #[doc = ""]
    #[doc = "This can be used whenever no special flags are required."]
    flag None = 0;
    
    #[doc = "Informs the link-layer that we're receiving messages from the peer."]
    #[doc = ""]
    #[doc = "= Remarks"]
    #[doc = ""]
    #[doc = "This can be used when sending UDP or raw messages so that the link-layer does not"]
    #[doc = "update the MAC address of the peer."]
    #[doc = ""]
    #[doc = "= See also"]
    #[doc = ""]
    #[doc = "* link:man:sendmsg(2) and MSG_CONFIRM therein"]
    flag Confirm = MSG_CONFIRM;

    #[doc = "Tells the kernel not to route this message."]
    #[doc = ""]
    #[doc = "= Remarks"]
    #[doc = ""]
    #[doc = "This can be used when sending message.  If set, messages are only sent to systems"]
    #[doc = "that are directly connected to this system without routers in between."]
    #[doc = ""]
    #[doc = "= See also"]
    #[doc = ""]
    #[doc = "* link:man:sendmsg(2) and MSG_DONTROUTE therein"]
    flag DontRoute = MSG_DONTROUTE;

    #[doc = "Tells the kernel not to block on system calls."]
    #[doc = ""]
    #[doc = "= Remarks"]
    #[doc = ""]
    #[doc = "This can be used in all socket-related system function calls. If set, the kernel"]
    #[doc = "will not suspend the calling thread if an operation would block and instead"]
    #[doc = "returns an error."]
    #[doc = ""]
    #[doc = "= See also"]
    #[doc = ""]
    #[doc = "* link:man:sendmsg(2) and MSG_DONTWAIT therein"]
    flag DontBlock = MSG_DONTWAIT;

    #[doc = "Marks the end of a SeqPacket record."]
    #[doc = ""]
    #[doc = "= Remarks"]
    #[doc = ""]
    #[doc = "This can be used with SeqPacket sockets when sending messages and appears in flags"]
    #[doc = "of received messages. It marks the end of a SeqPacket record."]
    #[doc = ""]
    #[doc = "= See also"]
    #[doc = ""]
    #[doc = "* link:man:sendmsg(2) and MSG_EOR therein"]
    #[doc = "* link:man:recvmsg(2) and MSG_EOR therein"]
    flag EndOfRecord = MSG_EOR;

    #[doc = "Tells the kernel not to send this message yet."]
    #[doc = ""]
    #[doc = "= Remarks"]
    #[doc = ""]
    #[doc = "This can be used when sending messages to assemble a message inside the kernel."]
    #[doc = "The kernel will not send the message until it a send call without this flag"]
    #[doc = "happens."]
    #[doc = ""]
    #[doc = "= See also"]
    #[doc = ""]
    #[doc = "* link:man:sendmsg(2) and MSG_MORE therein"]
    #[doc = "* link:lrs::socket::Socket::tcp_set_cork"]
    #[doc = "* link:lrs::socket::Socket::udp_set_cork"]
    flag More = MSG_MORE;

    #[doc = "Marks out-of-band data."]
    #[doc = ""]
    #[doc = "= Remarks"]
    #[doc = ""]
    #[doc = "This can be used when sending and receiving messages and appears in flags of"]
    #[doc = "received messages."]
    #[doc = ""]
    #[doc = "When sending messages, this flag marks the message as out-of-band data."]
    #[doc = ""]
    #[doc = "When receiving messages, this flag requests out-of-band data instead of regular"]
    #[doc = "data."]
    #[doc = ""]
    #[doc = "In message flags this flag signals that the message is out-of-band data."]
    #[doc = ""]
    #[doc = "= See also"]
    #[doc = ""]
    #[doc = "* link:man:sendmsg(2) and MSG_OOB therein"]
    #[doc = "* link:man:recvmsg(2) and MSG_OOB therein"]
    flag OutOfBand = MSG_OOB;

    flag ErrorQueue = MSG_ERRQUEUE;

    #[doc = "Receives data without removing it from the kernel queue."]
    #[doc = ""]
    #[doc = "= Remarks"]
    #[doc = ""]
    #[doc = "This can be used when receiving messages to read data while keeping the data"]
    #[doc = "available for further read calls."]
    #[doc = ""]
    #[doc = ":peek_off: link:lrs::socket::Socket::set_peek_offset[set_peek_offset]"]
    #[doc = ""]
    #[doc = "On Unix sockets this can be used together with {peek_off} to read from arbitrary"]
    #[doc = "positions in the stream."]
    #[doc = ""]
    #[doc = "= See also"]
    #[doc = ""]
    #[doc = "* link:man:recvmsg(2) and MSG_PEEK therein"]
    #[doc = "* {peek_off}"]
    flag Peek = MSG_PEEK;

    #[doc = "Tells the kernel to use read sizes instead of truncated sizes."]
    #[doc = ""]
    #[doc = "= Remarks"]
    #[doc = ""]
    #[doc = "This can be used when receiving on Udp or raw sockets and appears in message"]
    #[doc = "flags. When used with receiving calls, it tells the kernel to return the real size"]
    #[doc = "of the received message instead of the truncated one. If it appears in a message,"]
    #[doc = "then it means that a part of the message has been discarded because the provided"]
    #[doc = "buffer was too small."]
    #[doc = ""]
    #[doc = "= Examples"]
    #[doc = ""]
    #[doc = "----"]
    #[doc = "use lrs::socket::msg::{RealSize};"]
    #[doc = ""]
    #[doc = "let socket = {"]
    #[doc = "    // Create a datagram socket"]
    #[doc = "};"]
    #[doc = ""]
    #[doc = "let mut buf = [0; 128];"]
    #[doc = "let (size, _, _, flags) = socket.recv_msg(&mut buf, &mut [], &mut [],"]
    #[doc = "                                          RealSize).unwrap();"]
    #[doc = "if flags.is_set(RealSize) {"]
    #[doc = "    println!(\"Received a truncated message: buffer size: {}, real size: {}\","]
    #[doc = "             buf.len(), size);"]
    #[doc = "}"]
    #[doc = "----"]
    #[doc = ""]
    #[doc = "= See also"]
    #[doc = ""]
    #[doc = "* link:man:recvmsg(2) and MSG_TRUNC therein"]
    flag RealSize = MSG_TRUNC;

    #[doc = "Requests the full buffer to be filled."]
    #[doc = ""]
    #[doc = "= Remarks"]
    #[doc = ""]
    #[doc = "This can be used when receiving messages. It tells the kernel to fill the full"]
    #[doc = "provided buffer with data before returning."]
    #[doc = ""]
    #[doc = "= See also"]
    #[doc = ""]
    #[doc = "* link:man:recvmsg(2) and MSG_WAITALL therein"]
    flag WaitAll = MSG_WAITALL;

    flag WaitForOne = MSG_WAITFORONE;

    #[doc = "Marks truncated control messages."]
    #[doc = ""]
    #[doc = "= Remarks"]
    #[doc = ""]
    #[doc = "This flag appears in received messages if some control data was truncated because"]
    #[doc = "the provided buffer was too small."]
    #[doc = ""]
    #[doc = "= See also"]
    #[doc = ""]
    #[doc = "* link:man:recvmsg(2) and MSG_CTRUNC therein"]
    flag CMsgTruncated = MSG_CTRUNC;

    flag FastOpen = MSG_FASTOPEN;
    flag NoSignal = MSG_NOSIGNAL;
    flag CMsgCloexec = MSG_CMSG_CLOEXEC;
}
