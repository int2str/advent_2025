#![allow(dead_code)]

mod joltages;

use joltages::Joltages;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::Result;
use std::path::Path;

#[derive(Debug, Default, Clone)]
struct Machine {
    led_count: usize,
    led_target: u64,
    buttons: Vec<u64>,
    button_joltages: Vec<Joltages>,
    joltage_target: Joltages,
}

fn read_machines<P: AsRef<Path>>(path: P) -> Result<Vec<Machine>> {
    Ok(std::fs::read_to_string(path)?
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let mut machine = Machine::default();

            for part in parts {
                match part.chars().next().unwrap() {
                    '[' => {
                        machine.led_target = part[1..part.len() - 1].chars().enumerate().fold(
                            0u64,
                            |acc, (i, c)| {
                                if c == '#' { acc | (1 << i) } else { acc }
                            },
                        );
                        machine.led_count = part.len() - 2;
                    }
                    '(' => {
                        let button = part[1..part.len() - 1]
                            .split(',')
                            .map(|s| s.parse::<usize>().unwrap())
                            .fold(0u64, |acc, led_index| acc | (1 << led_index));
                        machine.buttons.push(button);
                        machine
                            .button_joltages
                            .push(Joltages::from_u64(button, machine.led_count));
                    }
                    '{' => {
                        machine.joltage_target = Joltages::new(machine.led_count);
                        part[1..part.len() - 1]
                            .split(',')
                            .map(|s| s.parse::<u64>().unwrap())
                            .enumerate()
                            .for_each(|(idx, j)| machine.joltage_target[idx] = j);
                    }
                    _ => {}
                }
            }

            machine
        })
        .collect())
}

fn button_combo(machine: &Machine) -> Option<usize> {
    let led_mask = (1 << machine.led_count) - 1;

    // Queue (led_state, last_button, depth)
    let mut queue: VecDeque<(u64, u64, usize)> = VecDeque::new();
    let mut visited: HashSet<(u64, u64)> = HashSet::new();

    for &button in &machine.buttons {
        let new_leds = button & led_mask;
        if new_leds == machine.led_target {
            return Some(1);
        }

        queue.push_back((new_leds, button, 1));
        visited.insert((new_leds, button));
    }

    while let Some((leds, last_button, depth)) = queue.pop_front() {
        for &button in machine.buttons.iter().filter(|&&btn| btn != last_button) {
            let new_leds = (leds ^ button) & led_mask;
            if new_leds == machine.led_target {
                return Some(depth + 1);
            }

            if visited.insert((new_leds, button)) {
                queue.push_back((new_leds, button, depth + 1));
            }
        }
    }

    None
}

#[derive(Debug, Clone)]
struct State {
    joltage: Joltages,
    cost: usize,
    next_button_idx: usize,
    gcd: u64,
}

fn find_all_possible_states(target: Joltages, buttons: &[Joltages]) -> Vec<State> {
    let mut queue = VecDeque::new();
    let mut results = Vec::new();

    queue.push_back(State {
        joltage: target,
        cost: 0,
        next_button_idx: 0,
        gcd: 1,
    });

    while let Some(state) = queue.pop_front() {
        for (button_idx, button) in buttons.iter().enumerate().skip(state.next_button_idx) {
            if let Some(next) = state.joltage - *button {
                let gcd = next.gcd();
                let next_state = State {
                    joltage: next,
                    cost: state.cost + 1,
                    next_button_idx: button_idx + 1,
                    gcd,
                };

                let is_result = next.is_zero();
                if gcd > 1 || is_result {
                    results.push(next_state.clone());
                }
                if !is_result {
                    queue.push_back(next_state);
                }
            }
        }
    }

    results
}

fn count_buttons(
    target: Joltages,
    buttons: &[Joltages],
    cache: &mut HashMap<Joltages, usize>,
) -> usize {
    if let Some(&cached) = cache.get(&target) {
        return cached;
    }

    if target.is_zero() {
        cache.insert(target, 0);
        return 0;
    }

    let mut min_cost = usize::MAX;
    let target_gcd = target.gcd();

    if target_gcd > 1 {
        let reduced = target / target_gcd;
        let cost_reduced = count_buttons(reduced, buttons, cache);
        if cost_reduced != usize::MAX {
            let count = cost_reduced.saturating_mul(target_gcd as usize);
            min_cost = min_cost.min(count);
        }
    }

    for state in find_all_possible_states(target, buttons) {
        let cost = state.cost;
        let gcd = state.gcd;

        if cost >= min_cost {
            continue;
        }

        if gcd == 1 {
            min_cost = cost;
            break;
        }

        let reduced = state.joltage / gcd;
        let cost_reduced = count_buttons(reduced, buttons, cache);
        if cost_reduced == usize::MAX {
            continue;
        }

        let count = (gcd as usize)
            .saturating_mul(cost_reduced)
            .saturating_add(cost);
        min_cost = min_cost.min(count);
    }

    cache.insert(target, min_cost);
    min_cost
}

fn joltage_combo(machine: &Machine) -> Option<usize> {
    let result = count_buttons(
        machine.joltage_target,
        &machine.button_joltages,
        &mut HashMap::new(),
    );
    (result != usize::MAX).then_some(result)
}

fn button_combos(machines: &[Machine]) -> usize {
    machines.iter().map(|m| button_combo(m).unwrap_or(0)).sum()
}

fn joltage_combos(machines: &[Machine]) -> usize {
    machines.iter().map(|m| joltage_combo(m).unwrap_or(0)).sum()
}

fn main() {
    println!("Run 'cargo test' instead ;)");
}

#[cfg(test)]
mod tests {
    mod day10_factory {
        use super::super::*;

        #[test]
        fn sample() {
            let machines = read_machines("data/sample.txt").unwrap();
            assert_eq!(button_combos(&machines), 7);
            assert_eq!(joltage_combos(&machines), 33);
        }

        #[test]
        #[cfg(feature = "private")]
        fn input() {
            let machines = read_machines("data/input.txt").unwrap();
            assert_eq!(button_combos(&machines), 488);
            assert_eq!(joltage_combos(&machines), 18771);
        }
    }
}
