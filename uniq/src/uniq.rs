use anyhow::Context;
use clap::{Arg, Command};
use std::error::Error;
use std::fs::{File, self};
use std::io::{self, BufRead, BufReader, Write};

pub struct Config {
    pub file_input: String,
    pub file_output: Option<String>,
    pub count: bool,
}

type MyResult<T> = anyhow::Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("uniqrust")
        .author("c631b")
        .version("0.0.1")
        .arg(
            Arg::new("file_input")
                .short('f')
                .long("file_input")
                .default_value("-")
                .num_args(1..)
                )
        .arg(Arg::new("file_output")
             .long("file_output")
             )
        .arg(Arg::new("count")
             .long("count")
             .short('c')
             )
        .get_matches();

    let file_input = matches
        .get_one::<String>("file_input")
        .context("missing your file")?;
    let file_output = matches.get_one::<String>("file_output")
        .map(|f| f.to_string());
    Ok(Config {
        file_input: file_input.to_string(),
        file_output,
        // file_output: Some(file_output?.to_string()),
        count: matches.args_present(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    //just checks if it's able to open file and read information.
    let mut file = open(&config.file_input)
        .map_err(|err| format!("{}: {}", config.file_input, err))?;

    let mut file_output: Box<dyn Write> = match &config.file_output {
        Some(name_output) => Box::new(File::create(name_output)?),
        _ => Box::new(io::stdout().lock())

    };

    let file_len = fs::read_to_string(&config.file_input)?
        .lines()
        .count();

    //pragmatic way of displaying info in file
    let mut print = |count: u64, text: &str| -> MyResult<()> {
        if count > 0 {
            if config.count {
                write!(file_output, "{} : {}", count, text)?;
            } else {
                write!(file_output, "{}", text)?;
            }
        };
        Ok(())
    };

    //line becomes String "buffer" which takes value till
    //the end of the line or EOF. read_line()
    let mut line = String::new();
    let mut previous = String::new();
    let mut count: u64 = 0;

    if file_len.eq(&0) {
        eprintln!("Your file: {:?} is empty", config.file_input)
    }

    //can be also done with simple loop {break;}
    for _ in 0..file_len {
        file.read_line(&mut line)?;
        if line.bytes().ne(previous.bytes()){
            print(count, &previous)?;
            //ToString also implements Copy trait, so can be "efficiently" used 
            //instead of clone.
            previous = line.to_string();
            //declare uniq count for every line
            count = 0;
        }
        count += 1;
        line.clear();
    }
    print(count, &previous)
}

pub fn open(file: &str) -> MyResult<Box<dyn BufRead>> {
    match file {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        //this part should be ::open, otherwise it will overwrite
        //content of the file and give error.
        _ => Ok(Box::new(BufReader::new(File::open(file)?)))
    }
}
