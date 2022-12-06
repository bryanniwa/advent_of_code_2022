use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, io};

use day2::{part_one, part_two};

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

    let my_score = part_one(&lines);
    println!("My score: {}", my_score);

    let second_score = part_two(&lines);
    println!("Second score: {}", second_score);

    Ok(())
}
