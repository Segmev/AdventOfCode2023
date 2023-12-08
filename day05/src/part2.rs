use std::collections::HashSet;

fn get_range_intersection(a: (usize, usize), b: (usize, usize)) -> (usize, usize) {
    (a.0.max(b.0), a.1.min(b.1))
}

fn get_ranges_diff(
    original_range: (usize, usize),
    intersected_range: (usize, usize),
) -> (Option<(usize, usize)>, Option<(usize, usize)>) {
    let mut res = (Option::None, Option::None);

    if intersected_range.0 > original_range.0 {
        res.0 = Option::Some((original_range.0, intersected_range.0 - 1));
    }
    if intersected_range.1 < original_range.1 {
        res.1 = Option::Some((intersected_range.1, original_range.1));
    }

    res
}

pub fn start(file_content: &str) {
    let mut seeds = vec![];

    let mut lines = file_content.lines();
    lines
        .next()
        .unwrap()
        .split("seeds: ")
        .collect::<Vec<&str>>()[1]
        .split(" ")
        .for_each(|seed| {
            let id = seed.parse::<usize>().unwrap();
            seeds.push(id);
        });

    let mut seeds_pair = HashSet::new();
    for i in 0..seeds.len() / 2 {
        seeds_pair.insert((seeds[i * 2], seeds[i * 2] + seeds[(i * 2) + 1]));
    }

    let mut to_be_removed = HashSet::new();
    let mut next_field_state = seeds_pair.clone();
    for line in lines.filter(|line| !line.is_empty()) {
        if line.contains(":") {
            for v in to_be_removed.iter() {
                next_field_state.remove(&v);
            }

            seeds_pair = next_field_state.clone();
            next_field_state.clear();
            to_be_removed.clear();
        } else {
            let tokens = line.split(" ").collect::<Vec<&str>>();
            if tokens.len() == 3 {
                let dest_start = tokens[0].parse::<usize>().unwrap();
                let source_start = tokens[1].parse::<usize>().unwrap();
                let range_len = tokens[2].parse::<usize>().unwrap();

                let cloned_seeds_pair = seeds_pair.clone();
                for (low_range, high_range) in cloned_seeds_pair.into_iter() {
                    let intersect = get_range_intersection(
                        (low_range, high_range),
                        (source_start, source_start + range_len - 1),
                    );
                    if intersect.0 >= low_range
                        && intersect.1 <= high_range
                        && intersect.0 < intersect.1
                    {
                        let outside_intersect =
                            get_ranges_diff((low_range, high_range), (intersect.0, intersect.1));
                        if let Some(left_outside_intersect) = outside_intersect.0 {
                            next_field_state.insert(left_outside_intersect);
                        }
                        if let Some(right_outside_intersect) = outside_intersect.1 {
                            next_field_state.insert(right_outside_intersect);
                        }
                        to_be_removed.insert((low_range, high_range));
                        next_field_state.insert((
                            dest_start + intersect.0 - source_start,
                            dest_start + intersect.1 - source_start,
                        ));
                    } else {
                        next_field_state.insert((low_range, high_range));
                    }
                }
            }
        }
    }

    let mut res: u64 = u64::MAX;
    for seed in seeds_pair {
        if seed.0 != 0 && res > seed.0 as u64 {
            res = seed.0 as u64;
        }
    }
    println!("{}", res);
}
