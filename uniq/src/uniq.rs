use anyhow::Context;
use clap::{Arg, Command};
use std::error::Error;
use std::fs::{File, self};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

pub struct Config {
    pub in_file: String,
    pub file_output: Option<String>,
    pub count: bool,
}

type MyResult<T> = anyhow::Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("uniqrust")
        .author("c631b")
        .version("0.0.1")
        .arg(
            Arg::new("in_file")
                .short('f')
                .long("in_file")
                .default_value("-")
                )
        .arg(Arg::new("file_output")
             .long("file_output")
             )
        .arg(Arg::new("count")
             .long("count")
             .short('c')
             )
        .get_matches();

    let in_file = matches
        .get_one::<String>("in_file")
        .context("missing your file")?;
    // let file_output = matches
    //     .get_one::<String>("file_output")
    //     .context("cannot produce your file");
    let file_output = matches.get_one::<String>("file_output").map(|v| v.to_string());
    Ok(Config {
        in_file: in_file.to_string(),
        file_output,
        // file_output: Some(file_output?.to_string()),
        count: matches.args_present(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut file = open(&config.in_file).map_err(|err| format!("{}: {}", config.in_file, err))?;

    let mut some: Box<dyn Write> = match &config.file_output {
        Some(name_output) => Box::new(File::create(name_output)?),
        _ => Box::new(io::stdout()),
    };

    let file_len = fs::read_to_string(&config.in_file)?
        .lines()
        .count();

    let mut print = |count: u64, file: &str| -> MyResult<()> {
        if count > 0 {
            if config.count {
                write!(some, "{} : {}", count, file)?;
            } else {
                write!(some, "{}", file)?;
            }
        };
        Ok(())
    };

    let mut line = String::new();
    let mut previous = String::new();
    let mut count: u64 = 0;

    for _ in 0..=file_len {
        let bytes = file.read_line(&mut line)?;
        if bytes.eq(&0) {
            eprintln!("Warning: file {} is empty", file);
        }
        if line != previous {
            print(count, &previous)?;
            previous = line.clone();
            count = 0;
        }
        count += 1;
        line.clear()
    }
    print(count, &previous)
}

pub fn open(file: &str) -> MyResult<Box<dyn BufRead>> {
    match file {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file)?))),
    }
}
