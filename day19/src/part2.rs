use std::collections::HashMap;

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
                let nb = nb[1..].parse::<i128>().unwrap();

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
}

struct Rule {
    skip: bool,
    to_comp: String,
    sign: char,
    nb: i128,
    next: String,
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
    let mut investigated_ranges: Vec<(String, HashMap<String, (i128, i128)>)> = vec![(
        "in".to_string(),
        HashMap::from([
            ("x".to_string(), (1, 4000)),
            ("m".to_string(), (1, 4000)),
            ("a".to_string(), (1, 4000)),
            ("s".to_string(), (1, 4000)),
        ]),
    )];
    let mut accepted_ranges: Vec<HashMap<String, (i128, i128)>> = vec![];

    while let Some(mut current_ranges) = investigated_ranges.pop() {
        if current_ranges.0 == "A" {
            accepted_ranges.push(current_ranges.1);
        } else if !(current_ranges.0 == "R") {
            let workflow = &workflows[&current_ranges.0];
            for rule in &workflow.rules {
                if rule.skip {
                    investigated_ranges.push((rule.next.clone(), current_ranges.1.clone()));
                    continue;
                }

                let mut rule_valid_range = (
                    current_ranges.1[&rule.to_comp].0,
                    current_ranges.1[&rule.to_comp].1,
                );

                match rule.sign {
                    '<' => {
                        rule_valid_range.1 = rule.nb - 1;
                        let mut valid_range = current_ranges.1.clone();
                        valid_range.insert(rule.to_comp.clone(), rule_valid_range);
                        investigated_ranges.push((rule.next.clone(), valid_range));
                        current_ranges.1.get_mut(&rule.to_comp).unwrap().0 = rule.nb;
                    }
                    '>' => {
                        rule_valid_range.0 = rule.nb + 1;
                        let mut valid_range = current_ranges.1.clone();
                        valid_range.insert(rule.to_comp.clone(), rule_valid_range);
                        investigated_ranges.push((rule.next.clone(), valid_range));
                        current_ranges.1.get_mut(&rule.to_comp).unwrap().1 = rule.nb;
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    println!(
        "{}",
        accepted_ranges
            .iter()
            .map(|ranges| (1 + ranges["s"].1 - ranges["s"].0)
                * (1 + ranges["m"].1 - ranges["m"].0)
                * (1 + ranges["a"].1 - ranges["a"].0)
                * (1 + ranges["x"].1 - ranges["x"].0))
            .sum::<i128>()
    );
}
