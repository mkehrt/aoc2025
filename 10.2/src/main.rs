use std::fmt::{Debug, Formatter};
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

#[derive(Clone, Debug)]
struct ButtonCounterIndices {
    pub inner: Vec<usize>,
}

impl FromStr for ButtonCounterIndices {
    type Err = anyhow::Error;
    fn from_str(delimed_str: &str) -> Result<Self, Self::Err> {
        let str = &delimed_str[1..delimed_str.len() - 1];
        let parts = str.split(",").collect::<Vec<&str>>();
        let mut button_counter_indices = Vec::new();
        for part in parts {
            let button = part.parse::<usize>()?;
            button_counter_indices.push(button);
        }
        Ok(ButtonCounterIndices {
            inner: button_counter_indices,
        })
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct JoltageCounters {
    pub desired: Vec<u64>,
}

impl FromStr for JoltageCounters {
    type Err = anyhow::Error;
    fn from_str(delimed_str: &str) -> Result<Self, Self::Err> {
        let str = &delimed_str[1..delimed_str.len() - 1];
        let parts = str.split(",").collect::<Vec<&str>>();
        let mut desired = Vec::new();
        for part in parts {
            let joltage = part.parse::<u64>()?;
            desired.push(joltage);
        }
        Ok(JoltageCounters { desired })
    }
}

#[derive(Clone, Debug)]
struct Machine {
    _indicators: Indicators,
    buttons: Vec<ButtonCounterIndices>,
    joltages_counters: JoltageCounters,
}

impl FromStr for Machine {
    type Err = anyhow::Error;
    fn from_str(delimed_str: &str) -> Result<Self, Self::Err> {
        let parts = delimed_str.split(" ").collect::<Vec<&str>>();
        let indicators = Indicators::from_str(parts[0])?;
        let mut buttons = Vec::new();
        for part in &parts[1..parts.len() - 1] {
            let button = ButtonCounterIndices::from_str(part)?;
            buttons.push(button);
        }
        let joltages_counters = JoltageCounters::from_str(parts[parts.len() - 1])?;
        Ok(Machine {
            _indicators: indicators,
            buttons,
            joltages_counters,
        })
    }
}

#[derive(Debug)]
struct JoltageAndButtonIndices {
    desired_joltage: u64,
    button_indices: Vec<usize>,
}

fn get_joltages_and_button_push_indiceses(machine: &Machine) -> Vec<JoltageAndButtonIndices> {
    let mut joltage_and_button_indices = Vec::new();
    for (joltage_index, desired_joltage) in machine.joltages_counters.desired.iter().enumerate() {
        let mut button_indices = Vec::new();
        for (button_index, button_counter_indices) in machine.buttons.iter().enumerate() {
            for button_counter_index in button_counter_indices.inner.iter() {
                if *button_counter_index == joltage_index {
                    button_indices.push(button_index);
                    break;
                }
            }
        }
        joltage_and_button_indices.push(JoltageAndButtonIndices {
            desired_joltage: *desired_joltage,
            button_indices,
        });
    }
    joltage_and_button_indices
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct ButtonPushCount {
    button_index: usize,
    count: Option<u64>,
}

#[derive(Eq, Hash, PartialEq)]
struct ButtonPushCounts {
    inner: Vec<ButtonPushCount>,
}

impl Debug for ButtonPushCounts {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ ")?;
        for i in 0..self.inner.len() {
            write!(
                f,
                "{} ",
                self.inner[i]
                    .count
                    .map(|c| c.to_string())
                    .unwrap_or("-".to_string())
            )?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl ButtonPushCounts {
    fn new_for_size(size: usize) -> Self {
        let mut inner = Vec::new();
        for i in 0..size {
            inner.push(ButtonPushCount {
                button_index: i,
                count: None,
            });
        }
        ButtonPushCounts { inner }
    }
}

impl Clone for ButtonPushCounts {
    fn clone(&self) -> Self {
        let mut new_button_push_counts = Vec::new();
        for button_push_count in &self.inner {
            new_button_push_counts.push(button_push_count.clone());
        }
        ButtonPushCounts {
            inner: new_button_push_counts,
        }
    }
}
impl ButtonPushCounts {
    fn total(&self) -> u64 {
        let mut total = 0;
        for button_push_count in &self.inner {
            total += button_push_count.count.unwrap_or(0);
        }
        total
    }
}

fn get_button_push_counts_total(
    button_count: usize,
    joltages_buttons_indiceses: &Vec<JoltageAndButtonIndices>,
) -> u64 {
    let button_push_counts = ButtonPushCounts::new_for_size(button_count);
    let initial_button_index = 0;
    let initial_joltage_index = 0;
    get_button_push_counts_total_inner(
        vec![button_push_counts],
        joltages_buttons_indiceses,
        initial_joltage_index,
        initial_button_index,
    )
    .unwrap()
}

fn get_button_push_counts_total_inner(
    button_push_counts_trace: Vec<ButtonPushCounts>,
    joltages_buttons_indiceses: &Vec<JoltageAndButtonIndices>,
    current_joltage_index: usize,
    current_button_index: usize,
) -> Option<u64> {
    let desired_joltage = joltages_buttons_indiceses[current_joltage_index].desired_joltage;
    let button_indices_for_joltage = joltages_buttons_indiceses[current_joltage_index]
        .button_indices
        .clone();
    println!("current_joltage_index: {}, current_button_index: {}, desired_joltage: {}, button_indices_for_joltage: {:?}", current_joltage_index, current_button_index, desired_joltage, button_indices_for_joltage);
    println!("button push counts: {:?}", button_push_counts_trace.last().unwrap());
    let total_joltage = button_push_counts_trace.last().unwrap().total();

    if current_button_index == button_indices_for_joltage.len() - 1 {
        let mut new_button_push_counts = button_push_counts_trace.last().unwrap().clone();
        let remaining_joltage = desired_joltage - total_joltage;
        let button_index = button_indices_for_joltage[current_button_index];
        if let Some(old_count) = new_button_push_counts.inner[button_index].count {
            if old_count != remaining_joltage {
                return None;
            }
        }
        new_button_push_counts.inner[button_index].count = Some(remaining_joltage);
        if current_joltage_index == joltages_buttons_indiceses.len() - 1 {
            let mut new_button_push_counts_trace = button_push_counts_trace.clone();
            new_button_push_counts_trace.push(new_button_push_counts);    
            println!("new_button_push_counts_trace: {:?}", new_button_push_counts_trace);
            println!("button push counts: {:?}, total: {}", new_button_push_counts_trace, new_button_push_counts_trace.last().unwrap().total());
            return Some(new_button_push_counts_trace.last().unwrap().total());
        }
        let mut new_button_push_counts_trace = button_push_counts_trace.clone();
        new_button_push_counts_trace.push(new_button_push_counts);
        let new_button_push_counts_total = get_button_push_counts_total_inner(
            new_button_push_counts_trace,
            joltages_buttons_indiceses,
            current_joltage_index + 1,
            0,
        );
        return new_button_push_counts_total;
    }
    let mut min_button_push_counts_total: Option<u64> = None;
    for new_total_joltage in total_joltage..desired_joltage + 1 {
        let new_count = new_total_joltage - total_joltage;
        let mut new_button_push_counts = button_push_counts_trace.last().unwrap().clone();
        let button_index = button_indices_for_joltage[current_button_index];
        if let Some(old_count) = new_button_push_counts.inner[button_index].count {
            if old_count != new_count {
                // This can be optimized by skipping this for loop in this case and just using the existing count if it's valid.
                continue;
            }
        }
        new_button_push_counts.inner[button_index].count = Some(new_count);
        let mut  new_button_push_counts_trace = button_push_counts_trace.clone();
        new_button_push_counts_trace.push(new_button_push_counts);
        let new_min_button_push_counts_total = get_button_push_counts_total_inner(
            new_button_push_counts_trace,
            joltages_buttons_indiceses,
            current_joltage_index,
            current_button_index + 1,
        );
        match (
            min_button_push_counts_total,
            new_min_button_push_counts_total,
        ) {
            (Some(min), Some(new_min)) => {
                min_button_push_counts_total = Some(min.min(new_min));
            }
            (None, Some(new_min)) => {
                min_button_push_counts_total = Some(new_min);
            }
            _ => {}
        }
    }
    min_button_push_counts_total
}

fn steps_to_desired_joltages(machine: &Machine) -> u64 {
    let joltages_and_button_indiceses = get_joltages_and_button_push_indiceses(machine);
    println!("joltages_and_button_indiceses: {:?}", joltages_and_button_indiceses);
    let button_count = machine.buttons.len();
    let all_button_push_counts =
        get_button_push_counts_total(button_count, &joltages_and_button_indiceses);
    all_button_push_counts
}

fn total_steps_to_desired_joltages(machines: &[Machine]) -> u64 {
    let mut total_steps = 0;
    for (index, machine) in machines.iter().enumerate() {
        println!(
            "\n\n\n>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> machine {}\n",
            index
        );
        println!("machine: {:?}", machine);
        let steps = steps_to_desired_joltages(machine);
        println!("steps: {}", steps);
        println!("EARLY EXIT");
        std::process::exit(1);
        total_steps += steps;
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
    let total_steps = total_steps_to_desired_joltages(&machines);
    println!("total steps: {}", total_steps);
}
