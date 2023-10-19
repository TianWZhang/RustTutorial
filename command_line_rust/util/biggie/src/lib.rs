use clap::{Command, Arg, ArgAction, value_parser};
use rand::{distributions::Alphanumeric, Rng};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use thousands::Separable;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    outfile: String,
    lines: usize,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("biggie")
        .version("0.1.0")
        .author("Zhang Tianwei <zhangtianwei1015@gmail.com>")
        .about("Make big text files to stress test a program")
        .arg(
            Arg::new("lines")
                .value_name("LINES")
                .long("lines")
                .short('n')
                .help("Number of lines")
                .default_value("100000")
                .value_parser(value_parser!(usize))
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("outfile")
                .value_name("FILE")
                .long("outfile")
                .short('o')
                .help("Output filename")
                .default_value("out")
                .action(ArgAction::Set)
        )
        .get_matches();

    Ok(Config {
        lines: *(matches.get_one("lines")).unwrap(),
        outfile: matches.get_one::<String>("outfile").unwrap().to_string(),
    })
}


// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let mut file = File::create(&config.outfile)?;
    for _ in 0..config.lines {
        let num_words = rand::thread_rng().gen_range(7..15);
        let mut words = vec![];
        for _ in 0..num_words {
            words.push(random_string());
        }
        writeln!(file, "{}", words.join(" "))?;
    }

    println!(
        "Done, wrote {} line{} to \"{}\".",
        config.lines.separate_with_commas(),
        if config.lines == 1 { "" } else { "s" },
        config.outfile
    );

    Ok(())
}

// --------------------------------------------------
fn random_string() -> String {
    let length = rand::thread_rng().gen_range(2..8);
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
