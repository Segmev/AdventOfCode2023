use std::collections::{BinaryHeap, HashMap};

pub fn start(file_content: &str) {
    let mut heat_map: Vec<Vec<u32>> = vec![];

    for line in file_content.lines() {
        heat_map.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    let mut heat_costs = HashMap::new();

    // negative cost because of order priority.
    let mut queue = BinaryHeap::from_iter([(0, (0, 0, (0, 0)))]);

    let min_steps = 4;
    let max_steps = 10;

    while let Some((cost, (x, y, direction))) = queue.pop() {
        if (x, y) == (heat_map[0].len() as i32 - 1, heat_map.len() as i32 - 1) {
            println!("{}", -cost);
            break;
        }
        if let Some(&c) = heat_costs.get(&(x, y, direction)) {
            if -cost > c {
                continue;
            }
        }

        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            // no back steps or already checked.
            if (dx, dy) == direction || (-dx, -dy) == direction {
                continue;
            }

            let mut heat_cost = 0;
            for dist in 1..=max_steps {
                let (nx, ny) = (x + dx * dist, y + dy * dist);
                if nx as usize >= heat_map[0].len() || ny as usize >= heat_map.len() {
                    continue;
                }

                heat_cost += heat_map[ny as usize][nx as usize];

                if dist < min_steps {
                    continue;
                }

                let next_cost = -cost + heat_cost as i32;
                let key = (nx, ny, (dx, dy));
                if next_cost < *heat_costs.get(&key).unwrap_or(&i32::MAX) {
                    heat_costs.insert(key, next_cost);
                    queue.push((-next_cost, key));
                }
            }
        }
    }
}
