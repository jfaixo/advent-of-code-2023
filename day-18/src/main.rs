use std::io::stdin;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    direction: Direction,
    distance: isize,
    color: usize,
}

fn parse_input() -> Vec<Instruction> {
    stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let parts = line.split_ascii_whitespace().collect::<Vec<_>>();

            let direction = match parts[0] {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                _ => Direction::Right,
            };

            Instruction {
                direction,
                distance: parts[1].parse().unwrap(),
                color: usize::from_str_radix(&parts[2][2..8], 16).unwrap(),
            }
        })
        .collect()
}

fn solve_part_1(instructions: &Vec<Instruction>) {
    compute_area(instructions);
}

fn solve_part_2(instructions: &Vec<Instruction>) {
    // Decode instructions
    let mut instructions_new = instructions.clone();
    for instruction in instructions_new.iter_mut() {
        instruction.direction = match instruction.color & 0xF {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            _ => Direction::Up,
        };
        instruction.distance = instruction.color as isize >> 4;
    }

    compute_area(&instructions_new);
}

fn compute_area(instructions: &Vec<Instruction>) {
    let mut current_position = (0, 0);
    let mut previous_position = (0, 0);

    let mut area: isize = 0;
    let mut border_length = 0;
    for instruction in instructions.iter() {
        match instruction.direction {
            Direction::Up => current_position.1 -= instruction.distance,
            Direction::Down => current_position.1 += instruction.distance,
            Direction::Left => current_position.0 -= instruction.distance,
            Direction::Right => current_position.0 += instruction.distance,
        }
        border_length += instruction.distance;

        area += previous_position.0 * current_position.1 - previous_position.1 * current_position.0;

        previous_position = current_position;
    }

    println!("{}", area / 2 + border_length / 2 + 1);
}

fn main() {
    let ref instructions = parse_input();

    solve_part_1(instructions);
    solve_part_2(instructions);
}

fn solve_part_1_first_edition(instructions: &Vec<Instruction>) {
    let mut border: Vec<(isize, isize)> = Vec::new();

    let mut current_position = (0, 0);
    let mut bounding_box = (0, 0, 0, 0);
    border.push((0, 0));
    for instruction in instructions {
        for d in 0..instruction.distance {
            match instruction.direction {
                Direction::Up => current_position.1 -= 1,
                Direction::Down => current_position.1 += 1,
                Direction::Left => current_position.0 -= 1,
                Direction::Right => current_position.0 += 1,
            }
            border.push(current_position);
            bounding_box.0 = bounding_box.0.min(current_position.0);
            bounding_box.1 = bounding_box.1.max(current_position.0);
            bounding_box.2 = bounding_box.2.min(current_position.1);
            bounding_box.3 = bounding_box.3.max(current_position.1);
        }
    }

    // Floodfill
    let width = (bounding_box.1 - bounding_box.0 + 3) as usize;
    let height = (bounding_box.3 - bounding_box.2 + 3) as usize;
    let mut map = vec![0; width * height];
    // Draw the border
    for (x, y) in &border {
        map[(y - bounding_box.2 + 1) as usize * width + (x - bounding_box.0 + 1) as usize] = 1;
    }

    // Floodfill
    let mut heap = Vec::new();
    for y in 0..height {
        heap.push((0, y));
        heap.push((width - 1, y));
    }
    for x in 0..width {
        heap.push((x, 0));
        heap.push((x, height - 1));
    }

    while let Some((x, y)) = heap.pop() {
        if map[y * width + x] == 0 {
            map[y * width + x] = 2;

            if y > 0 {
                heap.push((x, y - 1));
            }
            if y < height - 2 {
                heap.push((x, y + 1));
            }
            if x > 0 {
                heap.push((x - 1, y));
            }
            if x < width - 2 {
                heap.push((x + 1, y));
            }
        }
    }

    // Turn the rest to 1's
    for i in 0..map.len() {
        if map[i] == 0 {
            map[i] = 1;
        }
    }

    println!("{}", map.iter().filter(|&v| *v == 1).count());
}
