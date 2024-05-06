use lexer;
use std::{
    env,
    process::{exit},
};

pub fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => compile(&args[1]),
        _ => {
            println!("error no file given");
            exit(64);
        },
    }
}

fn compile(path: &String) {
    lexer::main(path);
}
