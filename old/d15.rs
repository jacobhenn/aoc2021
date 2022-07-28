use log::trace;
use std::{
    error::Error,
    io::{BufRead, Lines},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub struct Point {
    heuristic_cost: u16,
    lowest_cost: u16,
    pos: (usize, usize),
    cost: u8,
    visited: bool,
}

impl Point {
    pub fn new(pos: (usize, usize), cost: u8) -> Self {
        Self {
            pos,
            cost,
            visited: false,
            lowest_cost: u16::MAX,
            heuristic_cost: u16::MAX,
        }
    }
}

pub fn parse_input<R: BufRead>(lines: &mut Lines<R>) -> Result<Vec<Vec<Point>>, Box<dyn Error>> {
    let mut rows = Vec::new();
    let mut y = 0;

    while let Some(Ok(line)) = lines.next() {
        let mut row = Vec::new();
        let mut x = 0;

        for char in line.chars() {
            let cost = char.to_digit(10).ok_or("non-digit char")? as u8;
            row.push(Point::new((x, y), cost));
            x += 1;
        }

        rows.push(row);
        y += 1;
    }

    trace!("parsed input: {:?}", rows);

    Ok(rows)
}
