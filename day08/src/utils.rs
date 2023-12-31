use std::fs;

#[allow(dead_code)]
pub fn read_file(file_path: &str) -> String {
    return fs::read_to_string(&file_path).expect("File not found or unable to read");
}

#[allow(dead_code)]
pub fn get_input_path() -> String {
    let args: Vec<String> = std::env::args().collect();
    let mut file_path = "./input.txt";
    if args.len() >= 2 {
        file_path = &args[1];
    }
    return file_path.to_string();
}

#[allow(dead_code)]
pub fn get_file_content(file_path: &str) -> String {
    return read_file(file_path);
}

#[allow(dead_code)]
pub fn char_at(str: &str, index: usize) -> Option<char> {
    return str.chars().nth(index);
}
