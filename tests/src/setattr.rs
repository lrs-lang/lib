extern crate linux;

use linux::{file};

fn main() {
    file::set_attr("Makefile", "user.hurrl", b"durr");
}
