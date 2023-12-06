pub fn start(file_content: &str) {
    let mut lines = file_content.lines();

    let mut time_record_str =
        lines.next().unwrap().split(": ").collect::<Vec<&str>>()[1].to_string();
    time_record_str.retain(|x| !x.is_whitespace());

    let time_record = time_record_str.parse::<i128>().unwrap();

    let mut distance_record_str =
        lines.next().unwrap().split(": ").collect::<Vec<&str>>()[1].to_string();
    distance_record_str.retain(|x| !x.is_whitespace());

    let distance_record = distance_record_str.parse::<i128>().unwrap();

    let mut acc = 1;
    let mut s_elapsed_time = 0;
    while (s_elapsed_time * (time_record - s_elapsed_time)) <= distance_record {
        s_elapsed_time += 1;
    }

    let mut t_elapsed_time = time_record;
    while (t_elapsed_time * (time_record - t_elapsed_time)) <= distance_record {
        t_elapsed_time -= 1;
    }

    acc *= 1 + t_elapsed_time - s_elapsed_time;
    println!("{}", acc);
}
