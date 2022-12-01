use std::{env, io};
use std::fs::File;
use std::io::{BufReader, BufRead};
use day1::{single_elf, top_three};

fn get_input() -> io::Result<Vec<String>> {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];
    let f = File::open(path)?;
    let reader = BufReader::new(f);

    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?)
    }

    Ok(lines)
}

fn main() -> io::Result<()> {
    let lines = get_input()?;

    let single_largest = single_elf(&lines).unwrap();
    println!("single_largest: {single_largest}");

    let top_three = top_three(&lines).unwrap();
    println!("top_three total: {}", top_three);

    Ok(())
}
