use std::io::stdin;
use std::time::Instant;

fn parse_input() -> Vec<Vec<u8>> {
    stdin()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| if c == '#' { 1 } else { 0 })
                .collect()
        })
        .collect()
}

fn solve_part_1(galaxy: &Vec<Vec<u8>>) {
    let mut coords = Vec::new();
    let mut galaxy_height = galaxy.len();
    let mut galaxy_width = galaxy[0].len();
    for y in 0..galaxy_height {
        for x in 0..galaxy_width {
            if galaxy[y][x] != 0 {
                coords.push((x as isize, y as isize));
            }
        }
    }

    // Expand x
    let mut x = 0;
    while x < galaxy_width as isize {
        if coords.iter().filter(|(x_c, _)| *x_c == x).count() == 0 {
            coords.iter_mut().for_each(|(x_c, _)| {
                if *x_c > x {
                    *x_c += 1;
                }
            });
            galaxy_width += 1;
            x += 1;
        }
        x += 1;
    }

    // Expand y
    let mut y = 0;
    while y < galaxy_height as isize {
        if coords.iter().filter(|(_, y_c)| *y_c == y).count() == 0 {
            coords.iter_mut().for_each(|(_, y_c)| {
                if *y_c > y {
                    *y_c += 1;
                }
            });
            galaxy_height += 1;
            y += 1;
        }
        y += 1;
    }

    // Find closest of each pair
    let mut sum = 0;
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            sum += (coords[i].0 - coords[j].0).abs() + (coords[i].1 - coords[j].1).abs();
        }
    }

    println!("{}", sum);
}

fn solve_part_2(galaxy: &Vec<Vec<u8>>, expansion: isize) {
    let mut coords = Vec::new();
    let mut galaxy_height = galaxy.len() as isize;
    let mut galaxy_width = galaxy[0].len() as isize;
    for y in 0..galaxy_height as usize {
        for x in 0..galaxy_width as usize {
            if galaxy[y][x] != 0 {
                coords.push((x as isize, y as isize));
            }
        }
    }

    // Expand x
    let mut x = 0;
    while x < galaxy_width {
        if coords.iter().filter(|(x_c, _)| *x_c == x).count() == 0 {
            coords.iter_mut().for_each(|(x_c, _)| {
                if *x_c > x {
                    *x_c += expansion;
                }
            });
            galaxy_width += expansion;
            x += expansion;
        }
        x += 1;
    }

    // Expand y
    let mut y = 0;
    while y < galaxy_height {
        if coords.iter().filter(|(_, y_c)| *y_c == y).count() == 0 {
            coords.iter_mut().for_each(|(_, y_c)| {
                if *y_c > y {
                    *y_c += expansion;
                }
            });
            galaxy_height += expansion;
            y += expansion;
        }
        y += 1;
    }

    // Find closest of each pair
    let mut sum = 0;
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            sum += (coords[i].0 - coords[j].0).abs() + (coords[i].1 - coords[j].1).abs();
        }
    }

    println!("{}", sum);
}
fn main() {
    let mut start_time = Instant::now();
    let galaxy = parse_input();

    solve_part_1(&galaxy);
    solve_part_2(&galaxy, 999_999);

    eprintln!("{} µs", (Instant::now() - start_time).as_micros());
}
