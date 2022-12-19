use std::{str::FromStr, cmp::Ordering};

use anyhow::Result;

#[derive(PartialEq)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissor),
            _ => Err(String::from("cannot parse this string to a hand"))
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == &Hand::Rock && other == &Hand::Scissor {
            Some(Ordering::Greater)
        } else if self == &Hand::Scissor && other == &Hand::Rock {
            Some(Ordering::Less)
        } else {
            Some((*self as u8).cmp(&(*other as u8)))
        }
    }
}

enum Outcome {
    Win,
    Draw,
    Lose,
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(String::from("cannot parse this string to an outcome")),
        }
    }
}

fn get_result(hand1: &Hand, hand2: &Hand) -> u32 {
    match hand1.partial_cmp(hand2).unwrap() {
        Ordering::Less => 6 + *hand2 as u32, 
        Ordering::Greater => 0 + *hand2 as u32,
        Ordering::Equal => 3 + *hand2 as u32,
    }
}

fn part1() -> Result<u32> {
    Ok(
        aoc::read_file_as_lines::<String>("day2")?
            .into_iter()
            .map(|line| {
                let hands: Vec<Hand> = line
                    .split(" ")
                    .map(|s| s.parse::<Hand>().unwrap())
                    .collect();

                get_result(&hands[0], &hands[1])
            })
            .sum()
    )
}

fn part2() -> Result<u32> {
    Ok(
        aoc::read_file_as_lines::<String>("day2")?
            .into_iter()
            .map(|line| {
                let strs: Vec<&str> = line.split(" ").collect();

                let elf_hand = strs[0].parse::<Hand>().unwrap();
                let outcome = strs[1].parse::<Outcome>().unwrap();

                match outcome {
                    Outcome::Win => {
                        let my_hand = match elf_hand {
                            Hand::Rock => Hand::Paper,
                            Hand::Paper => Hand::Scissor,
                            Hand::Scissor => Hand::Rock,
                        };
                        6 + my_hand as u32
                    },
                    Outcome::Draw => 3 + elf_hand as u32,
                    Outcome::Lose => {
                        let my_hand = match elf_hand {
                            Hand::Rock => Hand::Scissor,
                            Hand::Paper => Hand::Rock,
                            Hand::Scissor => Hand::Paper,
                        };
                        0 + my_hand as u32
                    },
                }
            })
            .sum()
    )
}

fn main() -> Result<()> {
    println!("Part 1 result: {}", part1()?);
    println!("Part 2 result: {}", part2()?);

    Ok(())
}
