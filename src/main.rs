extern crate reqwest;

use std::io::Read;

fn main() {
    let mut resp = reqwest::get("https://www.rust-lang.org").unwrap();

    let mut s = String::new();
    resp.read_to_string(&mut s);
    println!("{:?}", s);
}
