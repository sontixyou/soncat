use clap::{parser::ValuesRef, Arg, ArgAction, ArgMatches, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(buf_read) => {
                let mut line_number = 1;
                for line in buf_read.lines() {
                    match line {
                        Err(err) => eprintln!("Failed to read line: {}", err),
                        Ok(line) => {
                            if config.number_lines {
                                println!("{} {}", line_number, line);
                                line_number += 1;
                            } else if config.number_nonblank_lines {
                                if !line.is_empty() {
                                    println!("{} {}", line_number, line);
                                    line_number += 1;
                                } else {
                                    println!();
                                }
                            } else {
                                println!("{}", line);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let cli: ArgMatches = Command::new("soncat")
        .author("sontixyou")
        .version("0.0.1")
        .about("print file's text")
        .arg(
            Arg::new("files")
                .value_name("FILE_NAMES")
                .help("Input files name")
                .num_args(1..)
                .default_value("-"),
        )
        .arg(
            Arg::new("number_lines")
                .short('n')
                .long("number")
                .help("Number all output lines")
                .conflicts_with("number_nonblank_lines")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .short('b')
                .long("number-nonblank")
                .help("Number only nonempty output lines")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    Ok(Config {
        files: cli
            .get_many::<String>("files")
            .unwrap()
            .map(|s| s.to_string())
            .collect(),
        number_lines: cli.get_flag("number_lines"),
        number_nonblank_lines: cli.get_flag("number_nonblank_lines"),
    })
}
