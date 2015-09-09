use std::env;
use std::io::prelude::*;
use std::io::BufReader;
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
    let f1 = try!(File::open(filename1));
    let f2 = try!(File::open(filename2));
    let s1 = BufReader::new(f1);
    let s2 = BufReader::new(f2);
    let mut l2 = s2.lines();
    for l in s1.lines() {
        print!("{} ||| ", try!(l));
        println!("{}", try!(match l2.next() {
            Some(val) => val,
            None => unimplemented!()
            })
        );
    }
    Ok(())
}
