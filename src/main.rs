use std::collections::HashMap;
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
    match f {
        Ok(f) => f,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(f) => f,
                Err(e) => panic!("Error creating file: {}", e)
            },
            _ => panic!("file error"),
        },
    };
    println!("Hello, world!");
}
