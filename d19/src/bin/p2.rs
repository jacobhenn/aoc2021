use std::io;

use anyhow::{Context, Result};

use log::trace;

use d19::{Point, Scanner};

fn main() -> Result<()> {
    simple_logger::init_with_env().context("couldn't initialize logger")?;
    let lines = io::stdin().lines();

    // parse
    let mut scanners = Vec::new();
    let mut beacons = Vec::new();
    let mut taxicabs = Vec::new();
    for line in lines {
        let line = line?;
        if let Ok(point) = line.parse::<Point>() {
            let pt_taxicab = point.abs_taxicab();
            let i = taxicabs.partition_point(|p| p < &pt_taxicab);
            taxicabs.insert(i, pt_taxicab);
            beacons.insert(i, point);
        } else if line.is_empty() {
            scanners.push(Scanner::new(beacons));
            beacons = Vec::new();
            taxicabs.clear();
        }
    }

    if !beacons.is_empty() {
        scanners.push(Scanner::new(beacons));
    }

    // actual algorithm
    let mut base = vec![0];
    let mut base_poss = vec![Point::default()];
    while {
        let mut any = false;
        for si in 0..scanners.len() {
            if !base.contains(&si) {
                let mut matched = None;
                for bi in &base {
                    trace!("checking base scanner {bi} against scanner {si}");
                    if let Some((rot, dsp)) = scanners[*bi].diff(&scanners[si]) {
                        trace!("success! scanner {si} has displacement {dsp} to base");
                        scanners[si].rotate(&rot);
                        scanners[si] += dsp;
                        matched = Some(dsp);
                        break;
                    }
                }
                if let Some(dsp) = matched {
                    base.push(si);
                    base_poss.push(Point::default() + dsp);
                    any = true;
                }
            }
        }
        any
    } {}

    let mut max = 0;
    for i in 0..base_poss.len() {
        for j in 0..base_poss.len() {
            if i != j {
                let dist_taxicab = base_poss[i].dist_taxicab(&base_poss[j]);
                if dist_taxicab > max {
                    max = dist_taxicab;
                }
            }
        }
    }

    println!("{max}");

    Ok(())
}
