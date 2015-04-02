extern crate linux;

use linux::file::{self, File};

fn main() {
    let file = File::open_read("testlink").unwrap();
    println!("{:?}", file.filename());
}
