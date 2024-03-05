use std::env;
use std::io::Result;

fn main() -> Result<()> {
    let dir = env::current_dir()?;
    println!("Current directory is: {}", dir.display());

    Ok(())
}