use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::hash::{Hash, Hasher};
use std::io::stdin;
use std::time::Instant;

struct Statement {
    width: usize,
    height: usize,
    map: Vec<Vec<usize>>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    x: usize,
    y: usize,
    direction: usize,
    straight_count: isize,
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.direction.hash(state);
    }
}

const DIRECTION: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reversed for min heap
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input() -> Statement {
    let map = stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.chars()
                .map(|c| c as usize - '0' as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let height = map.len();
    let width = map[0].len();

    Statement { width, height, map }
}
fn main() {
    let statement = parse_input();

    let start_time = Instant::now();
    solve_part_1(&statement);
    eprintln!("{} ms", (Instant::now() - start_time).as_millis());
    let start_time = Instant::now();
    solve_part_2(&statement);
    eprintln!("{} ms", (Instant::now() - start_time).as_millis());
}

fn solve_part_1(statement: &Statement) {
    let a = find_min_heat(
        statement,
        &State {
            cost: 0,
            x: 0,
            y: 0,
            direction: 5,
            straight_count: 0,
        },
        1,
        4,
    );

    eprintln!("{:?}", a);
}

fn solve_part_2(statement: &Statement) {
    let a = find_min_heat(
        statement,
        &State {
            cost: 0,
            x: 0,
            y: 0,
            direction: 5,
            straight_count: 0,
        },
        4,
        11,
    );

    eprintln!("{:?}", a);
}

fn find_min_heat(
    statement: &Statement,
    initial_state: &State,
    min_straight: isize,
    max_straight: isize,
) -> Option<usize> {
    let mut costs = vec![usize::MAX; statement.width * statement.height * 4];
    let mut heap = BinaryHeap::new();

    costs[0] = 0;
    costs[1] = 0;
    costs[2] = 0;
    costs[3] = 0;
    heap.push(*initial_state);

    while let Some(state) = heap.pop() {
        if state.x == statement.width - 1 && state.y == statement.height - 1 {
            return Some(state.cost);
        }

        if state.cost > costs[state.y * statement.width * 4 + state.x * 4 + state.direction] {
            continue;
        }

        for dir_new in 0..4 {
            if state.direction == dir_new || state.direction == (dir_new + 2) % 4 {
                continue;
            }
            for d in min_straight..max_straight {
                let y_new = state.y as isize + DIRECTION[dir_new].0 * d;
                let x_new = state.x as isize + DIRECTION[dir_new].1 * d;

                if x_new >= 0
                    && x_new < statement.width as isize
                    && y_new >= 0
                    && y_new < statement.height as isize
                {
                    let y_new = y_new as usize;
                    let x_new = x_new as usize;

                    let mut cost_new = state.cost;
                    if DIRECTION[dir_new].0 != 0 {
                        for dy in 1..=d {
                            cost_new += statement.map
                                [(state.y as isize + DIRECTION[dir_new].0 * dy) as usize][state.x];
                        }
                    }
                    if DIRECTION[dir_new].1 != 0 {
                        for dx in 1..=d {
                            cost_new += statement.map[state.y]
                                [(state.x as isize + DIRECTION[dir_new].1 * dx) as usize];
                        }
                    }

                    let next = State {
                        cost: cost_new,
                        x: x_new,
                        y: y_new,
                        direction: dir_new,
                        straight_count: d,
                    };

                    if cost_new < costs[y_new * statement.width * 4 + x_new * 4 + dir_new] {
                        heap.push(next);
                        costs[y_new * statement.width * 4 + x_new * 4 + dir_new] = cost_new;
                    }
                }
            }
        }
    }

    None
}
