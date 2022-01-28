#![warn(clippy::pedantic)]

use std::net::UdpSocket;
use client::socket::{copy, echo, list};
use client::proxy::Proxy;

fn main() {
    // Proxy-based API
    let rpc = Proxy::new("127.0.0.1:34254");

    let echo_response_p = rpc.echo("hello");
    let copy_response_p = rpc.copy("src", "dest");
    let list_response_p = rpc.list(".");

    println!("Echo response (proxy): {}", echo_response_p);
    println!("Copy response (proxy): {}", copy_response_p);
    println!("List response (proxy): {}", list_response_p);
    
    println!("{}", "~-".repeat(25));

    // Explicit socket-based API
    let socket = UdpSocket::bind("127.0.0.1:9877").unwrap();

    let echo_response = echo(&socket, "hello");
    let copy_response = copy(&socket, "src", "dest");
    let list_response = list(&socket, ".");

    println!("Echo response:         {}", echo_response);
    println!("Copy response:         {}", copy_response);
    println!("List response:         {}", list_response);
}
