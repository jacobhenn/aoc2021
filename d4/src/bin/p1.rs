use anyhow::{Result, Context};

use std::io;

fn main() -> Result<()> {
    simple_logger::init_with_env().context("couldn't initialize logger")?;
    let mut lines = io::stdin().lines();

    let nums = d4::parse_nums(&mut lines)?;
    let mut boards = d4::parse_boards(&mut lines)?;

    for num in nums {
        for board in &mut boards {
            board.draw_num(num);
            if board.has_won() {
                println!("winning score: {}", board.score(num));
                return Ok(());
            }
        }
    };

    Ok(())
}
