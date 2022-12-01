use adventofcode::read_lines;

#[test]
fn step1() -> Result<(), Box<dyn std::error::Error>> {
    // Open up step 1 input.
    let veclines = read_lines("inputs/day1.txt")?;

    // Split vector by blank lines
    let elves: Vec<&[String]> = veclines.split(|line| line.is_empty()).collect();

    let max_elf = elves
        .iter()
        .map(|elf| {
            let caloric_total: u32 = elf.iter().map(|val| val.parse::<u32>().unwrap()).sum();
            caloric_total
        })
        .max()
        .expect("no elves in file!");

    println!("Maximum caloric elf: {} calories", max_elf);
    Ok(())
}

#[test]
fn step2() -> Result<(), Box<dyn std::error::Error>> {
    // Open up step 1 input.
    let veclines = read_lines("inputs/day1.txt")?;

    // Split vector by blank lines
    let elves: Vec<&[String]> = veclines.split(|line| line.is_empty()).collect();

    let mut numeric_elves: Vec<u32> = elves
        .iter()
        .map(|elf| {
            let caloric_total: u32 = elf.iter().map(|val| val.parse::<u32>().unwrap()).sum();
            caloric_total
        })
        .collect();

    // Sort elves.
    numeric_elves.sort_unstable();

    // Take last three values.
    let last_three_elves: u32 = numeric_elves.iter().rev().take(3).sum();

    println!("Last three elves: {} calories", last_three_elves);

    Ok(())
}
