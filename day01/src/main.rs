mod utils;

fn part1(file_content: &str) {
    let mut acc = 0;
    for line in file_content.lines() {
        let mut first_digit = 0;
        let mut second_digit = 0;
        for c in line.chars() {
            if c.is_numeric() {
                first_digit = c.to_digit(10).unwrap();
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_numeric() {
                second_digit = c.to_digit(10).unwrap();
                break;
            }
        }
        acc += (first_digit * 10) + second_digit;
    }
    println!("{acc}");
}

fn find_first_litteral(line: &str) -> Option<u32> {
    let litteral_digits = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut best_result = 9999999;
    let mut found_nb = 0;
    let mut search_result;
    for litteral_digit in litteral_digits {
        search_result = line.find(litteral_digit.0);
        if search_result.is_some() {
            if search_result.unwrap() < best_result {
                best_result = search_result.unwrap();
                found_nb = litteral_digit.1;
            }
        }
    }
    if found_nb != 0 {
        return Some(found_nb);
    } else {
        return None;
    }
}

fn find_last_litteral(line: &str) -> Option<u32> {
    let litteral_digits = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut best_result = 0;
    let mut found_nb = 0;
    let mut search_result;
    for litteral_digit in litteral_digits {
        search_result = line.rfind(litteral_digit.0);
        if search_result.is_some() {
            if search_result.unwrap() > best_result || found_nb == 0 {
                best_result = search_result.unwrap();
                found_nb = litteral_digit.1;
            }
        }
    }
    if found_nb != 0 {
        return Some(found_nb);
    } else {
        return None;
    }
}

fn part2(file_content: &str) {
    let mut acc = 0;
    for line in file_content.lines() {
        let mut first_digit = 0;
        let mut second_digit = 0;
        let mut idx = 0;

        for c in line.chars() {
            if c.is_numeric() {
                first_digit = c.to_digit(10).unwrap();
                let substr = &line[..idx];
                let res = find_first_litteral(substr);
                if res.is_some() {
                    first_digit = res.unwrap();
                }
                break;
            }
            idx += 1;
        }
        if first_digit == 0 {
            first_digit = find_first_litteral(line).unwrap();
        }
        idx = 0;
        for c in line.chars().rev() {
            if c.is_numeric() {
                second_digit = c.to_digit(10).unwrap();

                let substr = &line[line.len() - idx..];

                let res = find_last_litteral(substr);
                if res.is_some() {
                    second_digit = res.unwrap();
                }
                break;
            }
            idx += 1;
        }
        if second_digit == 0 {
            second_digit = find_last_litteral(line).unwrap();
        }
        acc += (first_digit * 10) + second_digit;
    }
    println!("{acc}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut file_path = "./input.txt";
    if args.len() >= 2 {
        file_path = &args[1];
    }
    let file_content = utils::read_file(&file_path);

    part1(&file_content);
    part2(&file_content);
}
