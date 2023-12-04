use std::io::stdin;

fn main() {
    let cards = parse_input();

    solve_part_1(&cards);
    solve_part_2(&cards);
}

fn solve_part_2(cards: &Vec<Card>) {
    let mut card_count = vec![1; cards.len()];

    for card in cards {
        let mut winning_number_count = 0;
        for number in &card.winning_numbers {
            if card.numbers.contains(number) {
                winning_number_count += 1;
            }
        }

        for i in 0..winning_number_count {
            card_count[card.id + i] += card_count[card.id - 1];
        }
    }

    println!("{}", card_count.iter().sum::<usize>());
}

fn solve_part_1(cards: &Vec<Card>) {
    let score: i32 = cards.iter().map(|card| {
        let mut winning_number_count = 0;
        for number in &card.winning_numbers {
            if card.numbers.contains(number) {
                winning_number_count += 1;
            }
        }
        if winning_number_count > 0 {
            2_i32.pow(winning_number_count - 1)
        }
        else {
            0
        }
    }).sum();

    println!("{}", score);
}

struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>,
}

fn parse_input() -> Vec<Card> {
    stdin().lines().map(|line| {
        let line = line.unwrap();
        let mut parts = line.split(": ");
        let mut id = parts.next().unwrap().parse().unwrap();

        let mut all_numbers = parts.next().unwrap().split(" | ");
        let winning_numbers = all_numbers.next().unwrap().split_ascii_whitespace().map(|n| n.parse().unwrap()).collect();
        let numbers = all_numbers.next().unwrap().split_ascii_whitespace().map(|n| n.parse().unwrap()).collect();

        Card {
            id,
            winning_numbers,
            numbers
        }
    })
        .collect()
}