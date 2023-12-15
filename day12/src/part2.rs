use std::collections::HashMap;

pub fn start(file_content: &str) {
    let mut memoized = HashMap::new();
    let mut acc = 0;
    for line in file_content.lines() {
        let (springs_input, groups) = line.split_once(" ").unwrap();
        let groups_input: Vec<usize> = groups
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let mut groups: Vec<usize> = vec![];
        let mut springs = "".to_string();
        for i in 0..5 {
            springs += springs_input;
            if i < 4 {
                springs += "?";
            }
            groups.append(&mut groups_input.clone());
        }

        let res = count_valid_orders(
            (springs.to_owned() + ".").as_bytes(),
            groups.clone(),
            0,
            &mut memoized,
        );
        acc += res;
    }
    println!("{}", acc);
}

fn count_valid_orders(
    springs: &[u8],
    groups: Vec<usize>,
    current_group_count: usize,
    memoized: &mut HashMap<(Vec<u8>, Vec<usize>, usize), usize>,
) -> usize {
    if memoized.contains_key(&(springs.to_vec(), groups.clone(), current_group_count)) {
        return *memoized
            .get(&(springs.to_vec(), groups.clone(), current_group_count))
            .unwrap();
    }
    if springs.is_empty() {
        if groups.is_empty() && current_group_count == 0 {
            return 1;
        } else {
            return 0;
        }
    }

    let mut acc = 0;
    if springs[0] == b'#' || springs[0] == b'?' {
        acc += count_valid_orders(
            &springs[1..],
            groups.clone(),
            current_group_count + 1,
            memoized,
        );
    }
    if (springs[0] == b'.' || springs[0] == b'?')
        && (!groups.is_empty() && groups[0] == current_group_count || current_group_count == 0)
    {
        acc += count_valid_orders(
            &springs[1..],
            if current_group_count > 0 {
                groups[1..].to_vec()
            } else {
                groups.clone()
            },
            0,
            memoized,
        );
    }

    if !memoized.contains_key(&(springs.to_vec(), groups.clone(), current_group_count)) {
        memoized.insert((springs.to_vec(), groups.clone(), current_group_count), acc);
    }

    acc
}
