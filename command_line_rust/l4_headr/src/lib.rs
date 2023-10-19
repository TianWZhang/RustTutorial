use std::error::Error;
use clap::{Command, Arg, ArgAction, value_parser};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();
    for (i, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}\n", filename, err),
            Ok(mut f) => {
                //only print headers when there are multiple files
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if i > 0 {"\n"} else {""},
                        filename
                    );
                }
                if let Some(num_bytes) = config.bytes {
                    let mut handle = f.take(num_bytes as u64);
                    //create a mutable buffer of a fixed length filled with 0 to hold the bytes read from the file
                    let mut buffer = vec![0; num_bytes];
                    let bytes_read = handle.read(&mut buffer)?;
                    //convert the selected bytes(the bytes actually read) into a string
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                    
                    //alternative way of read the desired number of bytes from a file:
                    //The type annotation is necessary as the compiler infers the type of bytes
                    //as a slice, which has an unknown size.
                    // let bytes: Result<Vec<_>, _> = f.bytes().take(num_bytes).collect();
                    // print!("{}", String::from_utf8_lossy(&bytes?));
                } else {
                    //not preserving line endings while reading a file
                    // for line in f.line().take(config.lines) {
                    //     println!("{}", line?);
                    // }
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = f.read_line(&mut line)?;
                        //the file handle will return 0 bytes when it reaches the end
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }
            }
        }       
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("headr")
        .version("0.1.0")
        .author("Zhang Tianwei <zhangtianwei1015@gmail.com>")
        .about("Rust head")
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .help("Input files")
                .default_value("-")
                .action(ArgAction::Append)
        )
        .arg(
            Arg::new("lines")
                .value_name("LINES")
                .long("lines")
                .short('n')
                .help("the number of lines to print")
                .value_parser(value_parser!(usize))
                .default_value("10")
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("bytes")
                .value_name("BYTES")
                .long("bytes")
                .short('c')
                .help("the number of bytes to print")
                .value_parser(value_parser!(usize))
                .conflicts_with("lines")
                .action(ArgAction::Set)
        )
        .get_matches();
    let files = matches
        .get_many::<String>("files")
        .unwrap()
        .map(|v| v.into())
        .collect::<Vec<String>>();
    let lines: usize = *(matches.get_one("lines")).unwrap();
    let bytes = matches
        .get_one::<usize>("bytes")
        .map(|b| *b);
    Ok(Config {
        files,
        lines,
        bytes
    })
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        // or Err(From::from(val)), convert &str into an Error
        _ => Err(val.into())    
    }
} 

#[test] 
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}