use clap::{Arg, Command};
use std::io::{BufRead, BufReader, self};
use std::fs::File;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

pub struct Config {
    pub lines: u32,
    pub files: Vec<String>,
}

fn get_args() -> Result<Config, Box<dyn Error>> {
    let matches = Command::new("uniqrust")
        .author("cm39n")
        .name("uniqrust")
        .version("v0.0.1")
        .arg(
            Arg::new("uniq")
            .short('u')
            .long("uniq")
            .ignore_case(true)
            .help("fix me later."),
            )
        .arg(
            Arg::new("files")
            .short('f')
            .long("files")
            .help("enter files that u want to process"),
            )
        .arg(
            Arg::new("lines")
            .short('l')
            .long("lines")
            .help("enter amount of lines;"),
            )
        .get_matches();
    let lines: u32 = matches 
        .get_one::<String>("files")
        .expect("something")
        .parse::<u32>()
        .expect("something");

    let files: Vec<String> = matches
        .get_many::<String>("files")
        .expect("cheeses what is going on")
        .map(|f| f.into())
        .collect();
    Ok(Config {files, lines})
}

// pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
//
// }

pub fn open(file: &str) -> MyResult<Box<dyn BufRead>> {
    match file {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file)?))),
    }
}
