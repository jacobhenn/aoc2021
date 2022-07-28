use std::{io::BufRead, error::Error};

use crate::d15::SnailfishNumber;

pub fn go<R: BufRead>(handle: &mut R) -> Result<(), Box<dyn Error>> {
    let mut lines = handle.lines();
    let mut n = lines.next().ok_or("empty input")??.parse::<SnailfishNumber>()?;
    for line in lines {
        let m = line?.parse::<SnailfishNumber>()?;
        n += m;
    }

    println!("{}", n.magnitude());

    Ok(())
}
