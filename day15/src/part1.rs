pub fn start(file_content: &str) {
    let input = file_content.lines().next().unwrap();

    let instrs: Vec<&str> = input.split(',').collect();

    let mut acc: i128 = 0;
    for instr in instrs {
        let mut instr_acc = 0;
        for c in instr.chars() {
            instr_acc += c as i128;
            instr_acc *= 17;
            instr_acc %= 256;
        }
        acc += instr_acc;
    }
    println!("{}", acc);
}
