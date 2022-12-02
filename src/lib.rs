use std::io::{BufRead, BufReader};
use std::{fs::File, path::Path};

#[derive(Debug, thiserror::Error)]
pub enum AOCError {
    #[error("Generic error")]
    Generic(String),
}

#[derive(Debug)]
pub enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
    Error,
}

impl RockPaperScissors {
    // Your total score is the sum of your scores for each round.
    // The score for a single round is the score for the shape you
    // selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus
    // the score for the outcome of the round (0 if you lost, 3 if
    // the round was a draw, and 6 if you won).
    pub fn fight(&self, opponent: &RockPaperScissors) -> u32 {
        // Get score for shape.
        let shape_score = match self {
            RockPaperScissors::Rock => 1,
            RockPaperScissors::Paper => 2,
            RockPaperScissors::Scissors => 3,
            RockPaperScissors::Error => 0,
        };

        // Get score vs opponent.
        let fight_score = match (self, opponent) {
            (RockPaperScissors::Rock, RockPaperScissors::Rock) => 3,
            (RockPaperScissors::Rock, RockPaperScissors::Paper) => 0,
            (RockPaperScissors::Rock, RockPaperScissors::Scissors) => 6,
            (RockPaperScissors::Paper, RockPaperScissors::Rock) => 6,
            (RockPaperScissors::Paper, RockPaperScissors::Paper) => 3,
            (RockPaperScissors::Paper, RockPaperScissors::Scissors) => 0,
            (RockPaperScissors::Scissors, RockPaperScissors::Rock) => 0,
            (RockPaperScissors::Scissors, RockPaperScissors::Paper) => 6,
            (RockPaperScissors::Scissors, RockPaperScissors::Scissors) => 3,
            _ => 0,
        };

        // Return total score.
        shape_score + fight_score
    }
}

impl From<&str> for RockPaperScissors {
    fn from(input: &str) -> Self {
        match input {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            "X" => Self::Rock,
            "Y" => Self::Paper,
            "Z" => Self::Scissors,
            _ => Self::Error,
        }
    }
}

#[derive(Debug)]
pub enum Outcome {
    Win,
    Loss,
    Draw,
    Error,
}

// X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win.
impl From<&str> for Outcome {
    fn from(input: &str) -> Self {
        match input {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => Self::Error,
        }
    }
}

impl Outcome {
    pub fn determine_shape(&self, opponent: &RockPaperScissors) -> RockPaperScissors {
        match (self, opponent) {
            (Outcome::Win, RockPaperScissors::Rock) => RockPaperScissors::Paper,
            (Outcome::Win, RockPaperScissors::Paper) => RockPaperScissors::Scissors,
            (Outcome::Win, RockPaperScissors::Scissors) => RockPaperScissors::Rock,
            (Outcome::Loss, RockPaperScissors::Rock) => RockPaperScissors::Scissors,
            (Outcome::Loss, RockPaperScissors::Paper) => RockPaperScissors::Rock,
            (Outcome::Loss, RockPaperScissors::Scissors) => RockPaperScissors::Paper,
            (Outcome::Draw, RockPaperScissors::Rock) => RockPaperScissors::Rock,
            (Outcome::Draw, RockPaperScissors::Paper) => RockPaperScissors::Paper,
            (Outcome::Draw, RockPaperScissors::Scissors) => RockPaperScissors::Scissors,
            _ => RockPaperScissors::Error,
        }
    }
}

pub fn read_lines<P>(filename: P) -> std::io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    Ok(lines.map(|line| line.unwrap()).collect())
}
