use clap::{Command, Arg, ArgAction, value_parser};
use rand::{SeedableRng, rngs::StdRng, seq::SliceRandom};
use regex::{Regex, RegexBuilder};
use walkdir::WalkDir;
use std::{
    error::Error,
    fs::{File, self},
    io::{BufRead, BufReader}, 
    path::PathBuf, 
    ffi::OsStr
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    sources: Vec<String>,
    pattern: Option<Regex>,
    seed: Option<u64>,
}

#[derive(Debug)]
struct Fortune {
    source: String,//filename containing the record
    text: String //contents of the record up to but not including the terminating percent sign
}

pub fn run(config: Config) -> MyResult<()> {
    let files = find_files(&config.sources)?;
    let fortunes = read_fortunes(&files)?;
    if let Some(pattern) = config.pattern {
        let mut prev_source = None;
        for fortune in fortunes.iter().filter(|f| pattern.is_match(&f.text)) {
            // print all the fortunes matching the pattern
            // print the source header if the current source is not the same as the previous one seen
            if prev_source.as_ref().map_or(true, |s| s != &fortune.source) {
                eprintln!("({})\n%", fortune.source);
                prev_source = Some(fortune.source.clone());
            }
            println!("{}\n%", fortune.text);
        }
    } else {
        // select and print one fortune
        match pick_fortune(&fortunes, config.seed) {
            Some(fortune) => println!("{}", fortune),
            None => println!("No fortunes found")
        }
    }   
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("fortuner")
        .version("0.1.0")
        .author("Zhang Tianwei <zhangtianwei1015@gmail.com>")
        .about("Rust fortune")
        .arg(
            Arg::new("sources")
                .value_name("FILE")
                .help("Input files or directories")
                .required(true)
                .action(ArgAction::Append)
        )
        .arg(
            Arg::new("pattern")
                .long("pattern")
                .short('m')
                .value_name("PATTERN")
                .help("Pattern")
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("seed")
                .value_name("SEED")
                .long("seed")
                .short('s')
                .help("Random seed")
                .value_parser(value_parser!(u64))
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("insensitive")
                .long("insensitive")
                .short('i')
                .help("Case-insensitive pattern matching")
                .action(ArgAction::SetTrue)
        )
        .get_matches();
    let sources = matches
        .get_many::<String>("sources")
        .unwrap()
        .map(|v| v.into())
        .collect::<Vec<String>>();
    let pattern = matches.get_one::<String>("pattern")
        .map(|val| { //use Option::map to handle Some(val)
            RegexBuilder::new(val)
            .case_insensitive(matches.get_flag("insensitive"))
            .build()
            .map_err(|_| format!("Invalid --pattern \"{}\"", val))
        }) 
        .transpose()?;//The result of Option::map will be an Option<Result>, and Option::transpose will turn this into a Result<Option>.
    let seed = matches.get_one::<u64>("seed").map(|b| *b);
    Ok(Config {
        sources,
        pattern,
        seed
    })
}

fn find_files(paths: &[String]) -> MyResult<Vec<PathBuf>> {
    let dat = OsStr::new("dat");
    let mut res = vec![];
    for path in paths {
        match fs::metadata(path) {
            Err(e) => return Err(From::from(format!("{}: {}", path, e))),
            Ok(_) => res.extend(
                WalkDir::new(path)
                        .into_iter()
                        .filter_map(Result::ok)
                        .filter(|e| e.file_type().is_file() && e.path().extension() != Some(dat))
                        .map(|e| e.path().into()) //convert Path to PathBuf
            )
        }
    }
    res.sort();
    res.dedup(); //remove consecutive repeated values
    Ok(res)
}

fn read_fortunes(paths: &[PathBuf]) -> MyResult<Vec<Fortune>> {
    let mut res = vec![];
    let mut buf = vec![];
    for path in paths {
        let basename = path.file_name().unwrap().to_string_lossy().into_owned();
        let file = File::open(path)
            .map_err(|e| {
                format!("{}: {}", path.to_string_lossy().into_owned(), e)
            })?;
        for line in BufReader::new(file).lines().filter_map(Result::ok) {
            if line == "%" {
                //read the lines into a buffer and dump the buffer when the record or file terminator is found
                if !buf.is_empty() {
                    res.push(Fortune {source: basename.clone(), text: buf.join("\n")});
                    buf.clear();
                }
            } else {
                buf.push(line.to_string());
            }
        }
    }
    Ok(res)
}

