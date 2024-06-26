use lexer::{file_sys::file_to_string, scanner::Scanner};
use parser::parser::Parser;
use std::{env, process::exit};

pub fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => compile(&args[1]),
        _ => {
            println!("Error: No filename or path provided.");
            exit(64);
        }
    }
}

fn compile(path: &String) {
    // lexical analysis

    let file_path = file_to_string(path);
    let mut scanner = Scanner::new(&file_path);
    scanner.scan_tokens();
    let mut tokens = scanner.tokens;
    
    // parsing
    let mut parser = Parser::new(&mut tokens);
    let ast = parser.parse_program();
    ast.print_nodes();
}
