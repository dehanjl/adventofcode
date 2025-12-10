use indicatif::ParallelProgressIterator;
use indicatif::ProgressIterator;
use std::collections::VecDeque;

use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use crate::register_day;

struct Button {
    positions: Vec<u32>,
    value: u32,
}

impl Button {
    fn new_from_str(s: &str) -> Self {
        let positions = s
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split(',')
            .map(|part| part.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let value = from_bits(positions.clone());
        Button { positions, value }
    }
}

struct Machine {
    target: u32,
    buttons: Vec<Button>,
    joltages: Vec<u32>,
}

impl Machine {
    fn new_from_str(s: &str) -> Self {
        let seqs = s.split_whitespace().collect::<Vec<&str>>();

        let target = from_bits(
            seqs[0]
                .trim_start_matches('[')
                .trim_end_matches(']')
                .chars()
                .enumerate()
                .filter_map(|(i, c)| (c == '#').then_some(i as u32)),
        );

        let buttons = seqs[1..seqs.len() - 1]
            .iter()
            .map(|btn_str| Button::new_from_str(btn_str))
            .collect::<Vec<_>>();

        let joltages = seqs[seqs.len() - 1]
            .trim_start_matches('{')
            .trim_end_matches('}')
            .split(',')
            .map(|part| part.parse::<u32>().unwrap())
            .collect();

        Machine {
            target,
            buttons,
            joltages,
        }
    }
}

/// Toggles the bit at position `pos` in the integer `n`.
fn toggle_bit(n: u32, pos: u32) -> u32 {
    n ^ (1 << pos)
}

/// Sets the bit at position `pos` in the integer `n`.
fn set_bit(n: u32, pos: u32) -> u32 {
    n | (1 << pos)
}

/// Build a u32 with the given bit positions set.
/// Example: from_bits(&[0, 2, 5]) => 0b0010_0101
fn from_bits<I>(positions: I) -> u32
where
    I: IntoIterator<Item = u32>,
{
    positions
        .into_iter()
        .fold(0u32, |acc, pos| acc | (1 << pos))
}

fn parse_input(input: &str) -> Vec<Machine> {
    input.lines().map(Machine::new_from_str).collect()
}

fn part1(input: &str) {
    let machines = parse_input(input);

    let sum = machines
        .par_iter()
        .map(|machine| {
            // state, presses
            let mut q: VecDeque<(u32, u32)> =
                machine.buttons.iter().map(|b| (b.value, 1)).collect();
            loop {
                let (state, presses) = q.pop_front().unwrap();
                if state == machine.target {
                    return presses;
                }
                machine
                    .buttons
                    .iter()
                    .for_each(|b| q.push_back((state ^ b.value, presses + 1)));
            }
        })
        .sum::<u32>();

    println!("Day 10 Part 1: {}", sum);
}

fn bump_counter(mut counter: Vec<u32>, idxs: &[u32]) -> Vec<u32> {
    for &i in idxs {
        unsafe {
            *counter.get_unchecked_mut(i as usize) += 1;
        }
    }
    counter
}

fn part2(input: &str) {
    let machines = parse_input(input);

    let sum = machines
        .iter()
        .progress()
        .map(|machine| {
            // state, presses
            let mut q: VecDeque<(Vec<u32>, u32)> = machine
                .buttons
                .iter()
                .map(|b| {
                    (
                        bump_counter(vec![0; machine.joltages.len()], &b.positions),
                        1,
                    )
                })
                .collect();
            loop {
                let (state, presses) = q.pop_front().unwrap();
                if state == machine.joltages {
                    return presses;
                }
                if state.iter().zip(&machine.joltages).any(|(a, b)| a > b) {
                    continue;
                }
                machine.buttons.iter().for_each(|b| {
                    q.push_back((bump_counter(state.clone(), &b.positions), presses + 1))
                });
            }
        })
        .sum::<u32>();

    println!("Day 10 Part 2: {}", sum);
}

register_day!(2025, 10, part1, part2);
