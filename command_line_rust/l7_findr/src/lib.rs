use crate::EntryType::*;
use clap::{Command, Arg, ArgAction};
use regex::Regex;
use walkdir::{WalkDir, DirEntry};
use std::error::Error;


#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>
}

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    let type_filter = |entry: &DirEntry| {
        config.entry_types.is_empty() 
            || config.entry_types.iter().any(|entry_type| {
                match entry_type {
                    Link => entry.file_type().is_symlink(),
                    Dir => entry.file_type().is_dir(),
                    File => entry.file_type().is_file()
                }
        })
    };
    let name_filter = |entry: &DirEntry| {
        config.names.is_empty() 
            || config.names.iter().any(|re| {
                re.is_match(&entry.file_name().to_string_lossy())
        })
    };
    for path in config.paths {
        let entries = WalkDir::new(path)
            .into_iter()
            .filter_map(|e| match e {
                Err(e) => {
                    eprint!("{}", e);
                    None
                }
                Ok(entry) => Some(entry)
            })
            .filter(type_filter)
            .filter(name_filter)
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<_>>();
        println!("{}", entries.join("\n"));
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("findr")
        .version("0.1.0")
        .author("Zhang Tianwei <zhangtianwei1015@gmail.com>")
        .about("Rust find")
        .arg(
            Arg::new("paths")
                .value_name("PATH")
                .help("Search paths")
                .default_value(".")
                .action(ArgAction::Append)
        )
        .arg(
            Arg::new("names")
                .value_name("NAME")
                .long("name")
                .short('n')
                .help("Name")
                .action(ArgAction::Append)
        )
        .arg(
            Arg::new("types")
                .value_name("TYPE")
                .long("type")
                .short('t')
                .help("Entry type")
                .value_parser(["d", "l", "f"])
                .value_delimiter(',')
                .action(ArgAction::Append)
        )
        .get_matches();
    let paths = matches
        .get_many::<String>("paths")
        .unwrap()
        .map(|v| v.into())
        .collect::<Vec<String>>();
    let names = matches
        .get_many::<String>("names")
        .unwrap_or_default()
        .map(|v| Regex::new(v).map_err(|_| format!("Invalid --name \"{}\"", v)).unwrap())
        .collect::<Vec<Regex>>();
    let entry_types = matches
        .get_many::<String>("types")
        .unwrap_or_default()
        .map(|v| match (*v).as_str() {
            "d" => Dir,
            "f" => File,
            "l" => Link,
            _ => unreachable!("Invalid type")
        })
        .collect::<Vec<EntryType>>();
    Ok(Config {
        paths,
        names,
        entry_types
    })
}
