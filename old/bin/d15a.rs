use rudac::heap::FibonacciHeap;

use std::io::stdin;

use anyhow::Result;

use crate::d15::*;

pub fn neighbors(xmax: usize, ymax: usize, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut res = Vec::new();

    if x < xmax {
        res.push((y, x + 1))
    }
    if y > 0 {
        res.push((y - 1, x))
    }
    if x > 0 {
        res.push((y, x - 1))
    }
    if y < ymax {
        res.push((y + 1, x))
    }

    res
}

pub fn a_star(points: &mut Vec<Vec<Point>>, goal: (usize, usize)) -> u16 {
    let xmax = points[0].len() - 1;
    let ymax = points.len() - 1;

    let mut front = FibonacciHeap::init_min();

    points[0][0].lowest_cost = 0;
    front.push((0, 0, 0));

    while let Some((_, center_x, center_y)) = front.pop() {
        trace!("looking at point {:?}", (center_x, center_y));
        let center = &mut points[center_y][center_x];

        if center.visited {
            trace!("already visited");
            continue;
        }

        if center.pos == goal {
            return center.lowest_cost;
        }

        center.visited = true;

        let center_lowest_cost = center.lowest_cost;
        for (neighbor_x, neighbor_y) in neighbors(xmax, ymax, center.pos) {
            trace!(" looking at neighbor {:?}", (neighbor_x, neighbor_y));

            let neighbor = &mut points[neighbor_y][neighbor_x];

            if neighbor.visited {
                trace!("already visited, continuing");
                continue;
            }

            let new_cost = center_lowest_cost.saturating_add(neighbor.cost as u16);
            trace!("  the cost of going through center to this vertex would be: {:?}", new_cost);
            if new_cost < neighbor.lowest_cost {
                trace!("  it would be faster to go through center to this vertex");

                neighbor.lowest_cost = new_cost;

                let heuristic = (xmax - neighbor.pos.0 + ymax - neighbor.pos.1) as u16;
                neighbor.heuristic_cost = new_cost + heuristic;

                front.push((new_cost + heuristic, neighbor_x, neighbor_y));
            }
        }

    }

    panic!("destination unreachable")
}

fn main() -> Result<()> {
    let point1 = Point {
        heuristic_cost: 0,
        lowest_cost: 0,
        pos: (0, 0),
        cost: 6,
        visited: false,
    };
    let point2 = Point {
        heuristic_cost: 6,
        lowest_cost: 0,
        pos: (0, 0),
        cost: 0,
        visited: false,
    };

    assert!(point1 < point2);

    let mut lines = stdin().lines();

    let mut points = parse_input(&mut lines)?;

    let xmax = points[0].len() - 1;
    let ymax = points.len() - 1;

    println!("{:?}", a_star(&mut points, (xmax, ymax)));

    Ok(())
}
