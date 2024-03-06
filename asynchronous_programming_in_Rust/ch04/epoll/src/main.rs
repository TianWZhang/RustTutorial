mod ffi;
mod poll;

use std::{
    collections::HashSet,
    io::{self, Read, Result, Write},
    net::TcpStream,
};

use ffi::Event;
use poll::Poll;

// url: http://localhost:8080/2000/hello-world
fn get_req(path: &str) -> String {
    format!(
        "GET {path} HTTP/1.1\r\n\
             Host: localhost\r\n\
             Connection: close\r\n\
             \r\n"
    )
}

fn main() -> Result<()> {
    let mut poll = Poll::new()?;
    let n_events = 5;
    let mut streams = vec![];
    let addr = "localhost:8080";
    for i in 0..n_events {
        let delay = (n_events - i) * 1000;
        let url_path = format!("/{}/request-{}", delay, i);
        let request = get_req(&url_path);
        let mut stream = TcpStream::connect(addr)?;
        stream.set_nonblocking(true)?;
        stream.write_all(request.as_bytes())?;
        poll.registry().register(&stream, i, ffi::EPOLLIN | ffi::EPOLLET)?;
        streams.push(stream);
    }

    let mut handled_events = 0;
    let mut handled_ids = HashSet::new();
    while handled_events < n_events {
        let mut events = Vec::with_capacity(10);
        poll.poll(&mut events, None)?;
        if events.is_empty() {
            println!("TIMEOUT (OR SPURIOUS EVENT NOTIFICATION)");
            continue;
        }
        handled_events += handle_events(&events, &mut streams, &mut handled_ids)?;
    }
    println!("FINISHED");
    Ok(())
}

fn handle_events(events: &[Event], streams: &mut [TcpStream], handled_ids: &mut HashSet<usize>) -> Result<usize> {
    let mut handled_events = 0;
    for event in events {
        let index = event.token();
        let mut data = vec![0u8; 4096];
        // We create a loop since we might need to call read multiple times to be sure 
        // that we've actually drained the buffer. It's very important to fully drain the 
        // buffer when using epoll in edge-triggered mode.
        loop {
            match streams[index].read(&mut data) {
                // We have drained the buffer hence we consider the event as handled and break out of the loop.
                Ok(n) if n == 0 => {
                    // `insert` returns false if the value already existed in the set. We
                    // handle it here since we must be sure that the TcpStream is fully
                    // drained due to using edge triggered epoll.
                    if handled_ids.insert(index) {
                        handled_events += 1;
                    }
                    break;
                }
                Ok(n) => {
                    let txt = String::from_utf8_lossy(&data[..n]);
                    println!("RECEIVED: {:?}", event);
                    println!("{}\n------\n", txt);
                }
                // Not ready to read in a non-blocking manne. This could happen even if 
                // the event was reported as ready.
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(e) if e.kind() == io::ErrorKind::Interrupted => break,
                Err(e) => return Err(e),
            }
        }
    }
    Ok(handled_events)
}
