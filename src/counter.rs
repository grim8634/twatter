use std::io;

use std::io::prelude::*;
use std::fs::File;

pub fn set(value:u64)->Result<(), io::Error> {
    let mut f = try!(File::create("counter.txt"));
    f.write_all(value.to_string().as_bytes()).unwrap();
    Ok(())
}

pub fn get()->u64 {
    let mut f = File::open("counter.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let val = s.parse::<u64>();
    match val {
        Ok(value) => value,
        Err(err) => {
            println!("{:?}", err);
            0 as u64
        }
    }
}
