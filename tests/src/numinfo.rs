extern crate linux;

use linux::{sys};

fn main() {
    let mut numinfo = sys::NumInfo::new();
    numinfo.update();
    println!("{:?}", numinfo);
}
