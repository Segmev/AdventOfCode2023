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
                checked_tiles.insert(
                    (start_tile.0, start_tile.1),
                    Tile::new(x as i32, y as i32, c),
                );
            }

            tiles.insert((x as i32, y as i32), Tile::new(x as i32, y as i32, c));
        }
    }

    for i in -1..2 {
        for j in -1..2 {
            if ((i as i32).abs() + (j as i32).abs()) % 2 == 0 {
                continue;
            }
            if let Some(tile) = tiles.get(&(start_tile.0 + i, start_tile.1 + j)) {
                tiles_to_check.push((tile.x, tile.y));
            }
        }
    }

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

    println!("{}", checked_tiles.len() / 2);
}
