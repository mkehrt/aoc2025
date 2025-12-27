use std::collections::HashSet;
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

impl JoltageCounters {
    fn len(&self) -> usize {
        self.desired.len()
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

#[derive(Debug, Eq, Hash, PartialEq)]
struct ButtonPushCounts {
    inner: Vec<ButtonPushCount>,
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

    fn union(&self, other: &ButtonPushCounts) -> Option<ButtonPushCounts> {
        assert!(self.inner.len() == other.inner.len());
        let mut new_button_push_counts = Vec::new();
        for i in 0..self.inner.len() {
            let button_push_count;
            match (self.inner[i].count, other.inner[i].count) {
                (Some(count_self), Some(count_other)) if count_self == count_other => {
                    button_push_count = ButtonPushCount {
                        button_index: i,
                        count: Some(count_self),
                    };
                }
                (Some(_count_self), Some(_count_other)) => {
                    return None;
                }
                (Some(count_self), None) => {
                    button_push_count = ButtonPushCount {
                        button_index: i,
                        count: Some(count_self),
                    };
                }
                (None, Some(count_other)) => {
                    button_push_count = ButtonPushCount {
                        button_index: i,
                        count: Some(count_other),
                    };
                }
                (None, None) => {
                    button_push_count = ButtonPushCount {
                        button_index: i,
                        count: None,
                    };
                }
            }
            new_button_push_counts.push(button_push_count);
        }
        Some(ButtonPushCounts {
            inner: new_button_push_counts,
        })
    }
}

fn reify_button_push_counts_for_one_joltage(
    button_count: usize,
    desired_joltage: u64,
    button_indices_for_joltage: &Vec<usize>,
) -> Vec<ButtonPushCounts> {
    let button_push_counts = ButtonPushCounts::new_for_size(button_count);
    let initial_button_index = 0;
    reify_button_push_counts_for_one_joltage_inner(
        button_push_counts,
        desired_joltage,
        button_indices_for_joltage,
        initial_button_index,
    )
}

fn reify_button_push_counts_for_one_joltage_inner(
    mut button_push_counts: ButtonPushCounts,
    desired_joltage: u64,
    button_indices_for_joltage: &Vec<usize>,
    current_button_index: usize,
) -> Vec<ButtonPushCounts> {
    let total_joltage = button_push_counts.total();
    assert!(total_joltage <= desired_joltage);
    if current_button_index == button_indices_for_joltage.len() - 1 {
        let remaining_joltage = desired_joltage - total_joltage;
        let button_index = button_indices_for_joltage[current_button_index];
        button_push_counts.inner[button_index].count = Some(remaining_joltage);
        assert!(button_push_counts.total() == desired_joltage);
        return vec![button_push_counts];
    }
    let mut button_push_countses = Vec::new();
    for count in total_joltage..desired_joltage + 1 {
        let new_button_push_counts = button_push_counts.clone();
        let button_index = button_indices_for_joltage[current_button_index];
        button_push_counts.inner[button_index].count = Some(count);
        let new_button_push_countses = reify_button_push_counts_for_one_joltage_inner(
            new_button_push_counts,
            desired_joltage,
            button_indices_for_joltage,
            current_button_index + 1,
        );
        button_push_countses.extend(new_button_push_countses);
    }
    button_push_countses
}

fn get_all_button_push_counts(machine: &Machine) -> HashSet<ButtonPushCounts> {
    let joltages_and_button_indiceses = get_joltages_and_button_push_indiceses(machine);
    let mut current_button_push_countses: HashSet<ButtonPushCounts> = HashSet::new();
    for joltage_and_button_indices in joltages_and_button_indiceses {
        let button_count = machine.buttons.len();
        let desired_joltage = joltage_and_button_indices.desired_joltage;
        let button_indices_for_joltage = joltage_and_button_indices.button_indices;
        let new_button_push_countses = reify_button_push_counts_for_one_joltage(
            button_count,
            desired_joltage,
            &button_indices_for_joltage,
        );
        let mut replacement_button_push_countses = HashSet::new();
        for current_button_push_counts in current_button_push_countses.iter() {
            for new_button_push_counts in new_button_push_countses.iter() {
                let union = current_button_push_counts.union(&new_button_push_counts);
                if let Some(union) = union {
                    replacement_button_push_countses.insert(union);
                }
            }
        }
        current_button_push_countses = replacement_button_push_countses;
    }
    current_button_push_countses
}

fn steps_to_desired_joltages(machine: &Machine) -> u64 {
    let all_button_push_counts = get_all_button_push_counts(machine);
    let mut total_steps = u64::MAX;
    for min_button_push_counts in all_button_push_counts {
        let new_total_steps = min_button_push_counts.total();
        if total_steps < new_total_steps {
            total_steps = new_total_steps;
        }
    }
    total_steps
}

fn total_steps_to_desired_joltages(machines: &[Machine]) -> u64 {
    let mut total_steps = 0;
    for machine in machines {
        total_steps += steps_to_desired_joltages(machine);
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
