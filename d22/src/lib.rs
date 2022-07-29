use std::{
    cmp::{max, min},
    io,
    ops::RangeInclusive,
    str::FromStr,
};

use anyhow::{bail, Context, Result};

use derive_more::Display;

use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dim {
    X,
    Y,
    Z,
}

#[derive(Clone, Debug, PartialEq, Eq, Display)]
#[display(
    fmt = "x={}..{},y={}..{},z={}..{}",
    "self.x_range.start()",
    "self.x_range.end()",
    "self.y_range.start()",
    "self.y_range.end()",
    "self.z_range.start()",
    "self.z_range.end()"
)]
pub struct Cuboid {
    pub x_range: RangeInclusive<i32>,
    pub y_range: RangeInclusive<i32>,
    pub z_range: RangeInclusive<i32>,
}

impl Cuboid {
    pub fn new(
        x_range: RangeInclusive<i32>,
        y_range: RangeInclusive<i32>,
        z_range: RangeInclusive<i32>,
    ) -> Self {
        Self {
            x_range,
            y_range,
            z_range,
        }
    }

    pub fn volume(&self) -> u64 {
        (1 + self.x_range.end().abs_diff(*self.x_range.start()) as u64)
            * (1 + self.y_range.end().abs_diff(*self.y_range.start()) as u64)
            * (1 + self.z_range.end().abs_diff(*self.z_range.start()) as u64)
    }

    pub fn points(&self) -> impl Iterator<Item = (i32, i32, i32)> {
        self.x_range
            .clone()
            .cartesian_product(self.y_range.clone())
            .cartesian_product(self.z_range.clone())
            .map(|((x, y), z)| (x, y, z))
    }

    pub fn contains(&self, (x, y, z): (i32, i32, i32)) -> bool {
        self.x_range.contains(&x) && self.y_range.contains(&y) && self.z_range.contains(&z)
    }

    pub fn is_initial(&self) -> bool {
        self.x_range.start() >= &-50
            && self.x_range.end() <= &50
            && self.y_range.start() >= &-50
            && self.y_range.end() <= &50
            && self.z_range.start() >= &-50
            && self.z_range.end() <= &50
    }

    fn get(&self, dim: Dim) -> &RangeInclusive<i32> {
        match dim {
            Dim::X => &self.x_range,
            Dim::Y => &self.y_range,
            Dim::Z => &self.z_range,
        }
    }

    fn with_dim(self, dim: Dim, range: RangeInclusive<i32>) -> Self {
        match dim {
            Dim::X => Self {
                x_range: range,
                ..self
            },
            Dim::Y => Self {
                y_range: range,
                ..self
            },
            Dim::Z => Self {
                z_range: range,
                ..self
            },
        }
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.x_range.start() <= other.x_range.end()
            && self.x_range.end() >= other.x_range.start()
            && self.y_range.start() <= other.y_range.end()
            && self.y_range.end() >= other.y_range.start()
            && self.z_range.start() <= other.z_range.end()
            && self.z_range.end() >= other.z_range.start()
    }

    fn partition_dim_into(
        &self,
        cuboids: &mut Vec<Cuboid>,
        dim: Dim,
        subtrahend: &RangeInclusive<i32>,
    ) {
        if self.get(dim).start() < subtrahend.start() {
            cuboids.push(
                self.clone()
                    .with_dim(dim, *self.get(dim).start()..=(*subtrahend.start() - 1)),
            );
        }

        cuboids.push(self.clone().with_dim(
            dim,
            max(*self.get(dim).start(), *subtrahend.start())
                ..=min(*self.get(dim).end(), *subtrahend.end()),
        ));

        if self.get(dim).end() > subtrahend.end() {
            cuboids.push(
                self.clone()
                    .with_dim(dim, (*subtrahend.end() + 1)..=*self.get(dim).end()),
            )
        }
    }

    /// partition `self` into smaller cuboids such that, when combined, they produce the subset of `self` which does not intersect with `other`.
    pub fn partition_by(&self, other: Self) -> impl Iterator<Item = Cuboid> {
        let mut tmp0 = Vec::with_capacity(6);
        let mut tmp1 = Vec::with_capacity(6);
        self.partition_dim_into(&mut tmp0, Dim::X, &other.x_range);
        while let Some(cuboid) = tmp0.pop() {
            if cuboid.intersects(&other) {
                cuboid.partition_dim_into(&mut tmp1, Dim::Y, &other.y_range);
            } else {
                tmp1.push(cuboid);
            }
        }
        while let Some(cuboid) = tmp1.pop() {
            if cuboid.intersects(&other) {
                cuboid.partition_dim_into(&mut tmp0, Dim::Z, &other.z_range);
            } else {
                tmp0.push(cuboid);
            }
        }

        tmp0.into_iter().filter(move |c| !c.intersects(&other))
    }
}

pub fn parse_range(s: &str) -> Result<RangeInclusive<i32>> {
    let (start, end) = s.split_once("..").context("expected `..`")?;
    Ok(start
        .parse()
        .with_context(|| format!("couldn't parse `{start}` as a i32"))?
        ..=end
            .parse()
            .with_context(|| format!("couldn't parse `{end}` as a i32"))?)
}

#[derive(Clone, Copy, PartialEq, Eq, Display, Debug)]
pub enum Polarity {
    On,
    Off,
}

impl FromStr for Polarity {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            s => bail!("{s} is not a valid polarity"),
        }
    }
}

pub fn get_input() -> impl Iterator<Item = Result<(Polarity, Cuboid)>> {
    io::stdin().lines().map(|line| {
        let line = line.context("couldn't read line from stdin")?;
        let mut line = line.as_str();

        let polarity_str;
        (polarity_str, line) = line.split_once("x=").context("expected `x=`")?;
        let x_range_str;
        (x_range_str, line) = line.split_once(",y=").context("expected `,y=`")?;
        let x_range = parse_range(x_range_str).context("couldn't parse x range")?;

        let y_range_str;
        (y_range_str, line) = line.split_once(",z=").context("expected `,z=`")?;
        let y_range = parse_range(y_range_str).context("couldn't parse y range")?;

        let z_range = parse_range(line).context("couldn't parse z range")?;

        Ok((
            polarity_str
                .parse()
                .context("couldn't parse polarity str")?,
            Cuboid::new(x_range, y_range, z_range),
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partition3() {
        let cuboid0 = Cuboid::new(1..=3, 0..=0, 2..=6);
        let cuboid1 = Cuboid::new(3..=9, 0..=0, 0..=5);
        assert!(cuboid0.intersects(&cuboid1));
        assert!(cuboid1.intersects(&cuboid0));

        let mut tmp0 = Vec::with_capacity(6);
        let mut tmp1 = Vec::with_capacity(6);
        cuboid0.partition_dim_into(&mut tmp0, Dim::X, &cuboid1.x_range);
        while let Some(cuboid) = tmp0.pop() {
            if cuboid.intersects(&cuboid1) {
                cuboid.partition_dim_into(&mut tmp1, Dim::Y, &cuboid1.y_range);
            } else {
                tmp1.push(cuboid);
            }
        }
        while let Some(cuboid) = tmp1.pop() {
            if cuboid.intersects(&cuboid1) {
                cuboid.partition_dim_into(&mut tmp0, Dim::Z, &cuboid1.z_range);
            } else {
                tmp0.push(cuboid);
            }
        }

        println!("{tmp0:#?}");
    }
}
