use std::fs;
mod parse;

fn main() {
    let data = fs::read_to_string("/etc/hosts").expect("Unable to read file");
    println!("{}", data);
}