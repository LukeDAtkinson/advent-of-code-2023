use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() -> Result<(), String> {
    let lines = read_lines("input.txt").map_err(|e| e.to_string())?;
    let v: u32 = lines.map(|l| get_line_value(l.unwrap())).sum();
    println!("{}", v);
    Ok(())
}

fn get_line_value(line: String) -> u32 {
    let digits: Vec<u32> = line.chars().filter_map(|l| l.to_digit(10)).collect();

    10*digits.first().unwrap() + digits.last().unwrap()
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
