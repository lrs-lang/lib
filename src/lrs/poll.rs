// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! File descriptor polling.

pub use lrs_poll::{
    PollFlags, Event, Epoll, EMPTY_EVENT, POLL_READ, POLL_WRITE, POLL_READ_HANG_UP,
    POLL_PRIORITY, POLL_EDGE_TRIGGERED, POLL_ONE_SHOT, POLL_WAKE_UP,
};
