extern crate nom;

mod parser;

pub fn parse_mib(mib: &str) {
    println!("I have a mib of size {}", mib.len());
    parser::why();
}