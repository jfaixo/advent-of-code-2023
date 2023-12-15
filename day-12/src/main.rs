use std::io::stdin;
use std::time::Instant;

#[derive(Clone)]
struct Arrangement {
    template: String,
    parts: Vec<usize>,
}

fn parse_input() -> Vec<Arrangement> {
    stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let p = line.split_ascii_whitespace().collect::<Vec<_>>();

            let template = p[0].to_string();

            let parts = p[1].split(",").map(|p| p.parse().unwrap()).collect();

            Arrangement { template, parts }
        })
        .collect()
}

fn solve_part_2(statement: &Vec<Arrangement>) {
    // Unfold
    let mut statement = statement.clone();
    for i in 0..statement.len() {
        let mut template = statement[i].template.clone();
        template += "?";
        statement[i].template = template.repeat(5);
        let l = statement[i].template.len();
        statement[i].template.remove(l - 1);

        let parts = statement[i].parts.clone();
        for _ in 0..4 {
            let mut parts = parts.clone();
            statement[i].parts.append(&mut parts);
        }
    }

    solve_part_1(&statement);
}

fn count_arrangements(pattern: &str, groups: &Vec<usize>) -> usize {
    let pattern = ".".to_string() + pattern.trim_end_matches('.');
    let pattern = pattern.as_bytes();

    // dp[i][j] := Number of arrangements of the first j springs into the first i locations
    let mut dp = vec![0; pattern.len() + 1];
    dp[0] = 1;

    for (i, _) in pattern.iter().enumerate().filter(|(_, &c)| c != b'#') {
        dp[i + 1] = 1;
    }

    for &group in groups {
        let mut n_dp = vec![0; pattern.len() + 1];
        let mut chunk = 0;

        for (i, &c) in pattern.iter().enumerate() {
            if c != b'.' {
                chunk += 1;
            } else {
                chunk = 0;
            }

            if c != b'#' {
                n_dp[i + 1] += n_dp[i];
            }

            if chunk >= group && pattern[i - group] != b'#' {
                n_dp[i + 1] += dp[i - group];
            }
        }

        dp = n_dp;
    }

    *dp.last().unwrap()
}

fn solve_part_1(statement: &Vec<Arrangement>) {
    let mut sum = 0;

    for arrangement in statement {
        sum += count_arrangements(&arrangement.template, &arrangement.parts);
    }

    println!("{}", sum);
}

fn main() {
    let start_time = Instant::now();

    let ref statement = parse_input();

    solve_part_1(statement);
    solve_part_2(statement);

    eprintln!("{} µs", (Instant::now() - start_time).as_micros());
}
