pub fn start(file_content: &str) {
    let mut acc = 0;

    for line in file_content.split("\n") {
        let card_tokens = line.split(": ").collect::<Vec<&str>>();
        let (winning_numbers_str, card_numbers_str) = card_tokens[1].split_once(" | ").unwrap();

        let winning_numbers = winning_numbers_str
            .split(" ")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let card_numbers = card_numbers_str
            .split(" ")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut points = 0;
        for winning_number in winning_numbers {
            if card_numbers.contains(&winning_number) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }
        acc += points;
    }
    println!("{}", acc);
}
