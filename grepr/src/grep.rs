use anyhow::{Context};
use std::fs::{self, File};
use std::io::{BufReader, BufRead, Write, self};
use std::error::Error;
use clap::{Command, Arg};
pub struct Config {
    file_input: String,
    file_output: Option<String>,
    count: bool,
    word: String
}

pub fn getting_args() -> MyResult<Config> {
    let arguments = Command::new("grepr")
        .author("c631c")
        .version("0.0.1")
        .about("this is grep implementation written
        written in Rust language with additional functionality.")
        .term_width(100)
        .arg(Arg::new("word")
             .long("word")
             .short('w')
             .help("Provide the word you are searching for.")
             )
        .arg(Arg::new("file_input")
             .long("file")
             .short('f')
             .required(true)
             .ignore_case(true)
             .help("Provide file(-s) just right 
             after calling the command as the second argument.")
             )
        .arg(Arg::new("file_output")
             .long("output")
             .short('o')
             )
        .arg(Arg::new("count")
             .long("count")
             .short('c')
             .help("Set as true if you want to see count of lines,
             otherwise set as false")
             )
        .get_matches();

    let file_input = arguments.get_one::<String>("file_input").context("what")?;
    let file_output = arguments.get_one::<String>("file_output");
    let word = arguments.get_one::<String>("word").unwrap();
    Ok( Config {
        word: word.to_string(),
        file_input: file_input.to_string(),
        file_output: file_output.cloned().to_owned(),
        count: arguments.args_present()
    })
             
             
}
type MyResult<T> = Result<T, Box<dyn Error>>;
pub fn run(config: Config) -> MyResult<()> {
    let mut file = open(&config.file_input)
        .map_err(|err| format!("Got error: {err}"))?;
    
    let mut file_output: Box<dyn Write> = match config.file_output {
        Some(file_output) => Box::new(File::create(file_output)?),
        _ => Box::new(io::stdout()),
    };

    let mut print = | count: u64, text: &str | -> MyResult<()> {
        if count > 0 {
            if config.count {
                write!(file_output, "{} : {}", count, text)?;
            } else {
                write!(file_output, "{}", text)?;
            }
        }
        Ok(())
    };

    let mut another_print = |text: &str | -> MyResult<()> {
        write!(file_output, "{}", text)?;
        Ok(())
    };

    let mut line = String::new();
    let mut previous = String::new();
    let mut count: u64 = 0;
    loop { 
        let bytes = file.read_line(&mut line)?; if bytes == 0 {
            break;
        }

        if line.contains(&config.word) {
            print(count, &previous)?;
            previous = line.clone();
        }
        count += 1;
        line.clear();
    }
    print(count, &previous)?;
    Ok(())

}
pub fn open(file: &str) -> MyResult<Box<dyn BufRead>> {
    match file {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file)?))),
    }
}
