use clap::{Arg, ArgAction, Command};
use std::env;

pub struct Args {
    pub infile: String,
    pub outfile: String,
    pub silent: bool,
}

impl Args {
    pub fn parse() -> Self {
        let matches = Command::new("pipeviewer")
            .version("0.1.0")
            .author("Zhang Tianwei <zhangtianwei1015@gmail.com>")
            .about("Rust pipe viewer")
            .arg(
                Arg::new("infile")
                    .value_name("INFILE")
                    .help("Read from a file instead of stdin")
                    .action(ArgAction::Set),
            )
            .arg(
                Arg::new("outfile")
                    .value_name("OUTFILE")
                    .short('o')
                    .long("outfile")
                    .help("Write output to a file instead of stdout")
                    .action(ArgAction::Set),
            )
            .arg(
                Arg::new("silent")
                    .value_name("SILENT")
                    .short('s')
                    .long("silent")
                    .help("Do not print additional messages")
                    .action(ArgAction::SetTrue),
            )
            .get_matches();
        let infile = matches
            .get_one::<String>("infile")
            .map(|f| f.to_string())
            .unwrap_or_default();
        let outfile = matches
            .get_one::<String>("outfile")
            .map(|f| f.to_string())
            .unwrap_or_default();
        let silent =
            matches.get_flag("silent") || !env::var("PV_SILENT").unwrap_or_default().is_empty();
        Self {
            infile,
            outfile,
            silent,
        }
    }
}
