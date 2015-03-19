extern crate linux;

use linux::process::{ids};

fn main() {
    let user_ids = ids::User::get();
    println!("{:?}", user_ids);

    let mut sups = [0; 20];
    ids::supplementary_groups(&mut sups);
    println!("{:?}", sups);
}
