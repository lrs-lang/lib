extern crate linux;

use linux::file::{File};

fn main() {
    let file = File::open_read("/etc/fstab").unwrap();
    println!("{:?}", file.info());
}
