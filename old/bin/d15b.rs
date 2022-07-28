use std::io::stdout;

use crate::d15::*;

use anyhow::Result;

pub fn extendify(source: Vec<Vec<Point>>) -> Vec<Vec<Point>> {
    let xdim = source[0].len();
    let ydim = source.len();

    let mut rows = Vec::new();
    for y in 0..ydim*5 {
        let mut row = Vec::new();
        for x in 0..xdim*5 {
            let offset = (x / xdim + y / ydim) as u8;
            let mut cost = source[y % ydim][x % xdim].cost;
            cost = 1 + ((cost + offset - 1) % 9);

            row.push(Point::new((x, y), cost));
        }

        rows.push(row);
    }

    rows
}

fn main() -> Result<()> {
    let mut lines = stdout().lines();

    let mut points = extendify(parse_input(&mut lines)?);

    let xmax = points[0].len() - 1;
    let ymax = points.len() - 1;

    println!("{:?}", part1::a_star(&mut points, (xmax, ymax)));

    Ok(())
}
