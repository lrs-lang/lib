extern crate linux;

use linux::{sys};

fn main() {
    let mut strinfo = sys::StrInfo::new();
    strinfo.update().unwrap();
    println!("{:?}", strinfo);
}
