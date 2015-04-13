extern crate linux;

use linux::{fs};

fn main() {
    let flags = fs::MountFlags::new();
    fs::mount("none", "hurr", "tmpfs", flags, "").unwrap();
}
