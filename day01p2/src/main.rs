use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

use regex::Regex;

fn main() -> Result<(), String> {
    let lines = read_lines("../input.txt").map_err(|e| e.to_string())?;
    let v: u32 = lines.map(|l| get_line_value(l.unwrap().as_str()).unwrap()).sum();
    println!("{}", v);
    Ok(())
}


fn get_line_value(line: &str) -> Result<u32, String> {
    let re = Regex::new(r"([1-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let reverse_re = Regex::new(r"([1-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();
    let first = re.find(line).expect("Failed to get first match");
    let reverse_line = line.chars().rev().collect::<String>();
    let last = reverse_re.find(reverse_line.as_str()).expect("Failed to get second match");

    let f = match_to_number(first.as_str())?;
    let l = match_to_number(reverse(last.as_str()).as_str())?;
    Ok(10 * f + l)
}

fn reverse(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

fn match_to_number(m: &str) -> Result<u32, String> {
    match m {
        "0" | "zero" => Ok(0),
        "1" | "one" => Ok(1),
        "2" | "two" => Ok(2),
        "3" | "three" => Ok(3),
        "4" | "four" => Ok(4),
        "5" | "five" => Ok(5),
        "6" | "six" => Ok(6),
        "7" | "seven" => Ok(7),
        "8" | "eight" => Ok(8),
        "9" | "nine" => Ok(9),
        _ => Err(String::from("Failed to convert match to number"))
    }
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
