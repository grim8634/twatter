use std::io;

use std::io::prelude::*;
use std::fs::File;

pub fn set(value:u64, path: &str)->Result<(), io::Error> {
    let mut f = try!(File::create(path));
    f.write_all(value.to_string().as_bytes()).unwrap();
    Ok(())
}

pub fn get(path: &str)->u64 {
    match File::open(path) {
        Ok(mut f) => {
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
        },
        Err(err) => {
            println!("{:?}", err);
            0 as u64
        }
    }
}
