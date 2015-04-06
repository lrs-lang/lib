extern crate linux;

use linux::time::*;

fn main() {
    let clock = CLOCK_REALTIME;
    let now = clock.get_time().unwrap();

    let zone = Zone::local().unwrap();

    let exp = zone.explode(now);

    println!("{:?}", exp);
}
