use itertools::Itertools;
use std::error::Error;

const WIN: u32 = 6;
const DRAW: u32 = 3;
const LOSS: u32 = 0;
const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;

pub fn run(contents: String) -> Result<(), Box<dyn Error>> {
    let total_score = get_total_score(&contents);
    println!("The total score is {}", total_score);
    Ok(())
}

fn score_round(round: (&str, &str)) -> u32 {
    let score_shape = match round {
        ("A", "Y") | ("B", "X") | ("C", "Z") => ROCK,
        ("A", "Z") | ("B", "Y") | ("C", "X") => PAPER,
        ("A", "X") | ("B", "Z") | ("C", "Y") => SCISSORS,
        _ => panic!("Unknown error!"),
    };
    let score_outcome = match round.1 {
        "X" => LOSS,
        "Y" => DRAW,
        "Z" => WIN,
        _ => panic!("Unknown input!"),
    };

    score_shape + score_outcome
}

fn get_total_score(contents: &str) -> u32 {
    let mut total_score = 0;
    let rounds = contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .collect_tuple::<(&str, &str)>()
                .unwrap()
        })
        .collect::<Vec<_>>();
    for round in rounds {
        let round_score = score_round(round);
        total_score += round_score;
    }
    total_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_round() {
        let round = ("A", "Y");
        assert_eq!(4, score_round(round));
    }
    #[test]
    fn test_get_total_score() {
        let rounds = "A Y
B X
C Z";
        assert_eq!(12, get_total_score(rounds));
    }
}
