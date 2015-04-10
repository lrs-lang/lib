extern crate linux;

use linux::{fs};

fn main() {
    let mut flags = fs::MountFlags::new();
    fs::mount("none", "hurr", "tmpfs", flags, "").unwrap();
}
