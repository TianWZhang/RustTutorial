use crossbeam::channel::Receiver;
use std::fs::File;
use std::io::{self, BufWriter, ErrorKind, Result, Write};

pub fn write_loop(outfile: &str, write_rx: Receiver<Vec<u8>>) -> Result<()> {
    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };
    loop {
        //receive buffer from the read thread
        let buf = write_rx.recv().unwrap();
        if buf.is_empty() {
            break;
        }
        if let Err(e) = writer.write_all(&buf) {
            if e.kind() == ErrorKind::BrokenPipe {
                return Ok(()); //ignore the broken pipe error, stop the program cleanly
            }
            return Err(e);
        }
    }
    Ok(())
}
