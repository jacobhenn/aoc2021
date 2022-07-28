use std::{
    convert::{Infallible, TryFrom},
    fmt::Display,
    ops::{AddAssign, Add},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Leaf {
    depth: usize,
    num: u16,
}

impl Display for Leaf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}d{}", self.num, self.depth)
    }
}

#[derive(Clone)]
pub struct SnailfishNumber {
    leaves: Vec<Leaf>,
}

impl Display for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let leaves: Vec<String> = self.leaves.iter().map(ToString::to_string).collect();
        f.write_str(&leaves.join(" "))?;
        Ok(())
    }
}

impl FromStr for SnailfishNumber {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut leaves = Vec::with_capacity(16);
        let mut depth = 0;
        for c in s.chars() {
            match c {
                '[' => depth += 1,
                ']' => depth -= 1,
                c => {
                    if let Some(num) = c.to_digit(10) {
                        leaves.push(Leaf {
                            depth,
                            num: u16::try_from(num).unwrap(),
                        });
                    }
                }
            }
        }

        Ok(Self { leaves })
    }
}

impl Add for SnailfishNumber {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign for SnailfishNumber {
    fn add_assign(&mut self, mut rhs: Self) {
        self.leaves.append(&mut rhs.leaves);
        for leaf in &mut self.leaves {
            leaf.depth += 1;
        }
        self.reduce();
    }
}

impl SnailfishNumber {
    /// `score` and `go_score` expect their input to be a valid tree, and will silently succeed with wacky results if that is not the case.
    pub fn magnitude(&self) -> u16 {
        let mut stack: Vec<Leaf> = Vec::with_capacity(16);
        let mut leaves = self.leaves.iter();
        stack.push(*leaves.next().unwrap());
        for leaf in leaves {
            let mut leaf = *leaf;
            while stack.last().map(|l| l.depth) == Some(leaf.depth) {
                let top = stack.pop().unwrap();
                leaf.num *= 2;
                leaf.num += 3 * top.num;
                leaf.depth -= 1;
            }
            stack.push(leaf);
        }
        stack[0].num
    }

    pub fn reduce(&mut self) {
        while self.explode() || self.split() {}
    }

    fn explode(&mut self) -> bool {
        for (i, leaf) in self.leaves.iter_mut().enumerate() {
            if leaf.depth > 4 {
                let left = self.leaves.remove(i);
                if i > 0 {
                    self.leaves[i - 1].num += left.num;
                }

                if i < self.leaves.len() - 1 {
                    self.leaves[i + 1].num += self.leaves[i].num;
                }

                self.leaves[i].num = 0;
                self.leaves[i].depth -= 1;
                return true;
            }
        }

        false
    }

    fn split(&mut self) -> bool {
        for (i, leaf) in self.leaves.iter_mut().enumerate() {
            if leaf.num > 9 {
                leaf.depth += 1;
                let num_is_odd = leaf.num % 2 != 0;
                leaf.num /= 2;
                let left = *leaf;
                self.leaves.insert(i, left);
                if num_is_odd {
                    self.leaves[i+1].num += 1;
                }
                return true;
            }
        }

        false
    }
}
