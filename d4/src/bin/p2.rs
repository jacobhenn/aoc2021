use anyhow::{Result, Context};

use std::io;

fn main() -> Result<()> {
    simple_logger::init_with_env().context("couldn't initialize logger")?;
    let mut lines = io::stdin().lines();

    let nums = d4::parse_nums(&mut lines)?;
    let mut boards = d4::parse_boards(&mut lines)?;

    for num in nums {
        let mut tbd = Vec::new();
        for i in 0..boards.len() {
            let board = &mut boards[i];
            board.draw_num(num);
            if board.has_won() {
                let score = board.score(num);
                if boards.len() == 1 {
                    println!("winning score: {}", score);
                    return Ok(());
                } else {
                    tbd.insert(0, i);
                }
            }
        }

        for i in tbd {
            boards.remove(i);
        }
    };

    Ok(())
}
