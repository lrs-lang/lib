// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Kernel pipes.

pub use lrs_pipe::{Pipe};
pub use lrs_pipe::flags::{PipeFlags, TeeFlags, SpliceFlags};

pub mod flags {
    pub use lrs_pipe::flags::{
        PIPE_NONE, PIPE_CLOSE_ON_EXEC, PIPE_DONT_BLOCK, PIPE_PACKETS,
        TEE_NONE, TEE_DONT_BLOCK,
        SPLICE_NONE, SPLICE_DONT_BLOCK, SPLICE_MORE,
    };
}
