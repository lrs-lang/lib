// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::socket::{Socket};
use std::socket::unix::{UnixSockAddr};
use std::socket::flags::{SOCK_NONE};
use std::socket::ip::{Ipv4Addr, Ipv4SockAddr, Ipv6Addr, Ipv6SockAddr};

fn main() {
    let mut buf = [0; 128];
    {
        let addr = UnixSockAddr::from_path(&mut buf, "/tmp/socket").unwrap();
        println!("{:?}", addr);
    }
    {
        let addr = UnixSockAddr::from_unnamed(&mut buf).unwrap();
        println!("{:?}", addr);
    }
    {
        let addr = UnixSockAddr::from_abstract(&mut buf, "hurr\0durr").unwrap();
        println!("{:?}", addr);
    }

    {
        let addr = Ipv4SockAddr::from_addr_port(&mut buf, Ipv4Addr(128, 0, 0, 1),
                                                567).unwrap();
        println!("{:?}", addr);
    }

    {
        let addr = Ipv6SockAddr::from_addr_port(&mut buf,
                                                Ipv6Addr(0xffef, 0, 1, 1, 55, 1, 1, 1),
                                                567).unwrap();
        println!("{:?}", addr);
    }

    {
        let addr = Ipv6SockAddr::from_addr_port(&mut buf,
                                                Ipv6Addr(0, 0, 0, 0, 0, 0, 0xeeee, 0xeeee),
                                                0).unwrap();
        println!("{:?}", addr);
    }

    {
        let addr = Ipv6SockAddr::from_addr_port(&mut buf,
                                                Ipv6Addr(0, 0, 0, 0, 0, 0xffff, 0xeeee, 0xeeee),
                                                0).unwrap();
        println!("{:?}", addr);
    }

    {
        let addr = Ipv6SockAddr::from_addr_port(&mut buf,
                                                Ipv6Addr(0, 0, 0, 0, 0xffff, 0, 0xffee, 0xffee),
                                                0).unwrap();
        println!("{:?}", addr);
    }

    let socket = Socket::ipv4_stream(SOCK_NONE).unwrap();
    {
        let addr = Ipv6SockAddr::from_addr_port(&mut buf,
                                                Ipv6Addr::unspecified(), 0).unwrap();
        println!("{:?}", socket.bind(&addr));
    }
    {
        let addr = socket.get_addr(&mut buf).unwrap();
        println!("{:?}", addr);
    }



    loop { }

}
