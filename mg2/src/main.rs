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
    line: String,
}


impl fmt::Display for Hit<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FILE {} LINE {}: {}", self.filename, self.lineno, self.line)
    }
}


impl fmt::Debug for Hit<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DEBUG FILE {} LINE {}: {}", self.filename, self.lineno, self.line)
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
        let x = line.unwrap();
        if x.contains(&query) {
            let hit = Hit { lineno: lineno, filename: &filename, line: x };
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
