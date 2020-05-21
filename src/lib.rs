extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::path::Path;
use std::fs;

mod parser;

pub fn parse_file<P: AsRef<Path>>(mib_file: P) {
    let file = fs::read_to_string(mib_file);
    match file {
        Err(e) => println!("Failed to open {}", e),
        Ok(mib_string) => {
            println!("I have a mib of size {}", mib_string.len());
            parser::parse_mib(&mib_string);
        }
    }

}