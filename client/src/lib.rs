#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

/// Proxy-based RPC API.
///
/// # Examples
/// ```
/// // Create new RPC proxy at the given address
/// let rpc = Proxy::new("127.0.0.1:34254");
/// // Call the server's remote function `echo`
/// let echo_response = rpc.echo("hello");
/// ```
pub mod proxy {
    use std::net::UdpSocket;

    pub struct Proxy {
        src: UdpSocket,
        dest: String,
    }

    impl Proxy {
        #[must_use]
        pub fn new(dest: &str) -> Self {
            let src = UdpSocket::bind("127.0.0.1:9876").unwrap();
            Self {
                src,
                dest: dest.to_string(),
            }
        }

        #[must_use]
        pub fn send_request(&self, cmd: &str, args: &str) -> String {
            // Initialize a 1kb buffer of 0's
            let mut buffer = [0; 1024];

            // Build request string
            let request = format!("{} {}", cmd, args);

            // Send request bytes to destination address
            self.src.send_to(request.as_bytes(), &self.dest).unwrap();

            // Write response into buffer
            let (amt, _src) = self.src.recv_from(&mut buffer).unwrap();

            // Read first `amt` bytes from buffer into `response`
            let response = std::str::from_utf8(&buffer[..amt]).unwrap();

            // Return `response`
            response.to_string()
        }

        #[must_use]
        pub fn echo(&self, args: &str) -> String {
            self.send_request("echo", args)
        }

        #[must_use]
        pub fn copy(&self, src: &str, dest: &str) -> String {
            self.send_request("cp", &format!("{} {}", src, dest))
        }

        #[must_use]
        pub fn list(&self, dir: &str) -> String {
            self.send_request("ls", dir)
        }
    }
}

/// Explicit socket-as-argument API
pub mod socket {
    use std::net::UdpSocket;

    #[must_use]
    pub fn send_request(src: &UdpSocket, destination: &str, cmd: &str, args: &str) -> String {
        // Initialize a 1kb buffer of 0's
        let mut buf = [0; 1024];

        // Build request string
        let request = format!("{} {}", cmd, args);

        // Send request bytes to destination address
        src.send_to(request.as_bytes(), destination).unwrap();

        // Write response into buffer
        let (amt, _src) = src.recv_from(&mut buf).unwrap();

        // Read first `amt` bytes from buffer into `response`
        let response = std::str::from_utf8(&buf[..amt]).unwrap();

        // Return `response`
        response.to_string()
    }

    #[must_use]
    pub fn echo(target: &str, args: &str) -> String {
        let src_address = UdpSocket::bind("127.0.0.1:9876").unwrap();
        send_request(&src_address, target, "echo", args)
    }

    #[must_use]
    pub fn copy(target: &str, src: &str, dest: &str) -> String {
        let src_address = UdpSocket::bind("127.0.0.1:9876").unwrap();
        send_request(&src_address, target, "cp", &format!("{} {}", src, dest))
    }

    #[must_use]
    pub fn list(target: &str, dir: &str) -> String {
        let src_address = UdpSocket::bind("127.0.0.1:9876").unwrap();
        send_request(&src_address, target, "ls", dir)
    }
}
