use std::{
    collections::{HashMap, HashSet},
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

    fn matching(&self) -> u32 {
        let matching = self.winning_numbers.intersection(&self.revealed_numbers);
        matching.count() as u32
    }
}

fn main() {
    let lines = read_lines("day04p1/input.txt").unwrap();
    let mut card_counts: HashMap<u32, u32> = HashMap::new();
    lines
        .map(|line| line.unwrap())
        .map(|line| parse_line(line.as_str()).unwrap().1)
        .for_each(|card| {
            // add 1 to the count of cards for the initial card we had
            let current_card_count = *card_counts
                .entry(card.id)
                .and_modify(|id| *id += 1)
                .or_insert(1);

            let matching = card.matching();

            // Iterate over the next <matching> card IDs
            for i in 1..=matching {
                card_counts
                    .entry(card.id + i)
                    // For each of the card IDs add 1 extra card for each of the current card ID
                    .and_modify(|id| *id += &current_card_count)
                    // Or insert that many if we don't have any yet (remembering that we will add
                    // the initial one to the count when we get that far in the outer loop)
                    .or_insert(current_card_count);
            }
        });

    let total_cards: u32 = card_counts.values().sum();
    println!("Score: {:?}", total_cards);
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
