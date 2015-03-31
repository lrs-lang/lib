extern crate linux;

use linux::file::{can_access};

fn main() {
    assert_eq!(can_access("Makefile", "rw-".parse().unwrap()), Ok(true));
    assert_eq!(can_access("Makefile", "--x".parse().unwrap()), Ok(false));
}
