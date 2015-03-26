// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate linux;

fn main() {
    //let info = linux::fs::info::FileSystemInfo::from_path("/sys");
    let file = linux::file::File::open_read("/").unwrap();
    let info = file.fs_info();
    println!("{:?}", info);
}
