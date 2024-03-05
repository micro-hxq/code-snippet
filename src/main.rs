use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

fn main() -> Result<(), Error> {
    let path = Path::new("src/main.rs");
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        println!("line:{i} {}", line?);
    }
    Ok(())
}
