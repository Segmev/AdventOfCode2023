use std::collections::{HashMap, HashSet};

fn get_energized_count(
    mirrors: &HashMap<(i32, i32), char>,
    max_x: usize,
    max_y: usize,
    first_light: ((i32, i32), (i32, i32)),
) -> usize {
    let mut lights = vec![first_light];
    let mut energized = HashSet::new();
    let mut checked = HashSet::new();
    while let Some(light) = lights.pop() {
        if checked.contains(&light)
            || light.0 .0 < 0
            || light.0 .0 >= max_x as i32
            || light.0 .1 < 0
            || light.0 .1 >= max_y as i32
        {
            continue;
        }
        checked.insert(light);
        energized.insert(light.0);
        if mirrors.contains_key(&light.0) {
            match mirrors.get(&light.0) {
                Some('|') => {
                    if light.1 .0 != 0 {
                        lights.push(((light.0 .0, light.0 .1 + 1), (0, 1)));
                        lights.push(((light.0 .0, light.0 .1 - 1), (0, -1)));
                    } else {
                        lights.push(((light.0 .0, light.0 .1 + light.1 .1), light.1))
                    }
                }
                Some('-') => {
                    if light.1 .1 != 0 {
                        lights.push(((light.0 .0 + 1, light.0 .1), (1, 0)));
                        lights.push(((light.0 .0 - 1, light.0 .1), (-1, 0)));
                    } else {
                        lights.push(((light.0 .0 + light.1 .0, light.0 .1), light.1));
                    }
                }
                Some('/') => {
                    lights.push((
                        (
                            light.0 .0 + (light.1 .1 * -1),
                            light.0 .1 + (light.1 .0 * -1),
                        ),
                        (light.1 .1 * -1, light.1 .0 * -1),
                    ));
                }
                Some('\\') => lights.push((
                    (light.0 .0 + (light.1 .1), light.0 .1 + (light.1 .0)),
                    (light.1 .1, light.1 .0),
                )),
                Some(_) => {}
                None => {}
            }
        } else {
            lights.push(((light.0 .0 + light.1 .0, light.0 .1 + light.1 .1), light.1));
        }
    }
    energized.len()
}

pub fn start(file_content: &str) {
    let mut mirrors: HashMap<(i32, i32), char> = HashMap::new();

    for (y, line) in file_content.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                mirrors.insert((x as i32, y as i32), c);
            }
        }
    }
    let max_y = file_content.lines().count();
    let max_x = file_content.lines().next().unwrap().chars().count();

    let mut max_energized = 0;

    for y in 0..max_y {
        let res = get_energized_count(&mirrors, max_x, max_y, ((0, y as i32), (1, 0)));
        if res > max_energized {
            max_energized = res;
        }
        let res = get_energized_count(
            &mirrors,
            max_x,
            max_y,
            ((max_x as i32 - 1, y as i32), (-1, 0)),
        );
        if res > max_energized {
            max_energized = res;
        }
    }

    for x in 0..max_x {
        let res = get_energized_count(&mirrors, max_x, max_y, ((x as i32, 0), (0, 1)));
        if res > max_energized {
            max_energized = res;
        }
        let res = get_energized_count(
            &mirrors,
            max_x,
            max_y,
            ((x as i32, max_y as i32 - 1), (1, 0)),
        );
        if res > max_energized {
            max_energized = res;
        }
    }

    println!("{}", max_energized);
}
