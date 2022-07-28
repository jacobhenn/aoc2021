use std::{
    cmp::{max, min},
    ops::RangeInclusive,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dim {
    X,
    Y,
    Z,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cuboid {
    pub x_range: RangeInclusive<usize>,
    pub y_range: RangeInclusive<usize>,
    pub z_range: RangeInclusive<usize>,
}

impl Cuboid {
    pub fn new(
        x_range: RangeInclusive<usize>,
        y_range: RangeInclusive<usize>,
        z_range: RangeInclusive<usize>,
    ) -> Self {
        Self {
            x_range,
            y_range,
            z_range,
        }
    }

    pub fn get(&self, dim: Dim) -> &RangeInclusive<usize> {
        match dim {
            Dim::X => &self.x_range,
            Dim::Y => &self.y_range,
            Dim::Z => &self.z_range,
        }
    }

    pub fn with_dim(self, dim: Dim, range: RangeInclusive<usize>) -> Self {
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

    pub fn partition_by(
        &self,
        dim: Dim,
        subtrahend: RangeInclusive<usize>,
        cuboids: &mut Vec<Cuboid>,
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
            cuboids.push(self.clone().with_dim(dim, (*self.get(dim).end() + 1)..=*subtrahend.end()))
        }
    }
}
