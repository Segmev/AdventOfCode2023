use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Tile {
    x: i32,
    y: i32,
    c: char,
    directions: ((i32, i32), (i32, i32)),
}

impl Tile {
    fn new(x: i32, y: i32, c: char) -> Tile {
        Tile {
            x,
            y,
            c,
            directions: get_neighbors_directions(c),
        }
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Tile {}

fn get_neighbors_directions(c: char) -> ((i32, i32), (i32, i32)) {
    match c {
        '|' => ((0, 1), (0, -1)),
        '-' => ((1, 0), (-1, 0)),
        'L' => ((0, -1), (1, 0)),
        'J' => ((0, -1), (-1, 0)),
        '7' => ((0, 1), (-1, 0)),
        'F' => ((0, 1), (1, 0)),
        _ => ((0, 0), (0, 0)),
    }
}

fn get_letter_from_directions(direction_a: (i32, i32), direction_b: (i32, i32)) -> char {
    if direction_a == (0, 1) && direction_b == (0, -1)
        || direction_a == (0, -1) && direction_b == (0, 1)
    {
        '|'
    } else if direction_a == (1, 0) && direction_b == (-1, 0)
        || direction_a == (-1, 0) && direction_b == (1, 0)
    {
        '-'
    } else if direction_a == (0, -1) && direction_b == (1, 0)
        || direction_a == (1, 0) && direction_b == (0, -1)
    {
        'L'
    } else if direction_a == (0, -1) && direction_b == (-1, 0)
        || direction_a == (-1, 0) && direction_b == (0, -1)
    {
        'J'
    } else if direction_a == (0, 1) && direction_b == (-1, 0)
        || direction_a == (-1, 0) && direction_b == (0, 1)
    {
        '7'
    } else if direction_a == (0, 1) && direction_b == (1, 0)
        || direction_a == (1, 0) && direction_b == (0, 1)
    {
        'F'
    } else {
        '.'
    }
}

pub fn start(file_content: &str) {
    let mut tiles: HashMap<(i32, i32), Tile> = HashMap::new();
    let mut tiles_to_check = Vec::new();
    let mut start_tile = (0, 0);
    let mut checked_tiles: HashMap<(i32, i32), Tile> = HashMap::new();

    for (y, line) in file_content.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            if c == 'S' {
                start_tile = (x as i32, y as i32);
            }

            tiles.insert((x as i32, y as i32), Tile::new(x as i32, y as i32, c));
        }
    }

    let mut directions = vec![];
    for i in -1..2 {
        for j in -1..2 {
            if ((i as i32).abs() + (j as i32).abs()) % 2 == 0 {
                continue;
            }
            if let Some(tile) = tiles.get(&(start_tile.0 + i, start_tile.1 + j)) {
                tiles_to_check.push((tile.x, tile.y));
                directions.push((i, j));
            }
        }
    }
    checked_tiles.insert(
        (start_tile.0, start_tile.1),
        Tile::new(
            start_tile.0,
            start_tile.1,
            get_letter_from_directions(directions[0], directions[1]),
        ),
    );

    while !tiles_to_check.is_empty() {
        let (x, y) = tiles_to_check.pop().unwrap();

        let tile = tiles.get(&(x, y)).unwrap();

        checked_tiles.insert((x, y), tile.clone());
        if let Some(n_tile) = tiles.get(&(x + tile.directions.0 .0, y + tile.directions.0 .1)) {
            if !checked_tiles.contains_key(&(n_tile.x, n_tile.y)) {
                tiles_to_check.push((n_tile.x, n_tile.y));
            }
        }
        if let Some(n_tile) = tiles.get(&(x + tile.directions.1 .0, y + tile.directions.1 .1)) {
            if !checked_tiles.contains_key(&(n_tile.x, n_tile.y)) {
                tiles_to_check.push((n_tile.x, n_tile.y));
            }
        }
    }

    let mut inside_acc = 0;
    let mut y = 0;
    while y < file_content.lines().count() {
        let mut crossed_count = 0;
        let mut x = 0;
        while x < file_content.lines().nth(y).unwrap().chars().count() {
            if let Some(t) = checked_tiles.get(&(x as i32, y as i32)) {
                if t.c == 'F' || t.c == 'L' {
                    let mut end_border = '.';
                    while let Some(nt) = checked_tiles.get(&(x as i32, y as i32)) {
                        if nt.c != 'J' && nt.c != '7' {
                            x += 1;
                        } else {
                            end_border = nt.c;
                            break;
                        }
                    }
                    if (t.c == 'F' && end_border == 'J') || (t.c == 'L' && end_border == '7') {
                        crossed_count += 1;
                    }
                } else {
                    crossed_count += 1;
                }
            } else if crossed_count % 2 == 1 {
                inside_acc += 1;
            }
            x += 1;
        }
        y += 1;
    }

    println!("{}", inside_acc);
}
