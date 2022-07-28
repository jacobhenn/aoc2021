use anyhow::{Result, Context};

use log::trace;

use std::{
    io::{BufRead, Lines},
    str::FromStr,
};

pub const ROW_SIZE: usize = 5;
pub const COL_SIZE: usize = 5;

pub type Grid<T> = [[T; ROW_SIZE]; COL_SIZE];

#[derive(Default, Debug)]
pub struct Board {
    nums: Grid<u8>,
    mask: Grid<bool>,
}

impl Board {
    pub fn has_won(&self) -> bool {
        let mut row_pr = [true; COL_SIZE];
        let mut col_pr = [true; ROW_SIZE];
        for y in 0..COL_SIZE {
            for x in 0..ROW_SIZE {
                let b = self.mask[y][x];
                row_pr[y] &= b;
                col_pr[x] &= b;
            }
        }
        row_pr.iter().any(|b| *b) || col_pr.iter().any(|b| *b)
    }

    pub fn draw_num(&mut self, num: u8) {
        for y in 0..COL_SIZE {
            for x in 0..ROW_SIZE {
                self.mask[y][x] |= self.nums[y][x] == num;
            }
        }
    }

    pub fn score(&self, num: u8) -> u32 {
        let mut res = 0;
        for y in 0..COL_SIZE {
            for x in 0..ROW_SIZE {
                if !self.mask[y][x] {
                    res += self.nums[y][x] as u32;
                }
            }
        }
        res * num as u32
    }
}

pub fn parse_boards<R: BufRead>(lines: &mut Lines<R>) -> Result<Vec<Board>> {
    let mut res = Vec::new();

    while lines.next().is_some() {
        trace!("parsing new board");

        let mut board = Board::default();
        for y in 0..COL_SIZE {
            trace!("  parsing row {}", y);

            let line = lines.next().context("eof mid-board")??;
            let mut row = line.split_whitespace();

            for x in 0..ROW_SIZE {
                trace!("    parsing row {}, col {}", y, x);

                let n: u8 = row.next().context("row too short")?.parse()?;
                board.nums[y][x] = n;
            }
        }

        res.push(board);
    }

    Ok(res)
}

pub fn parse_nums<R: BufRead>(lines: &mut Lines<R>) -> Result<Vec<u8>> {
    let nums_raw = lines.next().context("empty input")??;
    let nums = nums_raw
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<Vec<u8>, <u8 as FromStr>::Err>>()?;
    Ok(nums)
}
