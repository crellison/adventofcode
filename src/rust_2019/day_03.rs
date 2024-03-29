use crate::utils::{get_input, InputType};
use std::collections::HashMap;
use std::io;

pub fn main() -> io::Result<()> {
    println!(
        "part one: {}",
        part_one(&get_input(2019, 3, InputType::Challenge, 0)?)
    );
    println!(
        "part two: {}",
        part_two(&get_input(2019, 3, InputType::Challenge, 0)?)
    );
    Ok(())
}

#[allow(dead_code)]
fn part_one(input: &str) -> i32 {
    let mut grid: HashMap<(i32, i32), bool> = HashMap::new();
    for line in input.lines() {
        let mut position = (0, 0);
        for command in line.split(",") {
            let dir = parse_dir(&command[0..1]);
            let dist: &i32 = &command[1..].parse().unwrap();
            for _ in 0..*dist {
                position.0 += dir.0;
                position.1 += dir.1;
                if grid.contains_key(&position) {
                    grid.insert(position, true);
                } else {
                    grid.insert(position, false);
                }
            }
        }
    }
    grid.retain(|_k, v| *v == true);
    let min_dist = grid.keys().map(|(x, y)| x.abs() + y.abs()).min().unwrap();
    min_dist
}

fn parse_dir(direction: &str) -> (i32, i32) {
    match direction {
        "R" => (1, 0),
        "L" => (-1, 0),
        "U" => (0, 1),
        "D" => (0, -1),
        _ => panic!("Unexpected direction: {}", direction),
    }
}

#[allow(dead_code)]
fn part_two(input: &str) -> i32 {
    let mut grid: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut iter_one = true;
    for line in input.lines() {
        let mut position = (0, 0);
        let mut steps = 0;
        for command in line.split(",") {
            let dir = parse_dir(&command[0..1]);
            let dist: &i32 = &command[1..].parse().unwrap();
            for _ in 0..*dist {
                steps += 1;
                position.0 += dir.0;
                position.1 += dir.1;
                if let Some(val) = grid.get_mut(&position) {
                    if !iter_one && val.1 == 0 {
                        *val = (val.0, steps);
                    }
                } else if iter_one {
                    // we don't care if we find a new space on the second wire
                    grid.insert(position, (steps, 0));
                }
            }
        }
        iter_one = false;
    }
    grid.retain(|_k, v| v.1 != 0);
    let min_dist = grid.values().map(|(x, y)| x + y).min().unwrap();
    min_dist
}
