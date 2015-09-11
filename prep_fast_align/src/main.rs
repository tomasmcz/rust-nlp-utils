use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
    match work() {
        Ok(()) => (),
        Err(e) => {
            println!("\nError: {}", e);
            std::process::exit(1);
        }
    }
}

fn work() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let ref filename1 = args[1];
    let ref filename2 = args[2];
    let f1 = try!(File::open(filename1));
    let f2 = try!(File::open(filename2));
    let s1 = BufReader::new(f1);
    let s2 = BufReader::new(f2);
    let mut it = s1.lines().zip(s2.lines());
    while let Some((Ok(l1), Ok(l2))) = it.next() { 
        println!("{} ||| {}", l1, l2);
    }
    Ok(())
}
