use std::collections::HashMap;

fn box_code(instr: &str) -> i128 {
    let mut acc: i128 = 0;
    for c in instr.chars() {
        if c == '-' || c == '=' {
            break;
        }
        acc += c as i128;
        acc *= 17;
        acc %= 256;
    }
    acc
}

pub fn start(file_content: &str) {
    let input = file_content.lines().next().unwrap();
    let instrs: Vec<&str> = input.split(',').collect();

    let mut boxes: HashMap<i128, Vec<String>> = HashMap::new();
    let mut lens_correspondance = HashMap::new();
    for instr in instrs {
        let instr_box_code = box_code(instr);
        let splitted_instr = instr.split("=").collect::<Vec<&str>>();
        let mut instr_label = splitted_instr[0].to_string();

        if splitted_instr.len() == 1 {
            instr_label.remove(instr_label.len() - 1);
            if let Some(b) = boxes.get_mut(&instr_box_code) {
                let elem_pos = b.iter_mut().position(|x| x == &instr_label);
                if let Some(pos) = elem_pos {
                    b.remove(pos);
                }
            }
        } else {
            let instr_focal = splitted_instr[1].to_string();
            let instr_box = boxes.entry(instr_box_code).or_insert(vec![]);
            if !instr_box.contains(&instr_label) {
                instr_box.push(instr_label.clone());
            }
            lens_correspondance.insert((instr_box_code, instr_label.clone()), instr_focal);
        }
    }

    let mut acc: i128 = 0;
    for b in boxes {
        for (i, len) in b.1.iter().enumerate() {
            if let Some(instr) = lens_correspondance.get(&(b.0, len.clone())) {
                acc += (b.0 + 1) * (i + 1) as i128 * instr.parse::<i128>().unwrap();
            }
        }
    }
    println!("{}", acc);
}
