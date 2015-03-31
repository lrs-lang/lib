extern crate linux;

use linux::file::{info, File};

fn main() {
    let info1 = File::current_dir().info().unwrap();
    let info2 = File::open_read(".").unwrap().info().unwrap();
    let info3 = info(".").unwrap();

    assert_eq!(info1, info2);
    assert_eq!(info1, info3);
}
