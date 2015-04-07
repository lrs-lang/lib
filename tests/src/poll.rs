extern crate linux;

use linux::{time, poll};

fn main() {
    let timer = time::Real.timer().unwrap();
    timer.interval(time::Time::seconds(5)).unwrap();

    let epoll = poll::Epoll::new().unwrap();
    let mut flags = poll::Flags::new();
    flags.set_readable(true);
    epoll.add(&timer, flags).unwrap();

    let mut buf = [poll::EMPTY_EVENT; 20];
    println!("{:?}", epoll.wait(&mut buf));
}
