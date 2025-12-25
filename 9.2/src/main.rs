use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn area(&self, &other: &Position) -> usize {
        let width = self.x.abs_diff(other.x) + 1;
        let height = self.y.abs_diff(other.y) + 1;
        width * height
    }
}

impl FromStr for Position {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts.next().unwrap().parse::<usize>().unwrap();
        let y = parts.next().unwrap().parse::<usize>().unwrap();
        Ok(Self { x, y})
    }
}

fn main() {
    let lines = std::io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let mut positions = Vec::new();
    for line in lines {
        let position = Position::from_str(&line).unwrap();
        positions.push(position);
    }
    println!("positions: {:?}", positions);
    
    let mut greatest_area = 0;
    for i in 0..positions.len() {
        for j in i + 1..positions.len() {
            if !is_valid(&positions, &positions[i], &positions[j]) {
                continue;
            }
            let area = positions[i].area(&positions[j]);
            if area > greatest_area {
                greatest_area = area;
            }
        }
    }
    println!("greatest area: {}", greatest_area);
}

fn is_valid(positions: &Vec<Position>, corner1: &Position, corner2: &Position) -> bool {
    let length = positions.len();
    for i in 0..length {
        let position1 = positions[i];
        let position2;
        if i == length - 1 {
            position2 = positions[0];
        } else {
            position2 = positions[i + 1];
        }

        let line_max_x = position1.x.max(position2.x);
        let line_min_x = position1.x.min(position2.x);
        let line_max_y = position1.y.max(position2.y);
        let line_min_y = position1.y.min(position2.y);
        
        let rect_max_x = corner1.x.max(corner2.x);
        let rect_min_x = corner1.x.min(corner2.x);
        let rect_max_y = corner1.y.max(corner2.y);
        let rect_min_y = corner1.y.min(corner2.y);

        if line_max_x <= rect_min_x {
            continue;
        }
        if line_min_x >= rect_max_x {
            continue;
        }
        if line_max_y <= rect_min_y {
            continue;
        }
        if line_min_y >= rect_max_y {
            continue;
        }
        return false;
    }
    return true;
}