use crate::{future::PollState, Future, runtime};
use std::io::{ErrorKind, Read, Write};
use mio::{Interest, Token};

fn get_req(path: &str) -> String {
    format!(
        "GET {path} HTTP/1.1\r\n\
             Host: localhost\r\n\
             Connection: close\r\n\
             \r\n"
    )
}

pub struct Http;
impl Http {
    // The string we return will be the response we get from the server.
    // The call to Http::get returns immediately with just a simple data structure.
    // We don't actually do anything related to the GET request.
    pub fn get(path: &str) -> impl Future<Output = String> {
        HttpGetFuture::new(path)
    }
}

// leaf future
struct HttpGetFuture {
    stream: Option<mio::net::TcpStream>,
    // We'll read all the data from the TcpStream and put it all in this buffer.
    buffer: Vec<u8>,
    path: String,
}

impl HttpGetFuture {
    fn new(path: &str) -> Self {
        Self {
            stream: None,
            buffer: vec![],
            path: path.to_string(),
        }
    }

    fn write_request(&mut self) {
        let stream = std::net::TcpStream::connect("127.0.0.1:8080").unwrap();
        stream.set_nonblocking(true).unwrap();
        let mut stream = mio::net::TcpStream::from_std(stream);
        stream.write_all(get_req(&self.path).as_bytes()).unwrap();
        self.stream = Some(stream);
    }
}

impl Future for HttpGetFuture {
    type Output = String;

    fn poll(&mut self) -> PollState<Self::Output> {
        if self.stream.is_none() {
            println!("First Poll - Start Operation");
            self.write_request();
            // register interest in READABLE events on this TcpStream
            runtime::registry()
                .register(self.stream.as_mut().unwrap(), Token(0), Interest::READABLE).unwrap();
        }
        let mut buff = vec![0u8; 4096];
        loop {
            match self.stream.as_mut().unwrap().read(&mut buff) {
                Ok(0) => {
                    let s = String::from_utf8_lossy(&self.buffer);
                    break PollState::Ready(s.to_string());
                }
                Ok(n) => {
                    self.buffer.extend(&buff[0..n]);
                    continue;
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => {
                    break PollState::Pending;
                }
                Err(e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => panic!("{:?}", e),
            }
        }
    }
}