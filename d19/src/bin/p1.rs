use std::io::stdin;

use anyhow::{Result, Context};

use log::trace;

use d19::{Point, Scanner};

fn main() -> Result<()> {
    simple_logger::init_with_env().context("couldn't initialize logger")?;

    // parse
    let mut scanners = Vec::new();
    let mut beacons = Vec::new();
    let mut taxicabs = Vec::new();
    let lines = stdin().lines();
    for line in lines {
        let line = line?;
        if let Ok(point) = line.parse::<Point>() {
            let pt_taxicab = point.abs_taxicab();
            let i = taxicabs.partition_point(|p| p > &pt_taxicab);
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
    loop {
        let mut any = false;
        for si in 0..scanners.len() {
            if !base.contains(&si) {
                let mut matched = false;
                for bi in &base {
                    if let Some((rot, dsp)) = scanners[*bi].diff(&scanners[si]) {
                        trace!("success! scanner {si} has displacement {dsp} to base");
                        scanners[si].rotate(&rot);
                        scanners[si] += dsp;
                        matched = true;
                        break;
                    }
                }
                if matched {
                    base.push(si);
                    any = true;
                }
            }
        }

        if !any {
            break;
        }
    }

    let mut final_scanner = scanners.remove(0);
    for bi in base {
        if bi != 0 {
            final_scanner.extend(&mut scanners[bi - 1]);
        }
    }

    println!("{}", final_scanner.beacons.len());

    Ok(())
}

#[cfg(test)]
mod tests {
    use d19::*;

    // #[test]
    // fn orientations() {
    //     let scanner = Scanner::new(vec![Point::new(1, 2, 3)]);
    //     let mut orientations = Orientations::new(scanner);

    //     let mut points = Vec::new();
    //     points.push(orientations.scanner.beacons[0]);
    //     while orientations.next().is_some() {
    //         points.push(orientations.scanner.beacons[0]);
    //     }
    //     // points.sort();
    //     for point in points {
    //         println!("{point}");
    //     }
    // }

    #[test]
    fn rotation() {
        let mut pt = Point::new(1, 2, 3);
        pt.rotate(&Rotation(
            Swap::Once(Dim::X, Dim::Z),
            Reflect::along(Dim::Y),
        ));
        assert_eq!(pt, Point::new(3, -2, 1));
    }

    // #[test]
    // fn swap_diff() {
    //     let old_pt = Point::new(1, 2, 3);
    //     let mut new_pt = old_pt;
    //     let rot = Rotation(Swap::Once(Dim::X, Dim::Y), Reflect::along(Dim::X));
    //     new_pt.rotate(&rot);
    //     assert_eq!(new_pt, Point::new(-2, 1, 3));
    //     assert_eq!(old_pt.swap_diff(new_pt), rot.0);
    // }

    #[test]
    fn rot_diff() {
        let old_pt = Point::new(188, -1408, -168);
        let mut new_pt = old_pt;
        let rot = Rotation(
            Swap::Twice(Dim::Z, Dim::Y, Dim::X),
            Reflect {
                x: true,
                y: false,
                z: true,
            },
        );
        new_pt.rotate(&rot);
        assert_eq!(new_pt, Point::new(168, 188, 1408));
        assert_eq!(old_pt.rot_diff(new_pt), rot);
    }

    #[test]
    fn unswap() {
        let old_pt = Point::new(188, -1408, -168);
        let mut new_pt = old_pt;
        let swap = Swap::Twice(Dim::Z, Dim::Y, Dim::X);
        new_pt.swap(&swap);
        new_pt.unswap(&swap);
        assert_eq!(old_pt, new_pt);
    }

    // #[test]
    // fn extend() {
    //     let mut scanner0 = Scanner {
    //         beacons: vec![
    //             Point::new(404, -588, -901),
    //             Point::new(528, -643, 409),
    //             Point::new(-838, 591, 734),
    //             Point::new(390, -675, -793),
    //             Point::new(-537, -823, -458),
    //             Point::new(-485, -357, 347),
    //             Point::new(-345, -311, 381),
    //             Point::new(-661, -816, -575),
    //             Point::new(-876, 649, 763),
    //             Point::new(-618, -824, -621),
    //             Point::new(553, 345, -567),
    //             Point::new(474, 580, 667),
    //             Point::new(-447, -329, 318),
    //             Point::new(-584, 868, -557),
    //             Point::new(544, -627, -890),
    //             Point::new(564, 392, -477),
    //             Point::new(455, 729, 728),
    //             Point::new(-892, 524, 684),
    //             Point::new(-689, 845, -530),
    //             Point::new(423, -701, 434),
    //             Point::new(7, -33, -71),
    //             Point::new(630, 319, -379),
    //             Point::new(443, 580, 662),
    //             Point::new(-789, 900, -551),
    //             Point::new(459, -707, 401),
    //         ],
    //     };
    //     let mut scanner1 = Scanner {
    //         beacons: vec![
    //             Point::new(686, 422, 578),
    //             Point::new(605, 423, 415),
    //             Point::new(515, 917, -361),
    //             Point::new(-336, 658, 858),
    //             Point::new(95, 138, 22),
    //             Point::new(-476, 619, 847),
    //             Point::new(-340, -569, -846),
    //             Point::new(567, -361, 727),
    //             Point::new(-460, 603, -452),
    //             Point::new(669, -402, 600),
    //             Point::new(729, 430, 532),
    //             Point::new(-500, -761, 534),
    //             Point::new(-322, 571, 750),
    //             Point::new(-466, -666, -811),
    //             Point::new(-429, -592, 574),
    //             Point::new(-355, 545, -477),
    //             Point::new(703, -491, -529),
    //             Point::new(-328, -685, 520),
    //             Point::new(413, 935, -424),
    //             Point::new(-391, 539, -444),
    //             Point::new(586, -435, 557),
    //             Point::new(-364, -763, -893),
    //             Point::new(807, -499, -711),
    //             Point::new(755, -354, -619),
    //             Point::new(553, 889, -390),
    //         ],
    //     };
    //     let diff = scanner0.diff(&scanner1).unwrap();
    //     scanner0.extend(&mut scanner1, diff);
    //     println!("{}", scanner0.beacons.len());
    // }
}
