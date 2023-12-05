use std::io::stdin;

#[derive(Default)]
struct Data {
    seeds: Vec<u128>,
    locations_maps: Vec<Vec<[u128; 3]>>,
}

fn parse_input() -> Data {
    let mut data = Data::default();

    let mut lines = stdin().lines();

    let line = lines.next().unwrap().unwrap();
    let seeds_str = line.split_ascii_whitespace().collect::<Vec<_>>();
    data.seeds = seeds_str[1..].iter().map(|&p| p.parse().unwrap()).collect();

    lines.next().unwrap().unwrap();

    let mut state = 0;
    let mut location_map = Vec::new();
    loop {
        match lines.next() {
            None => {
                break;
            }
            Some(line) => {
                match state {
                    0 => {
                        // Skip first line
                        state = 1;
                    }
                    _ => {
                        let line = line.unwrap();
                        if line.is_empty() {
                            data.locations_maps.push(location_map);
                            location_map = Vec::new();
                            state = 0;
                        } else {
                            let parts = line.split_ascii_whitespace().map(|p| p.parse::<u128>().unwrap()).collect::<Vec<_>>();
                            let mut p = [0; 3];
                            p.copy_from_slice(&parts[0..3]);
                            location_map.push(p);
                        }
                    }
                }
            }
        }
    }
    data.locations_maps.push(location_map);

    data
}

fn solve_part_1(data: &Data) {
    let mut lowest_location_number = u128::MAX;

    for &seed in &data.seeds {
        let mut current_location = seed;
        for location in &data.locations_maps {
            for r in location {
                if current_location >= r[1] && current_location < r[1] + r[2] {
                    current_location = r[0] + current_location - r[1];
                    break;
                }
            }
        }

        lowest_location_number = lowest_location_number.min(current_location);
    }

    println!("{}", lowest_location_number);
}

fn solve_part_2(data: &Data) {
    let mut lowest_location_number = u128::MAX;

    for i in (0..data.seeds.len()).step_by(2) {
        lowest_location_number = lowest_location_number.min(get_range_lowest_location(data, (data.seeds[i], data.seeds[i] + data.seeds[i + 1] - 1), 0));
    }

    println!("{}", lowest_location_number);
}

fn get_range_lowest_location(data: &Data, range: (u128, u128), location_index: usize) -> u128 {
    if location_index == data.locations_maps.len() {
        return range.0;
    }

    let mut lowest_location_number = u128::MAX;

    for r in &data.locations_maps[location_index] {
        // if the start of the range is inside the location
        if range.0 >= r[1] && range.0 < r[1] + r[2] {
            let new_location_start = r[0] + range.0 - r[1];
            let new_location_end = r[0] + range.1.min(r[1] + r[2] - 1) - r[1];
            lowest_location_number = lowest_location_number.min(get_range_lowest_location(data, (new_location_start, new_location_end), location_index + 1));

            // If the end of the range is not included in the location, iterate on the remaining part of the range
            if range.1 >= r[1] + r[2] {
                lowest_location_number = lowest_location_number.min(get_range_lowest_location(data, (r[1] + r[2], range.1), location_index));
            }
            return lowest_location_number;
        } else if range.1 >= r[1] && range.1 < r[1] + r[2] {
            // Now the start of the range is not included inside the location, but the end is
            let new_location_start = r[0];
            let new_location_end = r[0] + range.1 - r[1];
            lowest_location_number = lowest_location_number.min(get_range_lowest_location(data, (new_location_start, new_location_end), location_index + 1));
            // Iterate over the starting part of the range that is not in the location
            lowest_location_number = lowest_location_number.min(get_range_lowest_location(data, (range.0, r[1] - 1), location_index));

            return lowest_location_number;
        }
        else if range.0 < r[1] && range.1 > r[1] + r[2] {
            // If the range fully overlaps the location, split and iterate
            lowest_location_number = lowest_location_number.min(get_range_lowest_location(data, (range.0, r[1] - 1), location_index));
            lowest_location_number = lowest_location_number.min(get_range_lowest_location(data, (r[1] , range.1), location_index));

            return lowest_location_number;
        }
    }

    return get_range_lowest_location(data, range, location_index + 1);
}

fn main() {
    let data = parse_input();

    solve_part_1(&data);
    solve_part_2(&data);
}