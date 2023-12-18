use std::collections::{HashMap, HashSet};

fn get_corner(corner_type: &str) -> char {
    match corner_type {
        "UR" | "LD" => '┌',
        "RD" | "UL" => '┐',
        "DR" | "LU" => '└',
        "RU" | "DL" => '┘',
        _ => '#',
    }
}

pub fn start(file_content: &str) {
    let mut borders = HashMap::new();

    let mut dig_pos = (0, 0);

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for line in file_content.lines() {
        let tokens = line.split_whitespace().collect::<Vec<&str>>();
        let steps = tokens[1].parse::<i32>().unwrap();
        let dir = if ["U", "L"].contains(&tokens[0]) {
            -1
        } else {
            1
        };
        if ["R", "L"].contains(&tokens[0]) {
            borders
                .entry(dig_pos)
                .or_insert("".to_string())
                .push(tokens[0].chars().next().unwrap());
            for shift in 1..steps {
                borders.insert((dig_pos.0 + dir * shift, dig_pos.1), "#".to_string());
            }
            dig_pos = (dig_pos.0 + dir * steps, dig_pos.1);
            borders
                .entry(dig_pos)
                .or_insert("".to_string())
                .push(tokens[0].chars().next().unwrap());

            min_x = if min_x > dig_pos.0 { dig_pos.0 } else { min_x };
            max_x = if max_x < dig_pos.0 { dig_pos.0 } else { max_x };
        } else {
            borders
                .entry(dig_pos)
                .or_insert("".to_string())
                .push(tokens[0].chars().next().unwrap());
            for shift in 1..steps {
                borders.insert((dig_pos.0, dig_pos.1 + (dir * shift)), "#".to_string());
            }
            dig_pos = (dig_pos.0, dig_pos.1 + (dir * steps));
            borders
                .entry(dig_pos)
                .or_insert("".to_string())
                .push(tokens[0].chars().next().unwrap());
            min_y = if min_y > dig_pos.1 { dig_pos.1 } else { min_y };
            max_y = if max_y < dig_pos.1 { dig_pos.1 } else { max_y };
        }
    }
    let temp_borders = borders.clone();

    for temp_borders_entry in temp_borders.clone() {
        let (x, y) = temp_borders_entry.0;
        let entry = temp_borders_entry.1.as_str();
        if (x, y) == (0, 0) {
            borders.insert(
                (x, y),
                get_corner(entry.chars().rev().collect::<String>().as_str()).to_string(),
            );
        } else {
            borders.insert((x, y), get_corner(entry).to_string());
        }
    }

    let mut area = 0;
    let mut y = min_y - 1;
    let mut inside_map = HashSet::new();
    while min_y - 1 <= y && y <= max_y + 1 {
        let mut x = min_x - 1;
        let mut inside = false;
        while min_x - 1 <= x && x <= max_x + 1 {
            if borders.contains_key(&(x, y)) {
                let corner_entry = borders.get(&(x, y)).unwrap();

                while borders.contains_key(&(x, y)) {
                    area += 1;
                    inside_map.insert((x, y));
                    x += 1;
                }
                let corner_exit = borders.get(&(x - 1, y)).unwrap();
                if corner_entry == "#"
                    || ((corner_entry == "┌" && corner_exit == "┘")
                        || (corner_entry == "└" && corner_exit == "┐"))
                {
                    inside = !inside;
                }
            } else {
                if inside {
                    area += 1;
                    inside_map.insert((x, y));
                }

                x += 1;
            }
        }
        y += 1;
    }
    println!("{}", area);
}
