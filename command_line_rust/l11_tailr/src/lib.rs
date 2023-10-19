use crate::TakeValue::*;
use clap::{Command, Arg, ArgAction};
use regex::Regex;
use once_cell::sync::OnceCell;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Read, Seek, SeekFrom}
};

type MyResult<T> = Result<T, Box<dyn Error>>;
//create a lazily evaluated regular expression
static NUM_RE: OnceCell<Regex> = OnceCell::new();

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: TakeValue,
    bytes: Option<TakeValue>,
    quiet: bool
}

#[derive(Debug, PartialEq)]
enum TakeValue {
    PlusZero,
    TakeNum(i64)
}

pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();
    for (i, filename) in config.files.iter().enumerate() {
        match File::open(&filename) {
            Err(err) => eprintln!("{}: {}\n", filename, err),
            Ok(f) => {
                if num_files > 1 && !config.quiet {
                    println!(
                        "{}==> {} <==",
                        if i > 0 {"\n"} else {""},
                        filename
                    );
                }
                let (total_lines, total_bytes) = count_lines_bytes(&filename)?;
                let file = BufReader::new(f);
                if let Some(num_bytes) = &config.bytes {
                    print_bytes(file, num_bytes, total_bytes)?;
                } else {
                    print_lines(file, &config.lines, total_lines)?;
                }
            }
        }       
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("tailr")
        .version("0.1.0")
        .author("Zhang Tianwei <zhangtianwei1015@gmail.com>")
        .about("Rust tail")
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .help("Input files")
                .required(true)
                .action(ArgAction::Append)
        )
        .arg(
            Arg::new("lines")
                .value_name("LINES")
                .long("lines")
                .short('n')
                .help("Number of lines")
                .default_value("10")
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("bytes")
                .value_name("BYTES")
                .long("bytes")
                .short('c')
                .help("Number of bytes")
                .conflicts_with("lines")
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("quiet")
                .long("quiet")
                .short('q')
                .help("Suppress headers")
                .action(ArgAction::SetTrue)
        )
        .get_matches();
    let files = matches
        .get_many::<String>("files")
        .unwrap()
        .map(|v| v.into())
        .collect::<Vec<String>>();
    let lines = parse_num(
        matches.get_one::<String>("lines").unwrap().as_str()
    ).map_err(|e| format!("illegal line count -- {}", e))?;
    let bytes = matches
        .get_one::<String>("bytes")
        .map(|val| parse_num(val.as_str()))
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;
    Ok(Config {
        files,
        lines,
        bytes,
        quiet: matches.get_flag("quiet")
    })
}

fn parse_num(val: &str) -> MyResult<TakeValue> {
    let num_re = NUM_RE.get_or_init(|| Regex::new(r"^([+-]?)(\d+)$").unwrap());
    num_re.captures(val).ok_or(val.into()).and_then(|captures| {
        let sign = match &captures[1] {
            "" => "-",
            sign => sign,
        };
        //Before parsing, we have to group the sign and &captures[2] together 
        //Otherwise, -|id4::MIN| cannot be parsed since |i64::MIN| > i64::MAX 
        let num = format!("{}{}", sign, &captures[2]).parse::<i64>()?;
        if sign == "+" && num == 0 {
            Ok(PlusZero)
        } else {
            Ok(TakeNum(num))
        }
     })
} 

fn parse_num_plain(val: &str) -> MyResult<TakeValue> {
    let signs: &[char] = &['+', '-'];
    let res = val
        .starts_with(signs)
        .then(|| val.parse())
        //use i64::wrapping_neg to compute the negative value: a positive value will be returned as
        //negative, while a negative value will remain negative
        .unwrap_or_else(|| val.parse().map(i64::wrapping_neg));
    match res {
        Ok(num) => {
            if num == 0 && val.starts_with('+') {
                Ok(PlusZero)
            } else {
                Ok(TakeNum(num))
            }
        }
        _ => Err(From::from(val))
    }
}

// pass the filename instead of the filehandle because the filehandle will be consumed by the function,
//make it unavailable for use in selecting the bytes or lines
fn count_lines_bytes(filename: &str) -> MyResult<(i64, i64)> {
    let mut file = BufReader::new(File::open(filename)?);
    let mut num_lines = 0;
    let mut num_bytes = 0;
    let mut buf = Vec::new();
    loop {
        let bytes_read = file.read_until(b'\n', &mut buf)?;
        if bytes_read == 0 {
            break;
        }
        num_lines += 1;
        num_bytes += bytes_read as i64;
        buf.clear();
    }
    Ok((num_lines, num_bytes))
}

