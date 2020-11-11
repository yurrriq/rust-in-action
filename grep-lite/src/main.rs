extern crate clap;
extern crate regex;

use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::process;

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        match line_ {
            Ok(line) => match re.find(&line) {
                Some(_) => println!("{}", line),
                None => (),
            },
            Err(_) => {
                println!("Shit!");
                process::exit(100)
            }
        }
    }
}

fn main() {
    let args = App::new("grep-lite")
        .version("0.2.11")
        .about("Search for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("File to search")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    match args.value_of("input") {
        Some(input) => {
            let file = File::open(input).unwrap();
            let reader = BufReader::new(file);
            process_lines(reader, re);
        }
        None => {
            let stdin = io::stdin();
            let reader = stdin.lock();
            process_lines(reader, re)
        }
    }
}
