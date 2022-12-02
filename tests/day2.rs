use adventofcode::{read_lines, AOCError, Outcome, RockPaperScissors};

fn split_into_tuples(lines: &[String]) -> Result<Vec<(&str, &str)>, AOCError> {
    // Convert lines into tuples of letters
    lines
        .iter()
        .map(|round| {
            round
                .split_once(' ')
                .ok_or_else(|| AOCError::Generic("Round not as expected".to_string()))
        })
        .collect()
}

fn calculate_score_step1(lines: &[String]) -> Result<u32, AOCError> {
    // Convert lines into tuples of letters
    let score = split_into_tuples(lines)?
        .into_iter()
        .map(|(opponent, us)| {
            let us = RockPaperScissors::from(us);

            // Output score from fighting
            us.fight(&RockPaperScissors::from(opponent))
        })
        .sum();
    Ok(score)
}

fn calculate_score_step2(lines: &[String]) -> Result<u32, AOCError> {
    // Convert lines into tuples of letters
    let score = split_into_tuples(lines)?
        .into_iter()
        .map(|(opponent, outcome)| {
            let outcome = Outcome::from(outcome);
            let opponent = RockPaperScissors::from(opponent);
            let us = outcome.determine_shape(&opponent);

            // Output score from fighting
            us.fight(&opponent)
        })
        .sum();
    Ok(score)
}

#[test]
fn example_step1() -> Result<(), Box<dyn std::error::Error>> {
    let input = "A Y
B X
C Z";
    let veclines = input
        .split('\n')
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let score = calculate_score_step1(&veclines)?;
    println!("Example scored {}", score);

    Ok(())
}

#[test]
fn example_step2() -> Result<(), Box<dyn std::error::Error>> {
    let input = "A Y
B X
C Z";
    let veclines = input
        .split('\n')
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let score = calculate_score_step2(&veclines)?;
    println!("Step2: Example scored {}", score);

    Ok(())
}

#[test]
fn step1() -> Result<(), Box<dyn std::error::Error>> {
    let veclines = read_lines("inputs/day2.txt")?;

    let score = calculate_score_step1(&veclines)?;
    println!("Step 1 scored {}", score);

    Ok(())
}

// X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win.
#[test]
fn step2() -> Result<(), Box<dyn std::error::Error>> {
    let veclines = read_lines("inputs/day2.txt")?;

    let score = calculate_score_step2(&veclines)?;
    println!("Step 2 scored {}", score);

    Ok(())
}
