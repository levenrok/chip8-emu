use std::env;
use std::fs;

use anyhow::{Result, bail};

mod rom;
use rom::Rom;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        bail!("specify a rom file to load!");
    }

    let filename = &args[1];

    let _rom_data: Vec<u8> = fs::read(filename)?;

    println!("loading ROM: \x1b[1m{}\x1b[0m", filename);

    Ok(())
}
