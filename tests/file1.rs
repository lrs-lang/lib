extern crate linux;
extern crate rand;

use rand::{Rng};
use linux::fs::file::{File, Flags, Mode};

fn random_path() -> String {
    let mut path = "dragons/".to_string();
    let mut rng = rand::thread_rng();
    for _ in 0..16 {
        path.push(rng.gen_range(97u8, 123) as char);
    }
    path
}

fn _write(path: &str) {
    let mut flags = Flags::new();
    flags.set_writable(true);
    flags.set_truncate(true);
    flags.enable_create(Mode::new_file());
    let file = File::open(path, flags).unwrap();
    file.write(b"hello world").unwrap();
}

fn _write_at(path: &str) {
    let mut flags = Flags::new();
    flags.set_writable(true);
    flags.set_truncate(true);
    flags.enable_create(Mode::new_file());
    let file = File::open(path, flags).unwrap();
    file.write(b"hello world").unwrap();
    file.write_at(b"n", 6).unwrap();
}

fn _gather_write(path: &str) {
    let mut flags = Flags::new();
    flags.set_writable(true);
    flags.set_truncate(true);
    flags.enable_create(Mode::new_file());
    let file = File::open(path, flags).unwrap();
    file.gather_write(&[b"hello ", b"world"]).unwrap();
}

fn _gather_write_at(path: &str) {
    let mut flags = Flags::new();
    flags.set_writable(true);
    flags.set_truncate(true);
    flags.enable_create(Mode::new_file());
    let file = File::open(path, flags).unwrap();
    file.write(b"hello world").unwrap();
    file.gather_write_at(&[b"wol", b"rd"], 6).unwrap();
}

#[test]
fn write() {
    _write(&random_path());
}

#[test]
fn write_at() {
    _write_at(&random_path());
}

#[test]
fn read() {
    let path = random_path();
    _write(&path);
    let file = File::open_read(&path).unwrap();
    let mut buf = [0; 16];
    let len = file.read(&mut buf).unwrap();
    assert_eq!(&buf[..len], b"hello world");
}

#[test]
fn read_at() {
    let path = random_path();
    _write_at(&path);
    let file = File::open_read(&path).unwrap();
    let mut buf = [0; 16];
    let len = file.read_at(&mut buf, 6).unwrap();
    assert_eq!(&buf[..len], b"norld");
}

#[test]
fn gather_write() {
    _gather_write(&random_path());
}

#[test]
fn scatter_read() {
    let path = random_path();
    _gather_write(&path);
    let file = File::open_read(&path).unwrap();
    let mut buf1 = [0; 1];
    let mut buf2 = [0; 15];
    let len = file.scatter_read(&mut [&mut buf1, &mut buf2]).unwrap();
    assert_eq!(buf1[0], b'h');
    assert_eq!(&buf2[..len-1], b"ello world");
}

#[test]
fn gather_write_at() {
    _gather_write_at(&random_path());
}

#[test]
fn scatter_read_at() {
    let path = random_path();
    _gather_write_at(&path);
    let file = File::open_read(&path).unwrap();
    let mut buf1 = [0; 1];
    let mut buf2 = [0; 15];
    let len = file.scatter_read_at(&mut [&mut buf1, &mut buf2], 6).unwrap();
    assert_eq!(buf1[0], b'w');
    assert_eq!(&buf2[..len-1], b"olrd");
}
