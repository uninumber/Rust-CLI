use anyhow::Context;
use clap::{Arg, Command};
use regex::{Regex, RegexBuilder};
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::mem;
use walkdir::WalkDir;
#[derive(Debug, Clone)]
pub struct Config {
    pub files: Vec<String>,
    pub count: bool,
    pub pattern: Regex,
    pub recursive: bool,
}

pub fn getting_args() -> MyResult<Config> {
    let arguments = Command::new("grepr")
        .author("c631c")
        .version("0.0.1")
        .about(
            "this is grep implementation written
        written in Rust language with additional functionality.",
        )
        .term_width(100)
        .arg(
            Arg::new("pattern")
                .value_name("PATTERN")
                .help("Search pattern")
                .required(true),
        )
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .num_args(1..)
                .default_value("-"),
        )
        .arg(
            Arg::new("insensitive")
                .short('i')
                .long("insensitive")
                .help("Case-insensitive"),
        )
        .arg(
            Arg::new("recursive")
                .action(clap::ArgAction::SetTrue)
                .short('r')
                .long("recursive")
                .help("Recursive search"),
        )
        .arg(
            Arg::new("count")
                .action(clap::ArgAction::SetTrue)
                .short('c')
                .long("count")
                .help("Count occurrences"),
        )
        .arg(
            Arg::new("invert")
                .short('v')
                .long("invert-match")
                .help("Invert match"),
        )
        .get_matches();

    let files = arguments
        .get_one::<String>("files")
        .map(|v| vec![v.to_string()])
        .context("error occured during files initialization")?;
    let pattern = arguments
        .get_one::<String>("pattern")
        .context("error ocurred during pattern initialization")?;
    let pattern = RegexBuilder::new(&pattern)
        .build()
        .map_err(|_| format!("Invalid pattern \"{}\"", pattern))?;
    Ok(Config {
        count: arguments.args_present(),
        pattern,
        files,
        recursive: arguments.args_present()
    })
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    let entries = find_files(config.files, config.recursive);
    let file_num = entries.len();

    let print = |fname: &str, val: &str| {
        if file_num > 0 {
            print!("{} : {}", fname, val)
        } else {
            print!("{}", val);
        }
    };
    for entry in entries {
        match entry {
            Err(e) => eprintln!("{}", e),
            Ok(filename) => match open(&filename) {
                Err(e) => eprintln!("{}: {}", filename, e),
                Ok(file) => match find_lines(file, &config.pattern) {
                    Err(e) => eprintln!("{}", e),
                    Ok(files) => {
                        for line in &files {
                            print(&filename, line)
                        }
                    }
                },
            },
        }
    }
    Ok(())
}

pub fn find_lines<T: BufRead>(
    mut file: T,
    pattern: &Regex) -> MyResult<Vec<String>> {
    let mut matches = vec![];
    let mut line = String::new();

    loop {
        let bytes = file.read_line(&mut line)?;
        if line.contains(&pattern.to_string()) {
            matches.push(mem::take(&mut line));
        }
        line.clear();
        if bytes.eq(&0) {
            break;
        }
    }
    Ok(matches)
}

fn find_files(paths: Vec<String>, recursive: bool) -> Vec<MyResult<String>> {

    let mut results = vec![];

    for path in &paths {
        match path.as_str() {
            "-" => eprintln!("Provide some file"),
            _ => match fs::metadata(&path) {
                Ok(metadata) => {
                    if metadata.is_dir() {
                        if recursive {
                            for entry in WalkDir::new(path)
                                .into_iter()
                                .flatten()
                                .filter(|v| v.file_type().is_file())
                            {
                                results.push(Ok(entry.path().display().to_string()));
                            }
                        } else {
                            results.push(Err(From::from(format!("{} is a directory", path))));
                        }
                    } else if metadata.is_file() {
                        results.push(Ok(path.to_string()));
                    }
                }
                Err(e) => results.push(Err(From::from(format!("{}: {}", path, e)))),
            },
        }
    }
    results
}

pub fn open(file: &str) -> MyResult<Box<dyn BufRead>> {
    match file {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file)?))),
    }
}
