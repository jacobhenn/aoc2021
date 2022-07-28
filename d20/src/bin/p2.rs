use anyhow::{Result, Context};

fn main() -> Result<()> {
    simple_logger::init_with_env().context("couldn't initialize logger")?;

    let (alg, mut img) = d20::parse_input()?;
    for _ in 0..50 {
        img = img.enhance(&alg);
    }
    println!("{}", img.count());
    Ok(())
}
