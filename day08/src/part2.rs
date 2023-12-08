use num::Integer;
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

fn number_of_steps_until_end(
    nodes: &HashMap<String, Node>,
    direction: Vec<char>,
    start_node: String,
) -> usize {
    let mut current_node = start_node;
    let mut steps = 0;
    while !current_node.ends_with("Z") {
        current_node = nodes
            .get(&current_node)
            .unwrap()
            .get_next_node(direction[steps % direction.len()]);
        steps += 1;
    }
    steps
}

pub fn start(file_content: &str) {
    let mut lines = file_content.lines();

    let direction: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();

    let mut current_nodes = Vec::new();
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
        if tokens[0].ends_with("A") {
            current_nodes.push(tokens[0].to_string());
        }
    }

    let mut steps = vec![];
    for node in current_nodes {
        steps.push(number_of_steps_until_end(&nodes, direction.clone(), node));
    }
    let mut acc: i128 = 1;
    for step in steps {
        acc = acc.lcm(&(step as i128));
    }
    println!("{}", acc);
}
