use std::collections::VecDeque;

use regex::Regex;

// the usizes being which lights the button will toggle
type Button = Vec<usize>;

pub struct Machine {
    target: Vec<bool>,
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
        let target = lights_re
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
            target,
            buttons,
            joltages,
        });
    }
    machines
}

fn least_presses(target: &Vec<bool>, buttons: &Vec<Button>) -> usize {
    let mut q = VecDeque::new();
    q.push_back((vec![false; target.len()], 0));
    while q.len() > 0 {
        let (state, presses) = q.pop_front().unwrap();
        if state.eq(target) {
            return presses;
        }
        for button in buttons {
            let mut next_state = state.clone();
            for i in button {
                next_state[*i] = !next_state[*i];
            }
            q.push_back((next_state, presses + 1));
        }
    }
    0
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &Input) -> usize {
    input.iter().fold(0, |acc, m| acc + least_presses(&m.target, &m.buttons))
}
