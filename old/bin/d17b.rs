use anyhow::Error;

const XMIN: u32 = 236;
const XMAX: u32 = 262;
const YMIN: i32 = -78;
const YMAX: i32 = -58;

const XMINF: f32 = 236.0;

struct Probe {
    vx: u32,
    vy: i32,
    px: u32,
    py: i32,
}

/// An iterator yielding all snapshots of the probe before it falls below the target.
impl Probe {
    fn new(vx: u32, vy: i32) -> Self {
        Self {
            vx,
            vy,
            px: 0,
            py: 0,
        }
    }
}

impl Iterator for Probe {
    type Item = (u32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        self.px += self.vx;
        self.py += self.vy;

        self.vx = self.vx.saturating_sub(1);
        self.vy -= 1;

        if self.py >= YMIN && self.px <= XMAX {
            Some((self.px, self.py))
        } else {
            None
        }
    }
}

fn possibles(vx: u32, vy: i32) -> Vec<(usize, (u32, i32))> {
    let probe = Probe::new(vx, vy);
    probe.enumerate().filter(|p| p.1.1 <= YMAX).collect()
}


fn vxmin() -> u32 {
    (0.5 * (-1.0 + (8.0 * XMINF + 1.0).sqrt())).floor() as u32
}

fn in_x_target((_i, (x, _y)): &(usize, (u32, i32))) -> bool {
    x >= &XMIN && x <= &XMAX
}

fn increment(vx: u32, (i, (x, _y)): &mut (usize, (u32, i32))) {
    if i >= &mut (vx as usize) {
        *x += vx + 1;
    } else {
        *x += *i as u32 + 1;
    }
}

fn go() -> Result<()> {
    let vymax = -YMIN-1;
    let vxmin = vxmin();

    let mut ans = 0;
    for vy in YMIN..=vymax {
        let mut vx = vxmin;
        let mut possibles = possibles(vx, vy);
        if !possibles.is_empty() {
            while possibles[0].1.0 <= XMAX {
                if possibles.iter().any(in_x_target) {
                    ans += 1;
                }
                possibles.iter_mut().for_each(|p| increment(vx, p));
                vx += 1;
            }
        }
    }

    println!("answer: {ans}");

    Ok(())
}
