use std::collections::HashSet;
use std::io::stdin;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum BeamDirection {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct BeamState {
    x: isize,
    y: isize,
    direction: BeamDirection,
}

enum CellKind {
    Empty,
    MirrorUp,
    MirrorDown,
    SplitterHorizontal,
    SplitterVertical,
}

struct Statement {
    mirror_map: Vec<Vec<CellKind>>,
    width: usize,
    height: usize,
}

struct Solver<'a> {
    statement: &'a Statement,
    heap: Vec<BeamState>,
}

impl<'a> Solver<'a> {
    fn solve_part_2(&mut self) {
        let mut energized_max = 0;

        for x in 0..self.statement.width as isize {
            let energized = self.send_beam(BeamState {
                x,
                y: 0,
                direction: BeamDirection::Down,
            });
            energized_max = energized_max.max(energized);
            let energized = self.send_beam(BeamState {
                x,
                y: self.statement.height as isize - 1,
                direction: BeamDirection::Up,
            });
            energized_max = energized_max.max(energized);
        }

        for y in 0..self.statement.height as isize {
            let energized = self.send_beam(BeamState {
                x: 0,
                y,
                direction: BeamDirection::Right,
            });
            energized_max = energized_max.max(energized);
            let energized = self.send_beam(BeamState {
                x: self.statement.width as isize - 1,
                y,
                direction: BeamDirection::Left,
            });
            energized_max = energized_max.max(energized);
        }

        println!("{}", energized_max);
    }

    fn solve_part_1(&mut self) {
        let energized = self.send_beam(BeamState {
            x: 0,
            y: 0,
            direction: BeamDirection::Right,
        });

        println!("{}", energized);
    }
    fn send_beam(&mut self, initial_state: BeamState) -> u32 {
        self.heap.clear();

        let mut energized = vec![false; self.statement.width * self.statement.height];
        self.heap.push(initial_state);

        let mut already_visited =
            HashSet::with_capacity(self.statement.width * self.statement.height);

        while let Some(state) = self.heap.pop() {
            if !already_visited.contains(&state) {
                already_visited.insert(state);

                energized[state.y as usize * self.statement.width + state.x as usize] = true;
                self.move_beam(state.x, state.y, state.direction);
            }
        }

        energized.iter().map(|v| *v as u32).sum()
    }

    fn move_beam(&mut self, x: isize, y: isize, direction: BeamDirection) {
        match self.statement.mirror_map[y as usize][x as usize] {
            CellKind::Empty => match direction {
                BeamDirection::Up => self.add_to_heap(x, y - 1, direction),
                BeamDirection::Right => self.add_to_heap(x + 1, y, direction),
                BeamDirection::Down => self.add_to_heap(x, y + 1, direction),
                BeamDirection::Left => self.add_to_heap(x - 1, y, direction),
            },
            CellKind::MirrorUp => match direction {
                BeamDirection::Up => self.add_to_heap(x + 1, y, BeamDirection::Right),
                BeamDirection::Right => self.add_to_heap(x, y - 1, BeamDirection::Up),
                BeamDirection::Down => self.add_to_heap(x - 1, y, BeamDirection::Left),
                BeamDirection::Left => self.add_to_heap(x, y + 1, BeamDirection::Down),
            },
            CellKind::MirrorDown => match direction {
                BeamDirection::Up => self.add_to_heap(x - 1, y, BeamDirection::Left),
                BeamDirection::Right => self.add_to_heap(x, y + 1, BeamDirection::Down),
                BeamDirection::Down => self.add_to_heap(x + 1, y, BeamDirection::Right),
                BeamDirection::Left => self.add_to_heap(x, y - 1, BeamDirection::Up),
            },
            CellKind::SplitterHorizontal => match direction {
                BeamDirection::Right => self.add_to_heap(x + 1, y, BeamDirection::Right),
                BeamDirection::Left => self.add_to_heap(x - 1, y, BeamDirection::Left),
                BeamDirection::Up | BeamDirection::Down => {
                    self.add_to_heap(x - 1, y, BeamDirection::Left);
                    self.add_to_heap(x + 1, y, BeamDirection::Right);
                }
            },
            CellKind::SplitterVertical => match direction {
                BeamDirection::Up => self.add_to_heap(x, y - 1, BeamDirection::Up),
                BeamDirection::Down => self.add_to_heap(x, y + 1, BeamDirection::Down),
                BeamDirection::Right | BeamDirection::Left => {
                    self.add_to_heap(x, y + 1, BeamDirection::Down);
                    self.add_to_heap(x, y - 1, BeamDirection::Up);
                }
            },
        }
    }

    fn add_to_heap(&mut self, x: isize, y: isize, direction: BeamDirection) {
        if x >= 0
            && x < self.statement.width as isize
            && y >= 0
            && y < self.statement.height as isize
        {
            self.heap.push(BeamState { x, y, direction });
        }
    }
}

fn parse_input() -> Statement {
    let mirror_map: Vec<_> = stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();

            line.chars()
                .map(|c| match c {
                    '/' => CellKind::MirrorUp,
                    '\\' => CellKind::MirrorDown,
                    '-' => CellKind::SplitterHorizontal,
                    '|' => CellKind::SplitterVertical,
                    _ => CellKind::Empty,
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let width = mirror_map[0].len();
    let height = mirror_map.len();
    Statement {
        mirror_map,
        width,
        height,
    }
}

fn main() {
    let statement = parse_input();
    let mut solver = Solver {
        statement: &statement,
        heap: Vec::with_capacity(100),
    };

    solver.solve_part_1();
    solver.solve_part_2();
}
