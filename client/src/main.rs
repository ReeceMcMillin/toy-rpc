use client::proxy::Proxy;
use client::socket::{copy, echo, list};

fn main() {
    // RPC using proxy-based API
    let rpc = Proxy::new("127.0.0.1:34254");

    let echo_response_proxy = rpc.echo("hello");
    let copy_response_proxy = rpc.copy("src", "dest");
    let list_response_proxy = rpc.list(".");
    
    println!("Echo response (proxy): {}", echo_response_proxy);
    println!("Copy response (proxy): {}", copy_response_proxy);
    println!("List response (proxy): {}", list_response_proxy);

    println!("{}", "~-".repeat(25));

    // Need to free the proxy's bound port by dropping the resource
    drop(rpc);

    // RPC using explicit address-as-argument API
    let target = "127.0.0.1:34254";

    let echo_response = echo(target, "hello");
    let copy_response = copy(target, "src", "dest");
    let list_response = list(target, ".");

    println!("Echo response:         {}", echo_response);
    println!("Copy response:         {}", copy_response);
    println!("List response:         {}", list_response);
}
