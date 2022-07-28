use std::{error::Error, io::BufRead};

use super::Pair;

pub fn go<R: BufRead>(handle: &mut R) -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    handle.read_to_string(&mut input)?;
    let pairs = input
        .lines()
        .map(str::parse::<Pair>)
        .collect::<Result<Vec<_>, _>>()?;
    let res = pairs
        .into_iter()
        .reduce(|p, q| p + q)
        .ok_or_else(|| "empty input".to_string())?;
    println!("{}", res.magnitude());
    Ok(())
}
