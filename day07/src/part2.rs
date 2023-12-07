/*
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
*/

use std::{cmp::Ordering, collections::BTreeMap};

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    score: i32,
    bid: i32,
    values: BTreeMap<i32, i32>,
    cards: Vec<i32>,
}

impl Hand {
    fn new(values: &BTreeMap<i32, i32>, bid: i32, cards: Vec<i32>) -> Hand {
        Hand {
            score: get_best_hand_score(values.clone()),
            bid,
            values: values.clone(),
            cards,
        }
    }
}

fn symbol_to_value(symbol: char) -> i32 {
    match symbol {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        x => x.to_string().parse::<i32>().unwrap(),
    }
}

fn hand_values(hand: &str) -> BTreeMap<i32, i32> {
    let values = hand
        .chars()
        .into_iter()
        .map(symbol_to_value)
        .collect::<Vec<i32>>();
    let mut result = BTreeMap::new();
    for v in values {
        *result.entry(v).or_insert(0) += 1;
    }
    return result;
}

fn hand_score(hand: BTreeMap<i32, i32>) -> i32 {
    let mut threes_count = 0;
    let mut pairs_count = 0;
    for (_, v) in hand {
        match v {
            5 => return 105,
            4 => return 104,
            3 => threes_count += 1,
            2 => pairs_count += 1,
            _ => continue,
        }
    }

    if threes_count == 1 && pairs_count == 1 {
        return 103;
    }
    if threes_count == 1 {
        return 102;
    }
    if pairs_count == 2 {
        return 101;
    }
    if pairs_count == 1 {
        return 100;
    }
    return 0;
}

fn get_best_hand_score(hand: BTreeMap<i32, i32>) -> i32 {
    let joker_count = *hand.get(&1).or_else(|| Some(&0)).unwrap();

    let mut best_score = 0;
    if joker_count == 0 {
        return hand_score(hand);
    } else {
        for (i, _) in hand.iter() {
            let mut current_hand = hand.clone();
            current_hand.entry(1).and_modify(|x| *x -= 1);
            if *i != 1 {
                current_hand.entry(*i).and_modify(|x| *x += joker_count);
            }
            best_score = best_score.max(get_best_hand_score(current_hand));
        }
    }

    return best_score;
}

fn compare_hands(f_hand: &Hand, s_hand: &Hand) -> Ordering {
    if f_hand.score == s_hand.score {
        for i in 0..f_hand.cards.len() {
            if f_hand.cards[i] == s_hand.cards[i] {
                continue;
            }
            return f_hand.cards[i].cmp(&s_hand.cards[i]);
        }
        return Ordering::Equal;
    } else {
        return f_hand.score.cmp(&s_hand.score);
    }
}

pub fn start(file_content: &str) {
    let mut hands = vec![];
    for line in file_content.lines() {
        let (hand, bid) = line.split_once(' ').unwrap();
        hands.push(Hand::new(
            &hand_values(hand),
            bid.parse::<i32>().unwrap(),
            hand.chars()
                .map(|c| symbol_to_value(c))
                .collect::<Vec<i32>>(),
        ));
    }

    hands.sort_by(|a, b| compare_hands(a, b));

    let mut acc = 0;
    for (i, hand) in hands.iter().enumerate() {
        acc += (1 + i) as i32 * hand.bid;
    }
    println!("{}", acc);
}
