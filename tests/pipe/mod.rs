// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::pipe::{Pipe};
use std::pipe::flags::{PIPE_NONE, TEE_NONE, SPLICE_NONE};

#[test]
fn read_write() {
    let (write, read) = Pipe::new(PIPE_NONE).unwrap();
    test!(write.write(b"Hello World").unwrap() == 11);
    let mut buf = [0; 12];
    test!(read.read(buf.as_mut()).unwrap() == 11);
    test!(&buf[..] == "Hello World\0");
    test!(write.gather_write(&[b"Hello", b"World"]).unwrap() == 10);
    let mut buf1 = [0; 5];
    let mut buf2 = [0; 6];
    test!(read.scatter_read(&mut [buf1.as_mut(), buf2.as_mut()]).unwrap() == 10);
    test!(&buf1[..] == "Hello");
    test!(&buf2[..] == "World\0");
}

#[test]
fn capacity() {
    let (write, read) = Pipe::new(PIPE_NONE).unwrap();

    // XXX: fails with invalid argument in qemu-arm. however, we can see in strace that
    // qemu-arm never performs a syscall. probably a bug in qemu-arm.
    write.set_capacity(1000).unwrap();
    test!(read.capacity().unwrap() >= 1000);
}

#[test]
fn len() {
    let (write, read) = Pipe::new(PIPE_NONE).unwrap();
    write.write(b"test").unwrap();
    test!(read.len().unwrap() == 4);
    test!(write.len().unwrap() == 4);
}

#[test]
fn copy_to() {
    let (write1, read1) = Pipe::new(PIPE_NONE).unwrap();
    let (write2, read2) = Pipe::new(PIPE_NONE).unwrap();
    write1.write(b"Hello World").unwrap();
    test!(read1.copy_to(&write2, 6, TEE_NONE).unwrap() == 6);
    let mut buf = [0; 12];
    test!(read1.read(buf.as_mut()).unwrap() == 11);
    test!(buf.starts_with(b"Hello World"));
    test!(read2.read(buf.as_mut()).unwrap() == 6);
    test!(buf.starts_with(b"Hello "));
}

#[test]
fn read_from() {
    let (write1, read1) = Pipe::new(PIPE_NONE).unwrap();
    let (write2, read2) = Pipe::new(PIPE_NONE).unwrap();
    write1.write(b"Hello World").unwrap();
    test!(write2.read_from(&read1, 6, SPLICE_NONE).unwrap() == 6);
    let mut buf = [0; 12];
    test!(read1.read(buf.as_mut()).unwrap() == 5);
    test!(buf.starts_with(b"World"));
    test!(read2.read(buf.as_mut()).unwrap() == 6);
    test!(buf.starts_with(b"Hello "));
}

#[test]
fn read_from_at() {
    // TODO
}

#[test]
fn write_to() {
    let (write1, read1) = Pipe::new(PIPE_NONE).unwrap();
    let (write2, read2) = Pipe::new(PIPE_NONE).unwrap();
    write1.write(b"Hello World").unwrap();
    test!(read1.write_to(&write2, 6, SPLICE_NONE).unwrap() == 6);
    let mut buf = [0; 12];
    test!(read1.read(buf.as_mut()).unwrap() == 5);
    test!(buf.starts_with(b"World"));
    test!(read2.read(buf.as_mut()).unwrap() == 6);
    test!(buf.starts_with(b"Hello "));
}

#[test]
fn write_to_at() {
    // TODO
}