fn print_lines(
    mut f: impl BufRead,
    num_lines: &TakeValue,
    total_lines: i64
) -> MyResult<()> {
    if let Some(start) = get_start_index(num_lines, total_lines) {
        let mut line_num = 0;
        let mut buf = Vec::new();
        loop {
            let bytes_read = f.read_until(b'\n', &mut buf)?;
            if bytes_read == 0 {
                break;
            }
            if line_num >= start {
                print!("{}", String::from_utf8_lossy(&buf));
            }
            line_num += 1;
            buf.clear();
        }
    } 
    Ok(())
}

fn print_bytes<T: Read + Seek>(
    mut f: T,
    num_bytes: &TakeValue,
    total_bytes: i64
) -> MyResult<()> {
    if let Some(start) = get_start_index(num_bytes, total_bytes) {
        f.seek(SeekFrom::Start(start))?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;
        if !buf.is_empty() {
            print!("{}", String::from_utf8_lossy(&buf));
        }
    } 
    Ok(())
}

fn get_start_index(take_val: &TakeValue, total: i64) -> Option<u64> {
    match take_val {
        PlusZero => {
            if total > 0 {
                Some(0)
            } else {
                None
            }
        }
        TakeNum(num) => {
            //return None if the user wants to select nothing, the file is empty, or the user wants
            //to select more data than is available in the file
            if num == &0 || total == 0 || num > &total {
                None
            } else {
                //if the desired number of lines or bytes is negative, add it to the total; otherwise, subtract one from 
                //the desired number to get the zero-based offset.
                let start = if num > &0 { num - 1 } else { num + total };
                //if the user requested more lines or bytes than the file contains, i.e. start < 0, we should
                //print the entire file
                Some(if start < 0 {0} else {start as u64})
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{count_lines_bytes, parse_num, get_start_index, TakeValue::*};

    #[test]
    fn test_count_lines_bytes() {
        let res = count_lines_bytes("tests/inputs/one.txt");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), (1, 24));

        let res = count_lines_bytes("tests/inputs/ten.txt");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), (10, 49));
    }

    #[test]
    fn test_get_start_index() {
        // +0 from an empty file (0 lines/bytes) returns None
        assert_eq!(get_start_index(&PlusZero, 0), None);

        // +0 from a nonempty file returns an index that
        // is one less than the number of lines/bytes
        assert_eq!(get_start_index(&PlusZero, 1), Some(0));

        // Taking 0 lines/bytes returns None
        assert_eq!(get_start_index(&TakeNum(0), 1), None);

        // Taking any lines/bytes from an empty file returns None
        assert_eq!(get_start_index(&TakeNum(1), 0), None);

        // Taking more lines/bytes than is available returns None
        assert_eq!(get_start_index(&TakeNum(2), 1), None);

        // When starting line/byte is less than total lines/bytes,
        // return one less than starting number
        assert_eq!(get_start_index(&TakeNum(1), 10), Some(0));
        assert_eq!(get_start_index(&TakeNum(2), 10), Some(1));
        assert_eq!(get_start_index(&TakeNum(3), 10), Some(2));

        // When starting line/byte is negative and less than total,
        // return total - start
        assert_eq!(get_start_index(&TakeNum(-1), 10), Some(9));
        assert_eq!(get_start_index(&TakeNum(-2), 10), Some(8));
        assert_eq!(get_start_index(&TakeNum(-3), 10), Some(7));

        // When the starting line/byte is negative and more than the total,
        // return 0 to print the whole file
        assert_eq!(get_start_index(&TakeNum(-20), 10), Some(0));
    }

    #[test]
    fn test_parse_num() {
        // All integers should be interpreted as negative numbers
        let res = parse_num("3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(-3));

        // A leading "+" should result in a positive number
        let res = parse_num("+3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(3));

        // An explicit "-" value should result in a negative number
        let res = parse_num("-3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(-3));

        // Zero is zero
        let res = parse_num("0");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(0));

        // Plus zero is special
        let res = parse_num("+0");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), PlusZero);

        // Test boundaries
        let res = parse_num(&i64::MAX.to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MIN + 1));

        let res = parse_num(&(i64::MIN + 1).to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MIN + 1));

        let res = parse_num(&format!("+{}", i64::MAX));
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MAX));

        let res = parse_num(&i64::MIN.to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MIN));

        // A floating-point value is invalid
        let res = parse_num("3.14");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "3.14");

        // Any non-integer string is invalid
        let res = parse_num("foo");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "foo");
    }
}