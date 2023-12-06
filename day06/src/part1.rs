pub fn start(file_content: &str) {
    let mut lines = file_content.lines();
    let time_records = lines.next().unwrap().split(": ").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let distance_records = lines.next().unwrap().split(": ").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut acc = 1;
    for i in 0..time_records.len() {
        let mut s_elapsed_time = 0;
        while (s_elapsed_time * (time_records[i] - s_elapsed_time)) <= distance_records[i] {
            s_elapsed_time += 1;
        }

        let mut t_elapsed_time = time_records[i];
        while (t_elapsed_time * (time_records[i] - t_elapsed_time)) <= distance_records[i] {
            t_elapsed_time -= 1;
        }

        acc *= 1 + t_elapsed_time - s_elapsed_time;
    }
    println!("{}", acc);
}
