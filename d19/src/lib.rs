use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Add, AddAssign, Sub},
    ptr,
    str::FromStr,
};

use anyhow::{Context, Result};

use log::trace;

use derive_more::Display;

pub const MIN_OVERLAP: u8 = 12;

#[derive(Default, PartialEq, Eq, Clone, Copy, Debug, Display)]
#[display(fmt = "{},{},{}", x, y, z)]
pub struct Point {
    x: i16,
    y: i16,
    z: i16,
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut pt = Self::default();
        let mut ns = s.split(',');
        pt.x = ns.next().context("expected comma")?.parse::<i16>()?;
        pt.y = ns.next().context("expected comma")?.parse::<i16>()?;
        pt.z = ns.next().context("expected comma")?.parse::<i16>()?;
        Ok(pt)
    }
}

impl Sub<Self> for Point {
    type Output = Displacement;

    fn sub(self, rhs: Self) -> Self::Output {
        Displacement {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add<Displacement> for Point {
    type Output = Self;

    fn add(self, rhs: Displacement) -> Self::Output {
        Self {
            x: rhs.x + self.x,
            y: rhs.y + self.y,
            z: rhs.z + self.z,
        }
    }
}

impl AddAssign<Displacement> for Point {
    fn add_assign(&mut self, rhs: Displacement) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Point {
    pub const fn new(x: i16, y: i16, z: i16) -> Self {
        Self { x, y, z }
    }

    pub fn get_mut(&mut self, dim: Dim) -> &mut i16 {
        match dim {
            Dim::X => &mut self.x,
            Dim::Y => &mut self.y,
            Dim::Z => &mut self.z,
        }
    }

    pub fn swap(&mut self, swap: &Swap) {
        // SAFETY: all ptrs are cast from references, and so are valid and aligned.
        match swap {
            Swap::None => (),
            Swap::Once(a, b) => unsafe { ptr::swap(self.get_mut(*a), self.get_mut(*b)) },
            Swap::Twice(a, b, c) => unsafe {
                ptr::swap(self.get_mut(*a), self.get_mut(*b));
                ptr::swap(self.get_mut(*b), self.get_mut(*c));
            },
        }
    }

    pub fn unswap(&mut self, swap: &Swap) {
        // SAFETY: all ptrs are cast from references, and so are valid and aligned.
        match swap {
            Swap::None => (),
            Swap::Once(a, b) => unsafe { ptr::swap(self.get_mut(*a), self.get_mut(*b)) },
            Swap::Twice(a, b, c) => unsafe {
                ptr::swap(self.get_mut(*b), self.get_mut(*c));
                ptr::swap(self.get_mut(*a), self.get_mut(*b));
            },
        }
    }

    pub fn refl(&mut self, refl: &Reflect) {
        if refl.x {
            self.x *= -1;
        }
        if refl.y {
            self.y *= -1;
        }
        if refl.z {
            self.z *= -1;
        }
    }

    pub fn rotate(&mut self, Rotation(swap, refl): &Rotation) {
        self.swap(swap);
        self.refl(refl);
    }

    pub fn unrotate(&mut self, Rotation(swap, refl): &Rotation) {
        self.refl(refl);
        self.unswap(swap);
    }

    /// return the squared euclidean distance between `self` and `other`.
    pub const fn dist_squclid(&self, rhs: &Self) -> u32 {
        // (1000 - (-1000)).pow(2) == 4000000
        let xd = (self.x.abs_diff(rhs.x) as u32).pow(2);
        let yd = (self.y.abs_diff(rhs.y) as u32).pow(2);
        let zd = (self.z.abs_diff(rhs.z) as u32).pow(2);
        // 4000000 * 3 == 12000000 < u32::MAX
        xd + yd + zd
    }

    /// return the squared euclidean distance between `self` and the origin.
    pub const fn abs_squclid(&self) -> u32 {
        (self.x.unsigned_abs() as u32).pow(2)
            + (self.y.unsigned_abs() as u32).pow(2)
            + (self.z.unsigned_abs() as u32).pow(2)
    }

    pub const fn dist_taxicab(&self, rhs: &Self) -> u16 {
        self.x.abs_diff(rhs.x) + self.y.abs_diff(rhs.y) + self.z.abs_diff(rhs.z)
    }

    /// return the taxicab distance between `self` and the origin.
    pub const fn abs_taxicab(&self) -> u16 {
        self.x.unsigned_abs() + self.y.unsigned_abs() + self.z.unsigned_abs()
    }

    // TODO: make this work for edge cases
    /// assuming that these points have the same euclidean distance from the origin, and that they have already been `unswap`ped according to `swap_diff`, find what `Reflect`ion they differ by.
    /// if one of the dimensions is zero, this will always say that that dimension was not reflected.
    pub const fn refl_diff(&self, other: Self) -> Reflect {
        Reflect {
            x: self.x.signum() != other.x.signum(),
            y: self.y.signum() != other.y.signum(),
            z: self.z.signum() != other.z.signum(),
        }
    }

    // TODO: make this work for edge cases
    /// assuming that these points have the same euclidean distance from the origin, find what `Swap` could be perfomed to transform `self` into `other`.
    /// if two of the dimensions are the same, this will only output one of the possible swaps that the points differ by.
    pub const fn swap_diff(&self, other: Self) -> Swap {
        if abs_eq(self.x, other.x) {
            if abs_eq(self.y, other.y) && abs_eq(self.z, other.z) {
                Swap::None
            } else {
                Swap::Once(Dim::Y, Dim::Z)
            }
        } else if abs_eq(self.y, other.y) {
            Swap::Once(Dim::X, Dim::Z)
        } else if abs_eq(self.z, other.z) {
            Swap::Once(Dim::X, Dim::Y)
        } else if abs_eq(self.x, other.z) {
            Swap::Twice(Dim::X, Dim::Y, Dim::Z)
        } else {
            Swap::Twice(Dim::Z, Dim::Y, Dim::X)
        }
    }

    /// assuming it can be done, return the `Rotation` which would turn `self` into `other`.
    pub fn rot_diff(mut self, other: Self) -> Rotation {
        let swap = self.swap_diff(other);
        self.swap(&swap);
        let refl = self.refl_diff(other);
        Rotation(swap, refl)
    }
}

const fn abs_eq(lhs: i16, rhs: i16) -> bool {
    lhs == rhs || lhs + rhs == 0
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Display)]
pub enum Dim {
    X,
    Y,
    Z,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Display)]
#[display(
    fmt = "{}{}{}",
    r#"if *x { "-" } else { "+" }"#,
    r#"if *y { "-" } else { "+" }"#,
    r#"if *z { "-" } else { "+" }"#,
)]
pub struct Reflect {
    pub x: bool,
    pub y: bool,
    pub z: bool,
}

