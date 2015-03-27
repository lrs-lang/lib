extern crate linux;

use linux::file::{File};

fn main() {
    let file = File::open_read("/usr/bin/sudo").unwrap();
    println!("{:?}", file.info());
}
