pub mod proxy {
    use std::net::UdpSocket;

    pub struct Proxy {
        src: UdpSocket,
        dest: String,
    }
    
    impl Proxy {
        pub fn new(dest: &str) -> Self {
            let src = UdpSocket::bind("127.0.0.1:9876").unwrap();
            Self {
                src,
                dest: dest.to_string(),
            }
        }
    
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
    
        pub fn echo(&self, args: &str) -> String {
            self.send_request("echo", args)
        }
    
        pub fn copy(&self, src: &str, dest: &str) -> String {
            self.send_request("cp", &format!("{} {}", src, dest))
        }
    
        pub fn list(&self, dir: &str) -> String {
            self.send_request("ls", dir)
        }
    }
}  

// Explicit socket-based API
pub mod socket {
    use std::net::UdpSocket;

    pub fn send_request(socket: &UdpSocket, destination: &str, cmd: &str, args: &str) -> String {
        let mut buf = [0; 1024];
        let request = format!("{} {}", cmd, args);
        socket.send_to(request.as_bytes(), destination).unwrap();
        let (amt, _src) = socket.recv_from(&mut buf).unwrap();
        let response = std::str::from_utf8(&buf[..amt]).unwrap();

        response.to_string()
    }

    pub fn echo(socket: &UdpSocket, args: &str) -> String {
        send_request(socket, "127.0.0.1:34254", "echo", args)
    }

    pub fn copy(socket: &UdpSocket, src: &str, dest: &str) -> String {
        send_request(socket, "127.0.0.1:34254", "cp", &format!("{} {}", src, dest))
    }

    pub fn list(socket: &UdpSocket, dir: &str) -> String {
        send_request(socket, "127.0.0.1:34254", "ls", dir)
    }
}
