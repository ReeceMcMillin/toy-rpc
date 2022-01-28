#![warn(clippy::pedantic)]

use std::net::UdpSocket;

fn echo(args: &[&str]) -> String {
    args.join(" ")
}

fn copy(args: &[&str]) -> String {
    match args.len() {
        2 => {
            let (src, dest) = (args[0], args[1]);
            format!("pretending to copy from `{}` to `{}`", &src, &dest)
        },
        _ => String::from("usage: copy <file>"),
    }
}

fn list(args: &[&str]) -> String {
    match args.len() {
        0 => String::from("pretending to list current directory"),
        1 => {
            let dir = args[0];
            format!("pretending to list directory `{}`", &dir)
        },
        _ => String::from("usage: ls [directory]"),
    }
}

fn handle_rpc(cmd: &str, args: &[&str]) -> String {
    let width = 6usize;
    let cmd_str = format!("`{cmd}`");
    println!("\tHandling RPC: {cmd_str:width$} with args: {args:?}");
    match cmd.trim() {
        "echo" => echo(args),
        "cp" => copy(args),
        "ls" => list(args),
        _ => String::from("unimplemented"),
    }
}

fn listen() {
    let socket = UdpSocket::bind("127.0.0.1:34254").unwrap();
    println!("Listening on port 34254...");
    let mut buffer = [0; 1024];

    loop {
        let (amt, src) = socket.recv_from(&mut buffer).unwrap();
        let recv = String::from_utf8_lossy(&buffer[..amt]).to_string();
        let split = recv.split_whitespace().collect::<Vec<&str>>();
        let cmd = *split.first().unwrap();
        let args = split[1..].to_vec();

        let response = handle_rpc(cmd, &args);

        socket.send_to(response.to_string().as_bytes(), src).unwrap();
        buffer = [0; 1024];
    }
}

fn main() {
    listen();
}
