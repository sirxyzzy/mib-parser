#[macro_use]
extern crate log;
extern crate mib_parser;

use mib_parser::{parse_file, ParseOptions};
use std::path::Path;
use walkdir::WalkDir;
use std::time::Instant;


use clap::Clap;
/// Parse a MIB file or files
/// This program uses env_logger, for detailed tracing set RUST_LOG=trace
#[derive(Clap)]
#[clap(version = "1.0", author = "Andy P++ <andy@failfree.net>")]
struct Opts {
    /// The path to a single MIB file or a folder full of MIB files (batch mode)
    #[clap(short, long)]
    mib: String,

    /// Set verbose to list parsed files and show the location of parse fails
    #[clap(short,long)]
    verbose: bool,

    // Pretty print the parse tree
    #[clap(short,long)]
    pretty: bool,

    // Print the result as yaml
    #[clap(short,long)]
    yaml: bool,    
}

/// See if a given path has one of a number of file extensions (case insensitive)
/// The list of extensions to match MUST all be lowercase
pub fn match_ext<P: AsRef<Path>>(path: &P, extensions: &[&str]) -> bool {
    if let Some(ext) = path.as_ref().extension() {
        if let Some(sext) = ext.to_str() {
            return extensions.contains(&sext.to_lowercase().as_str())
        }
    }
    false
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();

    let opts: Opts = Opts::parse();

    env_logger::init();

    let path = Path::new(&opts.mib);

    let options = ParseOptions { pretty_print: opts.pretty };

    if path.is_dir() {
        // Batch load of MIBs
        let mut parsed_ok = 0;
        let mut parse_failed = 0;

        // Extensions we care about
        let extensions = vec!["txt", "mib"];

        for path in WalkDir::new(path).into_iter()
                 .filter_map(|e| e.ok())
                 .filter(|e| e.file_type().is_file())
                 .map(|e| e.into_path()) // Dir entries keep a file lock, so consume them into paths
                 .filter(|p| match_ext(p, &extensions)) { 
            match parse_file(&path, &options) {
                Ok(info) => {
                    parsed_ok += 1;
                    if opts.verbose {
                        println!("Parsed {}", path.display());
                    }

                    if opts.yaml {
                        println!("{}", serde_yaml::to_string(&info)?);
                    }
                },
                Err(e) => {
                    parse_failed += 1;
                    error!("Parsed failed for {}", path.display());
                    if opts.verbose {
                        println!("{}", e)
                    }
                }
            }
        }
        println!("{} files parsed, {} files failed to parse in {}ms", parsed_ok, parse_failed, now.elapsed().as_millis());
    } else {
        trace!("Parsing {}", path.display());
        match parse_file(&path, &options) {
            Err(e) => error!("Parse failed {}", e),
            Ok(info) => {
                if opts.yaml {
                    println!("{}", serde_yaml::to_string(&info)?);
                }
            }
        }
        trace!("Took {}ms", now.elapsed().as_millis()); 
    }

    Ok(())
}

