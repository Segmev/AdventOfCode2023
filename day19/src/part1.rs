use std::collections::HashMap;

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn new(line: &str) -> Workflow {
        let (name, rules_str) = line.split_once("{").unwrap();
        let mut rules_str = rules_str.to_string();
        rules_str.truncate(rules_str.len() - 1);

        let rules = rules_str
            .split(",")
            .map(|rule| {
                if !rule.contains(":") {
                    return Rule {
                        skip: true,
                        next: rule.to_string(),
                        to_comp: rule.to_string(),
                        sign: ' ',
                        nb: 0,
                    };
                }

                let (computation, next) = rule.split_once(":").unwrap();

                let mut sign_pos = 0;
                if let Some(pos) = computation.find(">") {
                    sign_pos = pos;
                } else if let Some(pos) = computation.find("<") {
                    sign_pos = pos;
                }

                let sign = computation.chars().nth(sign_pos).unwrap();
                let (to_comp, nb) = computation.split_at(sign_pos);
                let nb = nb[1..].parse::<i32>().unwrap();

                Rule {
                    skip: false,
                    next: next.to_string(),
                    to_comp: to_comp.to_string(),
                    sign,
                    nb,
                }
            })
            .collect();

        Workflow {
            name: name.to_string(),
            rules,
        }
    }

    fn compute_part(
        &self,
        workflows: &HashMap<String, Workflow>,
        part: &HashMap<String, i32>,
    ) -> i32 {
        for rule in &self.rules {
            if rule.is_valid(&part) {
                match rule.next.as_str() {
                    "A" => return part.values().sum::<i32>(),
                    "R" => return 0,
                    _ => {}
                }
                return workflows[&rule.next].compute_part(workflows, part);
            }
        }
        0
    }
}

#[derive(Debug)]
struct Rule {
    skip: bool,
    to_comp: String,
    sign: char,
    nb: i32,
    next: String,
}

impl Rule {
    fn is_valid(&self, part: &HashMap<String, i32>) -> bool {
        if self.skip {
            return true;
        }
        match self.sign {
            '>' => part[&self.to_comp] > self.nb,
            '<' => part[&self.to_comp] < self.nb,
            _ => unreachable!(),
        }
    }
}

pub fn start(file_content: &str) {
    let mut workflows = HashMap::new();
    let mut lines = file_content.lines();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let workflow = Workflow::new(line);
        workflows.insert(workflow.name.clone(), workflow);
    }

    let mut acc = 0;
    while let Some(line) = lines.next() {
        let mut line = line.to_string();
        line.remove(0);
        line.truncate(line.len() - 1);

        let mut part = HashMap::new();
        for elem in line.split(",") {
            let (elem_l, nb) = elem.split_once("=").unwrap();
            part.insert(elem_l.to_string(), nb.parse::<i32>().unwrap());
        }
        acc += workflows["in"].compute_part(&workflows, &part);
    }
    println!("{}", acc);
}
