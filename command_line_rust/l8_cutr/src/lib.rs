use crate::Extract::*;
use clap::{Command, Arg, ArgAction};
use csv::{StringRecord, ReaderBuilder, WriterBuilder};
use regex::Regex;
use std::{
    error::Error, 
    ops::Range, 
    num::NonZeroUsize,
    fs::File,
    io::{self, BufRead, BufReader}
};

type MyResult<T> = Result<T, Box<dyn Error>>;
type PositionList = Vec<Range<usize>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    delimiter: u8,
    extract: Extract
}

#[derive(Debug)]
pub enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList)
}

// cargo run -- -f 1,3 -d , tests/inputs/books.csv
pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open(filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(f) => match config.extract {
                Chars(ref char_pos) => {
                    for line in f.lines() {
                        println!("{}", extract_chars(&line?, &char_pos));
                    }
                }
                Bytes(ref byte_pos) => {
                    for line in f.lines() {
                        println!("{}", extract_bytes(&line?, &byte_pos));
                    }
                }    
                Fields(ref field_pos) => {
                    let mut reader = ReaderBuilder::new()
                        .delimiter(config.delimiter)
                        .has_headers(false) //Do not treat the first row as headers
                        .from_reader(f);
                    //correctly escape delimiters in the output
                    let mut wtr = WriterBuilder::new()
                        .delimiter(config.delimiter)
                        .from_writer(io::stdout());
                    for record in reader.records() {
                        let record = record?;
                        wtr.write_record(extract_fields(&record, &field_pos))?;
                    }
                }   
            }
        }
    }
    Ok(())
}

fn fmt_record(rec: &StringRecord) -> String {
    rec.into_iter().map(|v| format!("{:20}", v)).collect()
}

fn parse_index(input: &str) -> Result<usize, String> {
    let value_error = || format!("illegal list value: \"{}\"", input);
    input
        .starts_with('+')
        .then(|| Err(value_error()))
        .unwrap_or_else(|| {
            input
                .parse::<NonZeroUsize>()
                .map(|n| usize::from(n) - 1)
                .map_err(|_| value_error())
        })
}

// Multiple numbers and ranges can be separated with commas.
fn parse_pos(range: &str) -> MyResult<PositionList> {
    let range_re = Regex::new(r"^(\d+)-(\d+)$").unwrap();
    range
        .split(',')
        .into_iter()
        .map(|val| {
            //If parse_index parse a single number, then create a Range for the value. Otherwise,
            //note the error value e and continue trying to parse a range.
            parse_index(val).map(|n| n..n + 1).or_else(|e| {
                //If the Regex matches the value, the numbers in parentheses will be available throught
                //Regex::captures.
                range_re.captures(val).ok_or(e).and_then(|captures| {
                    let n1 = parse_index(&captures[1])?;
                    let n2 = parse_index(&captures[2])?;
                    if n1 >= n2 {
                        return Err(format!("First number in range ({}) must be lower than second number ({})", n1 + 1, n2 + 1)); 
                    }
                    Ok(n1..n2 + 1)
                })
            })
        })
        .collect::<Result<_, _>>()
        .map_err(From::from)
}

//imperative approach
// fn extract_chars(line: &str, char_pos: &[Range<usize>]) -> String {
//     let mut res = String::new();
//     for range in char_pos.iter().cloned() {
//         for i in range {
//             if let Some(c) = line.chars().nth(i) {
//                 res.push(c);
//             }
//         }
//     }
//     res
// }
//iterative approach
fn extract_chars(line: &str, char_pos: &[Range<usize>]) -> String {
    let chars: Vec<_> = line.chars().collect();

    char_pos
        .iter()
        .cloned() 
        .flat_map(|range| range.filter_map(|i| chars.get(i)))
        .collect()
}

// fn extract_bytes(line: &str, byte_pos: &[Range<usize>]) -> String {
//     let mut res = vec![];
//     for range in byte_pos.iter().cloned() {
//         res.extend(range.filter_map(|i| line.bytes().nth(i)));    
//     }
//     String::from_utf8_lossy(&res).to_string()
// }
fn extract_bytes(line: &str, byte_pos: &[Range<usize>]) -> String {
    let bytes = line.as_bytes();
    let selected: Vec<u8> = byte_pos
        .iter()
        .cloned()
        .flat_map(|range| range.filter_map(|i| bytes.get(i)).copied())
        .collect();
    String::from_utf8_lossy(&selected).into_owned()
}

fn extract_fields<'a>(record: &'a StringRecord, field_pos: &[Range<usize>]) -> Vec<&'a str> {
    field_pos
        .iter()
        .cloned()
        .flat_map(|range| range.filter_map(|i| record.get(i)))
        .collect()
}
#[cfg(test)]
mod unit_tests {
    use super::{extract_bytes, extract_chars, extract_fields, parse_pos};
    use csv::StringRecord;

    #[test]
    fn test_parse_pos() {
        // The empty string is an error
        assert!(parse_pos("").is_err());

        // Zero is an error
        let res = parse_pos("0");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"0\"",);

        let res = parse_pos("0-1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"0\"",);

        // A leading "+" is an error
        let res = parse_pos("+1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "illegal list value: \"+1\"",
        );

