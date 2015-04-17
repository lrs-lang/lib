// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[macro_export]
macro_rules! rv {
    ($x:expr) => {
        if $x < 0 {
            Err(::core::errno::Errno(-$x as ::core::cty::c_int))
        } else {
            Ok(())
        }
    };
    ($x:expr, -> $t:ty) => {
        if $x < 0 {
            Err(::core::errno::Errno(-$x as ::core::cty::c_int))
        } else {
            Ok($x as $t)
        }
    };
}
