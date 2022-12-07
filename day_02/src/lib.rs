use itertools::Itertools;
use std::error::Error;

const WIN: u32 = 6;
const DRAW: u32 = 3;
const LOSS: u32 = 0;

pub fn run(contents: String) -> Result<(), Box<dyn Error>> {
    let total_score = get_total_score(&contents);
    println!("The total score is {}", total_score);
    Ok(())
}

fn score_round(round: (&str, &str)) -> u32 {
    let score_outcome = match round {
        ("A", "X") | ("B", "Y") | ("C", "Z") => DRAW,
        ("A", "Y") | ("B", "Z") | ("C", "X") => WIN,
        ("A", "Z") | ("B", "X") | ("C", "Y") => LOSS,
        _ => panic!("Unknown error!"),
    };
    let score_shape = match round.1 {
        "X" => 1,
        "Y" => 2,
        _ => 3,
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
        assert_eq!(8, score_round(round));
    }
    #[test]
    fn test_get_total_score() {
        let rounds = "A Y
B X
C Z";
        assert_eq!(15, get_total_score(rounds));
    }
}
