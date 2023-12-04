use std::io::stdin;

fn main() {
    let data = parse_input();
    solve_part_1(&data);
    solve_part_2(&data);
}

fn solve_part_2(data: &Vec<Vec<char>>) {
    let mut gear_ratio_sum = Vec::new();

    for y in 0..data.len() {
        for x in 0..data[0].len() {
            // Find a gear
            if data[y][x] == '*' {
                // Look around it if there are any numbers

                // Above
                let mut gear_pn = Vec::new();
                for dy in (-1..=1) {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        match extract_number(data, x, y, dx, dy) {
                            None => {}
                            Some((v, x_end)) => {
                                gear_pn.push(v);
                                if x_end >= data[0].len() || x_end >= x {
                                    break;
                                }
                            }
                        }

                    }
                }

                if gear_pn.len() == 2 {
                    gear_ratio_sum.push(gear_pn[0] * gear_pn[1]);
                }
            }
        }
    }

    let sum : usize = gear_ratio_sum.iter().sum();
    println!("{}", sum);
}

fn solve_part_1(data: &Vec<Vec<char>>) {
    let mut engine_pn = Vec::new();

    for y in 0..data.len() {
        for x in 0..data[0].len() {
            // Find a symbol
            if data[y][x] != '.' && !data[y][x].is_numeric() {
                // Look around it if there are any numbers

                // Above
                for dy in (-1..=1) {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                            match extract_number(data, x, y, dx, dy) {
                                None => {}
                                Some((v, x_end)) => {
                                    engine_pn.push(v);
                                    if x_end >= data[0].len() || x_end >= x {
                                        break;
                                    }
                                }
                            }

                    }
                }
            }
        }
    }

    let sum : usize = engine_pn.iter().sum();
    println!("{}", sum);
}

fn extract_number(data: &Vec<Vec<char>>, x: usize, y: usize, dx: i32, dy: i32) -> Option<(usize, usize)> {
    let x_start = (x as isize + dx as isize);
    let y_start = (y as isize + dy as isize);
    if x_start < 0 || x_start >= data[0].len() as isize || y_start < 0 || y_start >= data.len() as isize {
        None
    }
    else {
        let x_start = x_start as usize;
        let y_start = y_start as usize;
        if data[y_start][x_start].is_numeric() {
            let mut x_num_start = x_start;
            loop {
                if x_num_start > 0 && data[y_start][x_num_start - 1].is_numeric() {
                    x_num_start -= 1;
                }
                else {
                    break;
                }
            }

            let mut x_num_end = x_start;
            loop {
                if x_num_end < data[0].len() - 1 && data[y_start][x_num_end + 1].is_numeric() {
                    x_num_end += 1;
                }
                else {
                    break;
                }
            }
            let v = data[y_start][x_num_start..=x_num_end].iter().collect::<String>().parse().unwrap();

            Some((v, x_num_end))
        }
        else {
            None
        }
    }
}

fn parse_input() -> Vec<Vec<char>> {
    stdin().lines().map(|line| line.unwrap().chars().collect::<Vec<_>>()).collect()
}