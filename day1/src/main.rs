use std::{env, io};
use std::fs::File;
use std::io::{BufReader, BufRead};
use day1::{single_elf, top_three};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];
    let f = File::open(path)?;
    let reader = BufReader::new(f);

    // let single_largest = single_elf(lines).unwrap();
    // println!("single_largest: {single_largest}");

    let top_three = top_three(reader).unwrap();
    println!("top_three total: {}", top_three);

    Ok(())
}
