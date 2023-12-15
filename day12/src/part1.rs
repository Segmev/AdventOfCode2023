pub fn start(file_content: &str) {
    let mut acc = 0;
    for line in file_content.lines() {
        let (springs, groups) = line.split_once(" ").unwrap();
        let groups: Vec<usize> = groups
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let res = count_valid_orders((springs.to_owned() + ".").as_bytes(), groups.clone(), 0);
        acc += res;
    }
    println!("{}", acc);
}

fn count_valid_orders(springs: &[u8], groups: Vec<usize>, current_group_count: usize) -> usize {
    if springs.is_empty() {
        if groups.is_empty() && current_group_count == 0 {
            return 1;
        } else {
            return 0;
        }
    }

    let mut acc = 0;
    if springs[0] == b'#' || springs[0] == b'?' {
        acc += count_valid_orders(&springs[1..], groups.clone(), current_group_count + 1);
    }
    if (springs[0] == b'.' || springs[0] == b'?')
        && (!groups.is_empty() && groups[0] == current_group_count || current_group_count == 0)
    {
        acc += count_valid_orders(
            &springs[1..],
            if current_group_count > 0 {
                groups[1..].to_vec()
            } else {
                groups
            },
            0,
        );
    }

    acc
}
