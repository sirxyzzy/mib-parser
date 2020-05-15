extern crate mib_parser;
use mib_parser::parse_file;

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

    parse_file(&mib_file);  
}


