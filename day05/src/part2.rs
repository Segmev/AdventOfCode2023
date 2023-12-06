use std::collections::{HashMap, HashSet};

/*
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
*/

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
        res.1 = Option::Some((intersected_range.1 + 1, original_range.1));
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
    println!("{:?}", seeds_pair);

    let mut next_field_state = seeds_pair.clone();
    for line in lines.filter(|line| !line.is_empty()) {
        if line.contains(":") {
            println!("Seeds state: {:?}", seeds_pair);
            println!("Next field state: {:?}", next_field_state);
            seeds_pair = next_field_state.clone();
            next_field_state.clear();
        } else {
            let tokens = line.split(" ").collect::<Vec<&str>>();
            if tokens.len() == 3 {
                let dest_start = tokens[0].parse::<usize>().unwrap();
                let source_start = tokens[1].parse::<usize>().unwrap();
                let range_len = tokens[2].parse::<usize>().unwrap();

                println!("Source: {} {}", source_start, source_start + range_len);
                let cloned_seeds_pair = seeds_pair.clone();
                for (low_range, high_range) in cloned_seeds_pair.into_iter() {
                    let intersect = get_range_intersection(
                        (low_range, high_range),
                        (source_start, source_start + range_len - 1),
                    );
                    println!(
                        "Field range: {:?},  Intersect: {:?}",
                        (low_range, high_range),
                        intersect
                    );
                    if intersect.0 < intersect.1 {
                        println!(
                            "kept: {:?} ===> {:?} (start: {} - diff: {} - range: {})",
                            (intersect.0, intersect.1),
                            (dest_start, dest_start + intersect.1 - intersect.0),
                            dest_start,
                            intersect.1 - intersect.0,
                            range_len
                        );
                        next_field_state
                            .insert((dest_start, dest_start + intersect.1 - intersect.0));
                        let outside_intersect =
                            get_ranges_diff((low_range, high_range), (intersect.0, intersect.1));
                        if let Some(left_outside_intersect) = outside_intersect.0 {
                            next_field_state.insert(left_outside_intersect);
                        }
                        if let Some(right_outside_intersect) = outside_intersect.1 {
                            next_field_state.insert(right_outside_intersect);
                        }
                    } else {
                        next_field_state.insert((low_range, high_range));
                    }
                }
            }
            println!("{}", next_field_state.len());
        }
    }

    let mut res: u64 = 9999999999999999999;
    for seed in seeds_pair {
        if res > seed.0 as u64 {
            res = seed.0 as u64;
        }
    }
    println!("Result: {}", res);
}
