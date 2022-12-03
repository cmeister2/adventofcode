use adventofcode::{read_lines, AOCError};
use itertools::Itertools;
use std::collections::HashSet;

// vJrwpWtwJgWrhcsFMMfFFhFp
// jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
// PmmdzqPrVvPwwTWBwg
// wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
// ttgJtRGJQctTZtZT
// CrZsJsPPZsGzwwsLwLmpwMDw
fn split_into_equal_length_strings(input: &str) -> Result<(&str, &str), AOCError> {
    if input.is_empty() {
        return Err(AOCError::Generic("String was empty".to_string()));
    }
    if input.len() % 2 != 0 {
        return Err(AOCError::Generic("odd string".to_string()));
    }
    Ok(input.split_at(input.len() / 2))
}

fn create_set_from_slice(input: &str) -> HashSet<char> {
    input.chars().collect()
}

fn get_intersections(input: &str) -> Result<HashSet<char>, adventofcode::AOCError> {
    let (part1, part2) = split_into_equal_length_strings(input)?;
    let part1set = create_set_from_slice(part1);
    let part2set = create_set_from_slice(part2);
    Ok(part1set.intersection(&part2set).copied().collect())
}

// Lowercase item types a through z have priorities 1 through 26.  (97-122)
// Uppercase item types A through Z have priorities 27 through 52. (65-90)
fn score_intersections(intersection: &HashSet<char>) -> u32 {
    intersection
        .iter()
        .map(|c| {
            let c_value = u32::from(*c);
            match c_value {
                97..=122 => c_value - 96,
                65..=90 => c_value - 38,
                _ => 0,
            }
        })
        .sum()
}

#[test]
fn example_step1() -> Result<(), anyhow::Error> {
    let score = score_intersections(&get_intersections("vJrwpWtwJgWrhcsFMMfFFhFp")?);
    println!("Score {}", score);

    let score = score_intersections(&get_intersections("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL")?);
    println!("Score {}", score);

    Ok(())
}

#[test]
fn step1() -> Result<(), anyhow::Error> {
    let lines = adventofcode::read_lines("inputs/day3.txt")?;

    let score: u32 = lines
        .iter()
        .map(|line| get_intersections(line))
        .collect::<Result<Vec<HashSet<char>>, AOCError>>()?
        .iter()
        .map(score_intersections)
        .sum();

    println!("Step 1 sum: {}", score);

    Ok(())
}

#[test]
fn step2() -> Result<(), anyhow::Error> {
    let lines = read_lines("inputs/day3.txt")?;

    let score: u32 = lines
        .iter()
        .map(|line| create_set_from_slice(line))
        .chunks(3)
        .into_iter()
        .map(|mut chunk| {
            let elf1 = chunk.next().unwrap();
            let elf2 = chunk.next().unwrap();
            let elf3 = chunk.next().unwrap();

            let elf1and2 = elf1.intersection(&elf2).copied().collect::<HashSet<char>>();
            elf1and2
                .intersection(&elf3)
                .copied()
                .collect::<HashSet<char>>()
        })
        .map(|intersection| score_intersections(&intersection))
        .sum();

    println!("Step 2 sum: {}", score);

    Ok(())
}
