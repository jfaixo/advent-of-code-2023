use std::cmp::Ordering;
use std::io::stdin;
use std::time::Instant;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum Card {
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn from_char(c: char) -> Card {
        match c {
            '2' => Card::N2,
            '3' => Card::N3,
            '4' => Card::N4,
            '5' => Card::N5,
            '6' => Card::N6,
            '7' => Card::N7,
            '8' => Card::N8,
            '9' => Card::N9,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            _ => Card::A,
        }
    }

    fn cards() -> Vec<Card> {
        vec![
            Card::N2,
            Card::N3,
            Card::N4,
            Card::N5,
            Card::N6,
            Card::N7,
            Card::N8,
            Card::N9,
            Card::T,
            Card::J,
            Card::Q,
            Card::K,
            Card::A,
        ]
    }

    fn card_values_p1() -> Vec<usize> {
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
    }

    fn card_values_p2() -> Vec<usize> {
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 10, 11, 12]
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Copy)]
struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
    hand_type_alt: HandType,
}

impl Hand {
    fn compare_part_1(&self, other: &Hand) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                let cards_self: Vec<usize> = self
                    .cards
                    .iter()
                    .map(|card| Card::card_values_p1()[*card as usize])
                    .collect();
                let cards_other: Vec<usize> = other
                    .cards
                    .iter()
                    .map(|card| Card::card_values_p1()[*card as usize])
                    .collect();
                cards_self.cmp(&cards_other)
            }
        }
    }

    fn compare_part_2(&self, other: &Hand) -> Ordering {
        match self.hand_type_alt.cmp(&other.hand_type_alt) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                let cards_self: Vec<_> = self
                    .cards
                    .iter()
                    .map(|card| Card::card_values_p2()[*card as usize])
                    .collect();
                let cards_other: Vec<_> = other
                    .cards
                    .iter()
                    .map(|card| Card::card_values_p2()[*card as usize])
                    .collect();
                cards_self.cmp(&cards_other)
            }
        }
    }
}

fn parse_input() -> Vec<(Hand, usize)> {
    stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
            let bid = parts[1].parse().unwrap();
            let hand = parse_hand(parts[0]);

            (hand, bid)
        })
        .collect::<Vec<_>>()
}

fn parse_hand(string: &str) -> Hand {
    let mut hand = Hand {
        cards: [Card::N2; 5],
        hand_type: HandType::HighCard,
        hand_type_alt: HandType::HighCard,
    };

    for (pos, c) in string.chars().enumerate() {
        hand.cards[pos] = Card::from_char(c);
    }

    let mut card_counts = Card::cards()
        .iter()
        .map(|card| {
            (
                hand.cards
                    .iter()
                    .filter(|&hand_card| hand_card == card)
                    .count(),
                *card,
            )
        })
        .collect::<Vec<_>>();

    card_counts.sort();
    card_counts.reverse();

    hand.hand_type = get_hand_type(&card_counts);
    hand.hand_type_alt = get_hand_type_p2(&card_counts);

    hand
}

fn get_hand_type(card_counts: &Vec<(usize, Card)>) -> HandType {
    match card_counts[0].0 {
        5 => HandType::FiveOfAKind,
        4 => HandType::FourOfAKind,
        3 => {
            if card_counts[1].0 == 2 {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        }
        2 => {
            if card_counts[1].0 == 2 {
                HandType::TwoPair
            } else {
                HandType::OnePair
            }
        }
        _ => HandType::HighCard,
    }
}

fn get_hand_type_p2(card_counts: &Vec<(usize, Card)>) -> HandType {
    let mut card_counts = card_counts.clone();
    let j_pos = card_counts.iter().position(|&c| c.1 == Card::J).unwrap();
    let j_count = card_counts[j_pos].0;
    card_counts.remove(j_pos);

    match card_counts[0].0 + j_count {
        5 => HandType::FiveOfAKind,
        4 => HandType::FourOfAKind,
        3 => {
            if card_counts[1].0 == 2 {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        }
        2 => {
            if card_counts[1].0 == 2 {
                HandType::TwoPair
            } else {
                HandType::OnePair
            }
        }
        _ => HandType::HighCard,
    }
}

fn solve_part_1(hands: &Vec<(Hand, usize)>) {
    let mut hands = hands.clone();
    hands.sort_by(|a, b| a.0.compare_part_1(&b.0));

    let result: usize = hands
        .iter()
        .enumerate()
        .map(|(position, hand)| (position + 1) * hand.1)
        .sum();

    println!("{}", result);
}

fn solve_part_2(hands: &Vec<(Hand, usize)>) {
    let mut hands = hands.clone();
    hands.sort_by(|a, b| a.0.compare_part_2(&b.0));

    let result: usize = hands
        .iter()
        .enumerate()
        .map(|(position, hand)| (position + 1) * hand.1)
        .sum();

    println!("{}", result);
}

fn main() {
    let start_time = Instant::now();
    let data = parse_input();

    solve_part_1(&data);
    solve_part_2(&data);

    eprintln!("{} µs", (Instant::now() - start_time).as_micros());
}
