// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Type markers

pub use linux_base::marker::{
    Sized, Copy, Sync, NoSync, Send, NoSend, Leak, PhantomData,
};
