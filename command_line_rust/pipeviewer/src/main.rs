use crossbeam::channel::{bounded, unbounded};
use pipeviewer::{args::Args, read, stats, write}; //can not use crate here because the binaries in the project are separate crates
use std::{io::Result, thread};

fn main() -> Result<()> {
    let Args {
        infile,
        outfile,
        silent,
    } = Args::parse();

    let (stats_tx, stats_rx) = unbounded();
    let (write_tx, write_rx) = bounded(1024);

    let read_handle = thread::spawn(move || read::read_loop(&infile, stats_tx, write_tx));
    let stats_handle = thread::spawn(move || stats::stats_loop(silent, stats_rx));
    let write_handle = thread::spawn(move || write::write_loop(&outfile, write_rx));

    //crash if any thread has crashed
    //`.join()` returns a `thread::Result<io::Result<()>>`
    //return an error if any thread returned an error
    read_handle.join().unwrap()?;
    stats_handle.join().unwrap()?;
    write_handle.join().unwrap()?;
    Ok(())
}

//dd if=/dev/urandom bs=1024 count=128 of=myfile
//cat myfile | target/debug/pipeviewer > myfile2
//diff myfile myfile2

//yes | cargo run | head -n 1 > /dev/null
//yes | cargo run -- -o yex.txt
//echo "apple" | cargo run -- -o fruit.txt


//cargo doc --no-deps --open