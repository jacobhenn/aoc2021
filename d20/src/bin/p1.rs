use anyhow::{Result, Context};

fn main() -> Result<()> {
    simple_logger::init_with_env().context("couldn't initialize logger")?;

    let (alg, mut img) = d20::parse_input()?;
    for _ in 0..2 {
        img = img.enhance(&alg);
    }
    println!("{}", img.count());
    Ok(())
}

#[cfg(test)]
mod tests {
    use aoc2021::d20::*;

    #[test]
    fn neighbors() {
        let point = Point::new(5, 10);
        for neighbor in point.neighbors() {
            println!("{neighbor}");
        }
    }

    #[test]
    fn get_enhanced_px() {
        let (alg, img) = parse_input().unwrap();
        println!("{}", img.get_enhanced_px(Point::new(2, 2), &alg));
    }
}
