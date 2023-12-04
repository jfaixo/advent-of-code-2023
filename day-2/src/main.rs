use std::io::stdin;

#[derive(Default)]
struct Game {
    id: usize,
    reveals: Vec<[usize; 3]>,
}

fn main() {
    let games = parse_input();
    solve_part1(&games);
    solve_part2(&games);
}

fn solve_part2(games: &Vec<Game>) {
    let total_power: usize = games.iter().map(|game| {
        let mut max_cubes = [0, 0, 0];

        for reveal in &game.reveals {
            max_cubes[0] = max_cubes[0].max(reveal[0]);
            max_cubes[1] = max_cubes[1].max(reveal[1]);
            max_cubes[2] = max_cubes[2].max(reveal[2]);
        }

        max_cubes[0] * max_cubes[1] * max_cubes[2]
    }).sum();
    println!("{}", total_power);
}

fn solve_part1(games: &Vec<Game>) {
    let possible_games_id_sum : usize = games.iter().map(|game| {
        let mut possible = true;

        for reveal in &game.reveals {
            if reveal[0] > 12 || reveal[1] > 13 || reveal[2] > 14 {
                possible = false;
                break;
            }
        }

        if possible {
            game.id
        } else {
            0
        }
    }).sum();
    println!("{}", possible_games_id_sum);
}

fn parse_input() -> Vec<Game> {
    stdin().lines().map(|line| {
        let mut game = Game::default();
        let line = line.unwrap();
        let mut p1 = line.split(": ");
        game.id = p1.next().unwrap()[5..].parse().unwrap();

        let reveals = p1.next().unwrap().split("; ");
        for reveal in reveals {
            let mut values = [0, 0, 0];
            let colors = reveal.split(", ");
            for color in colors {
                let mut v = color.split(" ");
                let count = v.next().unwrap().trim().parse::<usize>().unwrap();
                let color_id = match v.next() {
                    Some("red") => 0,
                    Some("green") => 1,
                    Some(_) => 2,
                    None => panic!("not happening"),
                };
                values[color_id] = values[color_id].max(count);
            }
            game.reveals.push(values);
        }

        game
    }).collect()
}