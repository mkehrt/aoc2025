#[derive(Debug)]
struct Bank {
    batteries: Vec<u64>,
}

impl From<String> for Bank {
    fn from(string: String) -> Self {
        let chars = string.chars().collect::<Vec<char>>();
        let batteries = chars.iter().map(|c| c.to_string().parse::<u64>().unwrap()).collect();
        Bank { batteries }
    }
}

fn main() {
    let lines = get_lines();
    let mut total = 0u64;
    for line in lines {
        let bank = Bank::from(line);
        let length = bank.batteries.len();
        let mut joltages = Vec::new();
        let mut index = 0;
        for i in 0..12 {
            let end = length - (12 - i) + 1;
            let (value, value_index) = find_largest_value_and_index_in_range(&bank.batteries, index, end);
            joltages.push(value);
            index = value_index + 1;
        }
        println!("bank: {:?}, joltages: {:?}", bank, joltages);
        let mut intermediate_total = 0u64;
        for joltage in joltages {
            intermediate_total = intermediate_total * 10u64 + joltage;
        }
        total += intermediate_total;
        println!("intermediate_total: {}", intermediate_total);
    }
    println!("total: {}", total);
}

fn find_largest_value_and_index_in_range(values: &[u64], start: usize, end: usize) -> (u64, usize) {
    let mut largest_value = 0;
    let mut index = 0;
    println!("start: {}, end: {}", start, end);
    for i in start..end {
        if values[i] > largest_value {
            largest_value = values[i];
            index = i;
        }
    }
    (largest_value, index)
}

fn get_lines() -> Vec<String> {
    let stdin = std::io::stdin();
    let lines = stdin.lines().map(|line| line.unwrap()).collect();
    lines
}