use std::collections::HashMap;

pub fn start(file_content: &str) {
    let mut round_rocks = HashMap::new();
    let mut square_rocks = HashMap::new();

    for (y, line) in file_content.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'O' {
                round_rocks.insert((x as i32, y as i32), true);
            } else if c == '#' {
                square_rocks.insert((x as i32, y as i32), false);
            }
        }
    }

    let max_x = file_content.lines().nth(0).unwrap().chars().count() as i32;
    let max_y = file_content.lines().count() as i32;

    let map_res = move_round_rocks_to_north(&round_rocks, &square_rocks, max_x + 1, max_y + 1);

    let mut acc = 0;
    for x in 0..max_x + 1 {
        for y in 0..max_y + 1 {
            if let Some(true) = map_res.get(&(x, y)) {
                acc += max_y - y;
            }
        }
    }
    println!("{} ", acc);
}

fn move_round_rocks_to_north(
    round_rocks: &HashMap<(i32, i32), bool>,
    square_rocks: &HashMap<(i32, i32), bool>,
    max_x: i32,
    max_y: i32,
) -> HashMap<(i32, i32), bool> {
    let mut rocks_res = square_rocks.clone();

    for y in 0..max_y {
        for x in 0..max_x {
            if round_rocks.contains_key(&(x, y)) {
                let mut ny = y - 1;
                while ny >= 0 {
                    if rocks_res.contains_key(&(x, ny)) {
                        break;
                    }
                    ny -= 1;
                }
                rocks_res.insert((x, ny + 1), true);
            }
        }
    }

    rocks_res
}
