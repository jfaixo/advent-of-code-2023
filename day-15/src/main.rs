use std::io::stdin;
use std::time::Instant;

fn hash(str: &str) -> usize {
    let mut v = 0;

    for c in str.chars() {
        v += c as usize;
        v *= 17;
        v &= 0xFF;
    }

    v
}
fn solve_part_1(sequence: &Vec<String>) {
    println!("{}", sequence.iter().map(|str| hash(&&str)).sum::<usize>());
}

fn solve_part_2(sequence: &Vec<String>) {
    let mut boxes = vec![Vec::new(); 256];

    for op in sequence {
        if op.chars().nth(op.len() - 1).unwrap() != '-' {
            let hash = hash(&op[0..op.len() - 2]);
            let n = (op.chars().nth(op.len() - 1).unwrap() as u8 - '0' as u8) as usize;
            if let Some(pos) = boxes[hash]
                .iter()
                .position(|(label, _)| label == &op[0..op.len() - 2])
            {
                boxes[hash][pos].1 = n;
            } else {
                boxes[hash].push((op[0..op.len() - 2].to_string(), n));
            }
        } else {
            let hash = hash(&op[0..op.len() - 1]);
            if let Some(pos) = boxes[hash]
                .iter()
                .position(|(label, _)| label == &op[0..op.len() - 1])
            {
                boxes[hash].remove(pos);
            }
        }
    }

    let mut result = 0;
    for (box_id, bx) in boxes.iter().enumerate() {
        for (lens_id, lens) in bx.iter().enumerate() {
            result += (box_id + 1) * (lens_id + 1) * lens.1;
        }
    }
    println!("{}", result);
}

fn parse_input() -> Vec<String> {
    stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|s| s.to_string())
        .collect()
}

fn main() {
    let start_time = Instant::now();

    let sequence = parse_input();

    solve_part_1(&sequence);
    solve_part_2(&sequence);

    eprintln!("{} µs", (Instant::now() - start_time).as_micros());
}
