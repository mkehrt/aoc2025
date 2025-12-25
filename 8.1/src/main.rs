use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
struct Position {
    x: usize,
    y: usize,
    z: usize,
}

impl Position {
    fn distance(&self, other: &Position) -> f64 {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        let dz = self.z.abs_diff(other.z);

        let xx = dx * dx;
        let yy = dy * dy;
        let zz = dz * dz;

        let dd = (xx + yy + zz) as f64;
        dd.sqrt()
    }
}

impl FromStr for Position {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts.next().unwrap().parse::<usize>().unwrap();
        let y = parts.next().unwrap().parse::<usize>().unwrap();
        let z = parts.next().unwrap().parse::<usize>().unwrap();
        Ok(Self { x, y, z })
    }
}

mod union_find {
    use std::fmt::Debug;

    #[derive(Clone, Debug, PartialEq)]
    pub struct UnionFindElement<T>
    where
        T: Debug + Clone,
    {
        head_index: usize,
        pub dominated_size: u64,
        pub data: T,
    }

    impl<T> UnionFindElement<T>
    where
        T: Debug + Clone,
    {
        fn new(head_index: usize, data: T) -> Self {
            Self {
                head_index,
                dominated_size: 1,
                data,
            }
        }
    }

    #[derive(Clone, PartialEq)]
    pub struct UnionFind<T>
    where
        T: Debug + Clone,
    {
        elements: Vec<UnionFindElement<T>>,
    }

    impl<T> Debug for UnionFind<T>
    where
        T: Debug + Clone,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Union {{\n")?;
            for (index, element) in self.elements.iter().enumerate() {
                write!(f, "  {}: {:?}\n", index, element)?;
            }
            write!(f, "}}")?;
            Ok(())
        }
    }

    impl<T> UnionFind<T>
    where
        T: Debug + Clone,
    {
        pub fn new(data: Vec<T>) -> Self {
            let mut elements = Vec::new();
            let mut index = 0;
            for datum in data {
                let element = UnionFindElement::new(index, datum);
                elements.push(element);
                index += 1;
            }
            Self { elements }
        }

        pub fn size(&self) -> usize {
            self.elements.len()
        }

        pub fn get(&self, index: usize) -> Option<&UnionFindElement<T>> {
            self.elements.get(index)
        }

        pub fn find(&self, index: usize) -> usize {
            let mut current_index = index;
            while self.elements.get(current_index).unwrap().head_index != current_index {
                current_index = self.elements.get(current_index).unwrap().head_index;
            }
            current_index
        }

        pub fn union(&mut self, index1: usize, index2: usize) {
            let head1 = self.find(index1);
            let head2 = self.find(index2);
            if head1 == head2 {
                panic!(
                    "Elements are already in the same union: {} and {} in union {:?}",
                    index1, index2, head1
                );
            }

            self.elements.get_mut(head2).unwrap().head_index = head1;
            self.elements.get_mut(head1).unwrap().dominated_size +=
                self.elements.get(head2).unwrap().dominated_size;
        }

        pub fn sorted_heads(&self) -> Vec<UnionFindElement<T>> {
            let mut elements = self
                .elements
                .clone()
                .into_iter()
                .enumerate()
                .filter(|(index, element)| element.head_index == *index)
                .map(|(_index, element)| element)
                .collect::<Vec<UnionFindElement<T>>>();
            elements.sort_by(|a, b| a.dominated_size.cmp(&b.dominated_size));
            elements.reverse();
            elements
        }
    }
}

fn main() {
    let lines = std::io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let mut positions = Vec::new();
    let iterations = if lines.len() <= 20 { 10 } else { 1000 };
    for line in lines {
        let position = Position::from_str(&line).unwrap();
        positions.push(position);
    }
    let mut union = union_find::UnionFind::new(positions);
    println!("union find: {:?}", union);

    let mut min_distance = 0.0;
    for _ in 0..iterations {
        min_distance = union_closest_boxes(&mut union, min_distance);
        println!("union find: {:?}", union);
    }
    let sorted_heads = union.sorted_heads();
    println!("sorted heads:");
    let mut index = 0;
    for head in sorted_heads.iter() {
        println!("{} -> {:?}", index, head);
        index += 1;
    }
    let mut product = 1;
    for i in 0..3 {
        product *= sorted_heads.get(i).unwrap().dominated_size;
    }
    println!("product: {}", product);
}

fn union_closest_boxes(union: &mut union_find::UnionFind<Position>, min_distance: f64) -> f64{
    let mut closest_distance = None;
    let mut closest_index1 = None;
    let mut closest_index2 = None;

    for i in 0..union.size() {
        for j in i + 1..union.size() {
            let element1 = union.get(i).unwrap().clone();
            let element2 = union.get(j).unwrap().clone();

            let position1 = element1.data;
            let position2 = element2.data;
            match closest_distance {
                None => {
                    closest_distance = Some(position1.distance(&position2));
                    closest_index1 = Some(i);
                    closest_index2 = Some(j);
                }
                Some(old_distance) => {
                    let distance = position1.distance(&position2);
                    if distance <= min_distance {
                        continue;
                    }
                    if distance < old_distance {
                        closest_distance = Some(distance);
                        closest_index1 = Some(i);
                        closest_index2 = Some(j);
                    }
                }
            }
        }
    }

    let head1 = union.find(closest_index1.unwrap());
    let head2 = union.find(closest_index2.unwrap());
    if head1 != head2 {
        union.union(closest_index1.unwrap(), closest_index2.unwrap());
    }

    closest_distance.unwrap()
}
