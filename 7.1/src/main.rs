use std::io::Read;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Space {
    Empty { active: bool },
    Splitter,
}

impl FromStr for Space {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "S" => Ok(Space::Empty { active: true }),
            "." => Ok(Space::Empty { active: false }),
            "^" => Ok(Space::Splitter),
            _ => Err(anyhow::anyhow!("Invalid space: {}", s)),
        }
    }
}

#[derive(Debug)]
struct Manifold {
    spaces: Vec<Vec<Space>>,
}

impl Manifold {
    fn get(&self, x: usize, y: usize) -> Option<Space> {
        self.spaces.get(y)?.get(x).copied()
    }

    fn set(&mut self, x: usize, y: usize, space: Space) {
        let slot: &mut Space = self.spaces.get_mut(y).unwrap().get_mut(x).unwrap();
        *slot = space;
    }
    
    fn get_width(&self) -> usize {
        self.spaces.first().unwrap().len()
    }
    
    fn get_height(&self) -> usize {
        self.spaces.len()
    }
}

impl FromStr for Manifold {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines();
        let mut spaces = Vec::new();
        for line in lines {
            let chars = line.chars().collect::<Vec<char>>();
            let mut inner_spaces = Vec::new();
            for char in chars {
                let space = Space::from_str(&char.to_string()).unwrap();
                inner_spaces.push(space);
            }
            spaces.push(inner_spaces);
        }

        Ok(Manifold { spaces })
    }
}

fn main() {
    let mut input = String::new();
    let _read_bytes = std::io::stdin().read_to_string(&mut input);
    let mut manifold = Manifold::from_str(&input).unwrap();
    println!("manifold: {:?}", manifold);

    let mut split_count = 0;
    for y in 1..manifold.get_height() {
        for x in 0..manifold.get_width() {
            let current_space = manifold.get(x, y);
            let above_space = manifold.get(x, y - 1);
            match (above_space, current_space) {
                (Some(Space::Empty { active: true }), Some(Space::Empty { active: _ })) => {
                    manifold.set(x, y, Space::Empty { active: true });
                }
                (Some(Space::Empty { active: true }), Some(Space::Splitter)) => {
                    // This fails if the splitter is on the edge of the manifold, or if splitters are next to each other.
                    manifold.set(x-1, y, Space::Empty { active: true });
                    manifold.set(x+1, y, Space::Empty { active: true });
                    split_count += 1;
                }
                _ => continue,
            }
        }
    }
    println!("split_count: {}", split_count);
}
