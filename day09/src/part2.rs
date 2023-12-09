pub fn start(file_content: &str) {
    let mut result = 0;
    for line in file_content.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let mut sequences = vec![numbers.clone()];
        let mut next_sequence = get_next_sequence(numbers.clone());
        while !is_seq_all_0(next_sequence.clone()) {
            sequences.push(next_sequence.clone());
            next_sequence = get_next_sequence(next_sequence.clone());
        }

        let mut acc = 0;
        for sequence in sequences.iter().rev() {
            acc = sequence.first().unwrap() - acc;
        }
        result += acc;
    }
    println!("{}", result);
}

fn is_seq_all_0(nb_sequence: Vec<i32>) -> bool {
    for nb in nb_sequence {
        if nb != 0 {
            return false;
        }
    }
    true
}

fn get_next_sequence(nb_sequence: Vec<i32>) -> Vec<i32> {
    let mut next_sequence = Vec::new();
    for i in 0..nb_sequence.len() - 1 {
        next_sequence.push(nb_sequence[i + 1] - nb_sequence[i]);
    }

    next_sequence
}
