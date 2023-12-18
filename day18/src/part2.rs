fn get_step_from_dex(hex_instr: &str) -> (char, i128) {
    let mut instr = hex_instr.to_string();
    instr.truncate(instr.len() - 1);
    instr.remove(0);
    instr.remove(0);
    let dir_c: char = match instr.remove(instr.len() - 1) {
        '0' => 'R',
        '1' => 'D',
        '2' => 'L',
        '3' => 'U',
        _ => unreachable!(),
    };
    let steps = i128::from_str_radix(&instr, 16).unwrap();
    (dir_c, steps)
}

pub fn start(file_content: &str) {
    let mut edges: Vec<(i128, i128)> = vec![];
    let mut current_pos = (0, 0);
    let mut perimeter = 1;
    for line in file_content.lines() {
        let tokens = line.split_whitespace().collect::<Vec<&str>>();
        let (dir_c, steps) = get_step_from_dex(tokens[2]);
        perimeter += steps;

        if ['R', 'L'].contains(&dir_c) {
            let direction = if dir_c == 'L' { -1 } else { 1 };
            current_pos = (current_pos.0 + direction * steps, current_pos.1);
            edges.push(current_pos);
        } else if ['D', 'U'].contains(&dir_c) {
            let direction = if dir_c == 'D' { -1 } else { 1 };
            current_pos = (current_pos.0, current_pos.1 + direction * steps);
            edges.push(current_pos);
        }
    }

    let mut area = 0;
    let mut previous_edge = edges.pop().unwrap();
    edges.reverse();
    while let Some(next_edge) = edges.pop() {
        area += (previous_edge.1 + next_edge.1) * (previous_edge.0 - next_edge.0);
        previous_edge = next_edge;
    }
    println!("{}", (area.abs() + perimeter + 1) / 2);
}
