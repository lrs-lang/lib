// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::atomic::{AtomicUsize};

fn dummy() -> Vec<i32> {
    vec!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9)
}

#[test]
fn drain() {
    let mut vec = dummy();
    vec.drain(..);
    assert!(vec.len() == 0);

    let mut vec = dummy();
    vec.drain(4..);
    assert!(&vec == &[0, 1, 2, 3][..]);

    let mut vec = dummy();
    vec.drain(..4);
    assert!(&vec == &[4, 5, 6, 7, 8, 9][..]);

    let mut vec = dummy();
    vec.drain(4..7);
    assert!(&vec == &[0, 1, 2, 3, 7, 8, 9][..]);

    let mut vec = dummy();
    vec.drain(0..);
    assert!(vec.len() == 0);

    let mut vec = dummy();
    vec.drain(..10);
    assert!(vec.len() == 0);

    let mut vec = dummy();
    vec.drain(0..10);
    assert!(vec.len() == 0);

    let mut vec = dummy();
    let mut drainer = vec.drain(..);
    for i in 0..10 {
        assert!(drainer.next() == Some(i));
    }
    assert!(drainer.next() == None);

    let mut vec = dummy();
    let mut drainer = vec.drain(4..);
    for i in 4..10 {
        assert!(drainer.next() == Some(i));
    }
    assert!(drainer.next() == None);

    let mut vec = dummy();
    let mut drainer = vec.drain(..4);
    for i in 0..4 {
        assert!(drainer.next() == Some(i));
    }
    assert!(drainer.next() == None);

    let mut vec = dummy();
    let mut drainer = vec.drain(4..7);
    for i in 4..7 {
        assert!(drainer.next() == Some(i));
    }
    assert!(drainer.next() == None);

    let mut vec = dummy();
    let mut drainer = vec.drain(0..);
    for i in 0..10 {
        assert!(drainer.next() == Some(i));
    }
    assert!(drainer.next() == None);

    let mut vec = dummy();
    let mut drainer = vec.drain(..10);
    for i in 0..10 {
        assert!(drainer.next() == Some(i));
    }
    assert!(drainer.next() == None);

    let mut vec = dummy();
    let mut drainer = vec.drain(0..10);
    for i in 0..10 {
        assert!(drainer.next() == Some(i));
    }
    assert!(drainer.next() == None);

    let mut vec = dummy();
    {
        let mut drainer = vec.drain(..);
        for _ in 0..2 {
            drainer.next();
        }
    }
    assert!(vec.len() == 0);

    let mut vec = dummy();
    {
        let mut drainer = vec.drain(4..);
        for _ in 0..2 {
            drainer.next();
        }
    }
    assert!(&vec == &[0, 1, 2, 3][..]);

    let mut vec = dummy();
    {
        let mut drainer = vec.drain(..4);
        for _ in 0..2 {
            drainer.next();
        }
    }
    assert!(&vec == &[4, 5, 6, 7, 8, 9][..]);

    let mut vec = dummy();
    {
        let mut drainer = vec.drain(4..7);
        for _ in 0..2 {
            drainer.next();
        }
    }
    assert!(&vec == &[0, 1, 2, 3, 7, 8, 9][..]);

    let mut vec = dummy();
    {
        let mut drainer = vec.drain(0..);
        for _ in 0..2 {
            drainer.next();
        }
    }
    assert!(vec.len() == 0);

    let mut vec = dummy();
    {
        let mut drainer = vec.drain(..10);
        for _ in 0..2 {
            drainer.next();
        }
    }
    assert!(vec.len() == 0);

    let mut vec = dummy();
    {
        let mut drainer = vec.drain(0..10);
        for _ in 0..2 {
            drainer.next();
        }
    }
    assert!(vec.len() == 0);




    struct X<'a>(&'a AtomicUsize);

    impl<'a> Drop for X<'a> {
        fn drop(&mut self) {
            self.0.add(1);
        }
    }

    let u = AtomicUsize::new(0);
    let mut vec: Vec<_> = vec!(X(&u), X(&u), X(&u), X(&u));
    vec.drain(1..3);
    assert!(u.load() == 2);
}
