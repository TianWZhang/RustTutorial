use std::error::Error;
use clap::{Command, Arg, ArgAction};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool
}

type MyResult<T> = Result<T, Box<dyn Error>>;

//cargo run -q -- -h
//cargo run -q 
//cargo run -q -- -n texts/inputs/fox.txt
//cargo run -q -- texts/inputs/*.txt
//cargo run -q -- xxx cant-touch-this tests/inputs/fox.txt

//cargo run -q -- - < tests/inputs/fox.txt      take input from the file and provide it to STDIN
//cat tests/inputs/fox.txt | cargo run -q       pipe STDOUT from the first command to the STDIN of the second command
pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(f) => {
                let mut line_num = 1;
                for line in f.lines() {
                    let line = line?;
                    if config.number_lines {
                        println!("{:>6}\t{}", line_num, line);
                        line_num += 1;
                    } else if config.number_nonblank_lines {
                        if line.is_empty() {
                            println!();
                        } else {
                            //print the current line number in a right-justified field six characters wide
                            println!("{:>6}\t{}", line_num, line);
                            line_num += 1;
                        }
                    } else {
                        println!("{}", line);
                    }
                    
                }
            }
        }       
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("Zhang Tianwei <zhangtianwei1015@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .help("files to cat")
                .default_value("-")
                .action(ArgAction::Append)
        )
        .arg(
            Arg::new("number_lines")
                .long("number")
                .short('n')
                .help("print the line numbers")
                .conflicts_with("number_nonblank_lines")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .long("number-nonblank")
                .short('b')
                .help("print line numbers only for nonblank lines")
                .action(ArgAction::SetTrue)
        )
        .get_matches();
    let files = matches
                    .get_many::<String>("files")
                    .unwrap()
                    .map(|v| v.into())
                    .collect::<Vec<String>>();
    Ok(Config {
        files,
        number_lines: matches.get_flag("number_lines"),
        number_nonblank_lines: matches.get_flag("number_nonblank_lines")
    })
}

// dyn BufRead does not have a size known at compile-time
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
    //Both a file handle and std::io::stdin implement the BufRead trait, which means they will 
    //respond to BufRead::lines function to produce lines of text.
}