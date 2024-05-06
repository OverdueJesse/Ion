pub mod types;

use std::process::exit;
use types::{CompilerError};

pub fn error(error: &CompilerError) {
    report(error);
}

fn report(error: &CompilerError) {
    println!("[LINE {}, COL: {}] {:?}: {}",
             error.line,
             error.col,
             error.error_type,
             error.message
             // error.location,
    );
    exit(65);
}
