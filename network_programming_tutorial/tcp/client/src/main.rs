use std::io::{self, BufReader, BufRead, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    for _ in 0..10 {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        stream.write(input.as_bytes())?;

        let mut reader = BufReader::new(&stream);
        let mut buffer = Vec::new();
        reader.read_until(b'\n', &mut buffer)?;
        println!("read from server: {}", std::str::from_utf8(&buffer).unwrap());
        println!("");
    }

    Ok(())
}
