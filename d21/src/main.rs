// use log::info;

use d21::Multiverse;

// real data
const P1_START: u8 = 6;
const P2_START: u8 = 8;

// test data
// const P1_START: u8 = 3;
// const P2_START: u8 = 7;

fn main() {
    let mut multiverse = Multiverse::new(P1_START, P2_START);
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
}
