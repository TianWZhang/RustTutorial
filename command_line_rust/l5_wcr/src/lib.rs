use std::error::Error;
use clap::{Command, Arg, ArgAction};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;
    for filename in &config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}\n", filename, err),
            Ok(f) => {
                if let Ok(info) = count(f) {
                    println!(
                        "{}{}{}{}{}",
                        format_field(info.num_lines, config.lines),
                        format_field(info.num_words, config.words),
                        format_field(info.num_bytes, config.bytes),
                        format_field(info.num_chars, config.chars),
                        if filename == "-" {
                            "".to_string()
                        } else {
                            format!(" {}", filename)
                        }
                    );
                    total_lines += info.num_lines;
                    total_words += info.num_words;
                    total_bytes += info.num_bytes;
                    total_chars += info.num_chars;
                }
            }
        }       
    }
    if config.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(total_lines, config.lines),
            format_field(total_words, config.words),
            format_field(total_bytes, config.bytes),
            format_field(total_chars, config.chars)
        );
    }
    Ok(())
}

// conditionally format a value or "" to support the printing or omission of information
// according to the flag arguments
fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }
        num_bytes += line_bytes;
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars
    })
}

#[cfg(test)]
mod tests {
    use super::{count, format_field, FileInfo};
    //to create a fake filehandle for testing a function that expects sth that implements BufRead
    use std::io::Cursor; 

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_bytes: 48,
            num_chars: 48
        };
        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, false), "");
        assert_eq!(format_field(3, true), "       3");
        assert_eq!(format_field(10, true), "      10");
    }
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("wcr")
        .version("0.1.0")
        .author("Zhang Tianwei <zhangtianwei1015@gmail.com>")
        .about("Rust wc")
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .help("files to wc")
                .default_value("-")
                .action(ArgAction::Append)
        )
        .arg(
            Arg::new("lines")
                .value_name("LINES")
                .long("lines")
                .short('l')
                .help("Show line count")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("bytes")
                .value_name("BYTES")
                .long("bytes")
                .short('c')
                .help("Show byte count")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("words")
                .value_name("WORDS")
                .long("words")
                .short('w')
                .help("Show word count")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("chars")
                .value_name("CHARS")
                .long("chars")
                .short('m')
                .conflicts_with("bytes")
                .help("Show character count")
                .action(ArgAction::SetTrue)
        )
        .get_matches();
    let files = matches
        .get_many::<String>("files")
        .unwrap()
        .map(|v| v.into())
        .collect::<Vec<String>>();
    let mut lines = matches.get_flag("lines");
    let mut words = matches.get_flag("words");
    let mut bytes = matches.get_flag("bytes");
    let chars = matches.get_flag("chars");
    //lines, words, bytes should be true by default
    //if any single flag is present, all the other flags not mentioned should be false
    if [lines, words, bytes, chars].iter().all(|v| v == &false) {
        lines = true;
        words = true;
        bytes = true;
    }
    Ok(Config {
        files,
        lines,
        words,
        bytes,
        chars
    })
}


fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}