        let res = parse_pos("+1-2");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "illegal list value: \"+1-2\"",
        );

        let res = parse_pos("1-+2");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "illegal list value: \"1-+2\"",
        );

        // Any non-number is an error
        let res = parse_pos("a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"a\"",);

        let res = parse_pos("1,a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"a\"",);

        let res = parse_pos("1-a");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "illegal list value: \"1-a\"",
        );

        let res = parse_pos("a-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "illegal list value: \"a-1\"",
        );

        // Wonky ranges
        let res = parse_pos("-");
        assert!(res.is_err());

        let res = parse_pos(",");
        assert!(res.is_err());

        let res = parse_pos("1,");
        assert!(res.is_err());

        let res = parse_pos("1-");
        assert!(res.is_err());

        let res = parse_pos("1-1-1");
        assert!(res.is_err());

        let res = parse_pos("1-1-a");
        assert!(res.is_err());

        // First number must be less than second
        let res = parse_pos("1-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (1) must be lower than second number (1)"
        );

        let res = parse_pos("2-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (2) must be lower than second number (1)"
        );

        // All the following are acceptable
        let res = parse_pos("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);

        let res = parse_pos("01");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);

        let res = parse_pos("1,3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);

        let res = parse_pos("001,0003");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);

        let res = parse_pos("1-3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);

        let res = parse_pos("0001-03");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);

        let res = parse_pos("1,7,3-5");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 6..7, 2..5]);

        let res = parse_pos("15,19-20");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![14..15, 18..20]);
    }

    #[test]
    fn test_extract_fields() {
        let rec = StringRecord::from(vec!["Captain", "Sham", "12345"]);
        assert_eq!(extract_fields(&rec, &[0..1]), &["Captain"]);
        assert_eq!(extract_fields(&rec, &[1..2]), &["Sham"]);
        assert_eq!(
            extract_fields(&rec, &[0..1, 2..3]),
            &["Captain", "12345"]
        );
        assert_eq!(extract_fields(&rec, &[0..1, 3..4]), &["Captain"]);
        assert_eq!(extract_fields(&rec, &[1..2, 0..1]), &["Sham", "Captain"]);
    }

    #[test]
    fn test_extract_chars() {
        assert_eq!(extract_chars("", &[0..1]), "".to_string());
        assert_eq!(extract_chars("ábc", &[0..1]), "á".to_string());
        assert_eq!(extract_chars("ábc", &[0..1, 2..3]), "ác".to_string());
        assert_eq!(extract_chars("ábc", &[0..3]), "ábc".to_string());
        assert_eq!(extract_chars("ábc", &[2..3, 1..2]), "cb".to_string());
        assert_eq!(
            extract_chars("ábc", &[0..1, 1..2, 4..5]),
            "áb".to_string()
        );
    }

    #[test]
    fn test_extract_bytes() {
        assert_eq!(extract_bytes("ábc", &[0..1]), "�".to_string());
        assert_eq!(extract_bytes("ábc", &[0..2]), "á".to_string());
        assert_eq!(extract_bytes("ábc", &[0..3]), "áb".to_string());
        assert_eq!(extract_bytes("ábc", &[0..4]), "ábc".to_string());
        assert_eq!(extract_bytes("ábc", &[3..4, 2..3]), "cb".to_string());
        assert_eq!(extract_bytes("ábc", &[0..2, 5..6]), "á".to_string());
    }
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("cutr")
        .version("0.1.0")
        .author("Zhang Tianwei <zhangtianwei1015@gmail.com>")
        .about("Rust cut")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input files")
                .default_value("-")
                .action(ArgAction::Append)
        )
        .arg(
            Arg::new("bytes")
                .value_name("BYTES")
                .long("bytes")
                .short('b')
                .conflicts_with_all(&["fields", "chars"])
                .help("Selected bytes")
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("chars")
                .value_name("CHARS")
                .long("chars")
                .short('c')
                .conflicts_with_all(&["bytes", "fields"])
                .help("Selected characters")
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("fields")
                .value_name("FIELDS")
                .long("fields")
                .short('f')
                .conflicts_with_all(&["bytes", "chars"])
                .help("Selected fields")
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("delimiter")
                .value_name("DELIMITER")
                .long("delim")
                .short('d')
                .help("Field delimiter")
                // .value_parser(value_parser!(u8))
                .default_value("\t")
                .action(ArgAction::Set)
        )
        .get_matches();
    let files = matches
        .get_many::<String>("files")
        .unwrap()
        .map(|v| v.into())
        .collect::<Vec<String>>();
    let extract = if let Some(fields) = matches.get_one::<String>("fields")  {
         Fields(parse_pos(fields)?)
    } else if let Some(bytes) = matches.get_one::<String>("bytes") {
        Bytes(parse_pos(bytes)?)
    } else if let Some(chars) = matches.get_one::<String>("chars") {
        Chars(parse_pos(chars)?)
    } else {
        return Err(From::from("Must have --fields, --bytes, or --chars"));
    };
    let delimiter = matches.get_one::<String>("delimiter").unwrap();
    if delimiter.len() != 1 {
        return Err(format!("--delim \"{}\" must be a single byte", delimiter).into());
    }
    Ok(Config {
        files,
        delimiter: *delimiter.as_bytes().first().unwrap(),
        extract
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}
