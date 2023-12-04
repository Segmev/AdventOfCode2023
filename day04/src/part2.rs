use std::collections::HashMap;

pub fn start(file_content: &str) {
    let mut cards: HashMap<usize, i32> = HashMap::new();

    for (idx, _) in file_content.split("\n").enumerate() {
        cards.insert(idx + 1, 1);
    }

    for line in file_content.split("\n") {
        let card_tokens = line.split(": ").collect::<Vec<&str>>();

        let card_id = card_tokens[0]
            .split(" ")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

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

        let mut supp_cards: usize = 0;
        for winning_number in winning_numbers {
            if card_numbers.contains(&winning_number) {
                supp_cards += 1;
            }
        }
        let card_quantity = *cards.get(&card_id).unwrap();
        while supp_cards > 0 {
            cards
                .entry(card_id + supp_cards)
                .and_modify(|x| *x += card_quantity);
            supp_cards -= 1;
        }
    }
    println!("{}", cards.into_values().sum::<i32>());
}
