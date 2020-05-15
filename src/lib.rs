extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;

mod parser;

pub fn parse_file(mib_file: &str) {
    let mib_string = fs::read_to_string(mib_file).unwrap();

    println!("I have a mib of size {}", mib_string.len());
    parser::parse_mib(&mib_string);
}