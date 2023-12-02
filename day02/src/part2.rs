use std::collections::HashMap;

fn set_minimal_game_bag_content(game: &str, bag_content: &mut HashMap<String, i32>) {
    let contents: Vec<&str> = game.split(", ").collect();
    for content in contents {
        let content_tokens: Vec<&str> = content.split(" ").collect();
        let color = content_tokens[1].to_string();
        let quantity = content_tokens[0].parse::<i32>().unwrap();
        if bag_content[&color] < quantity {
            bag_content.insert(color, quantity);
        }
    }
}

pub fn start(file_content: &str) {
    let mut acc = 0;

    for line in file_content.lines() {
        let mut bag_content = HashMap::new();
        bag_content.insert("red".to_string(), 0);
        bag_content.insert("green".to_string(), 0);
        bag_content.insert("blue".to_string(), 0);

        let tokens: Vec<&str> = line.split(": ").collect();
        let games: Vec<&str> = tokens[1].split("; ").collect();
        for game in games {
            set_minimal_game_bag_content(game, &mut bag_content)
        }
        acc += bag_content["red"] * bag_content["green"] * bag_content["blue"];
    }
    println!("{:?}", acc);
}
