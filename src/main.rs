extern crate serde_bytes;
extern crate serde_derive;
extern crate rmp_serde as rmps;
extern crate rmpv;

use serde_json;
use rmpv::Value;
use rmpv::decode;
use std::io::{self, Read};

fn parse(data : &str) -> serde_json::Result<serde_json::Value> {
    serde_json::from_str(data)
}

fn main() {
    let value : Value = decode::read_value(io::stdin().by_ref()).unwrap();
    let output = str::replace(&format!("{}", value), "nil", "null");
    match parse(&output) {
        Ok(parsed) => {
            match serde_json::to_string_pretty(&parsed) {
                Ok(s) => println!("{}", s),
                Err(_) => println!("{}", output),
            }
        },
        Err(_) => println!("{}", output),
    }
}