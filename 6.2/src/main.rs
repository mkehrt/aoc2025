use anyhow::anyhow;
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Multiply,
}

impl FromStr for Operator {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Add),
            "*" => Ok(Operator::Multiply),
            _ => Err(anyhow!("Invalid operation: {}", s)),
        }
    }
}

#[derive(Clone, Debug)]
struct Problem {
    operands: Vec<u64>,
    operator: Operator,
}

impl Problem {
    fn solve(&self) -> u64 {
        match self.operator {
            Operator::Add => self.operands.iter().sum(),
            Operator::Multiply => self.operands.iter().product(),
        }
    }
}

fn main() {
    let lines = std::io::stdin().lines().map(|line| line.unwrap()).collect::<Vec<String>>();
    let chars = lines.into_iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let height = chars.len();
    let width = chars.get(0).unwrap().len();
    let mut operands = Vec::new();
    let mut operandss = Vec::new();
    for x in 0..width {
        let mut operand = 0;
        for y in 0..height - 1 {
            let char = chars.get(y).unwrap().get(x).unwrap();
            let digit;
            if *char == ' ' {
                digit = None
            } else {
                digit = Some(u64::from_str(&char.to_string()).unwrap());
            }
            if let Some(digit) = digit {
                operand = operand * 10 + digit;
            }
        }
        if operand != 0 {
            operands.push(operand);
        } else {
            operandss.push(operands);
            operands = Vec::new();
        }
    }
    operandss.push(operands);
    println!("operandss: {:?}", operandss);
    let mut operators = Vec::new();
    let width = chars.get(height - 1).unwrap().len();
    for x in 0..width {
        let char = chars[height - 1][x];
        println!("char: {}", char);
        if let Ok(operator) = Operator::from_str(&char.to_string()) {
            operators.push(operator);
        }
    }
    println!("operators: {:?}", operators);
    let mut problems = Vec::new();
    for i in 0..operandss.len() {
        let operands = operandss.get(i).unwrap().clone();
        let operator = operators.get(i).unwrap().clone();
        let problem = Problem { operands, operator };
        problems.push(problem);
    }
    println!("problems: {:?}", problems);
    let mut total = 0;
    for problem in problems {
        total += problem.solve();
    }
    println!("total: {}", total);
}
