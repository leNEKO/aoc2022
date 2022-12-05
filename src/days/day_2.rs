use crate::helpers::read_lines;
use std::io::Result;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum HandShape {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl HandShape {
    fn from_str(c: &str) -> Self {
        match c {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissor,
            _ => todo!("Well, should not happen..."),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Loose = 0,
}

#[derive(Debug)]
struct Round {
    mine: HandShape,
    other: HandShape,
}

impl Round {
    fn new(mine: HandShape, other: HandShape) -> Self {
        Self { mine, other }
    }

    fn from_str(line: &str) -> Self {
        let chars: Vec<&str> = line.split(" ").collect();

        Self {
            mine: HandShape::from_str(chars[1]),
            other: HandShape::from_str(chars[0]),
        }
    }

    fn outcome(&self) -> Outcome {
        match (&self.mine, &self.other) {
            (HandShape::Rock, HandShape::Scissor) => Outcome::Win,
            (HandShape::Rock, HandShape::Paper) => Outcome::Loose,
            (HandShape::Scissor, HandShape::Paper) => Outcome::Win,
            (HandShape::Scissor, HandShape::Rock) => Outcome::Loose,
            (HandShape::Paper, HandShape::Rock) => Outcome::Win,
            (HandShape::Paper, HandShape::Scissor) => Outcome::Loose,
            _ => Outcome::Draw,
        }
    }

    fn score(&self) -> u32 {
        self.outcome() as u32 + self.mine as u32
    }
}

pub fn resolve(filepath: &str) -> Result<u32> {
    let line_iterator = read_lines::<String>(filepath)?;

    Ok(line_iterator
        .map(|line| match line {
            Ok(l) => Round::from_str(&l).score(),
            _ => 0,
        })
        .sum())
}

#[test]
fn test_part1() {
    let actual = resolve("data/day_2/test.txt");
    assert_eq!(15, actual.unwrap());

    dbg!(resolve("data/day_2/input.txt").unwrap());
}

#[test]
fn test_action() {
    assert_eq!(
        Outcome::Loose,
        Round::new(HandShape::Rock, HandShape::Paper).outcome()
    );
    assert_eq!(
        Outcome::Win,
        Round::new(HandShape::Rock, HandShape::Scissor).outcome()
    );
    assert_eq!(
        Outcome::Draw,
        Round::new(HandShape::Rock, HandShape::Rock).outcome()
    );
    assert_eq!(
        Outcome::Loose,
        Round::new(HandShape::Paper, HandShape::Scissor).outcome()
    );
}

#[test]
fn test_score() {
    assert_eq!(8, Round::new(HandShape::Paper, HandShape::Rock).score());
    assert_eq!(1, Round::new(HandShape::Rock, HandShape::Paper).score());
    assert_eq!(
        6,
        Round::new(HandShape::Scissor, HandShape::Scissor).score()
    );
}

#[test]
fn test_handshape_from_char() {
    assert_eq!(HandShape::Paper, HandShape::from_str("Y"));
}