fn pick_fortune(fortunes: &[Fortune], seed: Option<u64>) -> Option<String> {
    // if fortunes.len() == 0 {
    //     None
    // } else {
    //     let index = if let Some(seed) = seed {
    //         rand::rngs::StdRng::seed_from_u64(seed).gen_range(0..fortunes.len())
    //     } else {
    //        rand::thread_rng().gen_range(0..fortunes.len())
    //     };
    //     Some(fortunes[index].text.to_string())
    // }
    if let Some(val) = seed { 
        let mut rng = StdRng::seed_from_u64(val);
        fortunes.choose(&mut rng).map(|f| f.text.to_string())
    } else {
        let mut rng = rand::thread_rng();
        fortunes.choose(&mut rng).map(|f| f.text.to_string())
    }
}


#[cfg(test)]
mod tests {
    use super::{
        find_files, read_fortunes, Fortune, pick_fortune,  
    };
    use std::path::PathBuf;


    #[test]
    fn test_find_files() {
        // Verify that the function finds a file known to exist
        let res = find_files(&["./tests/inputs/jokes".to_string()]);
        assert!(res.is_ok());

        let files = res.unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(
            files.get(0).unwrap().to_string_lossy(),
            "./tests/inputs/jokes"
        );

        // Fails to find a bad file
        let res = find_files(&["/path/does/not/exist".to_string()]);
        assert!(res.is_err());

        // Finds all the input files, excludes ".dat"
        let res = find_files(&["./tests/inputs".to_string()]);
        assert!(res.is_ok());

        // Check number and order of files
        let files = res.unwrap();
        assert_eq!(files.len(), 5);
        let first = files.get(0).unwrap().display().to_string();
        assert!(first.contains("ascii-art"));
        let last = files.last().unwrap().display().to_string();
        assert!(last.contains("quotes"));

        // Test for multiple sources, path must be unique and sorted
        let res = find_files(&[
            "./tests/inputs/jokes".to_string(),
            "./tests/inputs/ascii-art".to_string(),
            "./tests/inputs/jokes".to_string(),
        ]);
        assert!(res.is_ok());
        let files = res.unwrap();
        assert_eq!(files.len(), 2);
        if let Some(filename) = files.first().unwrap().file_name() {
            assert_eq!(filename.to_string_lossy(), "ascii-art".to_string())
        }
        if let Some(filename) = files.last().unwrap().file_name() {
            assert_eq!(filename.to_string_lossy(), "jokes".to_string())
        }
    }

    #[test]
    fn test_read_fortunes() {
        // Parses all the fortunes without a filter
        let res = read_fortunes(&[PathBuf::from("./tests/inputs/jokes")]);
        assert!(res.is_ok());

        if let Ok(fortunes) = res {
            // Correct number and sorting
            assert_eq!(fortunes.len(), 6);
            assert_eq!(
                fortunes.first().unwrap().text,
                "Q. What do you call a head of lettuce in a shirt and tie?\n\
                A. Collared greens."
            );
            assert_eq!(
                fortunes.last().unwrap().text,
                "Q: What do you call a deer wearing an eye patch?\n\
                A: A bad idea (bad-eye deer)."
            );
        }

        // Filters for matching text
        let res = read_fortunes(&[
            PathBuf::from("./tests/inputs/jokes"),
            PathBuf::from("./tests/inputs/quotes"),
        ]);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().len(), 11);
    }

    #[test]
    fn test_pick_fortune() {
        // Create a slice of fortunes
        let fortunes = &[
            Fortune {
                source: "fortunes".to_string(),
                text: "You cannot achieve the impossible without \
                      attempting the absurd."
                    .to_string(),
            },
            Fortune {
                source: "fortunes".to_string(),
                text: "Assumption is the mother of all screw-ups."
                    .to_string(),
            },
            Fortune {
                source: "fortunes".to_string(),
                text: "Neckties strangle clear thinking.".to_string(),
            },
        ];

        // Pick a fortune with a seed
        assert_eq!(
            pick_fortune(fortunes, Some(1)).unwrap(),
            "Neckties strangle clear thinking.".to_string()
        );
    }
}