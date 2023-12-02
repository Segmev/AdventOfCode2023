use std::collections::HashMap;

fn is_game_possible(game: &str, bag_content: &HashMap<&str, i32>) -> bool {
    let contents: Vec<&str> = game.split(", ").collect();
    for content in contents {
        let content_tokens: Vec<&str> = content.split(" ").collect();
        let color = content_tokens[1];
        let quantity = content_tokens[0].parse::<i32>().unwrap();
        if bag_content[color] < quantity {
            return false;
        }
    }
    return true;
}

pub fn start(file_content: &str) {
    let mut acc = 0;
    let mut bag_content = HashMap::new();
    bag_content.insert("red", 12);
    bag_content.insert("green", 13);
    bag_content.insert("blue", 14);

    for line in file_content.lines() {
        let tokens: Vec<&str> = line.split(": ").collect();
        let game_id = tokens[0].split(" ").collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap();

        let games: Vec<&str> = tokens[1].split("; ").collect();
        let mut game_is_possible = true;
        for game in games {
            if !is_game_possible(game, &bag_content) {
                game_is_possible = false;
                break;
            }
        }
        if game_is_possible {
            acc += game_id;
        }
    }
    println!("{:?}", acc);
}
