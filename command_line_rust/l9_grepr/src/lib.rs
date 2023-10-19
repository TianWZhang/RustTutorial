use clap::{Command, Arg, ArgAction};
use regex::{Regex, RegexBuilder};
use walkdir::WalkDir;
use std::{
    error::Error,
    fs::{self, File},
    io::{self, BufRead, BufReader}, vec, mem
};

#[derive(Debug)]
pub struct Config {
    pattern: Regex,
    files: Vec<String>,
    recursive: bool,//whether or not to recursively search directories
    count: bool,//whether or not to display a count of the matches
    invert_match: bool//whether or not to find lines that do not match the pattern
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    let entries = find_files(&config.files, config.recursive);
    let mutli = entries.len() > 1;
    for entry in entries {
        match entry {
            Err(e) => eprintln!("{}", e),
            Ok(filename) => match open(&filename) {
                Err(e) => eprintln!("{}: {}", filename, e),
                Ok(f) => {
                    match find_lines(f, &config.pattern, config.invert_match) {
                        Err(e) => eprintln!("{}", e),
                        Ok(matches) => {
                            if config.count {
                                if mutli {
                                    println!("{}:{}", filename, matches.len());
                                } else {
                                    println!("{}", matches.len());
                                }
                                
                            } else {
                                for line in matches {
                                    if mutli {
                                        print!("{}:{}", filename, line);
                                    } else {
                                        print!("{}", line);
                                    }
                                }
                            }       
                        }
                    }
                               
                }
            }
        }
    }
    Ok(())
}

fn find_files(paths: &[String], recursive: bool) -> Vec<MyResult<String>> {
    let mut res = vec![];
    for path in paths {
        match path.as_str() {
            "-" => res.push(Ok("-".to_string())),
            _ => match fs::metadata(path) {
                Ok(metadata) => {
                    if metadata.is_dir() {
                        if recursive {
                            for entry in  WalkDir::new(path)
                                .into_iter()
                                .flatten()//Iterator::flatten will take the Ok or Some variants
                                .filter(|e| e.file_type().is_file())
                            {
                                res.push(Ok(entry.path().display().to_string()));
                            }
                        } else {
                            res.push(Err(format!("{} is a directory", path).into()));
                        }  
                    } else if metadata.is_file() {
                        res.push(Ok(path.to_string()));
                    }
                }
                //This arm will be triggered by nonexistent files
                Err(e) => res.push(Err(From::from(format!("{}: {}", path, e))))
            }
        }
    }
    res
}

fn find_lines<T: BufRead>(
    mut file: T,
    pattern: &Regex,
    invert_match: bool
) -> MyResult<Vec<String>> {
    let mut res = vec![];
    let mut line = String::new();
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        if pattern.is_match(line.as_str()) ^ invert_match {
            res.push(mem::take(&mut line));//To take the ownership of the line. Could also use clone, but take avoids an unnecessary copy
        }
        line.clear();
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::{find_files, find_lines};
    use rand::{distributions::Alphanumeric, Rng};
    use regex::{Regex, RegexBuilder};
    use std::io::Cursor;

    #[test]
    fn test_find_files() {
        // Verify that the function finds a file known to exist
        let files = find_files(&["./tests/inputs/fox.txt".to_string()], false);
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].as_ref().unwrap(), "./tests/inputs/fox.txt");

        // The function should reject a directory without the recursive option
        let files = find_files(&["./tests/inputs".to_string()], false);
        assert_eq!(files.len(), 1);
        if let Err(e) = &files[0] {
            assert_eq!(e.to_string(), "./tests/inputs is a directory");
        }

        // Verify the function recurses to find four files in the directory
        let res = find_files(&["./tests/inputs".to_string()], true);
        let mut files: Vec<String> = res
            .iter()
            .map(|r| r.as_ref().unwrap().replace("\\", "/"))
            .collect();
        files.sort();
        assert_eq!(files.len(), 4);
        assert_eq!(
            files,
            vec![
                "./tests/inputs/bustle.txt",
                "./tests/inputs/empty.txt",
                "./tests/inputs/fox.txt",
                "./tests/inputs/nobody.txt",
            ]
        );

        // Generate a random string to represent a nonexistent file
        let bad: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        // Verify that the function returns the bad file as an error
        let files = find_files(&[bad], false);
        assert_eq!(files.len(), 1);
        assert!(files[0].is_err());
    }

    #[test]
    fn test_find_lines() {
        let text = b"Lorem\nIpsum\r\nDOLOR";

        // The pattern _or_ should match the one line, "Lorem"
        let re1 = Regex::new("or").unwrap();
        let matches = find_lines(Cursor::new(&text), &re1, false);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 1);

        // When inverted, the function should match the other two lines
        let matches = find_lines(Cursor::new(&text), &re1, true);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 2);

        // This regex will be case-insensitive
        let re2 = RegexBuilder::new("or")
            .case_insensitive(true)
            .build()
            .unwrap();

        // The two lines "Lorem" and "DOLOR" should match
        let matches = find_lines(Cursor::new(&text), &re2, false);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 2);

        // When inverted, the one remaining line should match
        let matches = find_lines(Cursor::new(&text), &re2, true);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 1);
    }
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("grepr")
        .version("0.1.0")
        .author("Zhang Tianwei <zhangtianwei1015@gmail.com>")
        .about("Rust grep")
        .arg(
            Arg::new("pattern")
                .value_name("PATTERN")
                .help("Search pattern")
                .required(true)
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input files")
                .default_value("-")
                .action(ArgAction::Append)
        )
        .arg(
            Arg::new("recursive")
                .long("recursive")
                .short('r')
                .help("Recursive search")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("insensitive")
                .long("insensitive")
                .short('i')
                .help("Case-insensitive")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("invert-match")
                .long("invert-match")
                .short('v')
                .help("Invert match")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("count")
                .long("count")
                .short('c')
                .help("Count occurrences")
                .action(ArgAction::SetTrue)
        )
        .get_matches();
    let files = matches
        .get_many::<String>("files")
        .unwrap()
        .map(|v| v.into())
        .collect::<Vec<String>>();
    let pattern = matches.get_one::<String>("pattern").unwrap(); //it should be safe to unwrap the value since the pattern is required
    let pattern = RegexBuilder::new(pattern)
        .case_insensitive(matches.get_flag("insensitive"))
        .build()
        .map_err(|_| format!("Invalid pattern \"{}\"", pattern))?;
    Ok(Config {
        pattern,
        files,
        recursive: matches.get_flag("recursive"),
        count: matches.get_flag("count"),
        invert_match: matches.get_flag("invert-match")
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}
