use std::{io::stdin, cmp};

use anyhow::Error;

use super::Pair;

fn go() -> Result<()> {
    let pairs = stdin()
        .lines()
        .map(str::parse::<Pair>)
        .collect::<Result<Vec<_>, _>>()?;

    let mut current_best = 0;
    for pair1 in &pairs {
        for pair2 in &pairs {
            let sum1 = (pair1.clone() + pair2.clone()).magnitude();
            let sum2 = (pair2.clone() + pair1.clone()).magnitude();
            current_best = cmp::max(current_best, cmp::max(sum1, sum2));
        }
    }

    println!("{current_best}");

    Ok(())
}
