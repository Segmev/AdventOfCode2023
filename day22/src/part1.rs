use std::{collections::HashSet, ops::RangeInclusive};

#[derive(Debug, Clone)]
struct BrickRange {
    id: usize,
    range_x: RangeInclusive<i64>,
    range_y: RangeInclusive<i64>,
    range_z: RangeInclusive<i64>,
    supports: Vec<usize>,
}

impl BrickRange {
    fn new(info: (usize, &str)) -> Self {
        let (id, description) = info;
        let parts = description.split('~').collect::<Vec<&str>>();
        let starts = parts[0]
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let end = parts[1]
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        BrickRange {
            id,
            range_x: starts[0]..=end[0],
            range_y: starts[1]..=end[1],
            range_z: starts[2]..=end[2],
            supports: vec![],
        }
    }

    fn in_zone(&self, other: &BrickRange) -> bool {
        return (self.range_x.start() <= other.range_x.end()
            && self.range_x.end() >= other.range_x.start())
            && (self.range_y.start() <= other.range_y.end()
                && self.range_y.end() >= other.range_y.start());
    }
}

pub fn start(file_content: &str) {
    let mut snap_bricks = file_content
        .lines()
        .enumerate()
        .map(BrickRange::new)
        .collect::<Vec<BrickRange>>();

    snap_bricks.sort_by(|a, b| a.range_z.start().cmp(&b.range_z.start()));
    let mut bricks: Vec<BrickRange> = vec![];
    for brick in snap_bricks.iter() {
        let mut colliding_bricks = bricks
            .iter()
            .filter(|b| b.in_zone(brick))
            .collect::<Vec<_>>();

        let mut final_brick: BrickRange = brick.clone();
        if colliding_bricks.len() == 0 {
            final_brick.range_z = 1..=final_brick.range_z.end() - final_brick.range_z.start() + 1;
        } else {
            colliding_bricks.sort_by(|a, b| b.range_z.end().cmp(&a.range_z.end()));
            final_brick.range_z = colliding_bricks[0].range_z.end() + 1
                ..=colliding_bricks[0].range_z.end() + final_brick.range_z.end()
                    - final_brick.range_z.start()
                    + 1;

            colliding_bricks.iter().for_each(|b| {
                if b.range_z.contains(&(*final_brick.range_z.start() - 1)) {
                    final_brick.supports.push(b.id);
                }
            })
        }

        bricks.push(final_brick);
    }
    let mut unsafe_bricks = HashSet::new();
    for b in bricks.iter() {
        if b.supports.len() == 1 {
            unsafe_bricks.insert(b.supports[0]);
        }
    }
    println!("{}", bricks.len() - unsafe_bricks.len());
}
