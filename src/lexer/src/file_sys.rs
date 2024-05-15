use std::fs;
use std::fs::File;
use std::io::Read;
use std::process::exit;

pub fn file_to_string(path: &String) -> String {
    let mut f = match File::open(&path) {
        Ok(f) => f,
        Err(_) => {
            println!("Error: File not found at path \"{path}\"");
            exit(65);
        }
    };
    let metadata = fs::metadata(&path).expect("Unable to read file metadata.");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("Buffer overflow from file.");

    String::from_utf8_lossy(&*buffer).into_owned()
}

