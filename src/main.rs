use std::{
    env,
    process::{exit},
};
use lexer::{scanner::Scanner, file_sys::file_to_string};
use parser::parser::Parser;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => compile(&args[1]),
        _ => {
            println!("Error: No filename or path provided.");
            exit(64);
        },
    }
}

fn compile(path: &String) {
    // lexical analysis

    let file_path = file_to_string(path);
    let mut scanner = Scanner::new(&file_path);
    scanner.scan_tokens();
    let tokens = scanner.tokens;

    for token in tokens.iter() {
        println!("{}", token.to_string());
    }

    // parsing
    let _parser = Parser::new(tokens);
}
