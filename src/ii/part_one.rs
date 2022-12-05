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
        let (left, right) = round.split_once(" ").expect("Malformed input");

        let opp = Shape::from(left);
        let me = Shape::from(right);
        let outcome = me.compare(opp);

        acc + <Outcome as Into<usize>>::into(outcome) + <Shape as Into<usize>>::into(me)
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
    pub fn compare(self, rhs: Shape) -> Outcome {
        match self {
            Shape::Rock => match rhs {
                Shape::Rock => Outcome::Draw,
                Shape::Paper => Outcome::Lose,
                Shape::Scissors => Outcome::Win,
            },
            Shape::Paper => match rhs {
                Shape::Rock => Outcome::Win,
                Shape::Paper => Outcome::Draw,
                Shape::Scissors => Outcome::Lose,
            },
            Shape::Scissors => match rhs {
                Shape::Rock => Outcome::Lose,
                Shape::Paper => Outcome::Win,
                Shape::Scissors => Outcome::Draw,
            }
        }
    }
}

impl From<&str> for Shape {
    fn from(s: &str) -> Self {
        if s == "A" || s == "X" {
            Shape::Rock
        } else if s == "B" || s == "Y" {
            Shape::Paper
        } else if s == "C" || s == "Z" {
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

pub enum Outcome {
    Win,
    Lose,
    Draw
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
        assert_eq!(result, 15);
    } else {
        assert!(false);
    }
}

