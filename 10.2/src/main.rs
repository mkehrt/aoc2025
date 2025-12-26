use std::str::FromStr;
use std::collections::HashSet;
use std::time::Instant;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Indicator {
    On,
    Off,
}

impl FromStr for Indicator {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(Indicator::On),
            "." => Ok(Indicator::Off),
            _ => Err(anyhow::anyhow!("Invalid indicator: {}", s)),
        }
    }
}

impl Indicator {
    fn toggle(&self) -> Self {
        match self {
            Indicator::On => Indicator::Off,
            Indicator::Off => Indicator::On,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Indicators {
    pub indicators: Vec<Indicator>,
}

impl FromStr for Indicators {
    type Err = anyhow::Error;
    fn from_str(delimed_str: &str) -> Result<Self, Self::Err> {
        let str = &delimed_str[1..delimed_str.len() - 1];
        let chars = str.chars().map(|c| c.to_string()).collect::<Vec<String>>();
        let mut indicators = Vec::new();
        for char in chars {
            let indicator = Indicator::from_str(&char)?;
            indicators.push(indicator);
        }
        Ok(Indicators { indicators })
    }
}

impl Indicators {
    fn apply_buttons(&self, buttons: &Buttons) -> Self {
        let mut indicators = self.indicators.clone();
        for button in &buttons.buttons {
            let index = *button as usize;
            indicators[index] = indicators[index].toggle();
        }
        Indicators { indicators }
    }

    fn len(&self) -> usize {
        self.indicators.len()
    }
}

#[derive(Clone, Debug)]
struct Buttons {
    pub buttons: Vec<u64>,
}

impl FromStr for Buttons {
    type Err = anyhow::Error;
    fn from_str(delimed_str: &str) -> Result<Self, Self::Err> {
        let str = &delimed_str[1..delimed_str.len() - 1];
        let parts = str.split(",").collect::<Vec<&str>>();
        let mut buttons = Vec::new();
        for part in parts {
            let button = part.parse::<u64>()?;
            buttons.push(button);
        }
        Ok(Buttons { buttons })
    }
}


#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Joltages {
    pub joltages: Vec<u64>,
}

impl FromStr for Joltages {
    type Err = anyhow::Error;
    fn from_str(delimed_str: &str) -> Result<Self, Self::Err> {
        let str = &delimed_str[1..delimed_str.len() - 1];
        let parts = str.split(",").collect::<Vec<&str>>();
        let mut joltages = Vec::new();
        for part in parts {
            let joltage = part.parse::<u64>()?;
            joltages.push(joltage);
        }
        Ok(Joltages { joltages })
    }
}

impl Joltages {
    fn apply_buttons(&self, buttons: &Buttons) -> Self {
        let mut joltages = self.joltages.clone();
        for button in &buttons.buttons {
            let index = *button as usize;
            joltages[index] = joltages[index] + 1;
        }
        Joltages { joltages }
    }

    fn len(&self) -> usize {
        self.joltages.len()
    }
}

#[derive(Clone, Debug)]
struct Machine {
    indicators: Indicators,
    buttonses: Vec<Buttons>,
    joltages: Joltages,
}

impl FromStr for Machine {
    type Err = anyhow::Error;
    fn from_str(delimed_str: &str) -> Result<Self, Self::Err> {
        let parts = delimed_str.split(" ").collect::<Vec<&str>>();
        let indicators = Indicators::from_str(parts[0])?;
        let mut buttonses = Vec::new();
        for part in &parts[1..parts.len() - 1] {
            let buttons = Buttons::from_str(part)?;
            buttonses.push(buttons);
        }
        let joltages = Joltages::from_str(parts[parts.len() - 1])?;
        Ok(Machine { indicators, buttonses, joltages })
    }
}

fn steps_to_desired_state(machine: &Machine) -> u64 {
    let indicator_count = machine.indicators.len();
    let mut indicators_vec = Vec::new();
    for _ in 0..indicator_count {
        indicators_vec.push(Indicator::Off);
    }
    let indicators = Indicators { indicators: indicators_vec };
    if indicators == machine.indicators {
        return 0;
    }
    let mut current_indicatorses = vec![indicators];
    let mut step_count = 0;
    loop {
        step_count += 1;
        let mut next_indicatorses = Vec::new();
        for indicators in current_indicatorses {
            for buttons in &machine.buttonses {
                let next_indicators = indicators.apply_buttons(&buttons);
                if next_indicators == machine.indicators {
                    return step_count;
                }
                next_indicatorses.push(next_indicators);
            }
        }
        println!("step count: {}, size: {}", step_count, next_indicatorses.len());
        current_indicatorses = next_indicatorses;
    }
}

fn total_steps_to_desired_state(machines: &[Machine]) -> u64 {
    let mut total_steps = 0;
    for machine in machines {
        total_steps += steps_to_desired_state(machine);
    }
    total_steps
}

fn steps_to_desired_joltage(machine_index: usize, machine: &Machine) -> u64 {
    let joltage_count = machine.joltages.len();
    let mut joltages_vec = Vec::new();
    for _ in 0..joltage_count {
        joltages_vec.push(0);
    }
    let joltages = Joltages { joltages: joltages_vec };
    if joltages == machine.joltages {
        return 0;
    }
    let max_joltage = machine.joltages.joltages.iter().max().unwrap();
    let mut current_joltageses = HashSet::new();
    current_joltageses.insert(joltages);
    let mut step_count = 0;
    loop {
        let start_time = Instant::now();
        step_count += 1;
        let mut next_joltageses = HashSet::new();
        'joltages_loop: for joltages in current_joltageses {
            for buttons in &machine.buttonses {
                let next_joltages = joltages.apply_buttons(&buttons);
                if next_joltages == machine.joltages {
                    return step_count;
                }
                for j in 0..joltage_count {
                    if next_joltages.joltages[j] > machine.joltages.joltages[j] {
                        continue 'joltages_loop;
                    }
                }
                next_joltageses.insert(next_joltages);
            }
        }
        let duration = start_time.elapsed();
        println!("{:?} (max joltage: {})/{}: step count: {}, size: {}", duration, max_joltage, machine_index, step_count, next_joltageses.len());
        current_joltageses = next_joltageses;
    }
}

fn total_steps_to_desired_joltages(machines: &[Machine]) -> u64 {
    let mut total_steps = 0;
    for (machine_index, machine) in machines.iter().enumerate() {
        total_steps += steps_to_desired_joltage(machine_index, machine);
    }
    total_steps
}

fn main() {
    let lines = std::io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let mut machines = Vec::new();
    for line in lines {
        let machine = Machine::from_str(&line).unwrap();
        machines.push(machine);
    }
    println!("machines: {:?}", machines);
    let total_steps = total_steps_to_desired_joltages(&machines);
    println!("total_steps: {}", total_steps);
}