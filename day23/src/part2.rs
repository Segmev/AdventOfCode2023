use std::collections::{HashMap, HashSet};

pub fn start(file_content: &str) {
    let mut rocks: HashSet<(isize, isize)> = HashSet::new();
    let mut slops: HashMap<(isize, isize), (isize, isize)> = HashMap::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, line) in file_content.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                rocks.insert((x as isize, y as isize));
            } else if ['v', '>', '<', '^'].contains(&c) {
                match c {
                    'v' => {
                        slops.insert((x as isize, y as isize), (0, 1));
                    }
                    '>' => {
                        slops.insert((x as isize, y as isize), (1, 0));
                    }
                    '<' => {
                        slops.insert((x as isize, y as isize), (-1, 0));
                    }
                    '^' => {
                        slops.insert((x as isize, y as isize), (0, -1));
                    }
                    _ => {}
                }
            } else if c == '.' && y == 0 {
                start = (x as isize, y as isize);
                rocks.insert((x as isize, y as isize - 1));
            } else if c == '.' && y == file_content.lines().count() - 1 {
                end = (x as isize, y as isize);
                rocks.insert((x as isize, y as isize + 1));
            }
        }
    }

    let mut max_rope = 0;
    let mut ropes: Vec<Vec<(isize, isize)>> = vec![(vec![start])];

    while let Some(curr_rope) = ropes.pop() {
        let last_pos = curr_rope.last().unwrap();

        if *last_pos == end {
            if max_rope < curr_rope.len() {
                max_rope = curr_rope.len();
                // ndlr: "ahahahah."
                println!("{}", max_rope - 1);
                continue;
            }
        }
        for direction in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if curr_rope.contains(&(last_pos.0 + direction.0, last_pos.1 + direction.1)) {
                continue;
            }
            if rocks.contains(&(last_pos.0 + direction.0, last_pos.1 + direction.1)) {
                continue;
            }
            let mut new_rope = curr_rope.clone();
            new_rope.push((last_pos.0 + direction.0, last_pos.1 + direction.1));
            if !ropes.contains(&new_rope) {
                ropes.push(new_rope);
            }
        }
    }
    println!("{}", max_rope - 1);
}
