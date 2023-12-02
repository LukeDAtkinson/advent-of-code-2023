use std::{fs::File, io};
use std::{io::BufRead, path::Path};

use nom::branch::alt;
use nom::bytes::complete::tag;

use nom::character::complete::{space1, u32};

use nom::multi::separated_list1;
use nom::sequence::{delimited, separated_pair};
use nom::{Finish, IResult};

#[derive(Debug)]
struct Pick(u32, String);

impl From<(u32, &str)> for Pick {
    fn from(value: (u32, &str)) -> Self {
        Pick(value.0, value.1.to_owned())
    }
}

impl Pick {
    fn is_valid(&self) -> bool {
        match self.1.as_str() {
            "red" => self.0 <= 12,
            "green" => self.0 <= 13,
            "blue" => self.0 <= 14,
            _ => false,
        }
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn new(id: u32, rounds: Vec<Round>) -> Self {
        Self { id, rounds }
    }

    fn score(&self) -> Option<u32> {
        if self.is_valid() {
            Some(self.id)
        } else {
            None
        }
    }
    fn is_valid(&self) -> bool {
        self.rounds.iter().all(|round| round.is_valid())
    }
}

#[derive(Debug)]
struct Round {
    picks: Vec<Pick>,
}

impl Round {
    fn new(picks: Vec<Pick>) -> Self {
        Self { picks }
    }
    fn is_valid(&self) -> bool {
        self.picks.iter().all(|pick| pick.is_valid())
    }
}

fn main() {
    let score: u32 = read_lines("day02p1/input.txt")
        .unwrap()
        .map(|line| parse_line(&line.unwrap()).unwrap().1)
        .filter_map(|game| game.score())
        .sum();

    println!("Score: {}", score);
}

fn parse_line(line: &str) -> IResult<&str, Game> {
    let (remaining, game_id) = parse_game_id(line)?;
    let (remaining, rounds) = parse_rounds(remaining)?;
    Ok((remaining, Game::new(game_id, rounds))).finish()
}

fn parse_game_id(line: &str) -> IResult<&str, u32> {
    delimited(tag("Game "), u32, tag(": "))(line)
}

fn parse_rounds(line: &str) -> IResult<&str, Vec<Round>> {
    separated_list1(tag("; "), parse_round)(line)
}
fn parse_round(line: &str) -> IResult<&str, Round> {
    let (remaining, picks) = separated_list1(tag(", "), parse_pick)(line)?;
    Ok((remaining, Round::new(picks)))
}

fn parse_pick(line: &str) -> IResult<&str, Pick> {
    let (remaining, count_colour) = separated_pair(u32, space1, parse_colour)(line)?;
    Ok((remaining, count_colour.into()))
}
fn parse_colour(line: &str) -> IResult<&str, &str> {
    alt((tag("red"), tag("blue"), tag("green")))(line)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
