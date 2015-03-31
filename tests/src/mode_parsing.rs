extern crate linux;

use linux::file::{Mode};

fn main() {
    let mode: Mode = "rwxrwxrwx".parse().unwrap();
    assert_eq!(mode.to_string(), "rwxrwxrwx");
}
