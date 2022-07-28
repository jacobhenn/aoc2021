use anyhow::{Result, Context};

use log::trace;

use std::io;

fn main() -> Result<()> {
    simple_logger::init_with_env().context("couldn't initialize logger")?;
    let mut lines = io::stdin().lines();

    let mut grid = d11::parse_input(&mut lines)?;
    trace!("initial grid: {}", grid);

    let mut flashes = 0;
    for _ in 0..100 {
        grid.step(&mut flashes);
    }

    println!("{flashes}");

    Ok(())
}
