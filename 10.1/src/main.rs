use std::str::FromStr;

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


#[derive(Clone, Debug)]
struct Joltages {
    joltages: Vec<u64>,
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
        println!("step count: {}", step_count);
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
    let total_steps = total_steps_to_desired_state(&machines);
    println!("total_steps: {}", total_steps);
}