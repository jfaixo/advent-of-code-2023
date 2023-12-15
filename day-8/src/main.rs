use gcd::Gcd;
use std::collections::HashMap;
use std::io::stdin;

#[derive(Debug)]
struct Node {
    childs: [String; 2],
}

#[derive(Default)]
struct Statement {
    instructions: Vec<usize>,
    nodes: HashMap<String, Node>,
}

fn parse_input() -> Statement {
    let mut statement = Statement::default();

    let mut lines = stdin().lines();
    let line = lines.next().unwrap().unwrap();
    statement.instructions = line.chars().map(|c| if c == 'L' { 0 } else { 1 }).collect();

    let _ = lines.next().unwrap();

    while let Some(line) = lines.next() {
        let line = line.unwrap();

        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
        let node_label = parts[0].to_string();
        let child_left_label = parts[2][1..parts[2].len() - 1].to_string();
        let child_right_label = parts[3][0..parts[3].len() - 1].to_string();
        statement.nodes.insert(
            node_label.clone(),
            Node {
                childs: [child_left_label, child_right_label],
            },
        );
    }

    statement
}

fn reach_z(statement: &Statement, start_node: String) -> usize {
    let mut current_node = start_node;
    let mut current_instruction_index = 0;
    let mut path_length = 0;

    while !current_node.ends_with("Z") {
        current_node = statement.nodes[&current_node].childs
            [statement.instructions[current_instruction_index]]
            .clone();
        current_instruction_index = (current_instruction_index + 1) % statement.instructions.len();
        path_length += 1;
    }

    path_length
}

fn solve_part_1(statement: &Statement) {
    let path_length = reach_z(&statement, "AAA".to_string());
    println!("{}", path_length);
}

fn solve_part_2(statement: &Statement) {
    let all_path_length = statement
        .nodes
        .keys()
        .filter_map(|key| {
            if key.ends_with("A") {
                Some(reach_z(statement, key.clone()))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let lcm: usize = all_path_length
        .iter()
        .copied()
        .reduce(|a, b| lcm(a, b))
        .unwrap();
    println!("{}", lcm);
}

fn main() {
    let statement = parse_input();

    solve_part_1(&statement);
    solve_part_2(&statement);
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / first.gcd(second)
}
