extern crate linux;

use linux::{process};

fn main() {
    let user_ids = process::UserIds::get();
    println!("{:?}", user_ids);

    let mut sups = [0; 20];
    process::supplementary_groups(&mut sups);
    println!("{:?}", sups);
}
