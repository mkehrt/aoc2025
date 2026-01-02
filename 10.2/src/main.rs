use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use std::time::Instant;

static LOGGING_GRANULARITY: usize = 0;

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
        buttons.sort_by_key(|button| button.inner.len());
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
    machine_index: usize,
    button_count: usize,
    joltages_buttons_indiceses: &Vec<JoltageAndButtonIndices>,
) -> u64 {
    let button_push_counts = ButtonPushCounts::new_for_size(button_count);
    let initial_button_index = 0;
    let initial_joltage_index = 0;
    let joltage_count = joltages_buttons_indiceses.len();
    get_button_push_counts_total_inner(
        machine_index,
        button_push_counts,
        joltages_buttons_indiceses,
        0,
        joltage_count,
        initial_joltage_index,
        initial_button_index,
        &mut 0,
    )
    .unwrap()
}

fn log_stuff(machine_index: usize, current_joltage_index: usize, joltage_count: usize, start_time: Instant, attempts: &mut u64) {
    if joltage_count - current_joltage_index >= LOGGING_GRANULARITY {
        let end_time = Instant::now();
        let duration = end_time.duration_since(start_time);
        if duration.as_millis() > 100 {
            for _ in 0..current_joltage_index {
                print!("  ");
            }
            println!(
                "machine_index: {}, current_joltage_index: {}, duration: {:?}, attempts: {}",
                machine_index,
                current_joltage_index, duration, attempts
            );
        }
    }
}

fn get_button_push_counts_total_inner(
    machine_index: usize,
    button_push_counts: ButtonPushCounts,
    joltages_buttons_indiceses: &Vec<JoltageAndButtonIndices>,
    total_joltage_for_counter: u64,
    joltage_count: usize,
    current_joltage_index: usize,
    current_button_index: usize,
    attempts: &mut u64,
) -> Option<u64> {
    let start_time = Instant::now();

    let desired_joltage = joltages_buttons_indiceses[current_joltage_index].desired_joltage;
    let button_indices_for_joltage = joltages_buttons_indiceses[current_joltage_index]
        .button_indices
        .clone();
    let button_index = button_indices_for_joltage[current_button_index];
    // println!("current_joltage_index: {}, current_button_index: {}, desired_joltage: {}, button_indices_for_joltage: {:?}", current_joltage_index, current_button_index, desired_joltage, button_indices_for_joltage);
    // println!("button push counts: {:?}", button_push_counts);

    if current_button_index == button_indices_for_joltage.len() - 1 {
        let mut new_button_push_counts = button_push_counts.clone();
        let remaining_joltage = desired_joltage - total_joltage_for_counter;
        //println!(
        //    "distributing remaining joltage: {} to button: {}, old count: {:?}",
        //    remaining_joltage, button_index, new_button_push_counts.inner[button_index].count
        // );
        if let Some(old_count) = new_button_push_counts.inner[button_index].count {
            if old_count != remaining_joltage {
                // println!("failed");
                *attempts += 1;
                log_stuff(machine_index, current_joltage_index, joltage_count, start_time, attempts);
                return None;
            }
            // println!("succeeded with exisiting count");
        } else {
            // println!("succeeded with new count");
        }
        new_button_push_counts.inner[button_index].count = Some(remaining_joltage);
        if current_joltage_index == joltages_buttons_indiceses.len() - 1 {
            // println!(
            //    "button push counts: {:?}, total: {}",
            //     new_button_push_counts,
            //     new_button_push_counts.total()
            // );

            *attempts += 1;
            log_stuff(machine_index, current_joltage_index, joltage_count, start_time, attempts);
            return Some(new_button_push_counts.total());
        }
        let new_button_push_counts_total = get_button_push_counts_total_inner(
            machine_index,
            new_button_push_counts,
            joltages_buttons_indiceses,
            0,
            joltage_count,
            current_joltage_index + 1,
            0,
            attempts,
        );
        *attempts += 1;
        log_stuff(machine_index, current_joltage_index, joltage_count, start_time, attempts);
        return new_button_push_counts_total;
    }
    if let Some(old_count) = button_push_counts.inner[button_index].count {
        let new_total_joltage_for_counter = total_joltage_for_counter + old_count;
        if new_total_joltage_for_counter > desired_joltage {
            *attempts += 1;
            log_stuff(machine_index, current_joltage_index, joltage_count, start_time, attempts);
            return None;
        }
        let new_min_button_push_counts_total = get_button_push_counts_total_inner(
            machine_index,
            button_push_counts,
            joltages_buttons_indiceses,
            new_total_joltage_for_counter,
            joltage_count,
            current_joltage_index,
            current_button_index + 1,
            attempts,
        );
        *attempts += 1;
        log_stuff(machine_index, current_joltage_index, joltage_count, start_time, attempts);
        return new_min_button_push_counts_total;
    } else {
        for new_total_joltage in total_joltage_for_counter..desired_joltage + 1 {
            let new_count = new_total_joltage - total_joltage_for_counter;
            let mut new_button_push_counts = button_push_counts.clone();
            new_button_push_counts.inner[button_index].count = Some(new_count);
            let new_min_button_push_counts_total = get_button_push_counts_total_inner(
                machine_index,
                new_button_push_counts,
                joltages_buttons_indiceses,
                new_total_joltage,
                joltage_count,
                current_joltage_index,
                current_button_index + 1,
                attempts,
            );
            if let Some(new_min_button_push_counts_total) = new_min_button_push_counts_total {
                *attempts += 1;
                log_stuff(machine_index, current_joltage_index, joltage_count, start_time, attempts);
                return Some(new_min_button_push_counts_total);
            }
            /*   match (
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
            } */
        }
    }
    *attempts += 1;
    log_stuff(machine_index, current_joltage_index, joltage_count, start_time, attempts);
    None
}

fn steps_to_desired_joltages(machine_index: usize, machine: &Machine) -> u64 {
    let joltages_and_button_indiceses = get_joltages_and_button_push_indiceses(machine);
    // println!(
    //     "joltages_and_button_indiceses: {:?}",
    //     joltages_and_button_indiceses
    // );
    let button_count = machine.buttons.len();
    let all_button_push_counts =
        get_button_push_counts_total(machine_index, button_count, &joltages_and_button_indiceses);
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
        let steps = steps_to_desired_joltages(index, machine);
        println!("steps: {}", steps);
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
