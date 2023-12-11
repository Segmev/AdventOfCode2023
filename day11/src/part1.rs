use std::collections::HashMap;
use std::collections::HashSet;

fn distance_between(a: (i32, i32), b: (i32, i32), x_expension: i32, y_expension: i32) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs() + x_expension + y_expension
}

fn get_x_expension_between(mut xa: i32, mut xb: i32, empty_cols: &HashSet<i32>) -> i32 {
    let mut x_expension = 0;

    if xa > xb {
        let t = xb;
        xb = xa;
        xa = t;
    }

    for empty_col in empty_cols.iter() {
        if xa < *empty_col && *empty_col < xb {
            x_expension += 1;
        }
    }

    x_expension
}

fn get_y_expension_between(mut ya: i32, mut yb: i32, empty_lines: &HashSet<i32>) -> i32 {
    let mut y_expension = 0;

    if ya > yb {
        let t = yb;
        yb = ya;
        ya = t;
    }

    for empty_line in empty_lines.iter() {
        if ya < *empty_line && *empty_line < yb {
            y_expension += 1;
        }
    }

    y_expension
}

pub fn start(file_content: &str) {
    let mut star_map: HashSet<(i32, i32)> = HashSet::new();
    let mut unempty_cols: HashSet<i32> = HashSet::new();
    let mut unempty_lines: HashSet<i32> = HashSet::new();

    for (y, line) in file_content.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                star_map.insert((x as i32, y as i32));
                unempty_cols.insert(x as i32);
                unempty_lines.insert(y as i32);
            }
        }
    }

    let mut empty_cols = HashSet::new();
    for (x, _) in file_content.lines().nth(0).unwrap().chars().enumerate() {
        if !unempty_cols.contains(&(x as i32)) {
            empty_cols.insert(x as i32);
        }
    }

    let mut empty_lines = HashSet::new();
    for (y, _) in file_content.lines().enumerate() {
        if !unempty_lines.contains(&(y as i32)) {
            empty_lines.insert(y as i32);
        }
    }

    let mut stars_pairs_distances: HashMap<((i32, i32), (i32, i32)), i32> = HashMap::new();

    for star_a in star_map.iter() {
        for star_b in star_map.iter() {
            if star_a == star_b
                || stars_pairs_distances.contains_key(&((*star_a, *star_b)))
                || stars_pairs_distances.contains_key(&((*star_b, *star_a)))
            {
                continue;
            }

            let x_expension = get_x_expension_between(star_a.0, star_b.0, &empty_cols);
            let y_expension = get_y_expension_between(star_a.1, star_b.1, &empty_lines);
            let distance = distance_between(*star_a, *star_b, x_expension, y_expension);

            stars_pairs_distances.insert((*star_a, *star_b), distance);
        }
    }

    println!("{:?}", stars_pairs_distances.values().sum::<i32>());
}
