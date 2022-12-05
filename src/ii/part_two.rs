use std::convert::{Into, From};
use std::io;
use std::fs::read_to_string;
use std::path::Path;

pub fn solution<P: AsRef<Path>>(input: P) -> Result<usize, io::Error> {
    let input = read_to_string(input)?;

    let strategy_guide = input
        .strip_suffix("\n")
        .unwrap_or(&input)
        .split("\n");

    let points = strategy_guide.fold(0, |acc, round| {
        if round.is_empty() { return acc }

        let (left, right) = round.split_once(" ").expect("Malformed input");

        let opp = Shape::from(left);
        let desired_outcome = Outcome::from(right);
        let me = Shape::compute_desired_shape(desired_outcome, opp);

        acc + <Outcome as Into<usize>>::into(desired_outcome) + <Shape as Into<usize>>::into(me)
    });

    Ok(points)
}

#[derive(Clone, Copy, PartialEq)]
pub enum Shape {
    Rock,
    Paper,
    Scissors
}

impl Shape {
    pub fn compute_desired_shape(desired_outcome: Outcome, opponent: Shape) -> Self {
        match opponent {
            Shape::Rock => match desired_outcome {
                Outcome::Win => Shape::Paper,
                Outcome::Draw => Shape::Rock,
                Outcome::Lose => Shape::Scissors,
            },
            Shape::Paper => match desired_outcome {
                Outcome::Win => Shape::Scissors,
                Outcome::Draw => Shape::Paper,
                Outcome::Lose => Shape::Rock,
            },
            Shape::Scissors => match desired_outcome {
                Outcome::Win => Shape::Rock,
                Outcome::Draw => Shape::Scissors,
                Outcome::Lose => Shape::Paper,
            }
        }
    }
}

impl From<&str> for Shape {
    fn from(s: &str) -> Self {
        if s == "A" {
            Shape::Rock
        } else if s == "B" {
            Shape::Paper
        } else if s == "C" {
            Shape::Scissors
        } else {
            panic!("Invalid input '{s}' cannot be converted to Shape.")
        }
    }
}

impl Into<usize> for Shape {
    fn into(self) -> usize {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3
        }
    }
}

#[derive(Copy, Clone)]
pub enum Outcome {
    Win,
    Lose,
    Draw
}

impl From<&str> for Outcome {
    fn from(s: &str) -> Self {
        if s == "X" {
            Outcome::Lose
        } else if s == "Y" {
            Outcome::Draw
        } else if s == "Z" {
            Outcome::Win
        } else {
            panic!("Invalid input '{s}' cannot be converted to Outcome.")
        }
    }
}

impl Into<usize> for Outcome {
    fn into(self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0
        }
    }
}

#[test]
fn test_solution() {
    if let Ok(result) = solution("./test_cases/ii.txt") {
        assert_eq!(result, 12);
    } else {
        assert!(false);
    }
}

