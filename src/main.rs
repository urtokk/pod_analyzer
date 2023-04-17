use crossterm;
use color_eyre::eyre::Result;

mod pod;

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("Hello, world!");

    Ok(())
}
