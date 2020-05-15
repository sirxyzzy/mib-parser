extern crate mib_parser;
use mib_parser::parse_mib;

use std::fs;
use clap::Clap;
/// Parse a MIB file
#[derive(Clap)]
#[clap(version = "1.0", author = "Andy P++ <andy@failfree.net>")]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long)]
    mib: String
}


fn main() {
    let opts: Opts = Opts::parse();

    let mib_file = opts.mib;

    println!("Parsing {}", mib_file);

    let mib_string = fs::read_to_string(mib_file).unwrap();

    parse_mib(&mib_string);  
}


