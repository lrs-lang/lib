extern crate linux;

use linux::{file};

fn main() {
    assert_eq!(file::exists("Makefile"), Ok(true));
    assert_eq!(file::exists("doesnotexist"), Ok(false));
}
