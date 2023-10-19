use clap::{Command, Arg, ArgAction};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Write}
};

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    let mut file = open(&config.in_file)
        .map_err(|e| format!("{}: {}", config.in_file, e))?;
    let mut out_file: Box<dyn Write> = match &config.out_file {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout())
    };
    let mut line = String::new();
    let mut pre_line = String::new();
    let mut num = 0;
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        //both string are trimmed of any possible trailing whitespace
        if line.trim_end() != pre_line.trim_end() {
            write!(out_file, "{}{}", format_field(num, config.count && num > 0), pre_line)?;
            num = 0;
            pre_line = line.clone();
        }
        num += 1;
        line.clear();
    }
    write!(out_file, "{}{}", format_field(num, config.count && num > 0), pre_line)?;
    Ok(())
}

fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>4} ", value)
    } else {
        "".to_string()
    }
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("uniqr")
        .version("0.1.0")
        .author("Zhang Tianwei <zhangtianwei1015@gmail.com>")
        .about("Rust uniq")
        .arg(
            Arg::new("in_file")
                .value_name("IN_FILE")
                .help("Input file")
                .default_value("-")
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("out_file")
                .value_name("OUT_FILE")
                .help("Output file")
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("count")
                .value_name("COUNT")
                .long("count")
                .short('c')
                .help("Print the counts of each line")
                .action(ArgAction::SetTrue)
        )
        .get_matches();
    Ok(Config {
        in_file: matches.get_one::<String>("in_file").unwrap().to_string(),
        out_file: matches.get_one::<String>("out_file").map(String::from),
        count: matches.get_flag("count")
    })
}


fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}