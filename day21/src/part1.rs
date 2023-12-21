use std::collections::HashSet;

const STEPS: i32 = 64;

pub fn start(file_content: &str) {
    let mut rocks = HashSet::new();
    let mut stepped_plots = HashSet::new();

    for (y, line) in file_content.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                rocks.insert((x as i32, y as i32));
            } else if c == 'S' {
                stepped_plots.insert((x as i32, y as i32));
            }
        }
    }
    let max_x = file_content.lines().next().unwrap().chars().count() as i32;
    let max_y = file_content.lines().count() as i32;

    for _ in 0..STEPS {
        let mut next_stepped_plot = HashSet::new();

        for (x, y) in &stepped_plots {
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if !rocks.contains(&(x + dx, y + dy))
                    && x + dx < max_x
                    && y + dy < max_y
                    && x + dx >= 0
                    && y + dy >= 0
                {
                    next_stepped_plot.insert((x + dx, y + dy));
                }
            }
        }

        stepped_plots = next_stepped_plot;
    }

    println!("{}", stepped_plots.len());
}
