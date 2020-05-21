extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::path::Path;
use std::fs;

mod parser;

pub fn parse_file<P: AsRef<Path>>(mib_file: &P) -> Result<(), Box<dyn std::error::Error>> {
    let mib_string = fs::read_to_string(mib_file)?;
    parser::parse_mib(&mib_string)
}