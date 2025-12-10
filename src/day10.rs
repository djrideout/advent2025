use std::collections::{HashSet, VecDeque};

use regex::Regex;

type Button = Vec<usize>;

pub struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Button>,
    joltages: Vec<usize>
}

type Input = Vec<Machine>;

#[aoc_generator(day10)]
pub fn input_generator(in_lines: &str) -> Input {
    let lights_re = Regex::new(r"[#.]").unwrap();
    let buttons_re = Regex::new(r"\(([^)]*)\)").unwrap();
    let joltages_re = regex::Regex::new(r"\{([^}]*)\}").unwrap();
    let mut machines = vec![];
    for l in in_lines.lines() {
        let lights = lights_re
            .find_iter(l)
            .map(|m| m.as_str() == "#")
            .collect();
        let buttons = buttons_re
            .captures_iter(l)
            .map(|cap| {
                cap[1]
                    .split(',')
                    .filter_map(|s| s.trim().parse().ok())
                    .collect::<Vec<usize>>()
            })
            .collect();
        let joltages = joltages_re
            .captures_iter(l)
            .flat_map(|cap| {
                cap[1]
                    .split(',')
                    .filter_map(|s| s.trim().parse().ok())
                    .collect::<Vec<usize>>()
            })
            .collect();
        machines.push(Machine {
            lights,
            buttons,
            joltages,
        });
    }
    machines
}

fn least_presses_lights(target: &Vec<bool>, buttons: &Vec<Button>) -> usize {
    let mut cache = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((vec![false; target.len()], 0));
    while q.len() > 0 {
        let (state, presses) = q.pop_front().unwrap();
        cache.insert(state.clone());
        if state.eq(target) {
            return presses;
        }
        for button in buttons {
            let mut next_state = state.clone();
            for i in button {
                next_state[*i] = !next_state[*i];
            }
            if !cache.contains(&next_state) {
                q.push_back((next_state, presses + 1));
            }
        }
    }
    0
}

fn least_presses_joltage(target: &Vec<usize>, buttons: &Vec<Button>) -> usize {
    let mut matrix = vec![];
    for (i, t) in target.iter().enumerate() {
        let mut line = vec![];
        for b in buttons {
            line.push(b.contains(&i) as usize);
        }
        line.push(*t);
        matrix.push(line);
    }

    // I either need to solve this system of linear equations,
    // or go back to the breadth-first-search method with better pruning somehow.

    // Example system of linear equations for first demo machine:
    // 0A + 0B + 0C + 0D + 1E + 1F = 3
    // 0A + 1B + 0C + 0D + 0E + 1F = 5
    // 0A + 0B + 1C + 1D + 1E + 0F = 4
    // 1A + 1B + 0C + 1D + 0E + 0F = 7

    0
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &Input) -> usize {
    input.iter().fold(0, |acc, m| acc + least_presses_lights(&m.lights, &m.buttons))
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &Input) -> usize {
    input.iter().fold(0, |acc, m| acc + least_presses_joltage(&m.joltages, &m.buttons))
}
