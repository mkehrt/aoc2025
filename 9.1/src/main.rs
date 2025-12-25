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
            let area = positions[i].area(&positions[j]);
            if area > greatest_area {
                greatest_area = area;
            }
        }
    }
    println!("greatest area: {}", greatest_area);
}
