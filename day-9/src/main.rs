use std::io::stdin;

fn parse_input() -> Vec<Vec<i64>> {
    stdin()
        .lines()
        .map(|line| {
            line.unwrap()
                .split_ascii_whitespace()
                .map(|parts| parts.parse().unwrap())
                .collect()
        })
        .collect()
}

fn solve_part_1(statement: &Vec<Vec<i64>>) {
    let extrapolated_values: i64 = statement
        .iter()
        .map(|history| {
            let mut sequences = vec![history.clone()];

            // Down pass
            loop {
                // Stop condition
                if sequences
                    .last()
                    .unwrap()
                    .iter()
                    .find(|&v| *v != 0)
                    .is_none()
                {
                    break;
                }

                let mut sequence_new = Vec::new();
                let sequence_last = sequences.last().unwrap();
                for (a, b) in sequence_last[0..sequence_last.len() - 1]
                    .iter()
                    .zip(sequence_last[1..].iter())
                {
                    sequence_new.push(*b - *a);
                }

                sequences.push(sequence_new)
            }

            // Sum the lat elements
            sequences
                .iter()
                .map(|sequence| sequence.last().unwrap())
                .sum::<i64>()
        })
        .sum();

    println!("{}", extrapolated_values);
}

fn solve_part_2(statement: &Vec<Vec<i64>>) {
    let extrapolated_values: i64 = statement
        .iter()
        .map(|history| {
            let mut sequences = vec![history.clone()];

            // Down pass
            loop {
                // Stop condition
                if sequences
                    .last()
                    .unwrap()
                    .iter()
                    .find(|&v| *v != 0)
                    .is_none()
                {
                    break;
                }

                let mut sequence_new = Vec::new();
                let sequence_last = sequences.last().unwrap();
                for (a, b) in sequence_last[0..sequence_last.len() - 1]
                    .iter()
                    .zip(sequence_last[1..].iter())
                {
                    sequence_new.push(*b - *a);
                }

                sequences.push(sequence_new)
            }

            // Sum the lat elements
            let mut v = 0;
            for i in (0..sequences.len()).rev() {
                v = sequences[i][0] - v;
            }
            v
        })
        .sum();

    println!("{}", extrapolated_values);
}

fn main() {
    let statement = parse_input();

    solve_part_1(&statement);
    solve_part_2(&statement);
}
