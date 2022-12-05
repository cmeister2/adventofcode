use std::collections::VecDeque;

use adventofcode::{read_lines, AOCError};
use anyhow::{anyhow, Context};
use itertools::Itertools;
use regex::Regex;

enum CrateMover {
    Cm9000,
    Cm9001,
}

fn split_input_into_state_and_instructions(input: &[String]) -> VecDeque<&[String]> {
    input.split(|line| line.is_empty()).collect()
}

fn parse_state(state: &[String]) -> Result<Vec<VecDeque<char>>, anyhow::Error> {
    // Look at the last line first to work out the slots.
    let mut statevec = Vec::from(state);
    let numberline = statevec
        .pop()
        .ok_or_else(|| anyhow!("should have a numberline"))?;

    // Find out number of slots by looking at last number.
    let numberline = numberline.trim_end();

    let (_, bignumstr) = numberline
        .rsplit_once(' ')
        .ok_or_else(|| anyhow!("should have more than one column"))?;
    let num_slots = bignumstr
        .parse::<u32>()
        .context("failed to parse num slots")?;

    let mut vecofvecs = Vec::new();
    for _ in 0..num_slots {
        vecofvecs.push(VecDeque::new());
    }

    // For the rest of the lines, push each crate into the right slot.
    for line in statevec {
        for (slot, mut chunk) in line.chars().chunks(4).into_iter().enumerate() {
            match chunk.next() {
                Some(c) => match c {
                    ' ' => {} // Do nothing on a space
                    '[' => {
                        // On a crate, push letter into correct slot.
                        vecofvecs[slot].push_back(
                            chunk
                                .next()
                                .ok_or_else(|| anyhow!("missing package letter"))?,
                        )
                    }
                    _ => unreachable!("unexpected item in bagging area"),
                },
                None => unreachable!("no letters at all"),
            }
        }
    }

    Ok(vecofvecs)
}

fn do_a_move(
    state: &mut Vec<VecDeque<char>>,
    source: usize,
    destination: usize,
) -> Result<(), anyhow::Error> {
    // The source and destination are 1-based, make them 0-based
    let source = source - 1;
    let destination = destination - 1;
    let package = state[source]
        .pop_front()
        .ok_or_else(|| anyhow!("invalid move"))?;
    state[destination].push_front(package);
    Ok(())
}

fn move_n_block(
    state: &mut Vec<VecDeque<char>>,
    source: usize,
    destination: usize,
    quantity: usize,
) -> Result<(), anyhow::Error> {
    // The source and destination are 1-based, make them 0-based
    let source = source - 1;
    let destination = destination - 1;

    let sourcevec = &mut state[source];

    let chars = (0..quantity)
        .map(|_| {
            sourcevec
                .pop_front()
                .ok_or_else(|| anyhow!("failed to pop front"))
        })
        .collect::<Result<Vec<char>, _>>()?;

    for package in chars.iter().rev() {
        state[destination].push_front(*package);
    }
    Ok(())
}

fn parse_instruction(
    state: &mut Vec<VecDeque<char>>,
    instruction: &str,
    cm: &CrateMover,
) -> Result<(), anyhow::Error> {
    println!("{:?}; processing {}", state, instruction);

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)")?;
    let caps = re
        .captures(instruction)
        .ok_or_else(|| anyhow!("failed to find match"))?;

    let quantity = caps
        .get(1)
        .ok_or_else(|| anyhow!("failed to get quantity"))?
        .as_str()
        .parse::<usize>()
        .context("failed to parse quantity")?;
    let source = caps
        .get(2)
        .ok_or_else(|| anyhow!("failed to get source"))?
        .as_str()
        .parse::<usize>()
        .context("failed to parse source")?;
    let destination = caps
        .get(3)
        .ok_or_else(|| anyhow!("failed to get destination"))?
        .as_str()
        .parse::<usize>()
        .context("failed to parse destination")?;

    match cm {
        CrateMover::Cm9000 => {
            for _ in 0..quantity {
                do_a_move(state, source, destination)?;
            }
        }
        CrateMover::Cm9001 => move_n_block(state, source, destination, quantity)?,
    }
    Ok(())
}

fn read_state(state: &[VecDeque<char>]) -> Result<String, anyhow::Error> {
    state
        .iter()
        .map(|column| column.front().ok_or_else(|| anyhow!("column is empty")))
        .collect()
}

fn process_input(input: &[String], cm: CrateMover) -> Result<String, anyhow::Error> {
    let mut slices = split_input_into_state_and_instructions(input);
    let state = slices
        .pop_front()
        .ok_or_else(|| AOCError::Generic("must have state".to_string()))?;

    // Parse the initial crate state
    let mut state = parse_state(state)?;

    for instructions in slices {
        for instruction in instructions {
            parse_instruction(&mut state, instruction, &cm)?;
        }
    }

    read_state(&state)
}

#[test]
fn example_step1() -> Result<(), anyhow::Error> {
    let input = "    [D]
[N] [C]
[Z] [M] [P]
1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";
    let veclines = input.lines().map(String::from).collect::<Vec<String>>();
    let output = process_input(&veclines, CrateMover::Cm9000)?;
    println!("Output is {}", output);

    Ok(())
}

#[test]
fn step1() -> Result<(), anyhow::Error> {
    let veclines = read_lines("inputs/day5.txt")?;
    let output = process_input(&veclines, CrateMover::Cm9000)?;
    println!("Output is {}", output);

    Ok(())
}

#[test]
fn example_step2() -> Result<(), anyhow::Error> {
    let input = "    [D]
[N] [C]
[Z] [M] [P]
1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";
    let veclines = input.lines().map(String::from).collect::<Vec<String>>();
    let output = process_input(&veclines, CrateMover::Cm9001)?;
    println!("Output is {}", output);

    Ok(())
}

#[test]
fn step2() -> Result<(), anyhow::Error> {
    let veclines = read_lines("inputs/day5.txt")?;
    let output = process_input(&veclines, CrateMover::Cm9001)?;
    println!("Output is {}", output);

    Ok(())
}
