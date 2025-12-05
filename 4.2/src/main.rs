use std::io::Read;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Product {
    Paper,
    Nothing,
}

impl FromStr for Product {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "@" => Ok(Product::Paper),
            "." => Ok(Product::Nothing),
            _ => Err(anyhow::anyhow!("Invalid product: {}", s)),
        }
    }
}

#[derive(Debug)]
struct Wall {
    products: Vec<Vec<Product>>,
}

impl Wall {
    fn get(&self, x: usize, y: usize) -> Option<Product> {
        self.products.get(y)?.get(x).copied()
    }

    fn set(&mut self, x: usize, y: usize, product: Product) {
        let slot: &mut Product = self.products.get_mut(y).unwrap().get_mut(x).unwrap();
        *slot = product;
    }
    
    fn get_width(&self) -> usize {
        self.products.first().unwrap().len()
    }
    
    fn get_height(&self) -> usize {
        self.products.len()
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<Option<Product>> {
        let mut neighbors = Vec::new();
        for i in vec![-1i32, 0i32, 1i32] {
            for j in vec![-1i32, 0i32, 1i32] {
                if i == 0i32 && j == 0i32 {
                    continue;
                }
                let neighbor = self.get((x as i32 + i) as usize, (y as i32 + j) as usize);
                neighbors.push(neighbor);
            }
        }
        neighbors
    }

    fn count_neighbors(&self, x: usize, y: usize) -> usize {
        let neighbors = self.get_neighbors(x, y);
        neighbors.iter().filter(|n| n.is_some() && **n != Some(Product::Nothing)).count()
    }

    fn find_removable(&mut self) -> Option<(usize, usize)> {
        for i in 0..self.get_width() {
            for j in 0..self.get_height() {
                if self.get(i, j) == Some(Product::Paper) && self.count_neighbors(i, j) < 4 {
                    return Some((i, j));
                }
            }
        }
        None
    }
}

impl FromStr for Wall {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines();
        let mut products = Vec::new();
        for line in lines {
            let chars = line.chars().collect::<Vec<char>>();
            let mut inner_products = Vec::new();
            for char in chars {
                let product = Product::from_str(&char.to_string()).unwrap();
                inner_products.push(product);
            }
            products.push(inner_products);
        }

        Ok(Wall { products })
    }
}

fn main() {
    let mut input = String::new();
    let _read_bytes = std::io::stdin().read_to_string(&mut input);
    let mut wall = Wall::from_str(&input).unwrap();
    println!("wall: {:?}", wall);

    let mut removable_count = 0;
    loop {
        let removable = wall.find_removable();
        if removable.is_none() {
            break;
        }
        removable_count += 1;
        let (x, y) = removable.unwrap();
        wall.set(x, y, Product::Nothing);
    }
    println!("removable_count: {}", removable_count);
}
