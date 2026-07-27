use std::{fs, io::Error};
fn main() {
    match fs::read_to_string("logs.txt") {
        Err(e) => {
            println!("Error:=> {}", e);
        }
        Ok(v) => {
            println!("{}", v)
        }
    }
}
