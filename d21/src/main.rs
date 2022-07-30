use anyhow::{Result, Context};

fn main() -> Result<()> {
    let mut multiverse = d21::get_input().context("couldn't get input")?;
    let mut p1_win_count = 0;

    let mut step = 0;
    while !multiverse.state_counts.is_empty() {
        print!("step {step}: ");
        print!("len: {:>8}   ", multiverse.state_counts.len());
        let win_count = multiverse.tick_both();
        println!("p1 wins: {win_count}");
        p1_win_count += win_count;
        step += 1;
    }

    println!("{p1_win_count}");

    Ok(())
}
