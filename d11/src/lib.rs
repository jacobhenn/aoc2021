use anyhow::{Context, Result};

use log::trace;
use std::{
    collections::HashSet,
    fmt::Display,
    io::{BufRead, Lines},
    ops::{Deref, DerefMut},
};

pub const ROWS: usize = 10;
pub const COLS: usize = 10;

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
pub struct Point {
    row: usize,
    col: usize,
}

impl Point {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn neighbors(&self) -> Neighbors {
        Neighbors::new(*self)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

pub struct Neighbors {
    center: Point,
    index: usize,
}

impl Neighbors {
    pub fn new(center: Point) -> Self {
        Self { center, index: 0 }
    }
}

impl Iterator for Neighbors {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let Self {
            center: Point { row, col },
            ..
        } = *self;

        let nth_neighbor = |n| match n {
            0 => (col < COLS - 1).then(|| Point::new(row, col + 1)),
            1 => (row > 0 && col < COLS - 1).then(|| Point::new(row - 1, col + 1)),
            2 => (row > 0).then(|| Point::new(row - 1, col)),
            3 => (row > 0 && col > 0).then(|| Point::new(row - 1, col - 1)),
            4 => (col > 0).then(|| Point::new(row, col - 1)),
            5 => (row < ROWS - 1 && col > 0).then(|| Point::new(row + 1, col - 1)),
            6 => (row < ROWS - 1).then(|| Point::new(row + 1, col)),
            7 => (row < ROWS - 1 && col < COLS - 1).then(|| Point::new(row + 1, col + 1)),
            _ => None,
        };

        while self.index < 8 && nth_neighbor(self.index).is_none() {
            self.index += 1;
        }

        let neighbor = nth_neighbor(self.index);
        self.index += 1;
        neighbor
    }
}

#[derive(Default, Debug)]
pub struct Grid([[u8; COLS]; ROWS]);

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        self.iter().for_each(|row| {
            row.iter().for_each(|num| {
                out.push_str(&format!("{}", num));
            });
            out.push_str("\n");
        });
        write!(f, "\n{}", out)
    }
}

impl Deref for Grid {
    type Target = [[u8; COLS]; ROWS];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Grid {
    pub fn step(&mut self, flashes: &mut usize) {
        for row in 0..ROWS {
            for col in 0..COLS {
                self[row][col] += 1;
            }
        }

        trace!("looking for initial flashers");
        let mut queue = Vec::new();
        for row in 0..ROWS {
            for col in 0..COLS {
                let val = self[row][col];
                if val > 9 {
                    let point = Point::new(row, col);
                    trace!("queueing {}", point);
                    queue.push(point);
                }
            }
        }

        let mut already_flashed = HashSet::new();
        while let Some(point) = queue.pop() {
            trace!("popping {}", point);
            *flashes += 1;
            self[point.row][point.col] = 0;
            already_flashed.insert(point);

            let neighbors = point.neighbors();
            for neighbor in neighbors {
                if !already_flashed.contains(&neighbor) {
                    self[neighbor.row][neighbor.col] += 1;
                    if self[neighbor.row][neighbor.col] > 9 && !queue.contains(&neighbor) {
                        queue.push(neighbor);
                    }
                }
            }
        }
    }
}

pub fn vec_to_row(vec: Vec<u8>) -> [u8; COLS] {
    let mut res = [0; COLS];
    for col in 0..vec.len() {
        res[col] = vec[col];
    }
    res
}

pub fn parse_input<R: BufRead>(lines: &mut Lines<R>) -> Result<Grid> {
    let mut grid = Grid::default();
    for row in 0..ROWS {
        let line = lines
            .next()
            .context(format!("row {}: unexpected end of input", row + 1))??;
        let digits = line
            .chars()
            .map(|c| c.to_digit(10).map(|n| n as u8))
            .collect::<Option<Vec<u8>>>()
            .context("non-digit char in input")?;
        grid[row] = vec_to_row(digits);
    }
    Ok(grid)
}
