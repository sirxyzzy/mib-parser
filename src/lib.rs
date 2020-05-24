extern crate pest;
extern crate serde;
#[macro_use]
extern crate log;

use std::path::Path;
use std::fs;

pub use parser::MibInfo;

mod parser;

pub struct ParseOptions {
    pub pretty_print: bool
}

/// Parse a single file
pub fn parse_file<P: AsRef<Path>>(mib_file: &P, options: &ParseOptions) -> Result<MibInfo, Box<dyn std::error::Error>> {
    trace!("Reading {}", mib_file.as_ref().display());
    let mib_string = fs::read_to_string(mib_file)?; 
    trace!("Read {} characters", mib_string.len());
    Ok(parser::parse_mib(&mib_string, options)?)
}