use anyhow::anyhow;
use std::str::FromStr;

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

impl FromStr for Range {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let strs = s.split("-").collect::<Vec<&str>>();
        let start = strs[0].parse::<u64>()?;
        let end = strs[1].parse::<u64>()?;
        Ok(Range { start, end })
    }
}

impl Range {
    fn contains(&self, value: u64) -> bool {
        value >= self.start && value <= self.end
    }
}

fn main() {
    let mut lines = get_lines();
    let ranges = get_ranges(&mut lines);
    let values = get_values(&mut lines);
    println!("ranges: {:?}", ranges);
    println!("values: {:?}", values);
    let mut fresh = 0;
    for value in values {
        if is_contained(value, &ranges) {
            fresh += 1;
        }
    }
    println!("fresh: {}", fresh);
}

fn get_ranges(lines: &mut impl Iterator<Item = String>) -> Vec<Range> {
    let mut ranges = Vec::new();
    for line in lines {
        let range_result = Range::from_str(&line);
        match range_result {
            Ok(range) => ranges.push(range),
            Err(e) => return ranges
        }
    }
    panic!("Oops all ranges!");
}

fn get_values(lines: impl Iterator<Item = String>) -> Vec<u64> {
    let mut values = Vec::new();
    for line in lines {
        let value_result = u64::from_str(&line);
        match value_result {
            Ok(value) => values.push(value),
            Err(_) => return values
        }
    }
    values
}

fn is_contained(value: u64, ranges: &Vec<Range>) -> bool {
    for range in ranges {
        if range.contains(value) {
            return true;
        }
    }
    false
}

fn get_lines() -> impl Iterator<Item = String> {
    let stdin = std::io::stdin();
    let lines = stdin.lines().map(|line| line.unwrap());
    lines
}