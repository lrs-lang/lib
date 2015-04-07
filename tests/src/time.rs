extern crate linux;

use linux::time::*;

fn main() {
    let clock = Real;
    let now = clock.get_time().unwrap();

    let zone = Zone::local().unwrap();

    let exp = zone.expand(now);

    println!("{:?}", exp);
}
