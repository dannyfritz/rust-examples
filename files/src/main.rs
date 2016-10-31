use std::fs::File;
use std::io::Read;
use std::error::Error;

fn main () {
    let mut file = match File::open("content.txt") {
        Err(err) => panic!("Couldn't open: {}", err.description()),
        Ok(file) => file,
    };
    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Err(err) => panic!("Couldn't read: {}", err.description()),
        Ok(_) => println!("Content is: {}", data),
    };
}