use anyhow::{Context, Result};

use d22::{Cuboid, Polarity};
use log::trace;

fn main() -> Result<()> {
    simple_logger::init_with_env().context("couldn't initialize logger")?;
    let input = d22::get_input();

    let mut reactor: Vec<Cuboid> = Vec::new();
    for item in input {
        let (polarity, cuboid) = item.context("couldn't get input")?;

        // uncomment this to solve part 1
        // if !cuboid.is_initial() {
        //     continue;
        // }

        if polarity == Polarity::On {
            trace!("turning on {cuboid}");

            let collisions: Vec<Cuboid> = reactor
                .iter()
                .filter(|c| c.intersects(&cuboid))
                .cloned()
                .collect();

            let mut partitions = vec![cuboid];
            while !partitions.is_empty() {
                while let Some(partition) = partitions.pop() {
                    if let Some(collision) = collisions.iter().find(|c| c.intersects(&partition)) {
                        partitions.extend(partition.partition_by((*collision).clone()));
                    } else {
                        trace!("  turning on {partition}");
                        reactor.push(partition);
                    }
                }
            }
        } else {
            trace!("turning off {cuboid}");

            // the indexes of cuboids in `reactor` which intersect with `cuboid`. this must be collected before it is iterated through so that `reactor` can be simultaneously modified.
            let collisions: Vec<usize> = reactor
                .iter()
                .enumerate()
                .rev()
                .filter(|(_, c)| c.intersects(&cuboid))
                .map(|(i, _)| i)
                .collect();
            for i in collisions {
                // since `collisions` is ordered from high to low (notice the `rev` above), if the last element of `reactor` collides with `cuboid`, it will be the first one encountered in this loop, therefore removing the concern that `swap_remove` could change its position before it is reached.
                let collision = reactor.swap_remove(i);
                for partition in collision.partition_by(cuboid.clone()) {
                    trace!("    replacing with {partition}");
                    reactor.push(partition);
                }
            }
        }
    }

    println!("{}", reactor.iter().map(Cuboid::volume).sum::<u64>());

    Ok(())
}
