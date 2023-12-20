use std::collections::HashMap;

#[derive(Debug)]
struct Flipflop {
    name: String,
    state: bool,
    targets: Vec<String>,
}

impl Flipflop {
    fn flip(&mut self, high_signal: bool) -> Vec<(String, String, bool)> {
        if !high_signal {
            self.state = !self.state;

            return self
                .targets
                .iter()
                .map(|target| (self.name.clone(), target.clone(), self.state))
                .collect();
        }
        vec![]
    }
}

#[derive(Debug)]
struct Conjuction {
    name: String,
    watches: HashMap<String, bool>,
    targets: Vec<String>,
}

impl Conjuction {
    fn signaled(&mut self, from: String, high_signal: bool) -> Vec<(String, String, bool)> {
        self.watches.insert(from, high_signal);
        let next_signal = self.watches.iter().all(|(_, signal)| *signal);
        return self
            .targets
            .iter()
            .map(|target| (self.name.clone(), target.clone(), !next_signal))
            .collect();
    }
}

pub fn start(file_content: &str) {
    let mut conj_map = HashMap::new();
    let mut flipflop_map = HashMap::new();
    let mut first_targets = vec![];
    for line in file_content.lines() {
        let (mut conn, targets) = line.split_once(" -> ").unwrap();
        let targets: Vec<String> = targets
            .split(", ")
            .map(|target| target.to_string())
            .collect();
        if conn == "broadcaster" {
            first_targets = targets
                .iter()
                .map(|target| (conn.to_string(), target.to_string(), false))
                .collect();
        } else if conn.starts_with("%") {
            conn = &conn[1..];
            flipflop_map.insert(
                conn.to_string(),
                Flipflop {
                    name: conn.to_string(),
                    state: false,
                    targets,
                },
            );
        } else if conn.starts_with("&") {
            conn = &conn[1..];
            conj_map.insert(
                conn,
                Conjuction {
                    name: conn.to_string(),
                    watches: HashMap::new(),
                    targets,
                },
            );
        }
    }

    for (name, fliflop) in flipflop_map.iter() {
        for target in fliflop.targets.iter() {
            if conj_map.contains_key(target.as_str()) {
                conj_map
                    .get_mut(target.as_str())
                    .unwrap()
                    .watches
                    .insert(name.to_string(), false);
            }
        }
    }
    let mut low_pulse_count = 0;
    let mut high_pulse_count = 0;
    for _ in 0..1000 {
        let mut next_targets = first_targets.clone();
        low_pulse_count += 1 + next_targets.len();

        while next_targets.len() > 0 {
            let target = next_targets.remove(0);
            let (from, target, state) = target;

            if let Some(flipflop) = flipflop_map.get_mut(target.as_str()) {
                let pulses = flipflop.flip(state);
                pulses.iter().for_each(|(_, _, state)| {
                    if *state {
                        high_pulse_count += 1;
                    } else {
                        low_pulse_count += 1;
                    }
                });
                next_targets.extend(pulses);
            } else if let Some(conn) = conj_map.get_mut(target.as_str()) {
                let pulses = conn.signaled(from, state);
                pulses.iter().for_each(|(_, _, state)| {
                    if *state {
                        high_pulse_count += 1;
                    } else {
                        low_pulse_count += 1;
                    }
                });
                next_targets.extend(pulses);
            }
        }
    }
    println!("{}", high_pulse_count * low_pulse_count);
}
