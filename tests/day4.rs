// 2-4,6-8
// 2-3,4-5
// 5-7,7-9
// 2-8,3-7
// 6-6,4-6
// 2-6,4-8

use std::collections::HashSet;

use adventofcode::AOCError;

fn rangestr_to_bytenum(input: &str) -> Result<HashSet<u32>, anyhow::Error> {
    let (start, inclusiveend) = input.split_once('-').unwrap();
    let start = start.parse::<u32>()?;
    let inclusiveend = inclusiveend.parse::<u32>()?;
    Ok((start..inclusiveend + 1).collect())
}

fn elfor(elf1: &str, elf2: &str) -> Result<(HashSet<u32>, HashSet<u32>), anyhow::Error> {
    let elf1val = rangestr_to_bytenum(elf1)?;
    let elf2val = rangestr_to_bytenum(elf2)?;
    Ok((elf1val, elf2val))
}

fn encompass_check(elf1: &str, elf2: &str) -> Result<bool, anyhow::Error> {
    let (elf1val, elf2val) = elfor(elf1, elf2)?;

    let elfintersection = elf1val
        .intersection(&elf2val)
        .copied()
        .collect::<HashSet<u32>>();
    Ok((elfintersection == elf1val) || (elfintersection == elf2val))
}

fn overlap_check(elf1: &str, elf2: &str) -> Result<bool, anyhow::Error> {
    let (elf1val, elf2val) = elfor(elf1, elf2)?;
    Ok(elf1val.intersection(&elf2val).count() != 0)
}

#[test]
fn example_step1() -> Result<(), anyhow::Error> {
    // Split assignments into elves
    let (elf1, elf2) = "2-8,3-7".split_once(',').unwrap();
    let res = encompass_check(elf1, elf2)?;
    println!("Encompassed: {}", res);
    Ok(())
}

#[test]
fn step1() -> Result<(), anyhow::Error> {
    let lines = adventofcode::read_lines("inputs/day4.txt")?;
    let encompassed = lines
        .iter()
        .map(|line| match line.split_once(',') {
            Some((elf1, elf2)) => encompass_check(elf1, elf2),
            None => Err(AOCError::Generic("line couldn't be split".to_string()).into()),
        })
        .collect::<Result<Vec<bool>, _>>()?
        .into_iter()
        .filter(|b| *b)
        .count();

    println!("Encompassed: {}", encompassed);

    Ok(())
}

#[test]
fn step2() -> Result<(), anyhow::Error> {
    let lines = adventofcode::read_lines("inputs/day4.txt")?;

    let overlaps = lines
        .iter()
        .map(|line| match line.split_once(',') {
            Some((elf1, elf2)) => overlap_check(elf1, elf2),
            None => Err(AOCError::Generic("line couldn't be split".to_string()).into()),
        })
        .collect::<Result<Vec<bool>, _>>()?
        .into_iter()
        .filter(|b| *b)
        .count();

    println!("Overlaps: {}", overlaps);

    Ok(())
}
