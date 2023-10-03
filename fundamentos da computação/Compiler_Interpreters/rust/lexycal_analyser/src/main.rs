// use Scanner;
// use std::env;
use std::fs::{self};
use scanner::Scanner;

#[path = "lexycal/scanner.rs"] mod scanner;
#[path = "lexycal/token.rs"] mod token;


fn main() {
    let filename = fs::read_to_string("input.isi").expect("Fail to read the file");
    let mut scanner = Scanner::new(filename);

    let mut some_while: bool = false;

    while some_while == false {
        let token = scanner.next_token();
        if !token.is_none() {
            println!("{:?}" ,token.unwrap());
        } else if token.is_none() {
            some_while = true;
        }
    }
}
