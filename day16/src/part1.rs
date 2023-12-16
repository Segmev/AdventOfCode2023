use std::collections::{HashMap, HashSet};

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

    let mut energized = HashSet::new();
    let mut lights = vec![((0, 0), (1, 0))];
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
    println!("{}", energized.len());
}
