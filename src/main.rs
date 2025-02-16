use std::env;
use std::fs::File;
use std::io::BufReader;

pub mod components;
use crate::components::*;
pub mod parser;
use crate::parser::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = File::open(args.get(1).expect("Input file not specified")).expect("Could not open input file");
    parse_ical(&mut BufReader::new(file));
}