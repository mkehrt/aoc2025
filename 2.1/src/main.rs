use std::io::Read as _;

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

impl std::str::FromStr for Range {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').unwrap();
        Ok(Range { start: start.parse::<u64>()?, end: end.parse::<u64>()? })
    }
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut input = String::new();
    let _read_length: usize = stdin.read_to_string(&mut input).unwrap();
    let mut ranges = Vec::new();
    for string in input.split(',') {
        let range = string.parse::<Range>().unwrap();
        ranges.push(range);
    }
    let mut total = 0;
    for range in ranges {
        total += search_range(&range);
    }
    println!("total: {}", total);
}

 fn search_range(range: &Range) -> u64 {
    let mut total = 0;
    for i in range.start..=range.end {
        if is_invalid(i) {
            total += i;
        }
    }
    return total;
}

fn is_invalid(id: u64) -> bool {
    let mut digits = id;
    let mut total_digits = 0;
    while digits >0 {
        total_digits += 1;
        digits /= 10;
    }
    if total_digits % 2 == 1 {
        return false;
    }
    let mut divisor = 1;
    for _ in 0..(total_digits / 2) {
        divisor *= 10;   
    }

    let invalid = id / divisor == id % divisor;
    if invalid {
        println!("invalid id: {}", id);
    }
    return invalid;
}