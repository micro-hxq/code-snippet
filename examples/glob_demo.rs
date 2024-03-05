use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    for entry in glob::glob("**/*.rs")? {
        println!("{}", entry?.display());
    }
    Ok(())
}

