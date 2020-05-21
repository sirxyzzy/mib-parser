extern crate mib_parser;
use mib_parser::parse_file;
use std::path::Path;
use walkdir::WalkDir;
use std::time::Instant;

use clap::Clap;
/// Parse a MIB file
#[derive(Clap)]
#[clap(version = "1.0", author = "Andy P++ <andy@failfree.net>")]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long)]
    mib: String
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();

    let opts: Opts = Opts::parse();

    let mib_path = opts.mib;

    let path = Path::new(&mib_path);

    if path.is_dir() {
        // Batch load of MIBs
        let mut parsed_ok = 0;
        let mut parse_failed = 0;
        let extensions = vec!["txt", "mib"];
        for path in WalkDir::new(path).into_iter()
                 .filter_map(|e| e.ok())
                 .filter(|e| e.file_type().is_file())
                 .map(|e| e.into_path()) {
            if let Some(ext) = path.extension() {
                if let Some(sext) = ext.to_str() {
                    if extensions.contains(&sext.to_lowercase().as_str()) {
                        match parse_file(&path) {
                            Ok(_) => {
                                parsed_ok += 1;
                                println!("Parsed {}", path.display());
                            },
                            Err(_) => {
                                parse_failed += 1;
                                println!("Parsed failed for {}", path.display());
                            }
                        }
                    }
                }
            }
        }
        println!("{} files parsed, {} files failed to parse in {}ms", parsed_ok, parse_failed, now.elapsed().as_millis());
    } else {
        println!("Parsing {}", path.display());
        parse_file(&path)?;
        println!("Took {}ms", now.elapsed().as_millis()); 
    }

    Ok(())
}

