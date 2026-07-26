use std::{fs, io::Error};
fn main() {
    match divide(5.0, 0.0) {
        Err(e) => {
            println!("{}", e);
        }
        Ok(v) => {
            println!("{}", v)
        }
    }
}
fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("Cannot divide by 0"))
    } else {
        Ok(a / b)
    }
}
