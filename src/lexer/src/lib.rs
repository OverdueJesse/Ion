use crate::file_sys::file_to_string;
use scanner::Scanner;

mod file_sys;
pub mod types;
mod scanner;


pub fn main(path: &String) {
    run(file_to_string(path));
}

fn run(source: String) {
    let mut scanner = Scanner::new(&source);
    let tokens = scanner.scan_tokens();

    for token in tokens.iter() {
        println!("{}", token.to_string());
    }
}

