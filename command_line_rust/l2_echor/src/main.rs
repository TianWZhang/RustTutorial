use clap::{Command, Arg, ArgAction};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Zhang Tianwei <zhangtianwei1015@gmail.com>")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .action(ArgAction::Append)
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline, i.e. omit the trailing newline")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let text = matches
                                .get_many::<String>("text")
                                .unwrap()
                                .map(|v| v.into())
                                .collect::<Vec<String>>();
    let omit_newline = matches.get_flag("omit_newline");
    let ending = if omit_newline {""} else {"\n"};
    //print all the positional cmd args joined on spaces
    print!("{}{}", text.join(" "), ending);

    
    // println!("{:#?}", matches);
}

// cargo run -- -n Hello world
// cargo run -- -h
// cargo run -- -V
// cargo run 1>out 2>err