use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod components;
use crate::components::*;
pub mod parser;
use crate::parser::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = File::open(args.get(1).expect("Input file not specified")).expect("Could not open input file");
    match parse_ical(BufReader::new(file).lines()) {
        Ok(_) => println!("Successfully parsed iCal"),
        Err(err) => println!("Unable to parce iCal"),
    }
}