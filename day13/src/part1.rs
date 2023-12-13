fn get_horizontale_reflection_value(mirror: &Vec<String>) -> usize {
    for i in 0..mirror.len() - 1 {
        if mirror[i] == mirror[i + 1] {
            let mut ni = i + 1;
            let mut ri: isize = i as isize;

            while ri >= 0 && ni < mirror.len() {
                if mirror[ri as usize] == mirror[ni] {
                    ri -= 1;
                    ni += 1;
                } else {
                    break;
                }
            }
            if ri < 0 || ni >= mirror.len() {
                return (1 + i) * 100;
            }
        }
    }
    0
}

fn get_verticale_reflection_value(mirror: &Vec<String>) -> usize {
    for j in 0..(mirror[0].len() - 1) {
        let mut reflecting = true;
        let mut nj = j + 1;
        let mut rj: isize = j as isize;

        while rj >= 0 && nj < mirror[0].len() {
            for i in 0..mirror.len() {
                if mirror[i].chars().nth(rj as usize).unwrap() != mirror[i].chars().nth(nj).unwrap()
                {
                    reflecting = false;
                    break;
                }
            }
            if reflecting == true {
                nj += 1;
                rj -= 1;
            } else {
                break;
            }
        }
        if rj < 0 || nj >= mirror[0].len() {
            return 1 + j;
        }
    }

    0
}

fn get_mirror_value(mirror: &Vec<String>) -> usize {
    let mut res = 0;
    res += get_verticale_reflection_value(mirror);
    res += get_horizontale_reflection_value(mirror);

    res
}

pub fn start(file_content: &str) {
    let mut mirrors: Vec<Vec<String>> = vec![];

    let mut current_mirror: Vec<String> = vec![];
    for line in file_content.lines() {
        if line.is_empty() {
            mirrors.push(current_mirror);
            current_mirror = vec![];
        } else {
            current_mirror.push(line.to_string());
        }
    }
    mirrors.push(current_mirror);

    let mut acc = 0;
    for mirror in mirrors {
        acc += get_mirror_value(&mirror);
    }
    println!("{}", acc);
}
