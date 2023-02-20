use clap::{Arg, Command};
use std::io::{BufRead, BufReader, self};
use std::fs::File;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    pub in_file: String,
    pub out_file: Option<String>,
    pub count: bool,
}

pub fn get_args() -> Result<Config, Box<dyn Error>> {
    let matches = Command::new("uniqrust")
        .author("cm39n")
        .name("uniqrust")
        .version("v0.0.1")
        .arg(
            Arg::new("out_file")
            .long("uniq")
            .ignore_case(true)
            .help("fix me later."),
            )
        .arg(
            Arg::new("in_file")
            .long("in_file")
            .help("enter files that u want to process"),
            )
        .arg(
            Arg::new("counts")
            .short('c')
            .long("counts")
            .help("Show counts of lines"),
            )
        .get_matches();
    Ok( Config {
        in_file: matches.get_one::<String>("in_file").map(Into::into).unwrap(),
        out_file: matches.get_one::<String>("in_file").map(|v| v.to_string()),
        count: matches.args_present(),
    })

}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
                    //Go to bufread file. Read a lot about BufRead and std::io.
                    //You though about ipmlementing read() with bytes. Seems like a bad  idea on
                    //practice
                    //I believe in u.
    Ok(())
}

pub fn open(file: &str) -> MyResult<Box<dyn BufRead>> {
    match file {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file)?))),
    }
}
