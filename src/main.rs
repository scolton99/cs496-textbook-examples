mod tcp;

use std::net;
use std::net::Ipv4Addr;

fn main() {
    println!("Hello, world!");

    let x = Ipv4Addr::new(0, 0, 0, 0);

    crate::tcp::make_socket();
}