impl Reflect {
    pub const fn along(dim: Dim) -> Self {
        Self {
            x: matches!(dim, Dim::X),
            y: matches!(dim, Dim::Y),
            z: matches!(dim, Dim::Z),
        }
    }

    pub const fn total(&self) -> u8 {
        self.x as u8 + self.y as u8 + self.z as u8
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Display)]
pub enum Swap {
    #[display(fmt = "none")]
    None,
    #[display(fmt = "{}<>{}", _0, _1)]
    Once(Dim, Dim),
    #[display(fmt = "{}<>{}<>{}", _0, _1, _2)]
    Twice(Dim, Dim, Dim),
}

impl Swap {
    pub const fn total(&self) -> u8 {
        match self {
            Self::None => 0,
            Self::Once(_, _) => 1,
            Self::Twice(_, _, _) => 2,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Display)]
#[display(fmt = "{} {}", _0, _1)]
pub struct Rotation(pub Swap, pub Reflect);

impl Rotation {
    pub const fn is_valid(&self) -> bool {
        (self.0.total() + self.1.total()) % 2 == 0
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, Display, Default)]
#[display(fmt = "{},{},{}", x, y, z)]
pub struct Displacement {
    x: i16,
    y: i16,
    z: i16,
}

impl Displacement {
    pub const fn dist_taxicab(&self) -> i16 {
        self.x + self.y + self.z
    }
}

pub struct Scanner {
    pub beacons: Vec<Point>,
}

impl Display for Scanner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !f.alternate() {
            writeln!(f, "--- scanner ---")?;
        }
        for beacon in &self.beacons {
            writeln!(f, "{}", beacon)?;
        }

        Ok(())
    }
}

impl AddAssign<Displacement> for Scanner {
    fn add_assign(&mut self, rhs: Displacement) {
        for beacon in &mut self.beacons {
            *beacon += rhs;
        }
    }
}

impl Scanner {
    pub fn new(beacons: Vec<Point>) -> Self {
        Self { beacons }
    }

    pub fn rotate(&mut self, rot: &Rotation) {
        for beacon in &mut self.beacons {
            beacon.rotate(rot);
        }
    }

    /// return a list of distances from `center` to all points in `self`. distances are guaranteed to be in the same order as points in `self`.
    pub fn dists(&self, center: &Point) -> Vec<u32> {
        self.beacons
            .iter()
            .map(|b| b.dist_squclid(center))
            .collect()
    }

    /// take two scanners. return `Some(`the rotation & translation needed to turn `self` into `other``)` if they have at least `MIN_OVERLAP` points in common.
    pub fn diff(&self, other: &Self) -> Option<(Rotation, Displacement)> {
        // maps reference frame diffs to the number of pairs found for them.
        let mut possible_diffs: HashMap<(Rotation, Displacement), u8> = HashMap::new();

        // for each pair of points, assume that they are the same point. when they are, the diff will be found.
        for origin in &self.beacons {
            let dists = self.dists(origin);
            for target in &other.beacons {
                trace!("if {origin} == {target}:");
                // test distances from `target` to points in `other` to see if they match the distances from `origin` to points in `self`.
                for aux in &other.beacons {
                    let dist = target.dist_squclid(aux);
                    if dist == 0 {
                        continue;
                    }

                    for (i, _) in dists.iter().enumerate().filter(|&(_, &d)| d == dist) {
                        // found a match! assuming that `aux` and `self[i]` are also the same point, find what the diff between `self` and `other` is, then increment that entry in `possible_diffs`.
                        let rel_aux = Point::new(0, 0, 0) + (*aux - *target);
                        let rel_ith = Point::new(0, 0, 0) + (self.beacons[i] - *origin);
                        if rel_ith == Point::new(0, 0, 0) {
                            continue;
                        }

                        let rot = rel_aux.rot_diff(rel_ith);
                        if !rot.is_valid() {
                            continue;
                        }

                        let mut target = *target;
                        target.rotate(&rot);
                        let dsp = *origin - target;
                        let entry = possible_diffs
                            .entry((rot, dsp))
                            .or_insert(1 /*for origin->target*/);
                        *entry += 1;

                        trace!(
                            "    {aux} == {} ({rel_aux} == {rel_ith}) with rot {rot}",
                            self.beacons[i]
                        );

                        if *entry >= MIN_OVERLAP {
                            return Some((rot, dsp));
                        }
                    }
                }
            }
        }

        None
    }

    /// extend `self` with `other`, assuming that they have already been converted to be in the same reference frame. avoids duplicates.
    pub fn extend(&mut self, other: &mut Self) {
        while let Some(beacon) = other.beacons.pop() {
            if !self.beacons.contains(&beacon) {
                self.beacons.push(beacon);
            }
        }
    }
}
