use std::io::Read;
use std::str::FromStr;
use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq)]
enum Space {
    Empty { path_count: u64 },
    Splitter,
}

impl Debug for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Space::Splitter => write!(f, "^"),
            Space::Empty { path_count } => write!(f, "{}", path_count),
        }
    }
}

impl FromStr for Space {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "S" => Ok(Space::Empty { path_count: 1 }),
            "." => Ok(Space::Empty { path_count: 0 }),
            "^" => Ok(Space::Splitter),
            _ => Err(anyhow::anyhow!("Invalid space: {}", s)),
        }
    }
}

struct Manifold {
    spaces: Vec<Vec<Space>>,
}

impl Debug for Manifold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.spaces {
            for space in row {
                write!(f, "{:?}\t", space)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
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
    println!("manifold:\n{:?}", manifold);

    for y in 1..manifold.get_height() {
        for x in 0..manifold.get_width() {
            let current_space = manifold.get(x, y);
            let above_space = manifold.get(x, y - 1);
            match (above_space, current_space) {
                (Some(Space::Empty { path_count: above_path_count }), Some(Space::Empty { path_count })) => {
                    manifold.set(x, y, Space::Empty { path_count: above_path_count + path_count });
                }
                (Some(Space::Empty { path_count }), Some(Space::Splitter)) => {
                    // This fails if the splitter is on the edge of the manifold, or if splitters are next to each other.
                    let left_space = manifold.get(x-1, y);
                    match left_space {
                        Some(Space::Empty { path_count: left_path_count }) => {
                            manifold.set(x-1, y, Space::Empty { path_count: path_count + left_path_count });
                        }
                        other => { panic!("Unexpected space to left: {:?}", other); }
                    }
                    manifold.set(x+1, y, Space::Empty { path_count });
                }
                _ => continue,
            }
        }
    }
    println!("manifold:\n{:?}", manifold);
    let mut total_paths = 0;
    let max_y = manifold.get_height() - 1;
    for x in 0..manifold.get_width() {
        let space = manifold.get(x, max_y);
        match space {
            Some(Space::Empty { path_count }) => {
                total_paths += path_count;
            }
            _ => continue,
        }
    }
    println!("total_paths: {}", total_paths);
}
