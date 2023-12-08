use std::collections::HashMap;

struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn get_next_node(&self, direction: char) -> String {
        match direction {
            'L' => self.left.clone(),
            'R' => self.right.clone(),
            _ => self.name.clone(),
        }
    }
}

pub fn start(file_content: &str) {
    let mut lines = file_content.lines();

    let direction: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();

    let mut nodes: HashMap<String, Node> = HashMap::new();
    for line in lines {
        let tokens = line.split(" = ").collect::<Vec<&str>>();
        let (left, right) = tokens[1][1..tokens[1].len() - 1].split_once(", ").unwrap();
        nodes.insert(
            tokens[0].to_string(),
            Node {
                name: tokens[0].to_string(),
                left: left.to_string(),
                right: right.to_string(),
            },
        );
    }

    let mut current_node = "AAA".to_string();
    let target = "ZZZ".to_string();
    let mut steps = 0;
    while current_node != target {
        current_node = nodes
            .get(&current_node)
            .unwrap()
            .get_next_node(direction[steps % direction.len()]);
        steps += 1;
    }
    println!("{}", steps);
}
