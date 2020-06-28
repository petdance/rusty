#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]

use std::process;
use std::fmt;
use std::result;

use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;


pub struct Hit<'a> {
    lineno: usize,
    filename: &'a str,
}


impl fmt::Display for Hit<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LINE {}", self.lineno)
    }
}


impl fmt::Debug for Hit<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DEBUG LINE {}", self.lineno)
    }
}


fn main() -> io::Result<()> {
    let filename = "Cargo.toml".to_string();

    let f = match File::open(&filename) {
        Ok(v) => v,
        Err(e) => return Err(From::from(e))
    };

    let f = BufReader::new(f);

    let mut results = Vec::<Hit>::new();
    let query = "re".to_string();
    let mut lineno = 0;
    for line in f.lines() {
        lineno = lineno + 1;
        if line.unwrap().contains(&query) {
            let hit = Hit { lineno: lineno, filename: &filename };
            println!("HIT {:#?}", hit);
            println!("HIT {}", hit);
            results.push(hit)
        }
    }
    println!("Here are the results");
    println!("{:?}", results);
    for res in results {
        println!("{:?}", res);
    }

    Ok(())
}
