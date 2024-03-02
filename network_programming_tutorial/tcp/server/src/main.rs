use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0; 512];
    for _ in 0..1000 {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        stream.write(&buf[..bytes_read])?;
        thread::sleep(time::Duration::from_secs(1));
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let mut threads = Vec::new();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let handle = thread::spawn(move || {
            handle_client(stream).unwrap_or_else(|e| eprintln!("{:?}", e));
        });
        threads.push(handle);
    }

    for handle in threads {
        handle.join().unwrap();
    }
    Ok(())
}
