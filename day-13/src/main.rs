use std::io::stdin;

#[derive(Default, Clone)]
struct Pattern {
    data: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Pattern {
    fn find_horizontal_reflection(&self, previous: Option<usize>) -> Option<usize> {
        'next: for y_split in 1..self.height {
            for y in 0..y_split.min(self.height - y_split) {
                for x in 0..self.width {
                    let left = self.data[y_split - y - 1][x];
                    let right = self.data[y_split + y][x];
                    if left != right {
                        continue 'next;
                    }
                }
            }

            if previous != Some(y_split) {
                return Some(y_split);
            }
        }
        None
    }
    fn find_vertical_reflection(&self, previous: Option<usize>) -> Option<usize> {
        'next: for x_split in 1..self.width {
            for x in 0..x_split.min(self.width - x_split) {
                for y in 0..self.height {
                    let left = self.data[y][x_split - x - 1];
                    let right = self.data[y][x_split + x];
                    if left != right {
                        continue 'next;
                    }
                }
            }

            if previous != Some(x_split) {
                return Some(x_split);
            }
        }
        None
    }
}

fn parse_input() -> Vec<Pattern> {
    let mut patterns = Vec::new();
    let mut current_pattern = Pattern::default();
    for line in stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            patterns.push(current_pattern);
            current_pattern = Pattern::default();
        } else {
            current_pattern.height += 1;
            current_pattern.width = line.len();
            current_pattern.data.push(line.as_bytes().to_vec());
        }
    }
    patterns.push(current_pattern);

    patterns
}
fn main() {
    let patterns = parse_input();

    solve_part_1(&patterns);
    solve_part_2(&patterns);
}

fn solve_part_1(patterns: &Vec<Pattern>) {
    let mut score = 0;
    for (i, pattern) in patterns.iter().enumerate() {
        if let Some(cols) = pattern.find_vertical_reflection(None) {
            score += cols;
        } else if let Some(rows) = pattern.find_horizontal_reflection(None) {
            score += 100 * rows;
        } else {
            panic!("not possible");
        }
    }

    println!("{}", score);
}

fn solve_part_2(patterns: &Vec<Pattern>) {
    let mut score = 0;

    for (i, pattern) in patterns.iter().enumerate() {
        let mut previous_col = None;
        let mut previous_row = None;
        if let Some(cols) = pattern.find_vertical_reflection(None) {
            previous_col = Some(cols);
        } else if let Some(rows) = pattern.find_horizontal_reflection(None) {
            previous_row = Some(rows);
        } else {
            panic!("not possible");
        }

        let mut pattern = pattern.clone();
        'pattern: for y in 0..pattern.height {
            for x in 0..pattern.width {
                let previous = pattern.data[y][x];
                if pattern.data[y][x] == b'#' {
                    pattern.data[y][x] = b'.';
                } else {
                    pattern.data[y][x] = b'#';
                }

                let mut smudge = false;
                if let Some(cols) = pattern.find_vertical_reflection(previous_col) {
                    score += cols;
                    smudge = true;
                }
                if let Some(rows) = pattern.find_horizontal_reflection(previous_row) {
                    score += 100 * rows;
                    smudge = true;
                }

                if smudge {
                    break 'pattern;
                }

                pattern.data[y][x] = previous;
            }
        }
    }

    println!("{}", score);
}
