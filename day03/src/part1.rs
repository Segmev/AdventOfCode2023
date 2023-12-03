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

fn is_c_symbol(c: char) -> bool {
    !(c.is_numeric() || c == '.')
}

fn line_has_symbol(line: &str) -> bool {
    for c in line.chars() {
        if is_c_symbol(c) {
            return true;
        }
    }
    false
}

fn has_symbol_around(lines: &[&str], i: usize, start_j: usize, end_j: usize) -> bool {
    let first_j = start_j.saturating_sub(1);
    let last_j = (end_j + 1).min(lines[i].len() - 1);

    let mut nb_lines = 3;
    if i == 0 || i == lines.len() - 1 {
        nb_lines = 2;
    }
    for line in lines.iter().skip(i.saturating_sub(1)).take(nb_lines) {
        if let Some(c) = line.get(first_j..last_j + 1) {
            if line_has_symbol(c) {
                return true;
            }
        }
    }
    false
}

pub fn start(file_content: &str) {
    let lines: Vec<&str> = file_content.lines().collect();

    let mut acc = 0;
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
                    if has_symbol_around(&lines, i, j, j + end_j) {
                        if let Ok(parse_nb) = nb.parse::<i128>() {
                            acc += parse_nb;
                        }
                    }
                    end_of_nb_idx = j + end_j + 1;
                }
            }
        }
    }
    println!("{}", acc);
}
