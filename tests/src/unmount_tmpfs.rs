extern crate linux;

use linux::{fs};

fn main() {
    let mut flags = fs::UnmountFlags::new();
    flags.set_lazy(true);
    fs::unmount("hurr", flags).unwrap();
}
