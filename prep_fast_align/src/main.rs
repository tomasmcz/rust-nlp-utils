use std::env;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    match work() {
        Ok(()) => (),
        Err(e) => println!("Error: {}", e)
    }
}

fn work() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let ref filename1 = args[1];
    let ref filename2 = args[2];
    let mut f1 = try!(File::open(filename1));
    let mut f2 = try!(File::open(filename2));
    let mut s1 = String::new();
    let mut s2 = String::new();
    try!(f1.read_to_string(&mut s1));
    try!(f2.read_to_string(&mut s2));
    let mut l2 = s2.lines();
    for l in s1.lines() {
        print!("{} ||| ", l);
        println!("{}", match l2.next() {
            Some(val) => val,
            None => ""} );
    }
    Ok(())
}
