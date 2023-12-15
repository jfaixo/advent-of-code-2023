use std::io::stdin;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Tile {
    None,
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Start,
    Erased,
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '|' => Tile::NS,
            '-' => Tile::EW,
            'L' => Tile::NE,
            'J' => Tile::NW,
            '7' => Tile::SW,
            'F' => Tile::SE,
            'S' => Tile::Start,
            _ => Tile::None,
        }
    }

    fn find_next_cells(
        &self,
        y_current: isize,
        x_current: isize,
        tiles: &Vec<Vec<Tile>>,
    ) -> Option<((isize, isize), (isize, isize))> {
        match self {
            Tile::None | Tile::Erased => None,
            Tile::NS => Some(((y_current - 1, x_current), (y_current + 1, x_current))),
            Tile::EW => Some(((y_current, x_current - 1), (y_current, x_current + 1))),
            Tile::NE => Some(((y_current - 1, x_current), (y_current, x_current + 1))),
            Tile::NW => Some(((y_current - 1, x_current), (y_current, x_current - 1))),
            Tile::SW => Some(((y_current + 1, x_current), (y_current, x_current - 1))),
            Tile::SE => Some(((y_current + 1, x_current), (y_current, x_current + 1))),
            Tile::Start => {
                let mut coords = [(0, 0); 2];
                let mut i = 0;

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let y = y_current + dy;
                        let x = x_current + dx;
                        if y >= 0
                            && y < tiles.len() as isize
                            && x >= 0
                            && x < tiles[0].len() as isize
                            && (dx != 0 || dy != 0)
                        {
                            match tiles[y as usize][x as usize].find_next_cells(y, x, tiles) {
                                None => {}
                                Some(next_tiles) => {
                                    if next_tiles.0 == (y_current, x_current) {
                                        coords[i] = (y, x);
                                        i += 1;
                                    }
                                    if next_tiles.1 == (y_current, x_current) {
                                        coords[i] = (y, x);
                                        i += 1;
                                    }
                                }
                            }
                        }
                    }
                }

                Some((
                    (coords[0].0 as isize, coords[0].1 as isize),
                    (coords[1].0 as isize, coords[1].1 as isize),
                ))
            }
        }
    }
}

fn parse_input() -> Vec<Vec<Tile>> {
    stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.trim()
                .chars()
                .map(|c| Tile::from_char(c))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn find_start(tiles: &Vec<Vec<Tile>>) -> (isize, isize) {
    for y in 0..tiles.len() {
        match tiles[y].iter().position(|t| *t == Tile::Start) {
            None => {}
            Some(x) => {
                return (y as isize, x as isize);
            }
        }
    }
    panic!("Start not found");
}

fn solve_part_1(tiles: &Vec<Vec<Tile>>) -> Vec<(isize, isize)> {
    let coords_start = find_start(tiles);

    // Follow the pipe
    let mut pipe = vec![coords_start];
    let mut coords_current = coords_start;
    let mut coords_previous = coords_start;
    loop {
        let coords_next = tiles[coords_current.0 as usize][coords_current.1 as usize]
            .find_next_cells(coords_current.0, coords_current.1, tiles)
            .unwrap();

        let coords_next = if coords_next.0 != coords_previous {
            coords_next.0
        } else if coords_next.1 != coords_previous {
            coords_next.1
        } else {
            panic!("should not happen")
        };

        if coords_next != coords_start {
            pipe.push(coords_next);
            coords_previous = coords_current;
            coords_current = coords_next;
        } else {
            break;
        }
    }

    println!("{}", pipe.len() / 2);
    pipe
}

fn solve_part_2(tiles: &Vec<Vec<Tile>>, pipe: Vec<(isize, isize)>) {
    // Transform the map in something easier to manipulate
    // Remove all unnecessary pipes
    let mut map = tiles.clone();
    for y in 0..tiles.len() {
        for x in 0..tiles[0].len() {
            if map[y][x] != Tile::None && !pipe.contains(&(y as isize, x as isize)) {
                map[y][x] = Tile::Erased;
            }
        }
    }

    // Replace the start by its tile
    let coords_start = find_start(tiles);
    let coords_after_start = tiles[coords_start.0 as usize][coords_start.1 as usize]
        .find_next_cells(coords_start.0, coords_start.1, tiles)
        .unwrap();
    let mut dx = 0;
    let mut dy = 0;
    if coords_start.1 - coords_after_start.0 .1 == 1
        || coords_start.1 - coords_after_start.1 .1 == 1
    {
        dx = 1;
    }
    if coords_start.1 - coords_after_start.0 .1 == -1
        || coords_start.1 - coords_after_start.1 .1 == -1
    {
        dx = -1;
    }
    if coords_start.0 - coords_after_start.0 .0 == 1
        || coords_start.0 - coords_after_start.1 .0 == 1
    {
        dy = 1;
    }
    if coords_start.0 - coords_after_start.0 .0 == -1
        || coords_start.0 - coords_after_start.1 .0 == -1
    {
        dy = -1;
    }

    match (dy, dx) {
        (0, -1) => map[coords_start.0 as usize][coords_start.1 as usize] = Tile::EW,
        (-1, 0) => map[coords_start.0 as usize][coords_start.1 as usize] = Tile::NS,
        (1, -1) => map[coords_start.0 as usize][coords_start.1 as usize] = Tile::NE,
        (1, 1) => map[coords_start.0 as usize][coords_start.1 as usize] = Tile::NW,
        (-1, -1) => map[coords_start.0 as usize][coords_start.1 as usize] = Tile::SE,
        (-1, 1) => map[coords_start.0 as usize][coords_start.1 as usize] = Tile::SW,
        _ => panic!("not possible"),
    }

    // for y in 0..map.len() {
    //     eprintln!("{:?}", map[y]);
    // }

    let mut inside_count = 0;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == Tile::None || map[y][x] == Tile::Erased {
                // Count the number of pipes we traverse
                let mut pipe_count = 0;
                let mut on_border = false;
                let mut arrived_from_top = false;
                for x in x + 1..map[y].len() {
                    match map[y][x] {
                        Tile::None => {}
                        Tile::NS => pipe_count += 1,
                        Tile::EW => {}
                        Tile::NE => {
                            on_border = true;
                            arrived_from_top = true;
                        }
                        Tile::NW => {
                            if on_border && arrived_from_top == false {
                                pipe_count += 1;
                                on_border = false;
                            }
                        }
                        Tile::SW => {
                            if on_border && arrived_from_top == true {
                                pipe_count += 1;
                                on_border = false;
                            }
                        }
                        Tile::SE => {
                            on_border = true;
                            arrived_from_top = false;
                        }
                        Tile::Start => {}
                        Tile::Erased => {}
                    }
                }

                if pipe_count % 2 == 1 {
                    inside_count += 1;
                }
            }
        }
    }

    println!("{}", inside_count);
}

fn main() {
    let start_time = Instant::now();

    let tiles = parse_input();

    let pipe = solve_part_1(&tiles);
    solve_part_2(&tiles, pipe);

    eprintln!("{} µs", (Instant::now() - start_time).as_micros());
}
