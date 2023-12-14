use std::collections::HashMap;

const CYCLE_LEN: i32 = 1_000_000_000;

pub fn start(file_content: &str) {
    let mut rocks = HashMap::new();

    for (y, line) in file_content.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'O' {
                rocks.insert((x as i32, y as i32), true);
            } else if c == '#' {
                rocks.insert((x as i32, y as i32), false);
            }
        }
    }

    let max_x = file_content.lines().nth(0).unwrap().chars().count() as i32;
    let max_y = file_content.lines().count() as i32;

    let mut map_res = rocks.clone();

    let mut previous_maps = vec![];
    let mut first_stop = 0;
    let mut next_stop = 0;
    for i in 0..CYCLE_LEN {
        let res = move_round_rock_cycle(&map_res, max_x, max_y);
        if previous_maps.contains(&res) {
            first_stop = previous_maps.iter().position(|r| r == &res).unwrap() as i32;
            next_stop = i;
            break;
        }
        map_res = res;
        previous_maps.push(map_res.clone());
    }

    let cycle_len = next_stop - first_stop;
    while (next_stop + cycle_len) < CYCLE_LEN {
        next_stop += cycle_len;
    }

    for _ in next_stop..CYCLE_LEN {
        map_res = move_round_rock_cycle(&map_res, max_x, max_y);
    }

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

fn move_round_rock_cycle(
    rocks: &HashMap<(i32, i32), bool>,
    max_x: i32,
    max_y: i32,
) -> HashMap<(i32, i32), bool> {
    let mut rocks_res = move_round_rocks_to_north(rocks, max_x, max_y);
    rocks_res = move_round_rocks_to_west(&rocks_res, max_x, max_y);
    rocks_res = move_round_rocks_to_south(&rocks_res, max_x, max_y);

    move_round_rocks_to_east(&rocks_res, max_x, max_y)
}

fn move_round_rocks_to_north(
    rocks: &HashMap<(i32, i32), bool>,
    max_x: i32,
    max_y: i32,
) -> HashMap<(i32, i32), bool> {
    let mut rocks_res = HashMap::new();
    for rock in rocks.iter() {
        if *rock.1 == false {
            rocks_res.insert(*rock.0, *rock.1);
        }
    }

    for y in 0..max_y {
        for x in 0..max_x {
            if let Some(rock) = rocks.get(&(x, y)) {
                if rock == &false {
                    continue;
                }
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

fn move_round_rocks_to_west(
    rocks: &HashMap<(i32, i32), bool>,
    max_x: i32,
    max_y: i32,
) -> HashMap<(i32, i32), bool> {
    let mut rocks_res = HashMap::new();
    for rock in rocks.iter() {
        if *rock.1 == false {
            rocks_res.insert(*rock.0, *rock.1);
        }
    }

    for y in 0..max_y {
        for x in 0..max_x {
            if let Some(rock) = rocks.get(&(x, y)) {
                if rock == &false {
                    continue;
                }
                let mut nx = x - 1;
                while nx >= 0 {
                    if rocks_res.contains_key(&(nx, y)) {
                        break;
                    }
                    nx -= 1;
                }
                rocks_res.insert((nx + 1, y), true);
            }
        }
    }

    rocks_res
}

fn move_round_rocks_to_south(
    rocks: &HashMap<(i32, i32), bool>,
    max_x: i32,
    max_y: i32,
) -> HashMap<(i32, i32), bool> {
    let mut rocks_res = HashMap::new();
    for rock in rocks.iter() {
        if *rock.1 == false {
            rocks_res.insert(*rock.0, *rock.1);
        }
    }

    for y in (0..max_y + 1).rev() {
        for x in 0..max_x {
            if let Some(rock) = rocks.get(&(x, y)) {
                if rock == &false {
                    continue;
                }
                let mut ny = y + 1;
                while ny < max_y {
                    if rocks_res.contains_key(&(x, ny)) {
                        break;
                    }
                    ny += 1;
                }
                rocks_res.insert((x, ny - 1), true);
            }
        }
    }

    rocks_res
}

fn move_round_rocks_to_east(
    rocks: &HashMap<(i32, i32), bool>,
    max_x: i32,
    max_y: i32,
) -> HashMap<(i32, i32), bool> {
    let mut rocks_res = HashMap::new();
    for rock in rocks.iter() {
        if *rock.1 == false {
            rocks_res.insert(*rock.0, *rock.1);
        }
    }

    for y in 0..max_y {
        for x in (0..max_x + 1).rev() {
            if let Some(rock) = rocks.get(&(x, y)) {
                if rock == &false {
                    continue;
                }
                let mut nx = x + 1;
                while nx < max_x {
                    if rocks_res.contains_key(&(nx, y)) {
                        break;
                    }
                    nx += 1;
                }
                rocks_res.insert((nx - 1, y), true);
                // println!("{} {} (max_x: {}, max_y: {})", nx - 1, y, max_x, max_y);
            }
        }
    }

    rocks_res
}
