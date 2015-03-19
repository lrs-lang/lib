extern crate linux;

use linux::user::{self, UserInfo};
use linux::group::{self, GroupInfo};

fn main() {
    for user in group::iter(None) {
        println!("{:?}", user);
    }
}
