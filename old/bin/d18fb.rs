use std::{io::stdin, error::Error};

use anyhow::Result;

use crate::d18flat::SnailfishNumber;

fn main() -> Result<()> {
    let mut ns = Vec::new();
    for line in stdin().lines() {
        let n = line?.parse::<SnailfishNumber>()?;
        ns.push(n);
    }

    let mut max = 0;
    for (i, n) in ns.iter().enumerate() {
        for (j, m) in ns.iter().enumerate() {
            if i != j {
                let magnitude = (n.clone() + m.clone()).magnitude();
                if magnitude > max {
                    max = magnitude;
                }
            }
        }
    }

    println!("{max}");

    Ok(())
}
