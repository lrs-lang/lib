extern crate linux;

use linux::time::{self, Time};

fn main() {
    let timer = time::Real.timer().unwrap();
    timer.interval_in(Time::seconds(1), Time::seconds(5)).unwrap();

    time::Real.sleep_for(Time::seconds(10)).unwrap();

    println!("{:?}", timer.ticks().unwrap());
}
