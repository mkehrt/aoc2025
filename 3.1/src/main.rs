#[derive(Debug)]
struct Bank {
    batteries: Vec<i32>,
}

impl From<String> for Bank {
    fn from(string: String) -> Self {
        let chars = string.chars().collect::<Vec<char>>();
        let batteries = chars.iter().map(|c| c.to_string().parse::<i32>().unwrap()).collect();
        Bank { batteries }
    }
}

fn main() {
    let lines = get_lines();
    let mut total = 0;
    for line in lines {
        let bank = Bank::from(line);
        let (tens, index) = find_largest_value_and_index_before_last(&bank.batteries);
        let ones = find_largest_value_after_index(&bank.batteries, index);
        println!("bank: {:?}, tens: {}, ones: {}", bank, tens, ones);
        total += (10 * tens) + ones;
    }
    println!("total: {}", total);
}

fn find_largest_value_and_index_before_last(batteries: &[i32]) -> (i32, usize) {
    let mut largest_value = 0;
    let mut index = 0;
    let all_but_last = batteries[0..batteries.len() - 1].to_vec();
    for i in 0..all_but_last.len() {
        if all_but_last[i] > largest_value {
            largest_value = all_but_last[i];
            index = i;
        }
    }
    (largest_value, index)
}

fn find_largest_value_after_index(batteries: &[i32], index: usize) -> i32 {
    let mut largest_value = 0;
    for i in index + 1..batteries.len() {
        if batteries[i] > largest_value {
            largest_value = batteries[i];
        }
    }
    largest_value
}

fn get_lines() -> Vec<String> {
    let stdin = std::io::stdin();
    let lines = stdin.lines().map(|line| line.unwrap()).collect();
    lines
}