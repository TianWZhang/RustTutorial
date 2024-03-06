use std::{
    collections::HashSet,
    io::{self, Read, Result, Write},
};
use mio::event::Event;
use mio::net::TcpStream;
use mio::{Interest, Poll, Token};

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
        let std_stream = std::net::TcpStream::connect(addr)?;
        std_stream.set_nonblocking(true)?;

        let mut stream = TcpStream::from_std(std_stream);
        stream.write_all(request.as_bytes())?;
        poll.registry().register(&mut stream, Token(i), Interest::READABLE)?;
        streams.push(stream);
    }

    let mut handled_events = 0;
    let mut handled_ids = HashSet::new();
    while handled_events < n_events {
        let mut events = mio::Events::with_capacity(10);
        poll.poll(&mut events, None)?;
        if events.is_empty() {
            println!("TIMEOUT (OR SPURIOUS EVENT NOTIFICATION)");
            continue;
        }
        let events: Vec<Event> = events.into_iter().map(|e| e.clone()).collect();
        handled_events += handle_events(&events, &mut streams, &mut handled_ids)?;
    }
    println!("FINISHED");
    Ok(())
}

fn handle_events(events: &[Event], streams: &mut [TcpStream], handled_ids: &mut HashSet<usize>) -> Result<usize> {
    let mut handled_events = 0;
    for event in events {
        let index: usize = event.token().into();
        let mut data = vec![0u8; 4096];
        loop {
            match streams[index].read(&mut data) {
                Ok(n) if n == 0 => {
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
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(e) if e.kind() == io::ErrorKind::Interrupted => break,
                Err(e) => return Err(e),
            }
        }
    }
    Ok(handled_events)
}
