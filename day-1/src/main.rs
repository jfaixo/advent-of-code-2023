use std::io::stdin;

fn main() {
    let lines = parse_input();
    solve_part_1(&lines);
    solve_part_2(&lines);
}

fn parse_input() -> Vec<String> {
    stdin().lines().map(|line| line.unwrap()).collect()
}

fn solve_part_1(lines: &Vec<String>) {
    let sum : i32 = lines.iter().map(|line| extract_calibration_value(line)).sum();
    println!("{}", sum);
}

fn solve_part_2(lines: &Vec<String>) {
    let sum : i32 = lines.iter().map(|line| {
        let mut line_replaced = String::new();
        for i in 0..line.len() {
            let c = line.chars().nth(i).unwrap();
            if c.is_numeric() {
                line_replaced.push(c);
            }
            else {
                for (p, r) in [("one", '1'), ("two", '2'), ("three", '3'), ("four", '4'), ("five", '5'), ("six", '6'), ("seven", '7'), ("eight", '8'), ("nine", '9')] {
                    if &line[i.. line.len().min(i + p.len())] == p {
                        line_replaced.push(r);
                    }
                }
            }
        }
        extract_calibration_value(&line_replaced)
    }).sum();

    println!("{}", sum);
}

fn extract_calibration_value(line: &String) -> i32 {
    let first_digit_index = line.find(|c: char| c.is_numeric()).unwrap();
    let first_digit = line.chars().nth(first_digit_index).unwrap();

    let last_digit_index = line.rfind(|c: char| c.is_numeric()).unwrap();
    let last_digit = line.chars().nth(last_digit_index).unwrap();

    format!("{}{}", first_digit, last_digit).parse::<i32>().unwrap()
}
