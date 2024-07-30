use clap::{parser::ValuesRef, Arg, ArgAction, ArgMatches, Command};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    dbg!(&config);
    Ok(())
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
