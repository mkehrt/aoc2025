use std::str::FromStr;
use std::cmp::{Ord, Ordering, PartialOrd, PartialEq, Eq};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

impl Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.start.cmp(&other.start))
    }
}

impl Range {
    fn does_intersect(&self, other: &Range) -> bool {
        (self.start <= other.start && self.end >= other.start) || 
        (self.start <= other.end && self.end >= other.end)
    }

    fn join(&self, other: &Range) -> Range {
        Range {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}

fn main() {
    let mut lines = get_lines();
    let mut ranges = get_ranges(&mut lines);
    ranges.sort();
    let joined_ranges = join_ranges(&ranges);
    println!("ranges: {:?}", ranges);
    println!("joined ranges: {:?}", joined_ranges);
    let mut total_length = 0;
    for range in joined_ranges {
        total_length += range.end - range.start + 1;
    }
    println!("total length: {}", total_length);
}

fn join_ranges(ranges: &Vec<Range>) -> Vec<Range> {
    let mut joined_ranges = Vec::new();
    let mut current_range_option: Option<Range> = None;

    for range in ranges.iter() {
        match current_range_option {
            Some(current_range) if current_range.does_intersect(range) => {
                println!("joining ranges: {:?} and {:?}", current_range, range);
                current_range_option = Some(current_range.join(range));
            }
            Some(current_range) => {
                println!("NOT joining ranges: {:?} and {:?}", current_range, range);
                joined_ranges.push(current_range);
                current_range_option = Some(*range);
            }
            None => {
                current_range_option = Some(*range);
            }
        }
    }
    
    if let Some(current_range) = current_range_option {
        joined_ranges.push(current_range);
    }

    joined_ranges
}

fn get_ranges(lines: &mut impl Iterator<Item = String>) -> Vec<Range> {
    let mut ranges = Vec::new();
    for line in lines {
        let range_result = Range::from_str(&line);
        match range_result {
            Ok(range) => ranges.push(range),
            Err(_) => return ranges
        }
    }
    panic!("Oops all ranges!");
}

fn get_lines() -> impl Iterator<Item = String> {
    let stdin = std::io::stdin();
    let lines = stdin.lines().map(|line| line.unwrap());
    lines
}