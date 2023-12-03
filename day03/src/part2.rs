use std::collections::HashMap;

fn get_meaningful_nb(lines: &[&str], i: usize, j: usize) -> Option<(String, usize)> {
    let mut end_of_nb_idx = 0;
    for (idx, c) in lines[i].chars().skip(j).enumerate() {
        if !c.is_numeric() {
            end_of_nb_idx = idx;
            break;
        }
    }
    if end_of_nb_idx == 0 {
        end_of_nb_idx = lines[i].len() - 1;
    }

    Some((
        lines[i]
            .chars()
            .into_iter()
            .skip(j)
            .take(end_of_nb_idx)
            .collect::<String>(),
        end_of_nb_idx - 1,
    ))
}

fn line_stars_around(line: &str) -> Vec<usize> {
    let mut stars: Vec<usize> = Vec::new();
    for (i, c) in line.chars().enumerate() {
        if c == '*' {
            stars.push(i);
        }
    }
    return stars;
}

fn set_stars_around(
    lines: &[&str],
    stars_info: &mut HashMap<(usize, usize), Vec<i32>>,
    nb: i32,
    i: usize,
    start_j: usize,
    end_j: usize,
) {
    let first_j = start_j.saturating_sub(1);
    let last_j = (end_j + 1).min(lines[i].len() - 1);
    let mut idx_diff = 1;
    let mut nb_lines = 3;
    if i == 0 || i == lines.len() - 1 {
        nb_lines = 2;
        if i == 0 {
            idx_diff = 0;
        }
    }
    for (idx, line) in lines
        .iter()
        .skip(i.saturating_sub(1))
        .take(nb_lines)
        .enumerate()
    {
        if let Some(c) = line.get(first_j..last_j + 1) {
            let stars = line_stars_around(c);
            for star_j in stars {
                stars_info
                    .entry((i + idx - idx_diff, star_j + first_j))
                    .or_insert(Vec::new())
                    .push(nb);
            }
        }
    }
}

pub fn start(file_content: &str) {
    let lines: Vec<&str> = file_content.lines().collect();
    let mut stars_info: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        let line_chars = line.chars();
        let mut end_of_nb_idx = 0;
        for (j, c) in line_chars.enumerate() {
            if j < end_of_nb_idx {
                continue;
            }
            if c.is_numeric() {
                let res = get_meaningful_nb(&lines, i, j);
                if res.is_some() {
                    let (nb, end_j) = res.unwrap();
                    set_stars_around(
                        &lines,
                        &mut stars_info,
                        nb.parse::<i32>().unwrap(),
                        i,
                        j,
                        j + end_j,
                    );
                    end_of_nb_idx = j + end_j + 1;
                }
            }
        }
    }
    let mut acc = 0;
    for stars in stars_info.values() {
        if stars.len() == 2 {
            acc += stars[0] * stars[1];
        }
    }
    println!("{acc}");
}
