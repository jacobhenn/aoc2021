#![allow(dead_code)]

use std::{
    fmt::Display,
    ops::{Add, Not},
    str::FromStr,
};

pub mod parse;
pub mod part1;
pub mod part2;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
}

impl Not for Direction {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

type TreeIdx = Vec<Direction>;

fn go_left(mut idx: TreeIdx) -> TreeIdx {
    idx.push(Direction::Left);
    idx
}

fn go_right(mut idx: TreeIdx) -> TreeIdx {
    idx.push(Direction::Right);
    idx
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum GetError {
    Long,
    Short,
}

#[derive(Debug, Clone)]
enum Number {
    Pair(Pair),
    Number(u8),
}

impl Number {
    fn get(&self, idx: &[Direction]) -> Result<&u8, GetError> {
        match self {
            Self::Pair(p) => p.get(idx),
            Self::Number(n) => {
                if idx.is_empty() {
                    Ok(n)
                } else {
                    Err(GetError::Long)
                }
            }
        }
    }

    fn get_mut(&mut self, idx: &[Direction]) -> Result<&mut u8, GetError> {
        match self {
            Self::Pair(p) => p.get_mut(idx),
            Self::Number(n) => {
                if idx.is_empty() {
                    Ok(n)
                } else {
                    Err(GetError::Long)
                }
            }
        }
    }

    fn set_zero(&mut self, idx: &[Direction]) -> Result<(), GetError> {
        if idx.is_empty() {
            *self = Self::Number(0);
            Ok(())
        } else {
            match self {
                Self::Pair(p) => p.set_zero(idx),
                Self::Number(_) => Err(GetError::Long),
            }
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            Self::Pair(p) => p.magnitude(),
            Self::Number(n) => u32::from(*n),
        }
    }

    fn num_pair(x: u8, y: u8) -> Self {
        Self::Pair(Pair {
            left: Box::new(Self::Number(x)),
            right: Box::new(Self::Number(y)),
        })
    }

    fn split(&mut self) -> bool {
        match self {
            Self::Pair(p) => p.split(),
            Self::Number(n) => {
                if *n >= 10 {
                    let floor = *n / 2;
                    let ceil = if *n % 2 == 0 { floor } else { floor + 1 };
                    *self = Self::num_pair(floor, ceil);
                    true
                } else {
                    false
                }
            }
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pair(p) => write!(f, "{p}"),
            Self::Number(n) => write!(f, "{n}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pair {
    left: Box<Number>,
    right: Box<Number>,
}

impl Pair {
    fn get(&self, idx: &[Direction]) -> Result<&u8, GetError> {
        match idx.first() {
            Some(Direction::Left) => self.left.get(&idx[1..]),
            Some(Direction::Right) => self.right.get(&idx[1..]),
            None => Err(GetError::Short),
        }
    }

    fn get_mut(&mut self, idx: &[Direction]) -> Result<&mut u8, GetError> {
        match idx.first() {
            Some(Direction::Left) => self.left.get_mut(&idx[1..]),
            Some(Direction::Right) => self.right.get_mut(&idx[1..]),
            None => Err(GetError::Short),
        }
    }

    fn set_zero(&mut self, idx: &[Direction]) -> Result<(), GetError> {
        match idx.first() {
            Some(Direction::Left) => self.left.set_zero(&idx[1..]),
            Some(Direction::Right) => self.right.set_zero(&idx[1..]),
            None => Err(GetError::Short),
        }
    }

    fn neighbor_mut(&mut self, mut idx: TreeIdx, direction: Direction) -> Option<&mut u8> {
        if self.get(&idx) == Err(GetError::Long) || idx.iter().all(|d| *d == direction) {
            return None;
        }

        while idx.pop() == Some(direction) {}
        idx.push(direction);
        while self.get(&idx).is_err() {
            idx.push(!direction);
        }

        self.get_mut(&idx).ok()
    }

    fn explode(&mut self) -> Option<()> {
        let bomb = self.find_bomb()?;
        let bomb_left_idx = go_left(bomb.clone());
        let bomb_right_idx = go_right(bomb.clone());
        let bomb_right = *self.get(&bomb_right_idx).ok()?;
        let bomb_left = *self.get(&bomb_left_idx).ok()?;
        if let Some(left_neighbor) = self.neighbor_mut(bomb_left_idx, Direction::Left) {
            *left_neighbor += bomb_left;
        }

        if let Some(right_neighbor) = self.neighbor_mut(bomb_right_idx, Direction::Right) {
            *right_neighbor += bomb_right;
        }

        self.set_zero(&bomb).ok()?;
        Some(())
    }

    fn find_bomb(&self) -> Option<TreeIdx> {
        self.go_find_bomb(Vec::new())
    }

    fn go_find_bomb(&self, idx: TreeIdx) -> Option<TreeIdx> {
        let here = self.get(&idx);
        if here == Err(GetError::Short) {
            if idx.len() == 4 {
                Some(idx)
            } else {
                self.go_find_bomb(go_left(idx.clone()))
                    .or_else(|| self.go_find_bomb(go_right(idx)))
            }
        } else {
            None
        }
    }

    pub fn magnitude(&self) -> u32 {
        3 * self.left.magnitude() + 2 * self.right.magnitude()
    }

    fn split(&mut self) -> bool {
        self.left.split() || self.right.split()
    }

    pub fn reduce(&mut self) {
        while self.explode().is_some() || self.split() {}
    }
}

impl Add for Pair {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = Self {
            left: Box::new(Number::Pair(self)),
            right: Box::new(Number::Pair(rhs)),
        };
        out.reduce();
        out
    }
}

impl FromStr for Pair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, res) = parse::pair(s).map_err(|e| e.to_string())?;
        Ok(res)
    }
}

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.left, self.right)
    }
}
