use std::{convert::TryFrom, fmt::Display, io};

use derive_more::Display;

use anyhow::{bail, Context, Result};

/// # Errors
///
/// will return `Err` if the input does not match the expected day 20 input format.
pub fn parse_input() -> Result<([bool; 512], Image)> {
    let mut lines = io::stdin().lines();
    let alg_str = lines.next().context("missing input")??;
    let mut alg = [false; 512];
    for (i, c) in alg_str.chars().enumerate() {
        alg[i] = c == '#';
    }

    if !lines.next().context("missing input")??.is_empty() {
        bail!("invalid input");
    }

    let mut image = Image::default();
    for remaining_line in lines {
        let mut row = Vec::new();
        for c in remaining_line?.chars() {
            row.push(c == '#');
        }
        image.rows.push(row);
    }

    Ok((alg, image))
}

#[derive(Clone, Copy, Display, PartialEq, Eq)]
#[display(fmt = "{},{}", x, y)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub const fn neighbors(&self) -> Neighbors {
        Neighbors::new(*self)
    }
}

/// the neighbors of a point in a 3x3 area around it, *including the point itself*.
pub struct Neighbors {
    x: i32,
    y: i32,
    center: Point,
}

impl Neighbors {
    pub const fn new(center: Point) -> Self {
        Self {
            x: -1,
            y: -1,
            center,
        }
    }
}

impl Iterator for Neighbors {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y <= 1 {
            let res = Some(Point {
                x: self.center.x + self.x,
                y: self.center.y + self.y,
            });

            self.x += 1;
            if self.x > 1 {
                self.x = -1;
                self.y += 1;
            }

            res
        } else {
            None
        }
    }
}

#[derive(Default)]
pub struct Image {
    pub rows: Vec<Vec<bool>>,
    pub surrounded_by_true: bool,
}

impl Image {
    /// (width, height)
    pub fn dimensions(&self) -> (usize, usize) {
        (
            self.rows.first().map(Vec::len).unwrap_or_default(),
            self.rows.len(),
        )
    }

    pub fn get(&self, point: Point) -> bool {
        if point.x < 0 || point.y < 0 {
            self.surrounded_by_true
        } else {
            self.rows
                .get(point.y.unsigned_abs() as usize)
                .and_then(|r| r.get(point.x.unsigned_abs() as usize).copied())
                .unwrap_or(self.surrounded_by_true)
        }
    }

    pub fn get_enhanced_px(&self, point: Point, alg: &[bool]) -> bool {
        let mut idx = 0;
        for neighbor in point.neighbors() {
            idx <<= 1;
            if self.get(neighbor) {
                idx += 1;
            }
        }
        alg[idx]
    }

    #[must_use]
    /// # Panics
    ///
    /// will panic if `alg` is empty.
    pub fn enhance(&self, alg: &[bool]) -> Self {
        let (width, height) = self.dimensions();
        let mut new_img = Self {
            surrounded_by_true: if self.surrounded_by_true {
                *alg.last().unwrap()
            } else {
                *alg.first().unwrap()
            },
            ..Self::default()
        };

        for y in -1..=i32::try_from(height).unwrap() {
            let mut row = Vec::new();
            for x in -1..=i32::try_from(width).unwrap() {
                let value = self.get_enhanced_px(Point::new(x, y), alg);
                row.push(value);
            }
            new_img.rows.push(row);
        }
        new_img
    }

    pub fn count(&self) -> usize {
        self.rows
            .iter()
            .map(|r| r.iter().filter(|&&p| p).count())
            .sum()
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            for point in row {
                if *point {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
