use std::collections::HashMap;

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

pub fn start(file_content: &str) {
    let mut field: HashMap<usize, usize> = HashMap::new(); // <seed, position>
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
            field.insert(id, id);
        });

    let mut new_field = field.clone();
    for line in lines.filter(|line| !line.is_empty()) {
        if line.contains(":") {
            field = new_field.clone();
        } else {
            let tokens = line.split(" ").collect::<Vec<&str>>();
            if tokens.len() == 3 {
                let dest_start = tokens[0].parse::<usize>().unwrap();
                let source_start = tokens[1].parse::<usize>().unwrap();
                let range_len = tokens[2].parse::<usize>().unwrap();

                for (seed_key, actual_pos) in field.clone() {
                    if (source_start..(source_start + range_len)).contains(&actual_pos) {
                        new_field.insert(seed_key, actual_pos - source_start + dest_start);
                    }
                }
            }
        }
    }
    field = new_field;

    let mut res: i64 = 9999999999999999;
    for seed in field {
        if res > seed.1 as i64 {
            res = seed.1 as i64;
        }
    }
    println!("Result: {}", res);
}
