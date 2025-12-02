use anyhow::anyhow;
use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(anyhow!("Invalid direction: {}", s)),
        }
    }
}
#[derive(Debug)]
struct Turn {
    direction: Direction,
    distance: i32,
}

impl From<String> for Turn {
    fn from(s: String) -> Self {
        let mut chars = s.chars();
        let direction_string = chars.next().unwrap().to_string();
        let distance_string = chars.collect::<String>();
        let direction = direction_string.parse::<Direction>().unwrap();
        let distance = distance_string.parse::<i32>().unwrap();
        let turn = Turn { direction, distance };
        turn
    }
}

fn main() {
    let lines = get_lines();
    let mut location = 50;
    let mut zero_count = if location == 0 { 1 } else { 0 };
    for line in lines {
        let turn = Turn::from(line);
        let zero_count_delta = get_zero_count_delta(location, &turn);
        zero_count += zero_count_delta;
        location = get_new_location(location, &turn);
    }
    println!("zero_count: {}", zero_count);
}

fn get_zero_count_delta(location: i32, turn: &Turn) -> i32 {
    let count = match turn.direction {
        Direction::Left  if turn.distance < location => 0,
        Direction::Left => {
            let distance = turn.distance - location;
            let count = distance / 100;
            count + 1
        }
        Direction::Right if turn.distance < (100 - location) => 0,
        Direction::Right => {
            let distance = turn.distance - (100 - location);
            let count = distance / 100;
            count + 1
        }
    };
    count
}

fn get_new_location(location: i32, turn: &Turn) -> i32 {
    let mut new_location;
    match turn.direction {
        Direction::Left => new_location = location - turn.distance,
        Direction::Right => new_location = location + turn.distance,
    }
    new_location = new_location % 100;
    if new_location < 0 {
        new_location = 100 + new_location;
    }
    new_location
}

fn get_lines() -> Vec<String> {
    let stdin = std::io::stdin();
    let lines = stdin.lines().map(|line| line.unwrap()).collect();
    lines
}