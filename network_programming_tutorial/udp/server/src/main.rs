use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080")?;

    loop {
        let mut buf = [0u8; 1500];
        let (size, src) = socket.recv_from(&mut buf)?;
        println!("size = {}", size);

        let buf = &mut buf[..size];
        buf.reverse();
        socket.send_to(buf, src)?;
    }
}

