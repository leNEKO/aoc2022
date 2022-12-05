use crate::helpers::read_lines;
use std::{cmp::Ordering, io::Result};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum HandShape {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl PartialOrd for HandShape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use HandShape::*;
        Some(match (self, other) {
            (Rock, Scissor) => Ordering::Greater,
            (Scissor, Rock) => Ordering::Less,
            (Scissor, Paper) => Ordering::Greater,
            (Paper, Scissor) => Ordering::Less,
            (Paper, Rock) => Ordering::Greater,
            (Rock, Paper) => Ordering::Less,
            _ => Ordering::Equal,
        })
    }
}
#[test]
fn test_compare_handshape() {
    assert!(HandShape::Paper > HandShape::Rock);
    assert!(HandShape::Rock > HandShape::Scissor);
    assert!(HandShape::Scissor > HandShape::Paper);
    assert!(HandShape::Scissor == HandShape::Scissor);
    assert!(HandShape::Rock < HandShape::Paper);
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

    fn from_outcome(other: Self, outcome: Outcome) -> Self {
        use HandShape::*;
        match (other, outcome) {
            (Rock, Outcome::Draw) => Rock,
            (Rock, Outcome::Win) => Paper,
            (Rock, Outcome::Loose) => Scissor,
            (Paper, Outcome::Draw) => Paper,
            (Paper, Outcome::Win) => Scissor,
            (Paper, Outcome::Loose) => Rock,
            (Scissor, Outcome::Draw) => Scissor,
            (Scissor, Outcome::Win) => Rock,
            (Scissor, Outcome::Loose) => Paper,
        }
    }
}
#[test]
fn test_handshape_from_char() {
    assert_eq!(HandShape::Paper, HandShape::from_str("Y"));
}

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Loose = 0,
}

impl Outcome {
    fn from_str(c: &str) -> Self {
        match c {
            "X" => Self::Loose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => todo!("spoiler: never"),
        }
    }
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

    fn from_outcome_str(line: &str) -> Self {
        let chars: Vec<&str> = line.split(" ").collect();
        let other = HandShape::from_str(chars[0]);
        let outcome = Outcome::from_str(chars[1]);
        let mine = HandShape::from_outcome(other, outcome);

        Self { mine, other }
    }

    fn outcome(&self) -> Outcome {
        match &self.mine.partial_cmp(&self.other) {
            Some(d) => match d {
                Ordering::Equal => Outcome::Draw,
                Ordering::Greater => Outcome::Win,
                Ordering::Less => Outcome::Loose,
            },
            None => todo!(),
        }
    }

    fn score(&self) -> u32 {
        self.outcome() as u32 + self.mine as u32
    }
}
#[test]
fn test_round_from_outcome_str() {
    assert_eq!(4, Round::from_outcome_str("A Y").score());
    assert_eq!(1, Round::from_outcome_str("B X").score());
}
#[test]
fn test_outcome() {
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

pub fn part_1(filepath: &str) -> Result<u32> {
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
    let actual = part_1("data/day_2/test.txt");
    assert_eq!(15, actual.unwrap());

    dbg!(part_1("data/day_2/input.txt").unwrap());
}

pub fn part_2(filepath: &str) -> Result<u32> {
    let line_iterator = read_lines::<String>(filepath)?;

    let r = line_iterator
        .map(|line| match line {
            Ok(l) => Round::from_outcome_str(&l).score(),
            _ => 0,
        })
        .sum();

    Ok(r)
}
#[test]
fn test_part2() {
    let actual = part_2("data/day_2/test.txt");
    assert_eq!(12, actual.unwrap());
    dbg!(part_2("data/day_2/input.txt").unwrap());
}
