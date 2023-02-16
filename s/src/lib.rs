use clap::{Arg, Command};

use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct Config {
    pub lines: u32,
    pub files: Vec<String>,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("head")
        .version("0.0.1")
        .author("c39mn")
        .about("This is command line utilitie 'head' implemented in Rust language")
        .arg(
            Arg::new("files")
                .short('f')
                .long("file")
                .num_args(1..)
                .action(clap::ArgAction::Set)
                .default_value("-")
                .help("Provides some help"),
        )
        .arg(
            Arg::new("lines")
                .short('l')
                .long("lines")
                .default_value("10")
                .action(clap::ArgAction::Set)
                .help("Provides some help"),
        )
        .get_matches();

    let lines: u32 = matches
        .get_one::<String>("lines")
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let files: Vec<String> = (*matches
        .get_many::<String>("files")
        .expect("cheese")
        // .unwrap()
        .map(|s| s.into())
        // .into_iter()
        .collect::<Vec<String>>())
    .to_vec();

    Ok(Config {
        lines: lines,
        files: files,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let num_of_files = config.files.len();
    for (file_count, filename) in config.files.iter().enumerate() {
        match open(filename) {
            Err(error) => eprintln!("{} : {}", filename, error),
            Ok(mut file) => {
                if num_of_files > 1 {
                    println!(
                        "{} ==>{} ===",
                        if file_count > 0 { "\n" } else { "" },
                        filename
                    );
                }
                let mut line = String::new();

                for _ in 0..config.lines {
                    let bytes = file.read_line(&mut line)?;
                    if bytes == 0 {
                        break;
                    }
                    print!("{}", line);
                    line.clear();
                }
            }
        }
    }
    Ok(())
}

pub fn parse_positive(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(val) if val > 0 => Ok(val),
        _ => Err(From::from(val)),
    }
}

pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
