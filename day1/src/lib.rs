use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub fn single_elf(reader: BufReader<File>) -> Result<u32> {
    let mut total = 0;
    let mut largest = 0;
    for line in reader.lines() {
        let trimmed = line?;
        let trimmed = trimmed.trim();
        if trimmed.len() > 1 {
            total += trimmed.parse::<u32>().unwrap();
        } else {
            if total > largest {
                largest = total;
            }
            total = 0;
        }
    }

    Ok(largest)
}

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn top_three(reader: BufReader<File>) -> Result<u32> {
    let mut total = 0;
    let mut top_elves: BinaryHeap<Reverse<_>> = vec![0, 0, 0].into_iter().map(Reverse).collect();

    for line in reader.lines() {
        let line = line?;
        if line.len() > 1 {
            total += line.parse::<u32>().unwrap();
        } else {
            if let Some(Reverse(v)) = top_elves.peek() {
                if total > *v {
                    top_elves.pop();
                    top_elves.push(Reverse(total))
                }
            } else {
                top_elves.push(Reverse(total))
            }
            total = 0;
        }
    }

    println!("size: {}", top_elves.len());

    let mut grand_total = 0;
    while let Some(Reverse(v)) = top_elves.pop() {
        grand_total += v;
    }

    Ok(grand_total)
}
