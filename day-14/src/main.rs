use std::collections::HashMap;
use std::io::stdin;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    let statement = parse_input();

    solve_part_1(&statement);
    solve_part_2(&statement);

    eprintln!("{} µs", (Instant::now() - start_time).as_micros());
}

fn solve_part_1(board: &Vec<Vec<u8>>) {
    let height = board.len();
    let width = board[0].len();
    let mut board = board.clone();

    roll_north(height, width, &mut board);

    let load: usize = board
        .iter()
        .enumerate()
        .map(|(i, row)| (height - i) * row.iter().filter(|&c| *c == b'O').count())
        .sum();
    println!("{}", load);
}

fn solve_part_2(board: &Vec<Vec<u8>>) {
    let height = board.len();
    let width = board[0].len();
    let mut board = board.clone();

    roll_north(height, width, &mut board);
    roll_west(height, width, &mut board);
    roll_south(height, width, &mut board);
    roll_east(height, width, &mut board);

    let mut previous_states = HashMap::new();
    let mut i = 0;
    loop {
        previous_states.insert(board.clone(), i);

        roll_north(height, width, &mut board);
        roll_west(height, width, &mut board);
        roll_south(height, width, &mut board);
        roll_east(height, width, &mut board);

        if previous_states.contains_key(&board) {
            let j_start = previous_states[&board];
            let j = j_start + (1000000000 - 1 - j_start) % (i + 1 - j_start);
            board = previous_states
                .iter()
                .find(|&(_, k)| j == *k)
                .unwrap()
                .0
                .clone();

            break;
        }

        i += 1;
    }

    let load: usize = board
        .iter()
        .enumerate()
        .map(|(i, row)| (height - i) * row.iter().filter(|&c| *c == b'O').count())
        .sum();
    println!("{}", load);
}

fn roll_north(height: usize, width: usize, board: &mut Vec<Vec<u8>>) {
    loop {
        let mut move_occured = false;
        for y in 1..height {
            for x in 0..width {
                if board[y][x] == b'O' && board[y - 1][x] == b'.' {
                    board[y][x] = b'.';
                    board[y - 1][x] = b'O';
                    move_occured = true;
                }
            }
        }
        if !move_occured {
            break;
        }
    }
}

fn roll_south(height: usize, width: usize, board: &mut Vec<Vec<u8>>) {
    loop {
        let mut move_occured = false;
        for y in (0..height - 1).rev() {
            for x in 0..width {
                if board[y][x] == b'O' && board[y + 1][x] == b'.' {
                    board[y][x] = b'.';
                    board[y + 1][x] = b'O';
                    move_occured = true;
                }
            }
        }
        if !move_occured {
            break;
        }
    }
}

fn roll_west(height: usize, width: usize, board: &mut Vec<Vec<u8>>) {
    loop {
        let mut move_occured = false;
        for x in 1..width {
            for y in 0..height {
                if board[y][x] == b'O' && board[y][x - 1] == b'.' {
                    board[y][x] = b'.';
                    board[y][x - 1] = b'O';
                    move_occured = true;
                }
            }
        }
        if !move_occured {
            break;
        }
    }
}

fn roll_east(height: usize, width: usize, board: &mut Vec<Vec<u8>>) {
    loop {
        let mut move_occured = false;
        for x in (0..width - 1).rev() {
            for y in 0..height {
                if board[y][x] == b'O' && board[y][x + 1] == b'.' {
                    board[y][x] = b'.';
                    board[y][x + 1] = b'O';
                    move_occured = true;
                }
            }
        }
        if !move_occured {
            break;
        }
    }
}

fn parse_input() -> Vec<Vec<u8>> {
    stdin()
        .lines()
        .map(|line| line.unwrap().as_bytes().to_vec())
        .collect()
}
