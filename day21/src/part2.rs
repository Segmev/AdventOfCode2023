use std::collections::HashSet;

const STEPS: i128 = 26501365;
pub fn start(file_content: &str) {
    let mut rocks = HashSet::new();
    let mut stepped_plots = HashSet::new();
    let mut count_a = 0;
    let mut count_b = 0;
    let mut middle = 0;
    for (y, line) in file_content.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                rocks.insert((x as i128, y as i128));
            } else if c == 'S' {
                stepped_plots.insert((x as i128, y as i128));
                middle = x;
            }
        }
    }
    let max_x = file_content.lines().next().unwrap().chars().count() as i128;
    let max_y = file_content.lines().count() as i128;
    let steps_rem = middle as i128;
    let step_a = steps_rem;
    let step_b = steps_rem + max_x;
    let step_c = steps_rem + (max_x * 2);
    for i in 1..(steps_rem + (max_x * 2)) + 1 {
        let mut next_stepped_plot = HashSet::new();
        for (x, y) in &stepped_plots {
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if !rocks.contains(&(
                    (((x + dx) % max_x) + max_x) % max_x,
                    (max_y + ((y + dy) % max_y)) % max_y,
                )) {
                    next_stepped_plot.insert((x + dx, y + dy));
                }
            }
        }
        stepped_plots = next_stepped_plot;
        if i == step_a {
            count_a = stepped_plots.len() as i128;
        } else if i == step_b {
            count_b = stepped_plots.len() as i128;
        }
    }
    let count_c = stepped_plots.len() as i128;

    let first_member =
        count_a * ((STEPS - step_b) * (STEPS - step_c)) / ((step_a - step_b) * (step_a - step_c));
    let second_member =
        count_b * ((STEPS - step_a) * (STEPS - step_c)) / ((step_b - step_a) * (step_b - step_c));
    let third_member =
        count_c * ((STEPS - step_a) * (STEPS - step_b)) / ((step_c - step_a) * (step_c - step_b));
    println!("{}", first_member + second_member + third_member);
}
