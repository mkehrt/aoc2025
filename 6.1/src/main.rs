use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Operator {
    Plus,
    Times,
}

impl FromStr for Operator {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Plus),
            "*" => Ok(Operator::Times),
            _ => Err(anyhow::anyhow!("Invalid op: {}", s)),
        }
    }
}

#[derive(Debug)]
struct Problem {
    op: Operator,
    operands: Vec<u64>,
}

impl Problem {
    fn solve(&self) -> u64 {
        match self.op {
            Operator::Plus => self.operands.iter().sum(),
            Operator::Times => self.operands.iter().product(),
        }
    }
}

#[derive(Debug)]
struct Homework {
    problems: Vec<Problem>,
}

impl Homework {
    fn new_from_parts(untransposed_operands: Vec<Vec<u64>>, operators: Vec<Operator>) -> Self {
        let width = untransposed_operands.first().unwrap().len();
        let height = untransposed_operands.len();
        let mut problems = Vec::new();
        for x in 0..width {
            let mut operands = Vec::new();
            for y in 0..height {
                let operand = untransposed_operands[y][x];
                operands.push(operand);
            }
            let op = operators[x];
            problems.push(Problem { op, operands });
        }
        Self { problems }
    }

    fn solve_and_sum(&self) -> u64 {
        let mut sum = 0;
        for problem in &self.problems {
            sum += problem.solve();
        }
        sum
    }
}

fn main() {
    let lines = std::io::stdin().lines().map(|line| line.unwrap());
    let (untransposed_operands, operators) = get_parts_from_lines(lines);
    let homework = Homework::new_from_parts(untransposed_operands, operators);
    println!("homework: {:?}", homework);
    let sum = homework.solve_and_sum();
    println!("sum: {}", sum);
}

fn get_parts_from_lines(mut lines: impl Iterator<Item = String>) -> (Vec<Vec<u64>>, Vec<Operator>) {
    let mut untransposed_operands = Vec::new();
    let mut operators_line = String::new();
    'operands: while let Some(line) = lines.next() {
        let words = line.split_whitespace().collect::<Vec<&str>>();
        if let Err(_) = u64::from_str(words.first().unwrap()) {
            operators_line = line;
            break 'operands;
        }
        let operands = words.iter().map(|word| u64::from_str(word).unwrap()).collect::<Vec<u64>>();
        untransposed_operands.push(operands);
    }
    
    let operator_strings = operators_line.split_whitespace().collect::<Vec<&str>>(); 
    let operators = operator_strings.iter().map(|string| Operator::from_str(string).unwrap()).collect::<Vec<Operator>>();
    (untransposed_operands, operators)
}