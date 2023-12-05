use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use nom::{
    bytes::complete::tag,
    character::complete::{space0, space1, u32},
    multi::separated_list1,
    sequence::{delimited, preceded},
    Finish, IResult,
};

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    revealed_numbers: HashSet<u32>,
}
impl Card {
    fn new(id: u32, winning_numbers: HashSet<u32>, revealed_numbers: HashSet<u32>) -> Self {
        Self {
            id,
            winning_numbers,
            revealed_numbers,
        }
    }

    fn score(&self) -> usize {
        let matching = self.winning_numbers.intersection(&self.revealed_numbers);
        let count = matching.count() as u32;
        let result = match count {
            0 => 0,
            _ => usize::pow(2, count - 1),
        };

        println!(
            "Card {:?} had {:?} matching numbers for a score of {:?}",
            self.id, count, result
        );

        result
    }
}

fn main() {
    let lines = read_lines("day04p1/input.txt").unwrap();
    let score: usize = lines
        .map(|line| line.unwrap())
        .map(|line| parse_line(line.as_str()).unwrap().1)
        .map(|card| card.score())
        .sum();
    println!("Score: {:?}", score);
}

fn parse_line(line: &str) -> IResult<&str, Card> {
    let (remaining, id) = preceded(tag("Card"), delimited(space1, u32, tag(": ")))(line)?;
    let (remaining, winning_numbers) =
        delimited(space0, separated_list1(space1, u32), tag(" | "))(remaining)?;
    let (remaining, revealed_numbers) =
        delimited(space0, separated_list1(space1, u32), space0)(remaining)?;

    let card = Card::new(
        id,
        winning_numbers.into_iter().collect(),
        revealed_numbers.into_iter().collect(),
    );

    println!("Card: {:?}", card);

    Ok((remaining, card)).finish()
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
