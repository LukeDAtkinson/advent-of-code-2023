use std::cmp::max;
use std::collections::HashMap;
use std::{fs::File, io};
use std::{io::BufRead, path::Path};

use nom::branch::alt;
use nom::bytes::complete::tag;

use nom::character::complete::{space1, u32};

use nom::multi::separated_list1;
use nom::sequence::{delimited, separated_pair};
use nom::{Finish, IResult};

#[derive(Debug)]
struct Game {
    rounds: Vec<Round>,
}

impl Game {
    fn new(rounds: Vec<Round>) -> Self {
        Self { rounds }
    }

    fn score(self) -> u32 {
        let collected: HashMap<String, u32> = self
            .rounds
            .into_iter()
            .flat_map(|r| r.picks.into_iter())
            .fold(HashMap::new(), |mut acc, i| {
                acc.entry(i.0)
                    .and_modify(|e| *e = max(*e, i.1))
                    .or_insert(i.1);
                acc
            });
        println!("{:?}", collected);
        collected.values().product()
    }
}

#[derive(Debug)]
struct Round {
    picks: Vec<(String, u32)>,
}

impl Round {
    fn new(picks: Vec<(String, u32)>) -> Self {
        Self { picks }
    }
}

fn main() {
    let score: u32 = read_lines("day02p2/input.txt")
        .unwrap()
        .map(|line| parse_line(&line.unwrap()).unwrap().1)
        .map(|game| game.score())
        .sum();

    println!("Score: {}", score);
}

fn parse_line(line: &str) -> IResult<&str, Game> {
    let (remaining, _) = parse_game_id(line)?;
    let (remaining, rounds) = parse_rounds(remaining)?;
    Ok((remaining, Game::new(rounds))).finish()
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

fn parse_pick(line: &str) -> IResult<&str, (String, u32)> {
    let (remaining, (count, colour)) = separated_pair(u32, space1, parse_colour)(line)?;
    Ok((remaining, (colour.to_owned(), count)))
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
