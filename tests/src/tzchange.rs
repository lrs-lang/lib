extern crate linux;

use linux::{file, time};

fn main() {
    let info = file::info_no_follow("/etc/localtime").unwrap();
    let last_mod = info.last_modification();
    let tokyo = time::Zone::load("Asia/Tokyo").unwrap();
    let expanded = tokyo.expand(last_mod);

    println!("{:?}", expanded);
}
